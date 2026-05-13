use serde::Deserialize;
use std::collections::HashMap;
use std::sync::OnceLock;

const REGISTERS_JSON: &str = include_str!("../../doc/eg4_registers.json");

#[derive(Debug, Deserialize)]
struct RegistersFile {
    registers: Vec<RegisterGroup>,
}

#[derive(Debug, Deserialize)]
struct RegisterGroup {
    register_type: String,
    register_map: Vec<RegisterMeta>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RegisterMeta {
    pub register_number: u16,
    pub name: String,
    pub shortname: String,
    pub datatype: String,
    #[serde(default)]
    pub unit: String,
    #[serde(default = "default_scale")]
    pub unit_scale: f64,
    #[serde(default)]
    pub description: String,
}

fn default_scale() -> f64 {
    1.0
}

pub struct RegisterCatalog {
    input: HashMap<u16, RegisterMeta>,
}

impl RegisterCatalog {
    pub fn instance() -> &'static RegisterCatalog {
        static INSTANCE: OnceLock<RegisterCatalog> = OnceLock::new();
        INSTANCE.get_or_init(RegisterCatalog::load)
    }

    fn load() -> RegisterCatalog {
        let parsed: RegistersFile =
            serde_json::from_str(REGISTERS_JSON).expect("doc/eg4_registers.json is malformed");
        let mut input = HashMap::new();
        for group in parsed.registers {
            if group.register_type == "input" {
                for meta in group.register_map {
                    input.insert(meta.register_number, meta);
                }
            }
        }
        RegisterCatalog { input }
    }

    pub fn input(&self, register: u16) -> Option<&RegisterMeta> {
        self.input.get(&register)
    }

    pub fn input_key(&self, register: u16) -> String {
        match self.input.get(&register) {
            Some(meta) => meta.shortname.clone(),
            None => format!("r_{}", register),
        }
    }

    pub fn curated_input(&self) -> Vec<&RegisterMeta> {
        CURATED_INPUT_REGISTERS
            .iter()
            .filter_map(|n| self.input.get(n))
            .collect()
    }

    pub fn all_input_sorted(&self) -> Vec<&RegisterMeta> {
        let mut metas: Vec<&RegisterMeta> = self.input.values().collect();
        metas.sort_by_key(|m| m.register_number);
        metas
    }
}

const CURATED_INPUT_REGISTERS: &[u16] = &[
    0,  // inverter_status
    1,  // pv1_voltage
    2,  // pv2_voltage
    3,  // pv3_voltage
    4,  // battery_voltage
    5,  // battery_status (soc/soh packed)
    7,  // pv1_power
    8,  // pv2_power
    9,  // pv3_power
    10, // charge_power
    11, // discharge_power
    12, // grid_voltage_r
    15, // grid_frequency
    16, // output_power
    26, // grid_export_power
    27, // grid_import_power
    28, // pv1_energy_today
    29, // pv2_energy_today
    30, // pv3_energy_today
    33, // battery_charge_energy_today
    34, // battery_discharge_energy_today
    36, // grid_export_energy_today
    37, // grid_import_energy_today
    64, // internal_temp
    65, // radiator_temp_1
    67, // battery_temp
];

pub fn ha_device_class(meta: &RegisterMeta) -> Option<&'static str> {
    match meta.unit.as_str() {
        "V" => Some("voltage"),
        "A" => Some("current"),
        "W" => Some("power"),
        "VA" => Some("apparent_power"),
        "VAR" => Some("reactive_power"),
        "Hz" => Some("frequency"),
        "kWh" => Some("energy"),
        "°C" => Some("temperature"),
        "%" if meta.shortname == "battery_status" => Some("battery"),
        _ => None,
    }
}

pub fn ha_state_class(meta: &RegisterMeta) -> Option<&'static str> {
    if meta.unit == "kWh" {
        Some("total_increasing")
    } else if ha_device_class(meta).is_some() {
        Some("measurement")
    } else {
        None
    }
}

pub fn ha_unit(meta: &RegisterMeta) -> Option<&str> {
    if meta.unit.is_empty() {
        None
    } else {
        Some(meta.unit.as_str())
    }
}

pub fn signed_value(meta: &RegisterMeta, raw: u16) -> i32 {
    if meta.datatype == "int16" {
        (raw as i16) as i32
    } else {
        raw as i32
    }
}
