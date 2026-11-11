//! Homeport's desktop shell, on iced.

use iced::widget::text;
use iced::Element;

fn main() -> iced::Result {
    iced::application(State::default, update, view)
        .title("Homeport")
        .run()
}

#[derive(Default)]
struct State;

#[derive(Debug, Clone)]
enum Message {}

fn update(_: &mut State, message: Message) {
    match message {}
}

fn view(_: &State) -> Element<'_, Message> {
    text("Homeport").into()
}
