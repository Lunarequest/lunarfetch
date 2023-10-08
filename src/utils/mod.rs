pub mod hostname;
pub mod music;
pub mod package_managers;
pub mod ps;
pub mod which;
mod whitespace;

use anyhow::{anyhow, Result};
use std::env::var;

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
        "KDE" => Ok(String::from("Kde Plasma")),
        _ => Err(anyhow!("unkown value {desktop}")),
    }
}
