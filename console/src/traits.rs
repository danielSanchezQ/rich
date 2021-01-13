use crate::options::ConsoleOptions;
use crate::Console;
use segment::Segment;

// TODO: Use an Iterator to correct type instead of vec when RFC 2515 is implemented
// https://github.com/rust-lang/rust/issues/63063
//pub type RenderResult = impl Iterator<Item = &Segment>;
pub type RenderResult = Vec<Segment>;

pub trait Renderable {
    fn rich_console(&self, console: &Console, options: &ConsoleOptions) -> RenderResult;
}

impl<'a, T: Renderable> Renderable for &T {
    fn rich_console(&self, console: &Console, options: &ConsoleOptions) -> RenderResult {
        self.rich_console(console, options)
    }
}

trait ConsoleRenderable {
    fn rich_console(&self) -> RenderResult;
}

trait RichCast {
    fn rich(&self) -> Segment;
}

impl RichCast for &str {
    fn rich(&self) -> Segment {
        Segment::new(self.to_string().as_str(), None, false)
    }
}

impl Renderable for dyn ToString {
    fn rich_console(&self, console: &Console, options: &ConsoleOptions) -> RenderResult {
        // get console style
        // let style = console.style.clone;
        vec![Segment::new(self.to_string().as_str(), None, false)]
    }
}

impl Renderable for &str {
    fn rich_console(&self, console: &Console, options: &ConsoleOptions) -> RenderResult {
        vec![Segment::new(self.to_string().as_str(), None, false)]
    }
}
