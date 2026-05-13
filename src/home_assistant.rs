use crate::prelude::*;
use crate::eg4::packet::Register;

use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Availability {
    topic: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct Device {
    manufacturer: String,
    name: String,
    identifiers: [String; 1],
}

pub struct Config {
    inverter: config::Inverter,
    mqtt_config: config::Mqtt,
    global_config: config::ConfigWrapper,
}

// https://www.home-assistant.io/integrations/switch.mqtt/
#[derive(Debug, Serialize)]
pub struct Switch {
    name: String,
    state_topic: String,
    command_topic: String,
    value_template: String,
    unique_id: String,
    device: Device,
    availability: Availability,
}

// https://www.home-assistant.io/integrations/number.mqtt/
#[derive(Debug, Serialize)]
pub struct Number {
    name: String,
    state_topic: String,
    command_topic: String,
    value_template: String,
    unique_id: String,
    device: Device,
    availability: Availability,
    min: f64,
    max: f64,
    step: f64,
    unit_of_measurement: String,
}

// https://www.home-assistant.io/integrations/text.mqtt/
#[derive(Debug, Serialize)]
pub struct Text {
    name: String,
    state_topic: String,
    command_topic: String,
    command_template: String,
    value_template: String,
    unique_id: String,
    device: Device,
    availability: Availability,
    pattern: String,
}

impl Config {
    pub fn new(inverter: &config::Inverter, mqtt_config: &config::Mqtt, global_config: &config::ConfigWrapper) -> Self {
        Self {
            inverter: inverter.clone(),
            mqtt_config: mqtt_config.clone(),
            global_config: global_config.clone(),
        }
    }

    pub fn sensors(&self) -> Vec<mqtt::Message> {
        use crate::eg4::register_metadata::{
            ha_device_class, ha_state_class, ha_unit, RegisterCatalog,
        };

        let catalog = RegisterCatalog::instance();
        let registers = if self.mqtt_config.homeassistant().publish_all_registers() {
            catalog.all_input_sorted()
        } else {
            catalog.curated_input()
        };

        let datalog_str = self
            .inverter
            .datalog()
            .map(|s| s.to_string())
            .unwrap_or_default();
        let state_topic = format!(
            "{}/{}/inputs/all",
            self.mqtt_config.namespace(),
            datalog_str
        );

        let mut messages = Vec::new();

        for meta in registers {
            // battery_status is exploded into soc + soh derived keys in the
            // snapshot; emit those two sensors instead of the packed register.
            if meta.shortname == "battery_status" {
                messages.push(self.build_register_sensor(
                    &state_topic,
                    "soc",
                    "State of Charge",
                    Some("battery"),
                    Some("measurement"),
                    Some("%"),
                    "{{ value_json.soc }}",
                ));
                messages.push(self.build_register_sensor(
                    &state_topic,
                    "soh",
                    "State of Health",
                    Some("battery"),
                    Some("measurement"),
                    Some("%"),
                    "{{ value_json.soh }}",
                ));
                continue;
            }

            let value_template = if (meta.unit_scale - 1.0).abs() < f64::EPSILON {
                format!("{{{{ value_json.{} }}}}", meta.shortname)
            } else {
                let precision = Self::scale_precision(meta.unit_scale);
                format!(
                    "{{{{ (value_json.{} * {}) | round({}) }}}}",
                    meta.shortname, meta.unit_scale, precision
                )
            };

            messages.push(self.build_register_sensor(
                &state_topic,
                &meta.shortname,
                &meta.name,
                ha_device_class(meta),
                ha_state_class(meta),
                ha_unit(meta),
                &value_template,
            ));
        }

        messages
    }

    fn build_register_sensor(
        &self,
        state_topic: &str,
        key: &str,
        name: &str,
        device_class: Option<&'static str>,
        state_class: Option<&'static str>,
        unit: Option<&str>,
        value_template: &str,
    ) -> mqtt::Message {
        let datalog_str = self
            .inverter
            .datalog()
            .map(|s| s.to_string())
            .unwrap_or_default();
        let unique_id = format!("lxp_{}_{}", datalog_str, key);
        let availability_topic = format!("{}/LWT", self.mqtt_config.namespace());
        let device_id = format!("lxp_{}", datalog_str);

        let mut config = serde_json::json!({
            "unique_id": unique_id,
            "name": name,
            "state_topic": state_topic,
            "value_template": value_template,
            "device": {
                "manufacturer": "EG4",
                "name": device_id,
                "identifiers": [device_id],
            },
            "availability": { "topic": availability_topic },
        });

        if let Some(dc) = device_class {
            config["device_class"] = serde_json::Value::String(dc.to_string());
        }
        if let Some(sc) = state_class {
            config["state_class"] = serde_json::Value::String(sc.to_string());
        }
        if let Some(u) = unit {
            config["unit_of_measurement"] = serde_json::Value::String(u.to_string());
        }

        mqtt::Message {
            topic: self.ha_discovery_topic("sensor", key),
            retain: true,
            payload: serde_json::to_string(&config).unwrap(),
        }
    }

    fn scale_precision(scale: f64) -> u8 {
        if scale >= 1.0 {
            0
        } else if scale >= 0.1 {
            1
        } else if scale >= 0.01 {
            2
        } else {
            3
        }
    }


    pub fn all(&self) -> Result<Vec<mqtt::Message>> {
        if !self.global_config.homeassistant_enabled() {
            return Ok(Vec::new());
        }

        let mut r = vec![
            self.switch("ac_charge", "AC Charge")?,
            self.switch("charge_priority", "Charge Priority")?,
            self.switch("forced_discharge", "Forced Discharge")?,
            self.number_percent(Register::ChargePowerPercentCmd, "System Charge Rate (%)")?,
            self.number_percent(Register::DischgPowerPercentCmd, "System Discharge Rate (%)")?,
            self.number_percent(Register::AcChargePowerCmd, "AC Charge Rate (%)")?,
            self.number_percent(Register::AcChargeSocLimit, "AC Charge Limit %")?,
            self.number_percent(Register::ChargePriorityPowerCmd, "Charge Priority Rate (%)")?,
            self.number_percent(Register::ChargePrioritySocLimit, "Charge Priority Limit %")?,
            self.number_percent(Register::ForcedDischgSocLimit, "Forced Discharge Limit %")?,
            self.number_percent(Register::DischgCutOffSocEod, "Discharge Cutoff %")?,
            self.number_percent(
                Register::EpsDischgCutoffSocEod,
                "Discharge Cutoff for EPS %",
            )?,
            self.number_percent(
                Register::AcChargeStartSocLimit,
                "Charge From AC Lower Limit %",
            )?,
            self.number_percent(
                Register::AcChargeEndSocLimit,
                "Charge From AC Upper Limit %",
            )?,
            self.time_range("ac_charge/1", "AC Charge Timeslot 1")?,
            self.time_range("ac_charge/2", "AC Charge Timeslot 2")?,
            self.time_range("ac_charge/3", "AC Charge Timeslot 3")?,
            self.time_range("ac_first/1", "AC First Timeslot 1")?,
            self.time_range("ac_first/2", "AC First Timeslot 2")?,
            self.time_range("ac_first/3", "AC First Timeslot 3")?,
            self.time_range("charge_priority/1", "Charge Priority Timeslot 1")?,
            self.time_range("charge_priority/2", "Charge Priority Timeslot 2")?,
            self.time_range("charge_priority/3", "Charge Priority Timeslot 3")?,
            self.time_range("forced_discharge/1", "Forced Discharge Timeslot 1")?,
            self.time_range("forced_discharge/2", "Forced Discharge Timeslot 2")?,
            self.time_range("forced_discharge/3", "Forced Discharge Timeslot 3")?,
        ];

        r.append(&mut self.sensors());

        Ok(r)
    }

    fn ha_discovery_topic(&self, kind: &str, name: &str) -> String {
        format!(
            "{}/{}/lxp_{}/{}/config",
            self.mqtt_config.homeassistant().prefix(),
            kind,
            self.inverter.datalog().map(|s| s.to_string()).unwrap_or_default(),
            // The forward slash is used in some names (e.g. ac_charge/1) but
            // has semantic meaning in MQTT, so must be changed
            name.replace('/', "_"),
        )
    }

    fn switch(&self, name: &str, label: &str) -> Result<mqtt::Message> {
        let config = Switch {
            value_template: format!("{{{{ value_json.{}_en }}}}", name),
            state_topic: format!(
                "{}/{}/hold/21/bits",
                self.mqtt_config.namespace(),
                self.inverter.datalog().map(|s| s.to_string()).unwrap_or_default()
            ),
            command_topic: format!(
                "{}/cmd/{}/set/{}",
                self.mqtt_config.namespace(),
                self.inverter.datalog().map(|s| s.to_string()).unwrap_or_default(),
                name
            ),
            unique_id: format!("lxp_{}_{}", self.inverter.datalog().map(|s| s.to_string()).unwrap_or_default(), name),
            name: label.to_string(),
            device: self.device(),
            availability: self.availability(),
        };

        Ok(mqtt::Message {
            topic: self.ha_discovery_topic("switch", name),
            retain: true,
            payload: serde_json::to_string(&config)?,
        })
    }

    fn number_percent(&self, register: Register, label: &str) -> Result<mqtt::Message> {
        let config = Number {
            name: label.to_string(),
            state_topic: format!(
                "{}/{}/hold/{}",
                self.mqtt_config.namespace(),
                self.inverter.datalog().map(|s| s.to_string()).unwrap_or_default(),
                register as u16,
            ),
            command_topic: format!(
                "{}/cmd/{}/set/hold/{}",
                self.mqtt_config.namespace(),
                self.inverter.datalog().map(|s| s.to_string()).unwrap_or_default(),
                register as u16,
            ),
            value_template: "{{ float(value) }}".to_string(),
            unique_id: format!("lxp_{}_number_{:?}", self.inverter.datalog().map(|s| s.to_string()).unwrap_or_default(), register),
            device: self.device(),
            availability: self.availability(),
            min: 0.0,
            max: 100.0,
            step: 1.0,
            unit_of_measurement: "%".to_string(),
        };

        Ok(mqtt::Message {
            topic: self.ha_discovery_topic("number", &format!("{:?}", register)),
            retain: true,
            payload: serde_json::to_string(&config)?,
        })
    }

    // Models a time range as an MQTT Text field taking values like: 00:00-23:59
    fn time_range(&self, name: &str, label: &str) -> Result<mqtt::Message> {
        let config = Text {
            name: label.to_string(),
            state_topic: format!(
                "{}/{}/{}",
                self.mqtt_config.namespace(),
                self.inverter.datalog().map(|s| s.to_string()).unwrap_or_default(),
                name,
            ),
            command_topic: format!(
                "{}/cmd/{}/set/{}",
                self.mqtt_config.namespace(),
                self.inverter.datalog().map(|s| s.to_string()).unwrap_or_default(),
                name,
            ),
            command_template: r#"{% set parts = value.split("-") %}{"start":"{{ parts[0] }}", "end":"{{ parts[1] }}"}"#.to_string(),
            value_template: r#"{{ value_json["start"] }}-{{ value_json["end"] }}"#.to_string(),
            unique_id: format!("lxp_{}_text_{}", self.inverter.datalog().map(|s| s.to_string()).unwrap_or_default(), name),
            device: self.device(),
            availability: self.availability(),
            pattern: r"([01]?[0-9]|2[0-3]):[0-5][0-9]-([01]?[0-9]|2[0-3]):[0-5][0-9]".to_string(),
        };

        Ok(mqtt::Message {
            topic: self.ha_discovery_topic("text", name),
            retain: true,
            payload: serde_json::to_string(&config)?,
        })
    }

    fn device(&self) -> Device {
        Device {
            identifiers: [format!("lxp_{}", self.inverter.datalog().map(|s| s.to_string()).unwrap_or_default())],
            manufacturer: "LuxPower".to_owned(),
            name: format!("lxp_{}", self.inverter.datalog().map(|s| s.to_string()).unwrap_or_default()),
        }
    }

    fn availability(&self) -> Availability {
        Availability {
            topic: format!("{}/LWT", self.mqtt_config.namespace()),
        }
    }
}
