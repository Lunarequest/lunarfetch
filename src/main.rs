use anyhow::{anyhow, Context, Result};
use std::{
    env::consts::ARCH,
    process::{id, Command},
};

use crate::hostnamectl::Host;
mod hostnamectl;

fn get_package_number() -> Result<String> {
    let rpm_out = Command::new("rpm").arg("-qa").output()?;
    let output = String::from_utf8_lossy(&rpm_out.stdout).to_string();
    let rpm_number = output.lines().count();

    let flatpak_out = Command::new("flatpak").arg("list").output()?;
    let output = String::from_utf8_lossy(&flatpak_out.stdout).to_string();
    let flatpak_number = output.lines().count();

    return Ok(format!("(rpm, flaptak): {}", rpm_number + flatpak_number));
}

fn terminal() -> Result<String> {
    let pid = id();
    let pids = get_parent_pid(pid);
    println!("{:#?}", pids);
    if pids.is_empty() {
        return Err(anyhow!("fiailed to get parent pid for terminal detection"));
    }
    for pid in pids {
        println!("{}", get_pid_name(&pid));
        match get_pid_name(&pid).as_str() {
            "wezterm-gui" => return Ok(String::from("wezterm")),
            "code" => return Ok(String::from("vscode")),
            "code-insiders" => return Ok(String::from("vscode insiders")),
            "konsole" => return Ok(String::from("konsole")),
            "kitty" => return Ok(String::from("kitty")),
            _ => continue,
        }
    }

    Err(anyhow!(
        "anyhow could not match parents to any known terminal"
    ))
}

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

fn get_parent_pid(pid: u32) -> Vec<u32> {
    let mut pids: Vec<u32> = Vec::new();
    // ps -o ppid=66393
    let ret = Command::new("ps")
        .arg("-o")
        .arg(format!("ppid={}", pid))
        .output();
    println!("{:#?}", ret);
    if ret.is_err() {
        return pids;
    }

    let output = String::from_utf8_lossy(&ret.unwrap().stdout).to_string();
    for pid in output.split("\n") {
        let pid = remove_whitespace(pid);
        match pid.parse::<u32>() {
            Ok(p) => pids.push(p),
            Err(_) => break,
        }
    }
    pids
}

fn get_pid_name(pid: &u32) -> String {
    // ps -p 66393 -o comm=
    let ret = Command::new("ps")
        .arg("-p")
        .arg(format!("{}", pid))
        .arg("-o")
        .arg("comm=")
        .output();

    remove_whitespace(String::from_utf8_lossy(&ret.unwrap().stdout).as_ref())
}

fn main() -> Result<()> {
    let _a = get_package_number().context("Failed to get number of pakcages")?;
    let _terminal = terminal()?;
    let host = Host::new()?;
    let hostname = host.clone().hostname();
    let kernel_version = host.clone().kernel_release();
    let hardware_vendor = host.clone().hardware_vendor();
    let hardware_model = host.clone().hardware_model();
    let os = host.operating_system_pretty_name();
    let os_out = format!("{os} {ARCH}");

    Ok(())
}
