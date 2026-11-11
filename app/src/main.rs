//! Homeport's desktop shell, on iced.

use iced::widget::text;
use iced::Element;

fn main() -> iced::Result {
    iced::application("Homeport", update, view).run()
}

#[derive(Debug, Clone)]
enum Message {}

fn update(_: &mut (), message: Message) {
    match message {}
}

fn view(_: &()) -> Element<'_, Message> {
    text("Homeport").into()
}
