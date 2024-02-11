use anyhow::Result;
#[cfg(feature = "systemd")]
use {anyhow::anyhow, serde::Deserialize, serde_json::from_slice, std::process::Command};

#[cfg(feature = "posix")]
use std::fs::read_to_string;

#[cfg(feature = "systemd")]
#[derive(Debug, Clone, Deserialize)]
pub struct Host {
    #[serde(rename = "Hostname")]
    hostname: String,
    #[serde(rename = "OperatingSystemPrettyName")]
    operating_system_pretty_name: String,
    #[serde(rename = "KernelRelease")]
    kernel_release: String,
    #[serde(rename = "HardwareModel")]
    hardware_model: String,
}

#[cfg(feature = "posix")]
#[derive(Debug, Clone)]
pub struct Host {
    hostname: String,
    operating_system_pretty_name: String,
    kernel_release: String,
    hardware_model: String,
}

impl Host {
    #[cfg(feature = "systemd")]
    pub fn new() -> Result<Self> {
        let hostnamtctl = Command::new("hostnamectl").arg("--json=short").output()?;
        if !hostnamtctl.status.success() {
            return Err(anyhow!("Failed to spawn hostnamectl"));
        }
        let host = from_slice::<Host>(&hostnamtctl.stdout)?;
        Ok(host)
    }
    #[cfg(feature = "posix")]
    pub fn new() -> Result<Self> {
        use sys_info::{hostname, linux_os_release};
        use uname::Info;

        let hardware_model = &read_to_string("/sys/devices/virtual/dmi/id/board_name")?.trim();
        let hostname = hostname()?;
        let info = linux_os_release()?;
        let uname = Info::new()?;

        Ok(Self {
            hostname,
            operating_system_pretty_name: info.pretty_name().to_string(),
            kernel_release: uname.release,
            hardware_model: hardware_model.to_string(),
        })
    }

    pub fn hostname(self) -> String {
        self.hostname
    }
    pub fn kernel_release(self) -> String {
        self.kernel_release
    }

    pub fn hardware_model(self) -> String {
        self.hardware_model
    }

    pub fn operating_system_pretty_name(self) -> String {
        self.operating_system_pretty_name
    }
}
