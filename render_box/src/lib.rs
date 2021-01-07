use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use lazy_static::lazy_static;

use console::options::ConsoleOptions;
use utils::iter;

pub enum RenderBoxLevel {
    Head,
    Row,
    Foot,
    Mid,
}

/// Defines characters to render boxes.
/// ┌─┬┐ top
/// │ ││ head
/// ├─┼┤ head_row
/// │ ││ mid
/// ├─┼┤ row
/// ├─┼┤ foot_row
/// │ ││ foot
/// └─┴┘ bottom
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct RenderBox {
    inner_box: String,
    ascii: bool,
    top: String,
    top_divider: String,
    top_left: String,
    top_right: String,
    head_left: String,
    head_vertical: String,
    head_right: String,
    head_row_left: String,
    head_row_horizontal: String,
    head_row_cross: String,
    head_row_right: String,
    mid_left: String,
    mid_vertical: String,
    mid_right: String,
    row_left: String,
    row_horizontal: String,
    row_cross: String,
    row_right: String,
    foot_row_left: String,
    foot_row_horizontal: String,
    foot_row_cross: String,
    foot_row_right: String,
    foot_left: String,
    foot_vertical: String,
    foot_right: String,
    bottom: String,
    bottom_divider: String,
    bottom_left: String,
    bottom_right: String,
}

impl RenderBox {
    pub fn new(inner_box: &str, ascii: Option<bool>) -> Self {
        println!("{}", inner_box);
        let ascii = ascii.unwrap_or(false);

        let mut lines = inner_box.split('\n');
        let (top, head, head_row, mid, row, foot_row, foot, bottom) = (
            lines.next().expect("top line"),
            lines.next().expect("head line"),
            lines.next().expect("head row line"),
            lines.next().expect("mid line"),
            lines.next().expect("row line"),
            lines.next().expect("foot_row line"),
            lines.next().expect("foot line"),
            lines.next().expect("bottom line"),
        );

        let inner_box = inner_box.to_string();

        let mut top = top.chars().map(|c| c.to_string());
        let (top_left, top, top_divider, top_right) = (
            top.next().expect("top left"),
            top.next().expect("top"),
            top.next().expect("top divider"),
            top.next().expect("top right"),
        );

        let mut head = head.chars().map(|c| c.to_string());
        let (head_left, _, head_vertical, head_right) = (
            head.next().expect("head left"),
            head.next().expect("head space"),
            head.next().expect("head vertical"),
            head.next().expect("head right"),
        );

        let mut head_row = head_row.chars().map(|c| c.to_string());
        let (head_row_left, head_row_horizontal, head_row_cross, head_row_right) = (
            head_row.next().expect("head row left"),
            head_row.next().expect("head row horizontal"),
            head_row.next().expect("head row cross"),
            head_row.next().expect("head row right"),
        );

        let mut mid = mid.chars().map(|c| c.to_string());
        let (mid_left, _, mid_vertical, mid_right) = (
            mid.next().expect("mid left"),
            mid.next().expect("mid space"),
            mid.next().expect("mid vertical"),
            mid.next().expect("mid right"),
        );

        let mut row = row.chars().map(|c| c.to_string());
        let (row_left, row_horizontal, row_cross, row_right) = (
            row.next().expect("row left"),
            row.next().expect("row horizontal"),
            row.next().expect("row cross"),
            row.next().expect("row right"),
        );

        let mut foot_row = foot_row.chars().map(|c| c.to_string());
        let (foot_row_left, foot_row_horizontal, foot_row_cross, foot_row_right) = (
            foot_row.next().expect("foot row left"),
            foot_row.next().expect("foot row horizontal"),
            foot_row.next().expect("foot row cross"),
            foot_row.next().expect("foot row right"),
        );

        let mut foot = foot.chars().map(|c| c.to_string());
        let (foot_left, _, foot_vertical, foot_right) = (
            foot.next().expect("foot left"),
            foot.next().expect("foot space"),
            foot.next().expect("foot vertical"),
            foot.next().expect("foot right"),
        );

        let mut bottom = bottom.chars().map(|c| c.to_string());
        let (bottom_left, bottom, bottom_divider, bottom_right) = (
            bottom.next().expect("bottom left"),
            bottom.next().expect("bottom"),
            bottom.next().expect("bottom divider"),
            bottom.next().expect("bottom right"),
        );

        Self {
            inner_box,
            ascii,
            top,
            top_divider,
            top_left,
            top_right,
            head_left,
            head_vertical,
            head_right,
            head_row_left,
            head_row_horizontal,
            head_row_cross,
            head_row_right,
            mid_left,
            mid_vertical,
            mid_right,
            row_left,
            row_horizontal,
            row_cross,
            row_right,
            foot_row_left,
            foot_row_horizontal,
            foot_row_cross,
            foot_row_right,
            foot_left,
            foot_vertical,
            foot_right,
            bottom,
            bottom_divider,
            bottom_left,
            bottom_right,
        }
    }

    /// Substitute this box for another if it won't render due to platform issues
    pub fn substitute(&self, options: &ConsoleOptions, safe: Option<bool>) -> RenderBox {
        let safe = safe.unwrap_or(true);

        let mut new_render_box = self.clone();

        if options.legacy_windows && safe {
            new_render_box = LEGACY_WINDOWS_SUBSTITUTIONS
                .get(&new_render_box)
                .cloned()
                .unwrap_or(new_render_box);
        }

        if options.ascii_only() && !new_render_box.ascii {
            new_render_box = ASCII.clone();
        }
        new_render_box
    }

    /// Get the top of a simple box
    pub fn get_top<'a, Widths>(&self, widths: Widths) -> String
    where
        Widths: IntoIterator<Item = &'a usize>,
    {
        let mut parts = vec![self.top_left.clone()];
        for (last, width) in iter::loop_last(widths.into_iter()) {
            parts.push(self.top.repeat(*width));
            if !last {
                parts.push(self.top_divider.clone());
            }
        }
        parts.push(self.top_right.clone());
        parts.join("")
    }

    /// Get the row of a simple box
    pub fn get_row<'a, Widths>(
        &self,
        withds: Widths,
        level: Option<RenderBoxLevel>,
        edge: Option<bool>,
    ) -> String
    where
        Widths: IntoIterator<Item = &'a usize>,
    {
        let edge = edge.unwrap_or(true);
        let level = level.unwrap_or(RenderBoxLevel::Row);
        let space = " ".to_string();
        let (left, horizontal, cross, right) = match level {
            RenderBoxLevel::Head => (
                &self.head_row_left,
                &self.head_row_horizontal,
                &self.head_row_cross,
                &self.head_row_right,
            ),
            RenderBoxLevel::Row => (
                &self.row_left,
                &self.row_horizontal,
                &self.row_cross,
                &self.row_right,
            ),
            RenderBoxLevel::Foot => (
                &self.foot_row_left,
                &self.foot_row_horizontal,
                &self.foot_row_cross,
                &self.foot_row_right,
            ),
            RenderBoxLevel::Mid => (&self.mid_left, &space, &self.mid_vertical, &self.mid_right),
        };

        let mut parts = Vec::new();
        if edge {
            parts.push(left.clone());
        }
        for (last, width) in iter::loop_last(withds.into_iter()) {
            parts.push(horizontal.repeat(*width));
            if !last {
                parts.push(cross.to_string());
            }
        }
        if edge {
            parts.push(right.to_string());
        }
        parts.join("")
    }

    pub fn get_bottom<'a, Widths>(&self, widths: Widths) -> String
    where
        Widths: IntoIterator<Item = &'a usize>,
    {
        let mut parts = vec![self.bottom_left.clone()];
        for (last, width) in iter::loop_last(widths.into_iter()) {
            parts.push(self.bottom.repeat(*width));
            if !last {
                parts.push(self.bottom_divider.clone());
            }
        }
        parts.push(self.bottom_right.clone());
        parts.join("")
    }
}

impl Display for RenderBox {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner_box)
    }
}

lazy_static! {
    pub static ref ASCII: RenderBox = RenderBox::new(
        "\
+--+
| ||
|-+|
| ||
|-+|
|-+|
| ||
+--+
",
        Some(true)
    );
    pub static ref ASCII2: RenderBox = RenderBox::new(
        "\
+-++
| ||
+-++
| ||
+-++
+-++
| ||
+-++
",
        Some(true)
    );
    pub static ref ASCII_DOUBLE_HEAD: RenderBox = RenderBox::new(
        "\
+-++
| ||
+=++
| ||
+-++
+-++
| ||
+-++
",
        Some(true)
    );
    pub static ref SQUARE: RenderBox = RenderBox::new(
        "\
┌─┬┐
│ ││
├─┼┤
│ ││
├─┼┤
├─┼┤
│ ││
└─┴┘
",
        None
    );
    pub static ref SQUARE_DOUBLE_HEAD: RenderBox = RenderBox::new(
        "\
┌─┬┐
│ ││
╞═╪╡
│ ││
├─┼┤
├─┼┤
│ ││
└─┴┘
",
        None
    );
    pub static ref MINIMAL: RenderBox = RenderBox::new(
        "
  ╷ 
  │ 
╶─┼╴
  │ 
╶─┼╴
╶─┼╴
  │ 
  ╵ 
"
        .trim_start_matches("\n"),
        None
    );
    pub static ref MINIMAL_HEAVY_HEAD: RenderBox = RenderBox::new(
        "
  ╷ 
  │ 
╺━┿╸
  │ 
╶─┼╴
╶─┼╴
  │ 
  ╵ 
"
        .trim_start_matches("\n"),
        None
    );
    pub static ref MINIMAL_DOUBLE_HEAD: RenderBox = RenderBox::new(
        "
  ╷ 
  │ 
 ═╪ 
  │ 
 ─┼ 
 ─┼ 
  │ 
  ╵ 
"
        .trim_start_matches("\n"),
        None
    );
    pub static ref SIMPLE: RenderBox = RenderBox::new(
        "
    
    
 ── 
    
    
 ── 
    
    
"
        .trim_start_matches("\n"),
        None
    );
    pub static ref SIMPLE_HEAD: RenderBox = RenderBox::new(
        "
    
    
 ── 
    
    
    
    
    
"
        .trim_start_matches("\n"),
        None
    );
    pub static ref SIMPLE_HEAVY: RenderBox = RenderBox::new(
        "
    
    
 ━━ 
    
    
 ━━ 
    
    
"
        .trim_start_matches("\n"),
        None
    );
    pub static ref HORIZONTALS: RenderBox = RenderBox::new(
        "
 ── 
    
 ── 
    
 ── 
 ── 
    
 ── 
"
        .trim_start_matches("\n"),
        None
    );
    pub static ref ROUNDED: RenderBox = RenderBox::new(
        "\
╭─┬╮
│ ││
├─┼┤
│ ││
├─┼┤
├─┼┤
│ ││
╰─┴╯
",
        None
    );
    pub static ref HEAVY: RenderBox = RenderBox::new(
        "\
┏━┳┓
┃ ┃┃
┣━╋┫
┃ ┃┃
┣━╋┫
┣━╋┫
┃ ┃┃
┗━┻┛
",
        None
    );
    pub static ref HEAVY_EDGE: RenderBox = RenderBox::new(
        "\
┏━┯┓
┃ │┃
┠─┼┨
┃ │┃
┠─┼┨
┠─┼┨
┃ │┃
┗━┷┛
",
        None
    );
    pub static ref HEAVY_HEAD: RenderBox = RenderBox::new(
        "\
┏━┳┓
┃ ┃┃
┡━╇┩
│ ││
├─┼┤
├─┼┤
│ ││
└─┴┘
",
        None
    );
    pub static ref DOUBLE: RenderBox = RenderBox::new(
        "\
╔═╦╗
║ ║║
╠═╬╣
║ ║║
╠═╬╣
╠═╬╣
║ ║║
╚═╩╝
",
        None
    );
    pub static ref DOUBLE_EDGE: RenderBox = RenderBox::new(
        "\
╔═╤╗
║ │║
╟─┼╢
║ │║
╟─┼╢
╟─┼╢
║ │║
╚═╧╝
",
        None
    );
    pub static ref LEGACY_WINDOWS_SUBSTITUTIONS: HashMap<RenderBox, RenderBox> = {
        let mut m = HashMap::with_capacity(6);
        m.insert(ROUNDED.clone(), SQUARE.clone());
        m.insert(MINIMAL_HEAVY_HEAD.clone(), MINIMAL.clone());
        m.insert(SIMPLE_HEAVY.clone(), SIMPLE.clone());
        m.insert(HEAVY.clone(), SQUARE.clone());
        m.insert(HEAVY_EDGE.clone(), SQUARE.clone());
        m.insert(HEAVY_HEAD.clone(), SQUARE.clone());
        m
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use console::options::Encoding;
    use std::ops::Deref;

    #[test]
    fn test_string() {
        assert_eq!(
            ASCII.to_string(),
            "+--+\n| ||\n|-+|\n| ||\n|-+|\n|-+|\n| ||\n+--+\n"
        );
    }

    #[test]
    fn test_get_top() {
        assert_eq!(HEAVY.get_top(&[1, 2]), "┏━┳━━┓");
    }

    #[test]
    fn test_get_row() {
        assert_eq!(
            DOUBLE.get_row(&[3, 2, 1], Some(RenderBoxLevel::Head), None),
            "╠═══╬══╬═╣"
        );

        assert_eq!(
            ASCII.get_row(&[1, 2, 3], Some(RenderBoxLevel::Row), None),
            "|-+--+---|"
        );

        assert_eq!(
            ROUNDED.get_row(&[2, 1, 3], Some(RenderBoxLevel::Foot), None),
            "├──┼─┼───┤"
        );
    }

    #[test]
    fn test_get_bottom() {
        assert_eq!(HEAVY.get_bottom(&[1, 2, 3]), "┗━┻━━┻━━━┛");
    }

    // TODO: Use a macro to generate independent tests for each of them
    #[test]
    fn test_static_boxes_build() {
        ASCII.clone();
        ASCII2.clone();
        ASCII_DOUBLE_HEAD.clone();
        SQUARE.clone();
        SQUARE_DOUBLE_HEAD.clone();
        MINIMAL.clone();
        MINIMAL_HEAVY_HEAD.clone();
        MINIMAL_DOUBLE_HEAD.clone();
        SIMPLE.clone();
        SIMPLE_HEAD.clone();
        SIMPLE_HEAVY.clone();
        HORIZONTALS.clone();
        ROUNDED.clone();
        HEAVY.clone();
        HEAVY_EDGE.clone();
        HEAVY_HEAD.clone();
        DOUBLE.clone();
        DOUBLE_EDGE.clone();
    }

    #[test]
    fn test_box_substitute() {
        let mut options = ConsoleOptions {
            legacy_windows: true,
            min_width: 1,
            max_width: 100,
            is_terminal: true,
            encoding: Encoding::new("utf-8"),
            justify: None,
            overflow: None,
            no_wrap: None,
            highlight: None,
        };

        assert_eq!(HEAVY.substitute(&options, None), SQUARE.clone());

        options.legacy_windows = false;
        assert_eq!(HEAVY.substitute(&options, None), HEAVY.clone());

        options.encoding = Encoding::new("ascii");
        assert_eq!(HEAVY.substitute(&options, None), ASCII.clone());
    }
}
