use anyhow::{anyhow, Result};
use std::{
    fs::{read_to_string, File},
    io::{BufRead, BufReader},
    process::id,
};

pub fn terminal() -> Result<String> {
    let pid = id();
    let pids = get_parent_pids(pid)?;
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

fn get_parent_pid(pid: u32) -> Result<u32> {
    let file = File::open(format!("/proc/{}/status", pid))?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if line.starts_with("PPid:") {
            let ppid: u32 = line[6..].trim().parse().unwrap();
            return Ok(ppid);
        }
    }

    Err(anyhow!("PPid not found"))
}

fn get_parent_pids(pid: u32) -> Result<Vec<u32>> {
    let mut pids = Vec::new();
    let mut current_pid = pid;

    while current_pid != 1 {
        let ppid = get_parent_pid(current_pid)?;
        pids.push(ppid);
        current_pid = ppid;
    }

    Ok(pids)
}

fn get_pid_name(pid: &u32) -> Result<String> {
    let name = read_to_string(format!("/proc/{pid}/comm"))?;
    Ok(name.trim().to_string())
}
