use crate::bindings::midoku::settings::settings::{Number, Value};

/// Get a setting from the host.
#[doc(hidden)]
macro_rules! setting_get {
    ($key:expr) => {{
        #[cfg(not(test))]
        let setting = crate::bindings::midoku::settings::settings::get($key);
        #[cfg(test)]
        let setting = Err(());
        setting
    }};
}

pub struct HostSettings;

impl HostSettings {
    pub fn get_user_agent() -> String {
        match setting_get!("user_agent") {
            Ok(Value::String(value)) => value,
            _ => String::from("Midoku"),
        }
    }

    pub fn get_locale() -> String {
        match setting_get!("locale") {
            Ok(Value::String(value)) => value,
            _ => String::from("en"),
        }
    }

    pub fn get_languages() -> Vec<String> {
        match setting_get!("languages") {
            Ok(Value::Array(value)) => value,
            _ => vec![String::from("en")],
        }
    }

    pub fn get_cover_quality() -> u64 {
        match setting_get!("cover_quality") {
            Ok(Value::Number(Number::U64(value))) => value,
            _ => 0,
        }
    }

    pub fn get_blocked_groups() -> Vec<String> {
        match setting_get!("blocked_groups") {
            Ok(Value::Array(value)) => value,
            _ => vec![],
        }
    }

    pub fn get_blocked_uploaders() -> Vec<String> {
        match setting_get!("blocked_uploaders") {
            Ok(Value::Array(value)) => value,
            _ => vec![],
        }
    }

    pub fn get_force_port_443() -> bool {
        match setting_get!("force_port_443") {
            Ok(Value::Bool(value)) => value,
            _ => false,
        }
    }

    pub fn get_data_saver() -> bool {
        match setting_get!("data_saver") {
            Ok(Value::Bool(value)) => value,
            _ => false,
        }
    }
}
