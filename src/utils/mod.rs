pub mod hardware;
pub mod hostname;
pub mod music;
pub mod package_managers;
pub mod ps;
pub mod which;

use anyhow::{anyhow, Result};
use std::env::var;

pub fn desktop_env() -> Result<String> {
    let desktop = match var("XDG_CURRENT_DESKTOP") {
        Ok(de) => de,
        Err(_e) => return Ok("HEADLESS".to_string()),
    };

    match desktop.as_ref() {
        "KDE" => Ok(String::from("Kde Plasma")),
        _ => Err(anyhow!("unkown value {desktop}")),
    }
}
