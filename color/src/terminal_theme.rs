use lazy_static::lazy_static;

use crate::{
    palette::Palette,
    triplet::{ColorTriplet, ColortripletRaw},
};

/// A color theme used when exporting console content
pub struct TerminalTheme {
    pub background_color: ColorTriplet,
    pub foreground_color: ColorTriplet,
    pub ansi_colors: Palette,
}

impl TerminalTheme {
    pub fn new(
        background_color: ColortripletRaw,
        foreground_color: ColortripletRaw,
        normal: &[ColortripletRaw],
        bright: Option<&[ColortripletRaw]>,
    ) -> Self {
        let ansi_colors = match (normal, bright) {
            (normal, Some(bright)) => normal.iter().chain(bright.iter()).cloned(),
            (normal, None) => normal.iter().chain(normal.iter()).cloned(),
        };
        Self {
            background_color: ColorTriplet::from(background_color),
            foreground_color: ColorTriplet::from(foreground_color),
            ansi_colors: Palette::new(ansi_colors),
        }
    }
}

impl Default for TerminalTheme {
    fn default() -> Self {
        Self::new(
            (255, 255, 255),
            (0, 0, 0),
            &[
                (0, 0, 0),
                (128, 0, 0),
                (0, 128, 0),
                (128, 128, 0),
                (0, 0, 128),
                (128, 0, 128),
                (0, 128, 128),
                (192, 192, 192),
            ],
            Some(&[
                (128, 128, 128),
                (255, 0, 0),
                (0, 255, 0),
                (255, 255, 0),
                (0, 0, 255),
                (255, 0, 255),
                (0, 255, 255),
                (255, 255, 255),
            ]),
        )
    }
}

lazy_static! {
    pub static ref DEFAULT_TERMINAL_THEME: TerminalTheme = Default::default();
}
