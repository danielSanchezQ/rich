use crate::triplet::ColortripletRaw;
use std::ops::Index;

/// A palette of available colors
pub struct Palette {
    colors: Vec<ColortripletRaw>,
}

impl Palette {
    pub fn new<Seq>(colors: Seq) -> Self
    where
        Seq: IntoIterator<Item = ColortripletRaw>,
    {
        Self {
            colors: colors.into_iter().collect(),
        }
    }

    pub const fn from_vec_of_colors(colors: Vec<ColortripletRaw>) -> Self {
        Self { colors }
    }

    pub fn match_color(&self, color: ColortripletRaw) -> Option<usize> {
        let (r1, g1, b1) = color;
        // use [euclidean distance](https://en.wikipedia.org/wiki/Color_difference#Euclidean)
        self.colors
            .iter()
            .enumerate()
            .min_by_key(|(_, item)| {
                let (r2, g2, b2) = item;
                let red = r1 as i32 - *r2 as i32;
                let green = g1 as i32 - *g2 as i32;
                let blue = b1 as i32 - *b2 as i32;
                red * red + green * green + blue * blue
            })
            .map(|(i, _)| i)
    }
}

impl Index<usize> for Palette {
    type Output = ColortripletRaw;

    fn index(&self, index: usize) -> &Self::Output {
        self.colors.index(index)
    }
}
