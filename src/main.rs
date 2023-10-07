use anyhow::{anyhow, Result};
use std::{
    env::{consts::ARCH, var},
    thread::spawn,
};

use hostnamectl::Host;
use package_managers::get_package_number;
use utils::{music::get_song_dbus, ps::terminal};
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

#[tokio::main]
async fn main() -> Result<()> {
    let package_manager_handle = spawn(|| get_package_number());
    let terminal_handle = spawn(|| terminal());

    let desktop_handle = spawn(|| desktop_env());
    let music_handle = spawn(|| get_song_dbus());
    let host_handle = spawn(|| Host::new());

    let packages = package_manager_handle.join().unwrap()?;
    let terminal = terminal_handle.join().unwrap()?;
    let desktop = desktop_handle.join().unwrap()?;
    let music = music_handle.join().unwrap().await?;
    let host = host_handle.join().unwrap()?;

    let hostname = host.clone().hostname();
    let kernel_version = host.clone().kernel_release();
    let hardware_vendor = host.clone().hardware_vendor();
    let hardware_model = host.clone().hardware_model();
    let os = host.operating_system_pretty_name();

    let os_out = format!("{os} {ARCH}");

    println!("{packages}\n{terminal}\n{hostname}\n{kernel_version}\n{hardware_vendor}\n{hardware_model}\n{os_out}\n{desktop}\n{music}");

    Ok(())
}
