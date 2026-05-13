use crate::eg4::register_metadata::{self, RegisterCatalog};
use crate::prelude::*;
use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex};

// this just needs to be bigger than the max register we'll see
const REGISTER_COUNT: usize = 512;

#[derive(Clone, Debug)]
pub enum ChannelData {
    ReadRegister(u16, Arc<Mutex<Option<oneshot::Sender<u16>>>>),
    RegisterData(u16, u16),
    Shutdown,
}

pub struct RegisterCache {
    channels: Channels,
    register_data: Arc<Mutex<[u16; REGISTER_COUNT]>>,
    input_cache: Arc<Mutex<HashMap<Serial, BTreeMap<u16, u16>>>>,
}

impl RegisterCache {
    pub fn new(channels: Channels) -> Self {
        let register_data = Arc::new(Mutex::new([0; REGISTER_COUNT]));

        Self {
            channels,
            register_data,
            input_cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn start(&self) -> Result<()> {
        futures::try_join!(self.cache_getter(), self.cache_setter())?;

        Ok(())
    }

    // external helper method to simplify access to the cache, use like so:
    //
    //   RegisterCache::get(&self.channels, 1);
    //
    pub async fn get(channels: &Channels, register: u16) -> u16 {
        let (tx, rx) = oneshot::channel();
        let tx = Arc::new(Mutex::new(Some(tx)));
        let channel_data = ChannelData::ReadRegister(register, tx);
        debug!("Reading register {} from cache", register);
        let _ = channels.read_register_cache.send(channel_data);
        rx.await
            .expect("unexpected error reading from register cache")
    }

    // Record a batch of input-register values for a specific datalog. Used to back
    // the `inputs/all` MQTT snapshot for inverters whose ReadInput packets don't
    // fit the upstream ReadInputAll byte layout.
    pub fn record_input(&self, datalog: Serial, register: u16, values: &[u16]) {
        let mut cache = self.input_cache.lock().unwrap();
        let map = cache.entry(datalog).or_insert_with(BTreeMap::new);
        for (i, value) in values.iter().enumerate() {
            map.insert(register + i as u16, *value);
        }
    }

    // Build a JSON snapshot of all input registers seen so far for the given
    // datalog. Keys use the register shortname from doc/eg4_registers.json when
    // known, falling back to `r_<N>`. Signed-typed registers are converted to
    // their signed representation; everything else is published as raw u16.
    pub fn input_snapshot(&self, datalog: &Serial) -> Option<serde_json::Value> {
        let cache = self.input_cache.lock().unwrap();
        let map = cache.get(datalog)?;
        let catalog = RegisterCatalog::instance();
        let mut out = serde_json::Map::with_capacity(map.len() + 8);
        for (&register, &raw) in map.iter() {
            let key = catalog.input_key(register);
            let value = match catalog.input(register) {
                Some(meta) => register_metadata::signed_value(meta, raw),
                None => raw as i32,
            };
            out.insert(key, serde_json::Value::from(value));
        }

        // battery_status (reg 5) packs SOC (low byte) and SOH (high byte).
        if let Some(&raw) = map.get(&5) {
            out.insert("soc".to_string(), serde_json::Value::from(raw & 0xff));
            out.insert("soh".to_string(), serde_json::Value::from((raw >> 8) & 0xff));
        }
        // Combine low/high register pairs into u32 convenience keys.
        for (lo_reg, hi_reg, combined_key) in [
            (60_u16, 61_u16, "fault_code"),
            (62, 63, "warning_code"),
            (69, 70, "runtime"),
        ] {
            if let (Some(&lo), Some(&hi)) = (map.get(&lo_reg), map.get(&hi_reg)) {
                let combined = (lo as u32) | ((hi as u32) << 16);
                out.insert(combined_key.to_string(), serde_json::Value::from(combined));
            }
        }
        Some(serde_json::Value::Object(out))
    }

    async fn cache_getter(&self) -> Result<()> {
        let mut receiver = self.channels.read_register_cache.subscribe();

        debug!("register_cache getter starting");

        while let Ok(data) = receiver.recv().await {
            match data {
                ChannelData::ReadRegister(register, tx) => {
                    let value = self.register_data.lock().unwrap()[register as usize];
                    debug!("Cache hit for register {}: value = {}", register, value);
                    if let Ok(mut tx) = tx.lock() {
                        if let Some(tx) = tx.take() {
                            let _ = tx.send(value);
                        }
                    }
                }
                ChannelData::Shutdown => break,
                _ => (),
            }
        }

        Ok(())
    }

    async fn cache_setter(&self) -> Result<()> {
        let mut receiver = self.channels.to_register_cache.subscribe();

        debug!("register_cache setter starting");

        while let Ok(data) = receiver.recv().await {
            match data {
                ChannelData::RegisterData(register, value) => {
                    debug!("Caching register {} with value {}", register, value);
                    self.register_data.lock().unwrap()[register as usize] = value;
                }
                ChannelData::Shutdown => break,
                _ => (),
            }
        }

        Ok(())
    }
}
