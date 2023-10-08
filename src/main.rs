use anyhow::Result;
use std::thread::spawn;

use hostnamectl::Host;
use logo::render;
use package_managers::get_package_number;
use utils::{music::get_song_dbus, ps::terminal};
mod hostnamectl;
mod logo;
mod package_managers;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    let package_manager_handle = spawn(get_package_number);
    let music_handle = spawn(get_song_dbus);
    let host_handle = spawn(Host::new);

    let packages = package_manager_handle.join().unwrap()?;
    let music = music_handle.join().unwrap().await?;
    let host = host_handle.join().unwrap()?;

    render(packages, host, music).await
}
