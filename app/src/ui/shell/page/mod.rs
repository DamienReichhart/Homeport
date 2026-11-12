//! What a page stands on: the [`Context`] it is given, the [`World`] it
//! reads, and [`Pages`], the one interface through which the shell opens,
//! updates and draws whatever feature the route names.

pub mod context;
pub mod stub;
#[cfg(test)]
pub mod testing;
pub mod world;

use iced::Task;

use crate::ui::theme::Element;

pub use self::{
    context::{Context, Effects},
    world::{Update, World},
};

/// The pages of the app, behind one type: `ui::features::Page`, which has a
/// variant for each feature and sends each call to that feature's `open`,
/// `update`, `view` and `request`. The shell knows no feature by name.
pub trait Pages: Sized {
    type Message: std::fmt::Debug + Clone + Send + 'static;

    /// The page `ctx.location` leads to, once its gate lets it through,
    /// and what it starts by doing.
    fn open(ctx: &Context<'_>) -> (Self, Task<Self::Message>);

    fn update(&mut self, message: Self::Message, ctx: &Context<'_>) -> Task<Self::Message>;

    fn view<'a>(&'a self, ctx: &Context<'a>) -> Element<'a, Self::Message>;

    /// Starts one of the page's actions, as if its button was pressed: what
    /// a palette command or a page shortcut asks for (`useActionRequest`).
    fn request(&mut self, action: &str, ctx: &Context<'_>) -> Task<Self::Message>;
}
