use anyhow::Result;
use std::fs::read_to_string;
use sys_info::{hostname, linux_os_release};
use uname::Info;

#[derive(Debug, Clone)]
pub struct Host {
    hostname: String,
    operating_system_pretty_name: String,
    kernel_release: String,
    hardware_model: String,
}

impl Host {
    pub fn new() -> Result<Self> {
        let hardware_model = read_to_string("/sys/devices/virtual/dmi/id/board_name")?
            .trim()
            .to_string();
        let hostname = hostname()?;
        let info = linux_os_release()?;
        let uname = Info::new()?;

        Ok(Self {
            hostname,
            operating_system_pretty_name: info.pretty_name().to_string(),
            kernel_release: uname.release,
            hardware_model,
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
