use std::fmt::{Display, Formatter};
use clicolors_control::{colors_enabled, set_colors_enabled};
use clicolors_control::terminfo::supports_colors;

pub struct ColorString {
    string: String,
    color: Color,
    highlight: Color,
    bold: bool,
    underline: bool,
    reverse: bool,
    is_enabled: bool
}

#[derive(PartialEq, Eq)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Reset,
    Bright(ColorBright),
    None
}

#[derive(PartialEq, Eq)]
pub enum ColorBright {
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite
}

impl ColorBright {
    pub fn ansi(&self) -> &'static str {
        return match self {
            ColorBright::BrightBlack => "\x1b[90m",
            ColorBright::BrightRed => "\x1b[91m",
            ColorBright::BrightGreen => "\x1b[92m",
            ColorBright::BrightYellow => "\x1b[93m",
            ColorBright::BrightBlue => "\x1b[94m",
            ColorBright::BrightMagenta => "\x1b[95m",
            ColorBright::BrightCyan => "\x1b[96m",
            ColorBright::BrightWhite => "\x1b[97m",
        }
    }
}


impl Color {
    pub fn ansi(&self) -> &'static str {
        return match self {
            Color::Black => "\x1b[30m",
            Color::Red => "\x1b[31m",
            Color::Green => "\x1b[32m",
            Color::Yellow => "\x1b[33m",
            Color::Blue => "\x1b[34m",
            Color::Magenta => "\x1b[35m",
            Color::Cyan => "\x1b[36m",
            Color::White => "\x1b[37m",
            Color::Reset => "\x1b[0m",
            Color::None => "",
            Color::Bright(color) => color.ansi(),
            _ => ""
        }
    }
}

impl ColorString {
    pub fn new(string: &str) -> ColorString {
        let is_enabled = terminal_color_support();
        return ColorString {
            string: string.to_string(),
            color: Color::None,
            highlight: Color::None,
            bold: false,
            underline: false,
            reverse: false,
            is_enabled
        }
    }

    pub fn bold(mut self) -> ColorString {
        if !self.is_enabled {
            return self;
        }
        if self.bold {
            self.bold = false;
            self.string = self.string.replace("\x1b[1m", "");
            if !self.check_formatted() {
                self.string = self.string.replace("\x1b[1m", Color::Reset.ansi());
            }
        } else {
            if !self.check_formatted() {
                self.string += Color::Reset.ansi();
            }
            self.bold = true;
            self.string = "\x1b[1m".to_string() + &self.string;
        }
        return self;
    }

    pub fn underline(mut self) -> ColorString {
        if !self.is_enabled {
            return self;
        }
        if self.underline {
            self.underline = false;
            self.string = self.string.replace("\x1b[4m", "");
            if !self.check_formatted() {
                self.string = self.string.replace("\x1b[4m", Color::Reset.ansi());
            }
        } else {
            if !self.check_formatted() {
                self.string += Color::Reset.ansi();
            }
            self.underline = true;
            self.string = "\x1b[4m".to_string() + &self.string;
        }
        return self;
    }

    pub fn reverse(mut self) -> ColorString {
        if !self.is_enabled {
            return self;
        }
        if self.reverse {
            self.reverse = false;
            self.string = self.string.replace("\x1b[7m", "");
            if !self.check_formatted() {
                self.string = self.string.replace("\x1b[7m", Color::Reset.ansi());
            }
        } else {
            if !self.check_formatted() {
                self.string += Color::Reset.ansi();
            }
            self.reverse = true;
            self.string = "\x1b[7m".to_string() + &self.string;
        }
        return self;
    }

    pub fn colorize(mut self, color: Color) -> ColorString {
        if !self.is_enabled {
            return self;
        }
        if self.color != Color::None {
            self.string = self.string.replace(self.color.ansi(), color.ansi());
        } else {
            if !self.check_formatted() {
                self.string += Color::Reset.ansi();
            }
            self.string = color.ansi().to_string() + &self.string;
        }
        return self;
    }

    pub fn black(self) -> ColorString { return self.colorize(Color::Black); }
    pub fn red(self) -> ColorString { return self.colorize(Color::Red); }
    pub fn green(self) -> ColorString { return self.colorize(Color::Green); }
    pub fn yellow(self) -> ColorString { return self.colorize(Color::Yellow); }
    pub fn blue(self) -> ColorString { return self.colorize(Color::Blue); }
    pub fn magenta(self) -> ColorString { return self.colorize(Color::Magenta); }
    pub fn cyan(self) -> ColorString { return self.colorize(Color::Cyan); }
    pub fn white(self) -> ColorString { return self.colorize(Color::White); }

    fn check_formatted(&self) -> bool {
        return self.bold == true || self.underline == true || self.underline == true || self.color != Color::None || self.highlight != Color::None;
    }

    pub fn string(&self) -> String {
        return self.string.clone();
    }

}

impl Display for ColorString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string)
    }
}

pub fn colorize(string: String, color: Color) -> String {
    let mut color_string = string.clone();
    color_string = color_string.replace(Color::Reset.ansi(), &(Color::Reset.ansi().to_owned() + &color.ansi()));
    let color_string = color.ansi().to_string() + &color_string + Color::Reset.ansi();
    return color_string;
}

pub fn colorize_str(string: &str, color: Color) -> String {
    let cstring = string.replace(Color::Reset.ansi(), &(Color::Reset.ansi().to_owned() + color.ansi()));
    return color.ansi().to_owned() + &cstring + Color::Reset.ansi();
}

pub fn terminal_color_support() -> bool {
    return colors_enabled() || supports_colors();
}