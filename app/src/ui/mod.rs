//! The window: its state, its messages and what it draws.

mod hosts;

use std::sync::{Arc, Mutex, OnceLock};

use iced::{
    futures::{stream, Stream},
    widget::{column, container, text},
    Element, Length, Subscription, Task,
};
use tokio::sync::mpsc::{self, UnboundedReceiver};

use crate::app::{events::{Event, Events}, setup, AppState};

pub fn run() -> iced::Result {
    iced::application(Homeport::boot, Homeport::update, Homeport::view)
        .title("Homeport")
        .subscription(Homeport::subscription)
        .window_size((1100.0, 720.0))
        .run()
}

/// The receiving half of the backend's events, taken once by the subscription.
static EVENTS: OnceLock<Mutex<Option<UnboundedReceiver<Event>>>> = OnceLock::new();

enum Phase {
    Starting,
    Failed(String),
    Ready(Arc<AppState>),
}

struct Homeport {
    phase: Phase,
    hosts: hosts::Hosts,
}

#[derive(Debug, Clone)]
pub enum Message {
    Started(Result<Arc<AppState>, String>),
    Backend(Event),
    Hosts(hosts::Message),
}

impl Homeport {
    fn boot() -> (Self, Task<Message>) {
        let (sender, receiver) = mpsc::unbounded_channel();
        let _ = EVENTS.set(Mutex::new(Some(receiver)));
        let events = Events::new(sender);
        let start = Task::perform(
            async move {
                match setup::state(&events) {
                    Ok(state) => Ok(Arc::new(state)),
                    Err(setup::Failure::Database(e, path)) => {
                        Err(format!("{} ({})", e.message, path.display()))
                    }
                    Err(setup::Failure::Other(e)) => Err(e.to_string()),
                }
            },
            Message::Started,
        );
        (
            Self {
                phase: Phase::Starting,
                hosts: hosts::Hosts::default(),
            },
            start,
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Started(Ok(state)) => {
                self.phase = Phase::Ready(state.clone());
                self.hosts.load(&state).map(Message::Hosts)
            }
            Message::Started(Err(e)) => {
                self.phase = Phase::Failed(e);
                Task::none()
            }
            Message::Backend(event) => {
                self.hosts.on_event(&event);
                Task::none()
            }
            Message::Hosts(message) => {
                let Phase::Ready(state) = &self.phase else {
                    return Task::none();
                };
                self.hosts.update(state, message).map(Message::Hosts)
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let body: Element<'_, Message> = match &self.phase {
            Phase::Starting => text("Starting Homeport…").into(),
            Phase::Failed(e) => text(format!("Homeport couldn't start: {e}")).into(),
            Phase::Ready(_) => self.hosts.view().map(Message::Hosts),
        };
        container(column![body].spacing(12))
            .padding(16)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::run(backend_events).map(Message::Backend)
    }
}

fn backend_events() -> impl Stream<Item = Event> {
    let receiver = EVENTS
        .get()
        .and_then(|slot| slot.lock().ok()?.take());
    stream::unfold(receiver, |receiver| async move {
        let mut receiver = receiver?;
        let event = receiver.recv().await?;
        Some((event, Some(receiver)))
    })
}
