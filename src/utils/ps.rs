use super::whitespace::remove_whitespace;
use anyhow::{anyhow, Result};
use std::{
    fs::read_to_string,
    process::{id, Command},
};

pub fn terminal() -> Result<String> {
    let pid = id();
    let pids = get_parent_pid(pid);
    if pids.is_empty() {
        return Err(anyhow!("fiailed to get parent pid for terminal detection"));
    }
    for pid in pids {
        match get_pid_name(&pid)?.as_str() {
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

fn get_parent_pid(pid: u32) -> Vec<u32> {
    let mut pids: Vec<u32> = Vec::new();
    let ret = Command::new("ps")
        .arg("-o")
        .arg(format!("ppid={}", pid))
        .output();
    if ret.is_err() {
        return pids;
    }

    let output = String::from_utf8_lossy(&ret.unwrap().stdout).to_string();
    for pid in output.split('\n') {
        let pid = remove_whitespace(pid);
        match pid.parse::<u32>() {
            Ok(p) => pids.push(p),
            Err(_) => break,
        }
    }
    pids
}

fn get_pid_name(pid: &u32) -> Result<String> {
    let name = read_to_string(format!("/proc/{pid}/comm"))?;
    Ok(remove_whitespace(&name))
}
