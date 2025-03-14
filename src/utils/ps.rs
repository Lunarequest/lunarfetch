use anyhow::{Result, anyhow};
use std::{
    fs::{File, read_to_string},
    io::{BufRead, BufReader},
    process::id,
};

pub fn terminal() -> String {
    let pid = id();
    let pids = get_parent_pids(pid).unwrap_or_default();
    if pids.is_empty() {
        return "Unknown Terminal".into();
    }
    for pid in &pids {
        match get_pid_name(pid) {
            Ok(pname) => match pname.as_str() {
                "ghostty" => return "Ghostty".into(),
                "wezterm-gui" => return "Wezterm".into(),
                "code" => return "Vscode".into(),
                "code-insiders" => return "Vscode Insiders".into(),
                "konsole" => return "Konsole".into(),
                "kitty" => return "Kitty".into(),
                "sshd" => return "SSH session".into(),
                _ => continue,
            },
            Err(_e) => {
                #[cfg(debug_assertions)]
                println!("{_e}");
            }
        }
    }
    if pids.is_empty() {
        "Unknown Terminal".into()
    } else {
        match get_pid_name(&pid) {
            Ok(name) => name,
            Err(_) => "Unknown Terminal".into(),
        }
    }
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
