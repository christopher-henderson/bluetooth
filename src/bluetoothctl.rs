use clap::Args;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::process::{Child, Stdio};

#[derive(Args, Debug, Clone, Serialize, Deserialize)]
pub struct Pairing {
    adapter: String,
    device: String,
}

impl Pairing {
    pub fn connect(&self) {
        let mut child = Self::make_process();
        let device = &self.adapter;
        let target = &self.device;
        child
            .stdin
            .as_mut()
            .unwrap()
            .write_all(format!("select {device}\nconnect {target}\nexit\n").as_bytes())
            .unwrap();
        child.wait().unwrap();
    }

    pub fn disconnect(&self) {
        let mut child = Self::make_process();
        let device = &self.adapter;
        let target = &self.device;
        child
            .stdin
            .as_mut()
            .unwrap()
            .write_all(format!("select {device}\ndisconnect {target}\nexit\n").as_bytes())
            .unwrap();
        child.wait().unwrap();
    }

    fn make_process() -> Child {
        std::process::Command::new("bluetoothctl")
            .stdout(Stdio::null())
            .stdin(Stdio::piped())
            .spawn()
            .unwrap()
    }
}

impl Default for Pairing {
    fn default() -> Self {
        Self {
            adapter: DONGLE.to_string(),
            device: KEYBOARD.to_string(),
        }
    }
}

const KEYBOARD: &str = "DC:2C:26:00:11:EF";
const DONGLE: &str = "5C:F3:70:A8:B2:9F";
