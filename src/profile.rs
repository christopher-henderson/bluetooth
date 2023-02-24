use serde::{Deserialize, Serialize};

use crate::bluetoothctl::Pairing;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Profile {
    name: String,
    #[serde(flatten)]
    pairing: Pairing,
}

impl Profile {
    pub fn connect(&self) {
        self.pairing.connect();
    }

    pub fn disconnect(&self) {
        self.pairing.disconnect();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileConfig {
    profile: Vec<Profile>,
}

impl ProfileConfig {
    pub fn load(name: &str) -> Profile {
        let profiles = std::fs::read_to_string(
            std::env::current_exe()
                .unwrap()
                .parent()
                .unwrap()
                .join("profiles.toml"),
        )
        .unwrap();
        let profiles: ProfileConfig = toml::from_str(&profiles).unwrap();
        for profile in profiles.profile {
            if profile.name.to_lowercase() == name.to_lowercase() {
                return profile;
            }
        }
        panic!(r#"Profile "{name}" not found"#)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_toml() {
        let config: &str = r#"
[[profile]]
name = "Keyboard"
adapter = "5C:F3:70:A8:B2:9F"
device = "DC:2C:26:00:11:EF"

[[profile]]
name = "Echo"
adapter = "5C:F3:70:A8:B2:A5"
device = "F8:0F:F9:9C:E2:D4"
        "#;
        let profiles: ProfileConfig = toml::from_str(config).unwrap();
        println!("{profiles:?}")
    }
}
