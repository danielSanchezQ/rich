pub type ColortripletRaw = (u8, u8, u8);

pub type ColortripletRawNormalized = (f32, f32, f32);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct ColorTriplet {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl From<ColortripletRaw> for ColorTriplet {
    fn from((red, green, blue): (u8, u8, u8)) -> Self {
        Self { red, green, blue }
    }
}

impl ColorTriplet {
    pub fn as_raw(&self) -> ColortripletRaw {
        (self.red, self.green, self.blue)
    }

    pub fn hex(&self) -> String {
        format!(
            "#{r:02x}{g:02x}{b:02x}",
            r = self.red,
            g = self.green,
            b = self.blue
        )
    }

    pub fn rgb(&self) -> String {
        format!(
            "rgb({r},{g},{b})",
            r = self.red,
            g = self.green,
            b = self.blue
        )
    }

    pub fn normalized(&self) -> (f32, f32, f32) {
        let (r, g, b) = (self.red as f32, self.green as f32, self.blue as f32);
        (r / 255f32, g / 255f32, b / 255f32)
    }
}
