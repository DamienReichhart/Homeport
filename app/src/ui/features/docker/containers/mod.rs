//! `src/features/containers`: the list of a Host's containers ([`list`]),
//! and a container's page, which is a stub until its ticket ports it
//! (#258): the feature has both routes, so its state is one or the other.

mod list;

use iced::{Subscription, Task};

use crate::ui::{
    shell::{router::Route, stub, Context},
    theme::Element,
    widgets::overlay::Modal,
};

#[derive(Debug)]
pub enum State {
    List(Box<list::State>),
    /// A container's page (`containers/:id/:tab?`).
    Container(stub::State),
}

#[derive(Debug, Clone)]
pub enum Message {
    List(list::Message),
    Container(stub::Message),
}

pub fn open(ctx: &Context<'_>) -> (State, Task<Message>) {
    if matches!(ctx.location.route, Route::Containers) {
        let (state, task) = list::open(ctx);
        (State::List(Box::new(state)), task.map(Message::List))
    } else {
        let (state, task) = stub::open(ctx);
        (State::Container(state), task.map(Message::Container))
    }
}

pub fn update(state: &mut State, message: Message, ctx: &Context<'_>) -> Task<Message> {
    match (state, message) {
        (State::List(state), Message::List(message)) => {
            list::update(state, message, ctx).map(Message::List)
        }
        (State::Container(state), Message::Container(message)) => {
            stub::update(state, message, ctx).map(Message::Container)
        }
        // A message of the other page, which was left meanwhile.
        _ => Task::none(),
    }
}

pub fn view<'a>(state: &'a State, ctx: &Context<'a>) -> Element<'a, Message> {
    match state {
        State::List(state) => list::view(state, ctx).map(Message::List),
        State::Container(state) => stub::view(state, ctx).map(Message::Container),
    }
}

pub fn dialog<'a>(state: &'a State, ctx: &Context<'a>) -> Option<Modal<'a, Message>> {
    match state {
        State::List(state) => list::dialog(state, ctx).map(|modal| modal.map(Message::List)),
        State::Container(state) => {
            stub::dialog(state, ctx).map(|modal| modal.map(Message::Container))
        }
    }
}

pub fn request(state: &mut State, action: &str, ctx: &Context<'_>) -> Task<Message> {
    match state {
        State::List(state) => list::request(state, action, ctx).map(Message::List),
        State::Container(state) => stub::request(state, action, ctx).map(Message::Container),
    }
}

pub fn refresh(state: &mut State, ctx: &Context<'_>) -> Task<Message> {
    match state {
        State::List(state) => list::refresh(state, ctx).map(Message::List),
        State::Container(state) => stub::refresh(state, ctx).map(Message::Container),
    }
}

pub fn subscription(state: &State, ctx: &Context<'_>) -> Subscription<Message> {
    match state {
        State::List(state) => list::subscription(state, ctx).map(Message::List),
        State::Container(state) => stub::subscription(state, ctx).map(Message::Container),
    }
}
