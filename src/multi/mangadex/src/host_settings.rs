use crate::bindings::midoku::settings::settings::{get, Value};

pub struct HostSettings;

impl HostSettings {
    pub fn get_user_agent() -> String {
        match get("user_agent") {
            Ok(Value::String(value)) => value,
            _ => String::from("Midoku"),
        }
    }

    pub fn get_locale() -> String {
        match get("locale") {
            Ok(Value::String(value)) => value,
            _ => String::from("en"),
        }
    }

    pub fn get_languages() -> Vec<String> {
        match get("languages") {
            Ok(Value::Array(value)) => value,
            _ => vec![String::from("en")],
        }
    }

    pub fn get_force_port_443() -> bool {
        match get("force_port_443") {
            Ok(Value::Bool(value)) => value,
            _ => false,
        }
    }

    pub fn get_data_saver() -> bool {
        match get("data_saver") {
            Ok(Value::Bool(value)) => value,
            _ => false,
        }
    }
}
