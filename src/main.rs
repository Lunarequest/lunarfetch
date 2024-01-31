use anyhow::Result;
use std::thread::spawn;

use render::render;
use utils::{
    hardware::{cpu, ram},
    hostname::Host,
    music::get_song_dbus,
    package_managers::get_package_number,
    ps::terminal,
};
mod render;
mod utils;

#[cfg(all(feature = "systemd", feature = "posix"))]
compile_error!(
    "systemd and posix mode are mutually exclusive pick only one backend, systemd is faster"
);

#[tokio::main]
async fn main() -> Result<()> {
    let package_manager_handle = spawn(get_package_number);
    let music_handle = spawn(get_song_dbus);
    let host_handle = spawn(Host::new);
    let ram = ram();
    let cpu = cpu();

    let packages = package_manager_handle.join().unwrap()?;
    let music = music_handle.join().unwrap().await?;
    let host = host_handle.join().unwrap()?;

    render(packages, host, music, cpu, ram).await
}
