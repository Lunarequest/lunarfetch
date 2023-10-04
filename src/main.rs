use anyhow::{anyhow, Context, Result};
use std::env::{consts::ARCH, var};

use hostnamectl::Host;
use package_managers::get_package_number;
use utils::ps::terminal;
mod hostnamectl;
mod package_managers;
mod utils;

pub fn desktop_env() -> Result<String> {
    let desktop = match var("XDG_CURRENT_DESKTOP") {
        Ok(de) => de,
        Err(e) => {
            return Err(anyhow!(
                "{e}\nXDG_CURRENT_DESKTOP not set failed to determain desktop env"
            ))
        }
    };

    match desktop.as_ref() {
        "KDE" => return Ok(String::from("Kde Plasma")),
        _ => return Err(anyhow!("unkown value {desktop}")),
    }
}

fn main() -> Result<()> {
    let a = get_package_number().context("Failed to get number of pakcages")?;
    let terminal = terminal()?;
    let host = Host::new()?;
    let hostname = host.clone().hostname();
    let kernel_version = host.clone().kernel_release();
    let hardware_vendor = host.clone().hardware_vendor();
    let hardware_model = host.clone().hardware_model();
    let os = host.operating_system_pretty_name();
    let os_out = format!("{os} {ARCH}");
    let desktop = desktop_env()?;

    println!("{a}\n{terminal}\n{hostname}\n{kernel_version}\n{hardware_vendor}\n{hardware_model}\n{os_out}\n{desktop}");

    Ok(())
}
