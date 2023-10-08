use super::which::which;
use anyhow::Result;
use std::process::Command;
use walkdir::{DirEntry, WalkDir};

fn is_dir(entry: &DirEntry) -> bool {
    entry.path().is_dir()
}

pub fn get_package_number() -> Result<String> {
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
        for entry in walker.filter_entry(is_dir) {
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
        "{} {}",
        emerge_number + rpm_number + flatpak_number,
        out
    ))
}
