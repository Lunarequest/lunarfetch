use owo_colors::{AnsiColors, OwoColorize};

pub fn gen_colored_squares() -> String {
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
