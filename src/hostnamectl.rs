use std::process::Command;

use anyhow::{anyhow, Result};
use serde::Deserialize;
use serde_json::from_slice;

#[derive(Debug, Clone, Deserialize)]
pub struct Host {
    #[serde(rename = "Hostname")]
    hostname: String,
    #[serde(rename = "OperatingSystemPrettyName")]
    operating_system_pretty_name: String,
    #[serde(rename = "KernelRelease")]
    kernel_release: String,
    #[serde(rename = "HardwareVendor")]
    hardware_vendor: String,
    #[serde(rename = "HardwareModel")]
    hardware_model: String,
}

impl Host {
    pub fn new() -> Result<Self> {
        let hostnamtctl = Command::new("hostnamectl").arg("--json=short").output()?;
        if !hostnamtctl.status.success() {
            return Err(anyhow!("Failed to spawn hostnamectl"));
        }
        let host = from_slice::<Host>(&hostnamtctl.stdout)?;
        return Ok(host);
    }

    pub fn hostname(self) -> String {
        self.hostname
    }
    pub fn kernel_release(self) -> String {
        self.kernel_release
    }

    pub fn hardware_vendor(self) -> String {
        self.hardware_vendor
    }

    pub fn hardware_model(self) -> String {
        self.hardware_model
    }

    pub fn operating_system_pretty_name(self) -> String {
        self.operating_system_pretty_name
    }
}
