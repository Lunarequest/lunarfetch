use super::which::which;
use anyhow::Result;
use std::{fs::read_dir, process::Command};

pub fn get_package_number() -> Result<String> {
    let mut rpm_number = 0;
    let mut flatpak_number = 0;
    let mut emerge_number = 0;
    let mut brew = 0;
    let mut out = String::from("(");
    if which("rpm").is_some() {
        let rpm_out = Command::new("rpm").arg("-qa").output()?;
        let output = String::from_utf8_lossy(&rpm_out.stdout).to_string();
        rpm_number = output.lines().count();
        if rpm_number > 0 {
            if out == "(" {
                out.push_str("rpm");
            } else {
            }
        }
    }

    if which("emerge").is_some() {
        let walker = read_dir("/var/db/pkg/")?;
        for entry in walker {
            match entry {
                Err(_) => continue,
                Ok(dir) => {
                    if dir.path().is_dir() {
                        let subwalker =
                            read_dir(format!("/var/db/pkg/{}", dir.file_name().to_string_lossy()))?;
                        for subdir in subwalker {
                            match subdir {
                                Err(_) => continue,
                                Ok(sub_subfile) => {
                                    if sub_subfile.path().is_dir() {
                                        emerge_number += 1;
                                    }
                                }
                            }
                        }
                    }
                }
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

    if which("brew").is_some() {
        let brew_out = Command::new("brew").args(["list", "-l"]).output()?;
        let output = String::from_utf8_lossy(&brew_out.stdout).to_string();
        brew = output.lines().count() - 1;
        if brew > 0 {
            if out == "(" {
                out.push_str("brew");
            } else {
                out.push_str(",brew");
            }
        }
    }

    if which("flatpak").is_some() {
        let flatpak_out = Command::new("flatpak").arg("list").output()?;
        let output = String::from_utf8_lossy(&flatpak_out.stdout).to_string();
        flatpak_number = output.lines().count() - 1;
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
        emerge_number + rpm_number + flatpak_number + brew,
        out
    ))
}
