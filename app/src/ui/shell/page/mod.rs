//! What a page stands on: the [`Context`] it is given, the [`World`] it
//! reads, and [`Pages`], the one interface through which the shell opens,
//! updates and draws whatever feature the route names.

pub mod context;
pub mod stub;
#[cfg(test)]
pub mod testing;
pub mod world;

use iced::{Subscription, Task};

use crate::ui::{theme::Element, widgets::overlay::Modal};

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

    /// The dialog the page has open, if any: a confirmation, a form. The
    /// shell lays it over the whole window (the page itself is in a scroll
    /// area beside the sidebar, where a dialog can't be), so a page never
    /// calls `overlay::over` itself.
    fn dialog<'a>(&'a self, ctx: &Context<'a>) -> Option<Modal<'a, Self::Message>>;

    /// Starts one of the page's actions, as if its button was pressed: what
    /// a palette command or a page shortcut asks for (`useActionRequest`).
    fn request(&mut self, action: &str, ctx: &Context<'_>) -> Task<Self::Message>;

    /// What the page reads or the Hosts may have changed: the page watches
    /// what is there now and takes what its view needs from it. Called
    /// after every message of the window while the page stays, so it does
    /// nothing unless something it reads is another value than last time.
    fn refresh(&mut self, ctx: &Context<'_>) -> Task<Self::Message>;

    /// The streams the page follows while it shows (`data::streams`): each
    /// runs while this returns it and is dropped, so stopped, when it no
    /// longer does. What React's `subscribe()` in an effect was.
    fn subscription(&self, ctx: &Context<'_>) -> Subscription<Self::Message>;
}
