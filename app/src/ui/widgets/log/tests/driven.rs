//! The viewer under the pointer and the keyboard, on a window that lasts:
//! the same widget tree from one view to the next, as the real window
//! keeps it.

use iced::{
    advanced::mouse,
    keyboard::{self, key::Named, Key, Modifiers},
    widget::container,
    Event as Input, Length, Point, Rectangle,
};

use super::numbered;
use crate::{
    test_support::{
        snapshot::{in_language, InLanguage},
        stage::{Frame, Stage},
    },
    ui::{
        theme,
        widgets::{
            form,
            gallery::{self, log::view},
            log::{Event, Line, State},
        },
    },
};

type Page = gallery::Message;

const WINDOW: (f32, f32) = (900.0, 540.0);
const NONE: Modifiers = Modifiers::empty();

/// A page with one viewer, on a window that lasts.
struct Bench {
    stage: Stage,
    state: State,
    /// What the viewer told the page, in order.
    events: Vec<Event>,
    _language: InLanguage,
}

impl Bench {
    fn new(lines: impl IntoIterator<Item = Line>) -> Self {
        let language = in_language("en");
        let mut state = State::new(100_000);
        drop(state.extend(lines));
        let mut bench = Self {
            stage: Stage::new(WINDOW),
            state,
            events: vec![],
            _language: language,
        };
        // The first view tells the viewer how much room it has.
        bench.frame(|_| {});
        bench
    }

    /// Draws the page, lets `act` use it, and gives the viewer what that
    /// produced, running what the viewer asks for in return on the views
    /// that follow.
    fn frame(&mut self, act: impl FnOnce(&mut Frame<'_, '_, Page>)) {
        fn page(state: &State) -> iced::widget::Container<'_, Page, theme::Theme> {
            container(view(state))
                .padding(24)
                .width(Length::Fill)
                .height(Length::Fill)
                .style(theme::container::page)
        }
        let mut messages = self.stage.frame(page(&self.state), act);
        while !messages.is_empty() {
            let mut tasks = vec![];
            for message in messages.drain(..) {
                if let Page::Log(message) = message {
                    let (task, event) = self.state.update(message);
                    self.events.extend(event);
                    tasks.push(task.map(Page::Log));
                }
            }
            // What a task asks of the widgets needs them: the next view.
            let mut produced = vec![];
            messages = self.stage.frame(page(&self.state), |frame| {
                for task in tasks {
                    produced.extend(frame.perform(task));
                }
            });
            messages.extend(produced);
        }
    }

    fn key(&mut self, key: impl Into<Key>, modifiers: Modifiers) {
        let key = key.into();
        self.frame(|frame| frame.send(press_of(key, modifiers)));
    }

    /// What the rows in view say.
    fn rows(&mut self) -> Vec<String> {
        let mut said = vec![];
        self.frame(|frame| said = frame.texts());
        said.into_iter()
            .filter(|text| text.starts_with("line "))
            .collect()
    }

    fn copied(&mut self) -> Option<String> {
        self.events.iter().rev().find_map(|event| match event {
            Event::Copy(text) => Some(text.clone()),
            _ => None,
        })
    }
}

fn press_of(key: Key, modifiers: Modifiers) -> [Input; 2] {
    let physical = keyboard::key::Physical::Unidentified(keyboard::key::NativeCode::Unidentified);
    let pressed = keyboard::Event::KeyPressed {
        key: key.clone(),
        modified_key: key.clone(),
        physical_key: physical,
        location: keyboard::Location::Standard,
        modifiers,
        text: None,
        repeat: false,
    };
    let released = keyboard::Event::KeyReleased {
        key: key.clone(),
        modified_key: key,
        physical_key: physical,
        location: keyboard::Location::Standard,
        modifiers,
    };
    [Input::Keyboard(pressed), Input::Keyboard(released)]
}

fn row_of(frame: &mut Frame<'_, '_, Page>, text: &str) -> Rectangle {
    frame
        .find(text)
        .unwrap_or_else(|| panic!("the row {text:?} is not in view"))
}

/// Presses the rows, so they have focus.
fn focus_rows(bench: &mut Bench) {
    bench.frame(|frame| {
        let row = row_of(frame, "line 199");
        frame.point_at(Point::new(row.x + 300.0, row.y + 8.0));
        frame.press();
    });
}

#[test]
fn the_first_view_tells_the_viewer_its_room_and_shows_the_latest_lines() {
    let mut bench = Bench::new(numbered(0, 200));
    let metrics = bench.state.metrics;
    assert!((100..=125).contains(&metrics.cols), "{metrics:?}");
    assert!((20..=24).contains(&metrics.rows), "{metrics:?}");
    let rows = bench.rows();
    assert_eq!(rows.len(), metrics.rows, "only the rows in view are there");
    assert_eq!(rows.last().map(String::as_str), Some("line 199"));
}

/// How much text a layout shapes, counted: numbers no machine changes.
#[test]
fn only_what_comes_into_view_is_shaped_however_many_lines_there_are() {
    use crate::ui::widgets::log::{shaped, Message, Scroll};
    let mut bench = Bench::new(numbered(0, 300_000));
    let rows = bench.state.metrics.rows;
    let during = |bench: &mut Bench, act: &dyn Fn(&mut State)| {
        act(&mut bench.state);
        let before = shaped();
        bench.frame(|_| {});
        shaped() - before
    };
    let scroll = |by: Scroll| move |state: &mut State| drop(state.update(Message::Scroll(by)));
    // Nothing changed: nothing is shaped, of 300,000 lines.
    assert_eq!(during(&mut bench, &|_| {}), 0);
    // A row up: one line of one piece comes in.
    assert_eq!(during(&mut bench, &scroll(Scroll::Rows(-1))), 1);
    assert_eq!(during(&mut bench, &scroll(Scroll::Rows(-3))), 3);
    // To the middle of the lines: a screen of them, and no more.
    assert_eq!(during(&mut bench, &scroll(Scroll::To(0.5))), rows);
    // Lines come while the view is paused: nothing in view changed.
    let more = |state: &mut State| drop(state.extend(numbered(300_000, 5_000)));
    assert_eq!(during(&mut bench, &more), 0);
    // A search marks its matches behind the text: the text is as it was.
    let search = |state: &mut State| drop(state.update(Message::Query("line 15".to_owned())));
    assert_eq!(during(&mut bench, &search), 0);
    // Following again, a batch replaces every row in view.
    assert_eq!(during(&mut bench, &scroll(Scroll::End)), rows);
    assert_eq!(during(&mut bench, &more), rows);
}

#[test]
fn the_lines_a_stream_pushes_show_as_they_come_whatever_their_stream() {
    use homeport_core::logs::LogLine;
    let mut bench = Bench::new([]);
    let line = |stream: &str, text: &str| LogLine {
        source: None,
        stream: stream.to_owned(),
        text: text.to_owned(),
    };
    // What the React test pushed through its channel.
    let batch = [
        line("stdout", "listening on :80"),
        line("stderr", "warning: slow"),
    ];
    drop(bench.state.extend(batch.map(Line::from)));
    bench.frame(|frame| {
        assert!(frame.find("listening on :80").is_some());
        assert!(frame.find("warning: slow").is_some());
    });
    // The source said it has no more.
    bench.state.end();
    bench.frame(|frame| assert!(frame.find("\u{2014} end of logs \u{2014}").is_some()));
}

#[test]
fn the_wheel_pauses_and_jump_to_latest_follows_again() {
    let mut bench = Bench::new(numbered(0, 200));
    bench.frame(|frame| {
        let row = row_of(frame, "line 190");
        frame.point_at(row.center());
        frame.scroll(-1.0);
    });
    assert!(!bench.state.follows());
    let paused = bench.rows();
    assert_eq!(paused.last().map(String::as_str), Some("line 196"));
    // Lines come: the view stays, and the way back shows.
    drop(bench.state.extend(numbered(200, 300)));
    assert_eq!(bench.rows(), paused);
    bench.frame(|frame| frame.click("Jump to latest"));
    assert!(bench.state.follows());
    assert_eq!(bench.rows().last().map(String::as_str), Some("line 499"));
    bench.frame(|frame| assert!(frame.find("Jump to latest").is_none()));
    // Scrolled back down to the end with the wheel, it follows too.
    bench.frame(|frame| {
        let row = row_of(frame, "line 490");
        frame.point_at(row.center());
        frame.scroll(-2.0);
    });
    assert!(!bench.state.follows());
    bench.frame(|frame| frame.scroll(5.0));
    assert!(bench.state.follows());
}

#[test]
fn the_follow_toggle_pauses_and_follows() {
    let mut bench = Bench::new(numbered(0, 200));
    bench.frame(|frame| frame.click("Follow"));
    assert!(!bench.state.follows());
    drop(bench.state.extend(numbered(200, 50)));
    assert_eq!(bench.rows().last().map(String::as_str), Some("line 199"));
    bench.frame(|frame| frame.click("Follow"));
    assert!(bench.state.follows());
    assert_eq!(bench.rows().last().map(String::as_str), Some("line 249"));
}

#[test]
fn a_drag_selects_text_and_ctrl_c_copies_it() {
    let mut bench = Bench::new(numbered(0, 200));
    bench.frame(|frame| {
        let (from, to) = (row_of(frame, "line 195"), row_of(frame, "line 197"));
        // From before the fifth cell of one row to the middle of another.
        frame.point_at(Point::new(from.x + 5.0 * 7.2, from.y + 8.0));
        frame.send([Input::Mouse(mouse::Event::ButtonPressed(
            mouse::Button::Left,
        ))]);
        frame.point_at(Point::new(to.x + 4.0 * 7.2, to.y + 8.0));
        frame.send([Input::Mouse(mouse::Event::ButtonReleased(
            mouse::Button::Left,
        ))]);
    });
    assert!(bench.state.has_selection());
    bench.key(Key::Character("c".into()), Modifiers::COMMAND);
    assert_eq!(bench.copied().as_deref(), Some("195\nline 196\nline"));
    // The pointer moving with no button held selects nothing more.
    bench.frame(|frame| {
        let row = row_of(frame, "line 190");
        frame.point_at(row.center());
    });
    bench.key(Key::Character("c".into()), Modifiers::COMMAND);
    assert_eq!(bench.copied().as_deref(), Some("195\nline 196\nline"));
    // Escape lets go of it.
    bench.key(Named::Escape, NONE);
    assert!(!bench.state.has_selection());
}

#[test]
fn two_presses_take_a_word_three_a_line_and_shift_takes_it_further() {
    let mut bench = Bench::new(numbered(0, 200));
    let on = |bench: &mut Bench, text: &str, presses: usize| {
        bench.frame(|frame| {
            let row = row_of(frame, text);
            frame.point_at(Point::new(row.x + 5.4 * 7.2, row.y + 8.0));
            for _ in 0..presses {
                frame.press();
            }
        });
    };
    on(&mut bench, "line 195", 2);
    bench.key(Key::Character("c".into()), Modifiers::COMMAND);
    assert_eq!(bench.copied().as_deref(), Some("195"));
    on(&mut bench, "line 195", 3);
    bench.key(Key::Character("c".into()), Modifiers::COMMAND);
    assert_eq!(bench.copied().as_deref(), Some("line 195\n"));
    // A press, then one with Shift two rows down.
    on(&mut bench, "line 190", 1);
    bench.frame(|frame| {
        frame.send([Input::Keyboard(keyboard::Event::ModifiersChanged(
            Modifiers::SHIFT,
        ))]);
    });
    on(&mut bench, "line 192", 1);
    bench.key(Key::Character("c".into()), Modifiers::COMMAND);
    assert_eq!(bench.copied().as_deref(), Some("190\nline 191\nline "));
}

#[test]
fn the_keyboard_scrolls_selects_and_copies_once_the_rows_have_focus() {
    let mut bench = Bench::new(numbered(0, 200));
    let rows = bench.state.metrics.rows;
    // Without focus, the rows hear no key.
    bench.key(Named::PageUp, NONE);
    assert!(bench.state.follows());
    // Tab reaches them, after the controls: they are one stop.
    bench.frame(|frame| {
        for _ in 0..16 {
            frame.tab();
            if frame.focused().is_some_and(|bounds| bounds.height > 200.0) {
                return;
            }
        }
        panic!("Tab never reached the rows");
    });
    bench.frame(|frame| assert!(frame.shows_focus(), "the keyboard gave it: a ring"));
    bench.key(Named::ArrowUp, NONE);
    assert!(!bench.state.follows());
    assert_eq!(bench.rows().last().map(String::as_str), Some("line 198"));
    bench.key(Named::PageUp, NONE);
    let last = 198 - (rows - 1);
    assert_eq!(bench.rows().last().cloned(), Some(format!("line {last}")));
    bench.key(Named::ArrowDown, NONE);
    bench.key(Named::PageDown, NONE);
    assert!(bench.state.follows(), "back at the end");
    bench.key(Named::Home, NONE);
    assert_eq!(bench.rows().first().map(String::as_str), Some("line 0"));
    bench.key(Named::End, NONE);
    assert!(bench.state.follows());
    // Everything, then the clipboard.
    bench.key(Key::Character("a".into()), Modifiers::COMMAND);
    bench.key(Key::Character("c".into()), Modifiers::COMMAND);
    let text = bench.copied().expect("copied");
    assert_eq!(text.lines().count(), 200);
    // A key the rows don't use is left to the page.
    bench.frame(|frame| {
        frame.send(press_of(Key::Character("n".into()), NONE));
    });
}

#[test]
fn ctrl_f_goes_to_the_search_and_enter_steps_through_the_matches() {
    let mut bench = Bench::new(numbered(0, 200));
    focus_rows(&mut bench);
    bench.key(Key::Character("f".into()), Modifiers::COMMAND);
    // What is typed now goes to the field.
    bench.frame(|frame| frame.typewrite("line 19"));
    assert_eq!(bench.state.query().text, "line 19");
    assert_eq!(bench.state.matches(), (None, 11));
    // Enter: the next match, from the top of the view; with Shift, back.
    bench.key(Named::Enter, NONE);
    assert_eq!(bench.state.matches().0, Some(2), "line 190, after line 19");
    bench.key(Named::Enter, NONE);
    assert_eq!(bench.state.matches().0, Some(3));
    bench.key(Named::Enter, Modifiers::SHIFT);
    assert_eq!(bench.state.matches().0, Some(2));
    bench.frame(|frame| assert!(frame.find("2 of 11").is_some()));
    // The Previous button, reached with Tab past the three toggles.
    bench.frame(|frame| {
        for _ in 0..4 {
            frame.tab();
        }
        frame.tap_key(Named::Enter);
    });
    assert_eq!(bench.state.matches().0, Some(1));
    assert!(
        bench.rows().contains(&"line 19".to_owned()),
        "brought into view"
    );
    assert!(!bench.state.follows());
}

#[test]
fn the_search_field_is_what_slash_gives_focus_to() {
    let mut bench = Bench::new(numbered(0, 50));
    let id = bench.state.search_id();
    bench.frame(|frame| {
        frame.perform(form::focus_search::<Page>(id));
        frame.typewrite("line 4");
    });
    assert_eq!(bench.state.query().text, "line 4");
    assert_eq!(bench.state.matches().1, 11);
}

#[test]
fn the_toggles_change_how_the_lines_show() {
    let stamped = (0..40).map(|n| {
        Line::new(format!("2026-05-01T10:00:{n:02}Z {}", "word ".repeat(40))).timestamped(true)
    });
    let mut bench = Bench::new(stamped);
    let stamp = "2026-05-01T10:00:39Z";
    let starts_with_stamp = |bench: &mut Bench| {
        let mut said = vec![];
        bench.frame(|frame| said = frame.texts());
        said.iter().any(|text| text.starts_with(stamp))
    };
    assert!(!starts_with_stamp(&mut bench));
    bench.frame(|frame| frame.click("Timestamps"));
    assert!(bench.state.shows_timestamps());
    assert!(starts_with_stamp(&mut bench));
    assert!(matches!(bench.events.last(), Some(Event::Timestamps(true))));
    // Wrapped, a line of 220 cells is two rows; not wrapped, one.
    let rows = |bench: &mut Bench| {
        let mut said = vec![];
        bench.frame(|frame| said = frame.texts());
        said.iter().filter(|text| text.contains("word")).count()
    };
    let wrapped = rows(&mut bench);
    bench.frame(|frame| frame.click("Wrap lines"));
    assert!(!bench.state.wraps());
    assert!(starts_with_stamp(&mut bench));
    assert_eq!(rows(&mut bench), bench.state.metrics.rows);
    assert!(wrapped <= bench.state.metrics.rows + 1);
}

#[test]
fn the_thumb_dragged_goes_through_the_lines() {
    let mut bench = Bench::new(numbered(0, 100_000));
    let hold = |frame: &mut Frame<'_, '_, Page>, at: Point, to: Point| {
        frame.point_at(at);
        frame.send([Input::Mouse(mouse::Event::ButtonPressed(
            mouse::Button::Left,
        ))]);
        frame.point_at(to);
        frame.send([Input::Mouse(mouse::Event::ButtonReleased(
            mouse::Button::Left,
        ))]);
    };
    // The track is past the right edge of the rows and their padding.
    let rail = |row: Rectangle| row.x + row.width + 8.0 + 5.0;
    let rows = bench.state.metrics.rows;
    // A press in the middle of the track brings the thumb there.
    let mut middle = Point::ORIGIN;
    bench.frame(|frame| {
        let last = row_of(frame, "line 99999");
        // Half the rows above the last one.
        #[allow(clippy::cast_precision_loss)]
        let up = (rows / 2) as f32 * 16.0;
        middle = Point::new(rail(last), last.y - up);
        hold(frame, middle, middle);
    });
    assert!(!bench.state.follows());
    let first = bench.rows().first().cloned().expect("rows");
    let at: usize = first["line ".len()..].parse().expect("a number");
    assert!((30_000..70_000).contains(&at), "{first}");
    // Held and dragged past the top: the first line; past the bottom:
    // the latest, followed again.
    bench.frame(|frame| hold(frame, middle, Point::new(middle.x, -500.0)));
    assert_eq!(bench.rows().first().map(String::as_str), Some("line 0"));
    bench.frame(|frame| {
        let top = row_of(frame, "line 0");
        let on_thumb = Point::new(rail(top), top.y + 2.0);
        hold(frame, on_thumb, Point::new(on_thumb.x, 5000.0));
    });
    assert!(bench.state.follows());
    assert_eq!(bench.rows().last().map(String::as_str), Some("line 99999"));
}
