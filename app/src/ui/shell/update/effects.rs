//! What the shell does of what a page asked during its last call.

use iced::Task;

use super::super::{page::context::Effect, page::Pages, Event, Message, Shell};
use crate::data::Data;

impl<P: Pages> Shell<P> {
    /// Does what the page asked for during its last call.
    pub(super) fn run_effects(&mut self, data: &mut Data) -> Task<Message<P::Message>> {
        let mut tasks = vec![];
        for effect in self.effects.take() {
            tasks.push(match effect {
                Effect::Open(request) => self.go(request, data),
                Effect::Back => {
                    self.history.back();
                    Task::none()
                }
                Effect::Select(selection) => self.select(selection),
                Effect::Watch(key, start) => {
                    let Some(open) = &mut self.page else {
                        continue;
                    };
                    if open.watched.contains(&key) {
                        continue;
                    }
                    open.watched.push(key);
                    start(data, &self.state).map(Message::Data)
                }
                Effect::Unwatch(key) => {
                    // Only what this page watched is its to let go of.
                    if let Some(open) = &mut self.page {
                        if open.watched.contains(&key) {
                            open.watched.retain(|watched| *watched != key);
                            data.unwatch(&key);
                        }
                    }
                    Task::none()
                }
                Effect::Invalidate(scope) => {
                    data.invalidate(&self.state, &scope).map(Message::Data)
                }
                Effect::Failed(title, surfaced) => {
                    self.events.push(Event::Failed(title, surfaced));
                    Task::none()
                }
                Effect::Said(kind, said) => {
                    self.events.push(Event::Said(kind, said));
                    Task::none()
                }
                Effect::OpenUrl(url) => {
                    self.events.push(Event::OpenUrl(url));
                    Task::none()
                }
                Effect::Put(_, put) => {
                    put(data);
                    Task::none()
                }
                Effect::Keep { query, params } => {
                    self.history.amend(query, params);
                    Task::none()
                }
                Effect::KeepRoute(route) => {
                    // Another feature's route would be another page.
                    if route.feature() == self.location().route.feature() {
                        self.history.retarget(route);
                    }
                    Task::none()
                }
                Effect::Details(surfaced) => {
                    self.events.push(Event::Details(surfaced));
                    Task::none()
                }
                Effect::Copy(text) => {
                    self.events.push(Event::Copy(text));
                    Task::none()
                }
                Effect::Picker(message) => self.pick(message, data),
            });
        }
        Task::batch(tasks)
    }
}
