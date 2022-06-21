use std::fmt;

pub fn ansi_color(s: impl fmt::Display, c: u32) -> String {
    format!("\x1b[{}m{}\x1b[0m", c, s)
}

pub fn red(s: impl fmt::Display) -> String {
    ansi_color(s, 31)
}

pub fn green(s: impl fmt::Display) -> String {
    ansi_color(s, 32)
}

pub fn yellow(s: impl fmt::Display) -> String {
    ansi_color(s, 33)
}

pub fn blue(s: impl fmt::Display) -> String {
    ansi_color(s, 34)
}
