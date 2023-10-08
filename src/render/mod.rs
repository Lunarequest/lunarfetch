mod squares;
use crate::{
    terminal,
    utils::{desktop_env, hostname::Host},
};
use owo_colors::OwoColorize;
use pretty_duration::pretty_duration;
use squares::gen_colored_squares;
use std::env::{self, consts::ARCH};
use uptime_lib::get;

use anyhow::{Context, Result};

pub async fn render(packages: String, host: Host, music: String) -> Result<()> {
    let desktop = desktop_env()?;
    let terminal = terminal()?;
    let user = env::var("USER")?;
    let binding = env::var("SHELL")?;
    let shell = binding.split('/').last().context("bash shell")?;
    let uptime = get().unwrap();
    let blocks = gen_colored_squares();

    let os = host.clone().operating_system_pretty_name();
    let hardware = host.clone().hardware_model();
    let kernel = host.clone().kernel_release();
    let uptime_fmt = pretty_duration(&uptime, None);
    let user = format!("{}@{}", user, host.clone().hostname());
    let dashes = "-".repeat(user.len());

    let logo = format!(
        "{} {}  {}    {}        {}
{} {} {}  {}        {}
  {}     {}   {}   {}  {}          {} -> {} {}
  {}     {}    {}          {} -> {}
  {}     {}    {}          {} -> {}
  {}     {}   {}          {} -> {}
{} {} {}        {} -> {}
{} {}  {}        {} -> {}
             {} {} -> {}
              {} {} -> {}
              {} {} -> {}
              {}
",
        "▀▀▀▀▀▀▀".bright_red(),
        "▀▀▀▀▀▀▀▀▀▘".white(),
        "▀▀▀▀▀▀".bright_blue(),
        "▀▀▀▀▀▀".bright_blue(),
        user.purple(),
        "▀▀▀▀▀▀▀".bright_red(),
        "▀▀▀▀▀▀▀▀▀▀▀".white(),
        "▀▀▀▀▀▀▘".bright_blue(),
        "▝▀▀▀▀▀▀".bright_blue(),
        dashes,
        "▀▀▀".bright_red(),
        "▀▀▀".white(),
        "▀▀▀".white(),
        "▀▀▀▀▀".bright_blue(),
        "▀▀▀▀▀".bright_blue(),
        "Os".purple(),
        os,
        ARCH,
        "▀▀▀".bright_red(),
        "▀▀▀▀▀▀▀▀".white(),
        "▀▀▀▝▀▘▝▀▘▀▀▀".bright_blue(),
        "Host".purple(),
        hardware,
        "▀▀▀".bright_red(),
        "▀▀▀▀▀▀▀▀".white(),
        "▀▀▀ ▀▀▀▀ ▀▀▀".bright_blue(),
        "Kernel".purple(),
        kernel,
        "▀▀▀".bright_red(),
        "▀▀▀   ▀▀▀".white(),
        "▀▀▀ ▝▀▀▘ ▀▀▀".bright_blue(),
        "Uptime".purple(),
        uptime_fmt,
        "▀▀▀▀▀▀▀".bright_red(),
        "▀▀▀▀▀▀▀▀▀▀▀".white(),
        "▀▀▀▀▀  ▀▀  ▀▀▀▀▀".bright_blue(),
        "Packages".purple(),
        packages,
        "▀▀▀▀▀▀▀".bright_red(),
        "▀▀▀▀▀▀▀▀▀▘".white(),
        "▀▀▀▀▀      ▀▀▀▀▀".bright_blue(),
        "Shell".purple(),
        shell,
        "▗▄▄▖               ▄▄▖      ▗▖".white(),
        "Desktop".purple(),
        desktop,
        "▐▌ ▗▄▖▗▄▖ ▄▄▖ ▗▄▖ █ █ ▄▄  ▄▟▌".white(),
        "Terminal".purple(),
        terminal,
        "▐▌ █  ▗▄█ █ █ ▜▄▖ █▀▘ ▄▟▌▐▌▐▌".white(),
        "Song".purple(),
        music,
        "▐▌ █  ▜▄█ █ █ ▗▄▛ █  ▝▙▟▌▝▙▟▌".white(),
    );
    println!("{logo}");
    println!("{blocks}");
    Ok(())
}
