use crate::options::ConsoleOptions;
use crate::Console;

pub type RenderResult = dyn Iterator<Item = dyn Renderable>;

pub trait Renderable {
    fn rich_console(&self, console: &Console, options: &ConsoleOptions) -> RenderResult;
}

trait ConsoleRenderable {
    fn rich_console(&self) -> RenderResult;
}

trait RichCast {
    fn rich(&self) -> dyn ConsoleRenderable;
}
