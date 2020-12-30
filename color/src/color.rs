use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};

use lazy_static::lazy_static;

use crate::triplet::ColortripletRaw;
use crate::{
    palette::{EIGHT_BIT_PALETTE, STANDARD_PALETTE, WINDOWS_PALETTE},
    triplet::ColorTriplet,
};

#[derive(Clone, Copy)]
pub enum ColorSystem {
    Standard,
    EightBit,
    TrueColor,
    Windows,
}

#[derive(Clone, Copy)]
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
    pub static ref ANSI_COLOR_NAMES: HashMap<&'static str, u32> = {
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

pub type TerminalTheme = String;

/// Terminal color definition
pub struct Color {
    /// The name of the color (typically the input to Color.parse)
    name: String,
    /// They type of the color
    color_type: ColorType,
    /// The color number, if a standard color, or None
    number: Option<u8>,
    /// A triplet of color components, if an RGB color
    triplet: Option<ColorTriplet>,
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
        theme: Option<TerminalTheme>,
        foreground: Option<bool>,
    ) -> ColorTriplet {
        let theme = theme.unwrap_or_else(Default::default);
        match self.color_type {
            ColorType::Default => {
                assert!(self.number.is_none());
                // TODO: return actual value from theme
                unimplemented!();
            }
            ColorType::Standard => {
                assert!(self.number.is_some());
                // TODO: return actual value from theme
                unimplemented!();
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
}
