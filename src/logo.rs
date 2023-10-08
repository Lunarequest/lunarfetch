use crate::{hostnamectl::Host, terminal, utils::desktop_env};

use std::env::{self, consts::ARCH};

use anyhow::{Context, Result};
use owo_colors::{AnsiColors, OwoColorize};
use pretty_duration::pretty_duration;
use uptime_lib::get;

fn gen_colored_squares() -> String {
    let block_width = 3;

    let block = " ".repeat(block_width);

    let row_1_colors = [
        AnsiColors::Black,
        AnsiColors::Red,
        AnsiColors::Green,
        AnsiColors::Blue,
        AnsiColors::Yellow,
        AnsiColors::Magenta,
        AnsiColors::Cyan,
        AnsiColors::White,
    ];

    let row_2_colors = [
        AnsiColors::BrightBlack,
        AnsiColors::BrightRed,
        AnsiColors::BrightGreen,
        AnsiColors::BrightBlue,
        AnsiColors::BrightYellow,
        AnsiColors::BrightMagenta,
        AnsiColors::BrightCyan,
        AnsiColors::BrightWhite,
    ];
    let padding = " ".repeat(45);
    let mut row1 = String::new();
    let mut row2 = String::new();

    for color in row_1_colors {
        let colored_block = block.on_color(color).to_string();
        row1.push_str(&colored_block);
    }
    for color in row_2_colors {
        let colored_block = block.on_color(color).to_string();
        row2.push_str(&colored_block);
    }

    format!("{padding}{row1}\n{padding}{row2}")
}

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
