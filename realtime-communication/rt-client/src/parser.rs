use alloc::string::String;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct WifiConfiguration {
    ssid: String,
    password: String,
}

fn get_wifi_configuration(raw_string: String) -> Option<WifiConfiguration> {
    toml::from_str::<WifiConfiguration>(&raw_string).ok()
}
