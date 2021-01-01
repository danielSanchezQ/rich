use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use lazy_static::lazy_static;

use crate::{
    palette::{EIGHT_BIT_PALETTE, STANDARD_PALETTE, WINDOWS_PALETTE},
    terminal_theme::{TerminalTheme, DEFAULT_TERMINAL_THEME},
    triplet::{ColorTriplet, ColortripletRaw, ColortripletRawNormalized},
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Color {original} could not be parsed due to: {message}")]
    ParseColor { original: String, message: String },
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum ColorSystem {
    Standard,
    EightBit,
    TrueColor,
    Windows,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum ColorType {
    Default,
    Standard,
    EightBit,
    TrueColor,
    Windows,
}

impl From<ColorType> for ColorSystem {
    fn from(color_type: ColorType) -> Self {
        match color_type {
            ColorType::Default | ColorType::Standard => ColorSystem::Standard,
            ColorType::EightBit => ColorSystem::EightBit,
            ColorType::TrueColor => ColorSystem::TrueColor,
            ColorType::Windows => ColorSystem::Windows,
        }
    }
}

lazy_static! {
    pub static ref ANSI_COLOR_NAMES: HashMap<&'static str, u8> = {
        let mut m = HashMap::new();
        m.insert("black", 0);
        m.insert("red", 1);
        m.insert("green", 2);
        m.insert("yellow", 3);
        m.insert("blue", 4);
        m.insert("magenta", 5);
        m.insert("cyan", 6);
        m.insert("white", 7);
        m.insert("bright_black", 8);
        m.insert("bright_red", 9);
        m.insert("bright_green", 10);
        m.insert("bright_yellow", 11);
        m.insert("bright_blue", 12);
        m.insert("bright_magenta", 13);
        m.insert("bright_cyan", 14);
        m.insert("bright_white", 15);
        m.insert("grey0", 16);
        m.insert("navy_blue", 17);
        m.insert("dark_blue", 18);
        m.insert("blue3", 20);
        m.insert("blue1", 21);
        m.insert("dark_green", 22);
        m.insert("deep_sky_blue4", 25);
        m.insert("dodger_blue3", 26);
        m.insert("dodger_blue2", 27);
        m.insert("green4", 28);
        m.insert("spring_green4", 29);
        m.insert("turquoise4", 30);
        m.insert("deep_sky_blue3", 32);
        m.insert("dodger_blue1", 33);
        m.insert("green3", 40);
        m.insert("spring_green3", 41);
        m.insert("dark_cyan", 36);
        m.insert("light_sea_green", 37);
        m.insert("deep_sky_blue2", 38);
        m.insert("deep_sky_blue1", 39);
        m.insert("spring_green2", 47);
        m.insert("cyan3", 43);
        m.insert("dark_turquoise", 44);
        m.insert("turquoise2", 45);
        m.insert("green1", 46);
        m.insert("spring_green1", 48);
        m.insert("medium_spring_green", 49);
        m.insert("cyan2", 50);
        m.insert("cyan1", 51);
        m.insert("dark_red", 88);
        m.insert("deep_pink4", 125);
        m.insert("purple4", 55);
        m.insert("purple3", 56);
        m.insert("blue_violet", 57);
        m.insert("orange4", 94);
        m.insert("grey37", 59);
        m.insert("medium_purple4", 60);
        m.insert("slate_blue3", 62);
        m.insert("royal_blue1", 63);
        m.insert("chartreuse4", 64);
        m.insert("dark_sea_green4", 71);
        m.insert("pale_turquoise4", 66);
        m.insert("steel_blue", 67);
        m.insert("steel_blue3", 68);
        m.insert("cornflower_blue", 69);
        m.insert("chartreuse3", 76);
        m.insert("cadet_blue", 73);
        m.insert("sky_blue3", 74);
        m.insert("steel_blue1", 81);
        m.insert("pale_green3", 114);
        m.insert("sea_green3", 78);
        m.insert("aquamarine3", 79);
        m.insert("medium_turquoise", 80);
        m.insert("chartreuse2", 112);
        m.insert("sea_green2", 83);
        m.insert("sea_green1", 85);
        m.insert("aquamarine1", 122);
        m.insert("dark_slate_gray2", 87);
        m.insert("dark_magenta", 91);
        m.insert("dark_violet", 128);
        m.insert("purple", 129);
        m.insert("light_pink4", 95);
        m.insert("plum4", 96);
        m.insert("medium_purple3", 98);
        m.insert("slate_blue1", 99);
        m.insert("yellow4", 106);
        m.insert("wheat4", 101);
        m.insert("grey53", 102);
        m.insert("light_slate_grey", 103);
        m.insert("medium_purple", 104);
        m.insert("light_slate_blue", 105);
        m.insert("dark_olive_green3", 149);
        m.insert("dark_sea_green", 108);
        m.insert("light_sky_blue3", 110);
        m.insert("sky_blue2", 111);
        m.insert("dark_sea_green3", 150);
        m.insert("dark_slate_gray3", 116);
        m.insert("sky_blue1", 117);
        m.insert("chartreuse1", 118);
        m.insert("light_green", 120);
        m.insert("pale_green1", 156);
        m.insert("dark_slate_gray1", 123);
        m.insert("red3", 160);
        m.insert("medium_violet_red", 126);
        m.insert("magenta3", 164);
        m.insert("dark_orange3", 166);
        m.insert("indian_red", 167);
        m.insert("hot_pink3", 168);
        m.insert("medium_orchid3", 133);
        m.insert("medium_orchid", 134);
        m.insert("medium_purple2", 140);
        m.insert("dark_goldenrod", 136);
        m.insert("light_salmon3", 173);
        m.insert("rosy_brown", 138);
        m.insert("grey63", 139);
        m.insert("medium_purple1", 141);
        m.insert("gold3", 178);
        m.insert("dark_khaki", 143);
        m.insert("navajo_white3", 144);
        m.insert("grey69", 145);
        m.insert("light_steel_blue3", 146);
        m.insert("light_steel_blue", 147);
        m.insert("yellow3", 184);
        m.insert("dark_sea_green2", 157);
        m.insert("light_cyan3", 152);
        m.insert("light_sky_blue1", 153);
        m.insert("green_yellow", 154);
        m.insert("dark_olive_green2", 155);
        m.insert("dark_sea_green1", 193);
        m.insert("pale_turquoise1", 159);
        m.insert("deep_pink3", 162);
        m.insert("magenta2", 200);
        m.insert("hot_pink2", 169);
        m.insert("orchid", 170);
        m.insert("medium_orchid1", 207);
        m.insert("orange3", 172);
        m.insert("light_pink3", 174);
        m.insert("pink3", 175);
        m.insert("plum3", 176);
        m.insert("violet", 177);
        m.insert("light_goldenrod3", 179);
        m.insert("tan", 180);
        m.insert("misty_rose3", 181);
        m.insert("thistle3", 182);
        m.insert("plum2", 183);
        m.insert("khaki3", 185);
        m.insert("light_goldenrod2", 222);
        m.insert("light_yellow3", 187);
        m.insert("grey84", 188);
        m.insert("light_steel_blue1", 189);
        m.insert("yellow2", 190);
        m.insert("dark_olive_green1", 192);
        m.insert("honeydew2", 194);
        m.insert("light_cyan1", 195);
        m.insert("red1", 196);
        m.insert("deep_pink2", 197);
        m.insert("deep_pink1", 199);
        m.insert("magenta1", 201);
        m.insert("orange_red1", 202);
        m.insert("indian_red1", 204);
        m.insert("hot_pink", 206);
        m.insert("dark_orange", 208);
        m.insert("salmon1", 209);
        m.insert("light_coral", 210);
        m.insert("pale_violet_red1", 211);
        m.insert("orchid2", 212);
        m.insert("orchid1", 213);
        m.insert("orange1", 214);
        m.insert("sandy_brown", 215);
        m.insert("light_salmon1", 216);
        m.insert("light_pink1", 217);
        m.insert("pink1", 218);
        m.insert("plum1", 219);
        m.insert("gold1", 220);
        m.insert("navajo_white1", 223);
        m.insert("misty_rose1", 224);
        m.insert("thistle1", 225);
        m.insert("yellow1", 226);
        m.insert("light_goldenrod1", 227);
        m.insert("khaki1", 228);
        m.insert("wheat1", 229);
        m.insert("cornsilk1", 230);
        m.insert("grey100", 231);
        m.insert("grey3", 232);
        m.insert("grey7", 233);
        m.insert("grey11", 234);
        m.insert("grey15", 235);
        m.insert("grey19", 236);
        m.insert("grey23", 237);
        m.insert("grey27", 238);
        m.insert("grey30", 239);
        m.insert("grey35", 240);
        m.insert("grey39", 241);
        m.insert("grey42", 242);
        m.insert("grey46", 243);
        m.insert("grey50", 244);
        m.insert("grey54", 245);
        m.insert("grey58", 246);
        m.insert("grey62", 247);
        m.insert("grey66", 248);
        m.insert("grey70", 249);
        m.insert("grey74", 250);
        m.insert("grey78", 251);
        m.insert("grey82", 252);
        m.insert("grey85", 253);
        m.insert("grey89", 254);
        m.insert("grey93", 255);
        m
    };
}

/// Terminal color definition
#[derive(Clone)]
pub struct Color {
    /// The name of the color (typically the input to Color.parse)
    pub name: String,
    /// They type of the color
    pub color_type: ColorType,
    /// The color number, if a standard color, or None
    pub number: Option<u8>,
    /// A triplet of color components, if an RGB color
    pub triplet: Option<ColorTriplet>,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<color {} ({})>", self.name, self.name.to_lowercase())
    }
}

impl Default for Color {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            color_type: ColorType::Default,
            ..Default::default()
        }
    }
}

lazy_static! {
    pub static ref RE_COLOR: regex::Regex =
        regex::Regex::new(r#"^\#([0-9a-f]{6})$|color\(([0-9]{1,3})\)$|rgb\(([\d\s,]+)\)$"#)
            .unwrap();
}

impl Color {
    /// Create a Color number from it's 8-bit ansi number
    pub fn from_ansi(number: u8) -> Self {
        let color_type = if number < 16 {
            ColorType::Standard
        } else {
            ColorType::EightBit
        };
        Self {
            name: format!("color({})", number),
            color_type,
            number: Some(number),
            ..Default::default()
        }
    }

    /// Create a true color RGB color from a triplet of values
    pub fn from_triplet(triplet: ColorTriplet) -> Self {
        Self {
            name: triplet.hex(),
            color_type: ColorType::TrueColor,
            triplet: Some(triplet),
            ..Default::default()
        }
    }

    pub fn from_rgb(rgb: ColortripletRaw) -> Self {
        Self::from_triplet(rgb.into())
    }

    /// Get the native color system for this color
    pub fn system(&self) -> ColorSystem {
        self.color_type.into()
    }

    /// Check if the color is ultimately defined by the system
    pub fn is_system_defined(&self) -> bool {
        !matches!(
            self.system(),
            ColorSystem::EightBit | ColorSystem::TrueColor
        )
    }

    /// Check if the color is a default color
    pub fn is_default(&self) -> bool {
        matches!(self.color_type, ColorType::Default)
    }

    pub fn get_true_color(
        &self,
        theme: Option<&TerminalTheme>,
        foreground: Option<bool>,
    ) -> ColorTriplet {
        let theme = theme.unwrap_or(&DEFAULT_TERMINAL_THEME);
        let foreground = foreground.unwrap_or(false);
        match self.color_type {
            ColorType::Default => {
                assert!(self.number.is_none());
                if foreground {
                    theme.foreground_color
                } else {
                    theme.background_color
                }
            }
            ColorType::Standard => {
                assert!(self.number.is_some());
                theme.ansi_colors[self.number.unwrap() as usize].into()
            }
            ColorType::EightBit => {
                assert!(self.number.is_some());
                EIGHT_BIT_PALETTE[self.number.unwrap() as usize].into()
            }
            ColorType::TrueColor => {
                assert!(self.triplet.is_some());
                self.triplet.unwrap()
            }
            ColorType::Windows => {
                assert!(self.number.is_some());
                STANDARD_PALETTE[self.number.unwrap() as usize].into()
            }
        }
    }

    fn default_ansi_codes(foreground: bool) -> Vec<String> {
        if foreground {
            vec!["39".to_string()]
        } else {
            vec!["49".to_string()]
        }
    }

    fn standard_ansi_codes(n: u8, foreground: bool) -> Vec<String> {
        let (fore, back) = if n < 8 { (30, 40) } else { (82, 92) };
        if foreground {
            vec![format!("{}", fore + n as u16)]
        } else {
            vec![format!("{}", back + n as u16)]
        }
    }

    fn eight_bit_ansi_codes(n: u8, foreground: bool) -> Vec<String> {
        let fst = if foreground {
            "38".to_string()
        } else {
            "48".to_string()
        };
        vec![fst, "5".to_string(), n.to_string()]
    }

    fn truecolor_ansi_codes((r, g, b): ColortripletRaw, foreground: bool) -> Vec<String> {
        let fst = if foreground {
            "38".to_string()
        } else {
            "48".to_string()
        };
        vec![
            fst,
            "2".to_string(),
            r.to_string(),
            g.to_string(),
            b.to_string(),
        ]
    }

    fn windows_ansi_code(n: u8, foreground: bool) -> Vec<String> {
        let ret = if foreground { 30 } else { 40 } + n as u16;
        vec![ret.to_string()]
    }

    /// Get the ANSI escape codes for this color
    pub fn get_ansi_codes(&self, foreground: Option<bool>) -> Vec<String> {
        let foreground = foreground.unwrap_or(false);
        match self.color_type {
            ColorType::Default => Self::default_ansi_codes(foreground),
            ColorType::Standard => {
                assert!(self.number.is_some());
                let n = self.number.unwrap();
                Self::standard_ansi_codes(n, foreground)
            }
            ColorType::EightBit => {
                assert!(self.number.is_some());
                Self::eight_bit_ansi_codes(self.number.unwrap(), foreground)
            }
            ColorType::TrueColor => {
                assert!(self.triplet.is_some());
                Self::truecolor_ansi_codes(self.triplet.unwrap().as_raw(), foreground)
            }
            ColorType::Windows => {
                assert!(self.number.is_some());
                Self::windows_ansi_code(self.number.unwrap(), foreground)
            }
        }
    }

    /// Downgrade a color system to a system with fewer colors
    pub fn downgrade(&self, system: ColorSystem) -> Self {
        if self.color_type == ColorType::Default {
            return self.clone();
        }
        if ColorSystem::from(self.color_type) == system {
            return self.clone();
        }
        match (system, self.system()) {
            (ColorSystem::EightBit, ColorSystem::TrueColor) => {
                assert!(self.triplet.is_some());
                truecolor_2_eightbit(&self.name, self.triplet.unwrap().normalized())
            }
            (ColorSystem::Standard, ColorSystem::TrueColor) => {
                assert!(self.triplet.is_some());
                truecolor_2_standard(&self.name, self.triplet.unwrap())
            }
            (ColorSystem::Standard, ColorSystem::EightBit) => {
                assert!(self.number.is_some());
                eightbit_2_standard(&self.name, self.number.unwrap())
            }
            (ColorSystem::Windows, ColorSystem::TrueColor) => {
                assert!(self.triplet.is_some());
                truecolor_2_windows(&self.name, self.triplet.unwrap())
            }
            (ColorSystem::Windows, ColorSystem::EightBit) => {
                assert!(self.number.is_some());
                eightbit_2_windows(&self.name, self.number.unwrap())
            }
            _ => self.clone(),
        }
    }

    pub fn parse(color: &str) -> Result<Self, Error> {
        let original_color = color.to_string();
        let cleaned_color = color.to_lowercase().trim().to_string();
        if color == "default" {
            return Ok(Self::default());
        }

        if let Some(color_number) = ANSI_COLOR_NAMES.get(cleaned_color.as_str()) {
            Ok(parsed_ansi_color(&cleaned_color, *color_number))
        } else if let Some(color_match) = RE_COLOR.captures(&cleaned_color) {
            parsed_regex_captures(&original_color, &cleaned_color, color_match)
        } else {
            Err(Error::ParseColor {
                original: original_color,
                message: "unable to match color".to_string(),
            })
        }
    }
}

fn parsed_regex_captures(
    original_color: &str,
    color_name: &str,
    captures: regex::Captures,
) -> Result<Color, Error> {
    let (color_24, color_8, color_rgb) = (captures.get(0), captures.get(1), captures.get(2));
    if let Some(color) = color_24 {
        Ok(Color {
            name: color_name.to_string(),
            color_type: ColorType::TrueColor,
            triplet: Some(parse_rgb_hex(color.as_str())),
            ..Default::default()
        })
    } else if let Some(color) = color_8 {
        let number = u8::from_str_radix(color.as_str(), 10).map_err(|_| Error::ParseColor {
            original: original_color.to_string(),
            message: "color number must be <= 255".to_string(),
        })?;

        let color_type = if number < 16 {
            ColorType::Standard
        } else {
            ColorType::EightBit
        };
        Ok(Color {
            name: color_name.to_string(),
            color_type,
            number: Some(number),
            ..Default::default()
        })
    } else if let Some(color) = color_rgb {
        let components: Vec<String> = color.as_str().split(',').map(|s| s.to_string()).collect();
        match &components[..] {
            [r, g, b] => {
                let triplet = ColorTriplet::from((
                    u8::from_str_radix(&r, 10).map_err(|_| Error::ParseColor {
                        original: original_color.to_string(),
                        message: "red component must be <= 255".to_string(),
                    })?,
                    u8::from_str_radix(&g, 10).map_err(|_| Error::ParseColor {
                        original: original_color.to_string(),
                        message: "green component must be <= 255".to_string(),
                    })?,
                    u8::from_str_radix(&b, 10).map_err(|_| Error::ParseColor {
                        original: original_color.to_string(),
                        message: "blue component must be <= 255".to_string(),
                    })?,
                ));
                Ok(Color {
                    name: color_name.to_string(),
                    color_type: ColorType::TrueColor,
                    triplet: Some(triplet),
                    ..Default::default()
                })
            }
            _ => Err(Error::ParseColor {
                original: original_color.to_string(),
                message: "expected three components (r, g, b)".to_string(),
            }),
        }
    } else {
        unreachable!()
    }
}

fn parsed_ansi_color(color_name: &str, color_number: u8) -> Color {
    let color_type = if color_number < 16 {
        ColorType::Standard
    } else {
        ColorType::EightBit
    };
    Color {
        name: color_name.to_string(),
        color_type,
        number: Some(color_number),
        ..Default::default()
    }
}

fn truecolor_2_eightbit(name: &str, normalized_color: ColortripletRawNormalized) -> Color {
    let (r, g, b) = normalized_color;
    let hsl = colorsys::Hsl::from(colorsys::Rgb::from(normalized_color));
    // If saturation is under 10% assume it is grayscale
    if hsl.get_saturation() < 0.1 {
        let gray = f32::round(hsl.get_lightness() as f32 * 25.0) as u8;
        let color_number = match gray {
            0 => 16,
            25 => 231,
            _ => 231 + gray,
        };
        Color {
            name: name.to_string(),
            color_type: ColorType::EightBit,
            number: Some(color_number),
            ..Default::default()
        }
    } else {
        let color_number = 16
            + 36 * f32::round(r * 5.0) as u8
            + 6 * f32::round(g * 5.0) as u8
            + f32::round(b * 5.0) as u8;
        Color {
            name: name.to_string(),
            color_type: ColorType::EightBit,
            number: Some(color_number),
            ..Default::default()
        }
    }
}

fn truecolor_2_standard(name: &str, triplet: ColorTriplet) -> Color {
    // it safe to unwrap here because STANDARD_PALETTE is guaranteed to have data
    let color_number = STANDARD_PALETTE.match_color(triplet.as_raw()).unwrap();
    Color {
        name: name.to_string(),
        color_type: ColorType::Standard,
        number: Some(color_number as u8),
        ..Default::default()
    }
}
fn eightbit_2_standard(name: &str, number: u8) -> Color {
    let color = EIGHT_BIT_PALETTE[number as usize];
    // it safe to unwrap here because STANDARD_PALETTE is guaranteed to have data
    let color_number = STANDARD_PALETTE.match_color(color).unwrap();
    Color {
        name: name.to_string(),
        color_type: ColorType::Standard,
        number: Some(color_number as u8),
        ..Default::default()
    }
}

fn truecolor_2_windows(name: &str, triplet: ColorTriplet) -> Color {
    // it safe to unwrap here because WINDOWS_PALETTE is guaranteed to have data
    let color_number = WINDOWS_PALETTE.match_color(triplet.as_raw()).unwrap();
    Color {
        name: name.to_string(),
        color_type: ColorType::Standard,
        number: Some(color_number as u8),
        ..Default::default()
    }
}

fn eightbit_2_windows(name: &str, number: u8) -> Color {
    if number < 8 {
        Color {
            name: name.to_string(),
            color_type: ColorType::Windows,
            number: Some(number),
            ..Default::default()
        }
    } else if number < 16 {
        Color {
            name: name.to_string(),
            color_type: ColorType::Windows,
            number: Some(number - 8),
            ..Default::default()
        }
    } else {
        let color = EIGHT_BIT_PALETTE[number as usize];
        // it safe to unwrap here because WINDOWS_PALETTE is guaranteed to have data
        let color_number = WINDOWS_PALETTE.match_color(color).unwrap();
        Color {
            name: name.to_string(),
            color_type: ColorType::Windows,
            number: Some(color_number as u8),
            ..Default::default()
        }
    }
}

/// Parse six hex characters in to RGB triplet
pub fn parse_rgb_hex(hex_color: &str) -> ColorTriplet {
    assert_eq!(
        hex_color.len(),
        6,
        "Hex color must be 6 characters long, found {}",
        hex_color.len()
    );
    let r = u8::from_str_radix(&hex_color[0..2], 16).expect("Valid u8 hex encoded red value");
    let g = u8::from_str_radix(&hex_color[2..4], 16).expect("Valid u8 hex encoded green value");
    let b = u8::from_str_radix(&hex_color[4..6], 16).expect("Valid u8 hex encoded blue value");
    ColorTriplet::from((r, g, b))
}

/// Blend one RGB color in to another
pub fn blend_rgb(
    color1: ColorTriplet,
    color2: ColorTriplet,
    cross_fade: Option<f32>,
) -> ColorTriplet {
    let cross_fade = cross_fade.unwrap_or(0.5f32);
    let r = color1.red as f32 + (color2.red as f32 - color1.red as f32) * cross_fade;
    let g = color1.green as f32 + (color2.green as f32 - color1.green as f32) * cross_fade;
    let b = color1.blue as f32 + (color2.blue as f32 - color1.blue as f32) * cross_fade;
    ColorTriplet::from((r as u8, g as u8, b as u8))
}
