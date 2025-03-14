use anyhow::Result;
use std::thread::spawn;

use render::render;
use utils::{
    hardware::{cpu, gpu, ram},
    hostname::Host,
    music::get_song_dbus,
    package_managers::get_package_number,
    ps::terminal,
};
mod render;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    let package_manager_handle = spawn(get_package_number);
    let music_handle = spawn(get_song_dbus);
    let host_handle = spawn(Host::new);
    let gpu_handle = spawn(gpu);
    let ram = ram();
    let cpu = cpu();

    let gpus = gpu_handle.join().unwrap();
    let packages = package_manager_handle.join().unwrap()?;
    let music = music_handle.join().unwrap().await?;
    let host = host_handle.join().unwrap()?;

    render(packages, host, music, cpu, ram, gpus).await
}
