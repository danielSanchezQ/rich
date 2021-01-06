use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use lazy_static::lazy_static;

use console::options::ConsoleOptions;
use utils::iter;

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
    pub fn substitute(&self, options: ConsoleOptions, safe: Option<bool>) -> RenderBox {
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

    pub fn get_top<Widths>(&self, widths: Widths) -> String
    where
        Widths: Iterator<Item = usize>,
    {
        let mut parts = vec![self.top_left.clone()];
        for (last, width) in iter::loop_last(widths) {
            parts.push(self.top.repeat(width));
            if last {
                parts.push(self.top_divider.clone());
            }
        }
        parts.push(self.top_right.clone());
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
+--+",
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
+-++",
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
+-++",
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
└─┴┘",
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
└─┴┘",
        None
    );
    pub static ref MINIMAL: RenderBox = RenderBox::new(
        "\
  ╷ 
  │ 
╶─┼╴
  │ 
╶─┼╴
╶─┼╴
  │ 
  ╵",
        None
    );
    pub static ref MINIMAL_HEAVY_HEAD: RenderBox = RenderBox::new(
        "\
  ╷ 
  │ 
╺━┿╸
  │ 
╶─┼╴
╶─┼╴
  │ 
  ╵",
        None
    );
    pub static ref MINIMAL_DOUBLE_HEAD: RenderBox = RenderBox::new(
        "\
  ╷ 
  │ 
 ═╪ 
  │ 
 ─┼ 
 ─┼ 
  │ 
  ╵",
        None
    );
    pub static ref SIMPLE: RenderBox = RenderBox::new(
        "\
    
    
 ── 
    
    
 ── 
    
    ",
        None
    );
    pub static ref SIMPLE_HEAD: RenderBox = RenderBox::new(
        "\
    
    
 ── 
    
    
    
    
    ",
        None
    );
    pub static ref SIMPLE_HEAVY: RenderBox = RenderBox::new(
        "\
    
    
 ━━ 
    
    
 ━━ 
    
   ",
        None
    );
    pub static ref HORIZONTALS: RenderBox = RenderBox::new(
        "\
 ── 
    
 ── 
    
 ── 
 ── 
    
 ── ",
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
╰─┴╯",
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
┗━┻┛",
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
┗━┷┛",
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
└─┴┘",
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
╚═╩╝",
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
╚═╧╝",
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
