use crate::options::ConsoleOptions;
use crate::Console;

// TODO: Use an Iterator to correct type instead of vec of nothing
// pub type RenderResult = dyn Iterator<Item = dyn Renderable>;
pub type RenderResult = Vec<()>;

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
    fn rich(&self) -> dyn ConsoleRenderable;
}
