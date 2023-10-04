use anyhow::{Context, Result};
use std::{env::consts::ARCH, process::Command};
use walkdir::{DirEntry, WalkDir};

use hostnamectl::Host;
use utils::{ps::terminal, which::which};
mod hostnamectl;
mod utils;

fn is_dir(entry: &DirEntry) -> bool {
    entry.path().is_dir()
}

fn get_package_number() -> Result<String> {
    let mut rpm_number = 0;
    let mut flatpak_number = 0;
    let mut emerge_number = 0;
    let mut out = String::from("(");
    if which("rpm").is_some() {
        let rpm_out = Command::new("rpm").arg("-qa").output()?;
        let output = String::from_utf8_lossy(&rpm_out.stdout).to_string();
        rpm_number = output.lines().count();
        if rpm_number > 0 {
            if out == "(" {
                out.push_str("rpm");
            } else {
                out.push_str(",rpm");
            }
        }
    }

    if which("emerge").is_some() {
        let walker = WalkDir::new("/var/db/pkg/").into_iter();
        for entry in walker.filter_entry(|e| is_dir(e)) {
            match entry {
                Err(_) => continue,
                Ok(_) => emerge_number += 1,
            }
        }
        if emerge_number > 0 {
            if out == "(" {
                out.push_str("emerge");
            } else {
                out.push_str(",emerge");
            }
        }
    }

    if which("flatpak").is_some() {
        let flatpak_out = Command::new("flatpak").arg("list").output()?;
        let output = String::from_utf8_lossy(&flatpak_out.stdout).to_string();
        flatpak_number = output.lines().count();
        if flatpak_number > 0 {
            if out == "(" {
                out.push_str("flatpak");
            } else {
                out.push_str(",flatpak");
            }
        }
    }

    out.push(')');

    Ok(format!(
        "{}: {}",
        out,
        emerge_number + rpm_number + flatpak_number
    ))
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

    println!("{a}\n{terminal}\n{hostname}\n{kernel_version}\n{hardware_vendor}\n{hardware_model}\n{os_out}");

    Ok(())
}
