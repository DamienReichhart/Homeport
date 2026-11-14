//! A window that lasts: the same widget tree from one view to the next, as
//! the real window keeps it. [`super::window::drive`] builds a new tree for
//! every step, so it cannot tell whether a scroll offset or the focus
//! survive a dialog that opens and closes. This can:
//!
//! ```ignore
//! let mut stage = Stage::new((800.0, 600.0));
//! let messages = stage.frame(page.view(), |frame| frame.click("Remove"));
//! // feed `messages` to `update`, then draw the next view on the same tree
//! stage.frame(page.view(), |frame| assert!(frame.focused().is_some()));
//! ```

use iced::{
    advanced::{
        clipboard, mouse,
        renderer::Headless,
        widget::{
            operation::{self, Focusable, Scrollable},
            Id, Operation,
        },
    },
    futures::executor::block_on,
    keyboard, window, Event, Point, Rectangle, Size, Vector,
};
use iced_runtime::{user_interface::Cache, UserInterface};
use iced_test::{simulator, Selector, Simulator};

use super::snapshot::settings;
use crate::ui::theme::{self, Element, Theme};

/// The window's size, its pointer and the tree its views share.
pub struct Stage {
    renderer: iced::Renderer,
    cache: Option<Cache>,
    size: Size,
    cursor: mouse::Cursor,
}

/// One view on the stage, to press and type on and to ask about.
pub struct Frame<'s, 'a, Message> {
    ui: UserInterface<'a, Message, Theme, iced::Renderer>,
    renderer: &'s mut iced::Renderer,
    cursor: &'s mut mouse::Cursor,
    messages: Vec<Message>,
}

impl Stage {
    pub fn new(size: impl Into<Size>) -> Self {
        let size = size.into();
        let settings = settings();
        // A simulator loads the fonts into the process, as it does for the
        // snapshots.
        drop(Simulator::<(), Theme, iced::Renderer>::with_size(
            settings.clone(),
            size,
            iced::widget::Space::new(),
        ));
        let renderer = block_on(<iced::Renderer as Headless>::new(
            theme::font::DEFAULT,
            settings.default_text_size,
            Some("tiny-skia"),
        ))
        .expect("a renderer without a window");
        Self {
            renderer,
            cache: Some(Cache::default()),
            size,
            cursor: mouse::Cursor::Unavailable,
        }
    }

    /// Draws `view` on the tree the last one left, lets `act` use it, and
    /// returns the messages that produced, in order.
    pub fn frame<'a, Message: 'a>(
        &mut self,
        view: impl Into<Element<'a, Message>>,
        act: impl FnOnce(&mut Frame<'_, 'a, Message>),
    ) -> Vec<Message> {
        let cache = self.cache.take().unwrap_or_default();
        let ui = UserInterface::build(view, self.size, cache, &mut self.renderer);
        let mut frame = Frame {
            ui,
            renderer: &mut self.renderer,
            cursor: &mut self.cursor,
            messages: vec![],
        };
        // What a window does first: widgets settle on a redraw.
        frame.redraw();
        act(&mut frame);
        frame.redraw();
        let Frame { ui, messages, .. } = frame;
        self.cache = Some(ui.into_cache());
        messages
    }
}

/// Finds the widget that has focus, where it is drawn: what a scrollable
/// holds is moved by how far it is scrolled.
#[derive(Default)]
struct Focused {
    found: Option<Rectangle>,
    scrolled: Vector,
    entering: Vector,
}

impl Operation for Focused {
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation)) {
        let outside = self.scrolled;
        self.scrolled = outside + std::mem::take(&mut self.entering);
        operate(self);
        self.scrolled = outside;
    }

    fn scrollable(
        &mut self,
        _id: Option<&Id>,
        _bounds: Rectangle,
        _content_bounds: Rectangle,
        translation: Vector,
        _state: &mut dyn Scrollable,
    ) {
        self.entering = translation;
    }

    fn focusable(&mut self, _id: Option<&Id>, bounds: Rectangle, state: &mut dyn Focusable) {
        if state.is_focused() {
            self.found = Some(bounds - self.scrolled);
        }
    }
}

/// Collects what each text that may be cut shows of what it says.
struct Cut(Vec<crate::ui::widgets::basic::truncate::Fitted>);

impl Operation for Cut {
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation)) {
        operate(self);
    }

    fn custom(&mut self, _id: Option<&Id>, _bounds: Rectangle, state: &mut dyn std::any::Any) {
        use crate::ui::widgets::basic::truncate::Fitted;
        if let Some(fitted) = state.downcast_ref::<Fitted>() {
            self.0.push(fitted.clone());
        }
    }
}

/// Collects what every text of the view says.
struct Said(Vec<String>);

impl Operation for Said {
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation)) {
        operate(self);
    }

    fn text(&mut self, _id: Option<&Id>, _bounds: Rectangle, text: &str) {
        self.0.push(text.to_owned());
    }
}

struct Scrolled(Vec<Vector>);

impl Operation for Scrolled {
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation)) {
        operate(self);
    }

    fn scrollable(
        &mut self,
        _id: Option<&Id>,
        _bounds: Rectangle,
        _content_bounds: Rectangle,
        translation: Vector,
        _state: &mut dyn Scrollable,
    ) {
        self.0.push(translation);
    }
}

impl<Message> Frame<'_, '_, Message> {
    /// Sends raw events, as the window would.
    pub fn send(&mut self, events: impl IntoIterator<Item = Event>) {
        let events: Vec<Event> = events.into_iter().collect();
        let _ = self.ui.update(
            &events,
            *self.cursor,
            self.renderer,
            &mut clipboard::Null,
            &mut self.messages,
        );
    }

    pub fn redraw(&mut self) {
        self.send([Event::Window(window::Event::RedrawRequested(
            std::time::Instant::now(),
        ))]);
    }

    pub fn tap_key(&mut self, key: impl Into<keyboard::Key>) {
        self.send(simulator::tap_key(key, None));
    }

    pub fn typewrite(&mut self, text: &str) {
        self.send(simulator::typewrite(text));
    }

    pub fn point_at(&mut self, at: impl Into<Point>) {
        *self.cursor = mouse::Cursor::Available(at.into());
        self.send([Event::Mouse(mouse::Event::CursorMoved {
            position: self.cursor.position().unwrap_or_default(),
        })]);
    }

    /// Tab as the window does it: the key, then focus to the next widget
    /// that takes it unless something (a dialog, a panel) kept the key.
    pub fn tab(&mut self) {
        let events: Vec<Event> = simulator::tap_key(keyboard::key::Named::Tab, None).collect();
        let (_, statuses) = self.ui.update(
            &events,
            *self.cursor,
            self.renderer,
            &mut clipboard::Null,
            &mut self.messages,
        );
        if statuses.first() == Some(&iced::event::Status::Ignored) {
            // Counting what takes focus, then giving it: two passes.
            let mut next: Box<dyn Operation<()>> = Box::new(operation::focusable::focus_next());
            loop {
                self.ui
                    .operate(self.renderer, &mut operation::black_box(&mut next));
                match next.finish() {
                    operation::Outcome::Chain(then) => next = then,
                    _ => break,
                }
            }
        }
        self.redraw();
    }

    /// Shift+Tab, as the window does it.
    pub fn tab_back(&mut self) {
        let shift = keyboard::Modifiers::SHIFT;
        let events: Vec<Event> = simulator::tap_key(keyboard::key::Named::Tab, None)
            .map(|event| match event {
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key,
                    modified_key,
                    physical_key,
                    location,
                    text,
                    repeat,
                    ..
                }) => Event::Keyboard(keyboard::Event::KeyPressed {
                    key,
                    modified_key,
                    physical_key,
                    location,
                    modifiers: shift,
                    text,
                    repeat,
                }),
                other => other,
            })
            .collect();
        let (_, statuses) = self.ui.update(
            &events,
            *self.cursor,
            self.renderer,
            &mut clipboard::Null,
            &mut self.messages,
        );
        if statuses.first() == Some(&iced::event::Status::Ignored) {
            self.operate(Box::new(operation::focusable::focus_previous()));
        }
        self.redraw();
    }

    /// Runs a widget operation to its end, as the window does one a task
    /// asked for.
    fn operate(&mut self, mut operation: Box<dyn Operation<()>>) {
        loop {
            self.ui
                .operate(self.renderer, &mut operation::black_box(&mut operation));
            match operation.finish() {
                operation::Outcome::Chain(then) => operation = then,
                _ => break,
            }
        }
    }

    /// Runs what an `update` returned on this view: what it asks of the
    /// widgets (focus this field, select its text) is done here, and the
    /// messages it produced are returned.
    pub fn perform<T: Send + 'static>(&mut self, task: iced::Task<T>) -> Vec<T> {
        use iced::futures::StreamExt;
        let Some(mut actions) = iced_runtime::task::into_stream(task) else {
            return vec![];
        };
        let mut produced = vec![];
        while let Some(action) = block_on(actions.next()) {
            match action {
                iced_runtime::Action::Output(message) => produced.push(message),
                iced_runtime::Action::Widget(operation) => self.operate(operation),
                _ => {}
            }
        }
        self.redraw();
        produced
    }

    /// [`Frame::perform`] of what is ready now: a writer that runs until
    /// its session ends is let go of where it first waits.
    pub fn perform_now<T: Send + 'static>(&mut self, task: iced::Task<T>) -> Vec<T> {
        use iced::futures::{FutureExt, StreamExt};
        let Some(mut actions) = iced_runtime::task::into_stream(task) else {
            return vec![];
        };
        let mut produced = vec![];
        while let Some(Some(action)) = actions.next().now_or_never() {
            match action {
                iced_runtime::Action::Output(message) => produced.push(message),
                iced_runtime::Action::Widget(operation) => self.operate(operation),
                _ => {}
            }
        }
        self.redraw();
        produced
    }

    /// Whether a button shows a focus ring.
    pub fn shows_focus(&mut self) -> bool {
        let mut shown = crate::ui::widgets::focus::Shown(false);
        self.ui.operate(self.renderer, &mut shown);
        shown.0
    }

    /// Presses and releases the left button where the pointer is.
    pub fn press(&mut self) {
        self.send(simulator::click());
    }

    /// Where the text `label` is drawn, if it is in view.
    pub fn find(&mut self, label: &str) -> Option<Rectangle> {
        let mut find = Selector::find(label);
        self.ui
            .operate(self.renderer, &mut operation::black_box(&mut find));
        match find.finish() {
            // Where it shows: what a scrollable holds has moved with it.
            operation::Outcome::Some(found) => found.and_then(|text| text.visible_bounds()),
            _ => None,
        }
    }

    /// Points at the text `label` and clicks it.
    pub fn click(&mut self, label: &str) {
        let bounds = self
            .find(label)
            .unwrap_or_else(|| panic!("nothing says {label:?}"));
        self.point_at(bounds.center());
        self.press();
    }

    /// Scrolls by `lines` under the pointer.
    pub fn scroll(&mut self, lines: f32) {
        self.send([Event::Mouse(mouse::Event::WheelScrolled {
            delta: mouse::ScrollDelta::Lines { x: 0.0, y: -lines },
        })]);
    }

    /// Where the widget that has keyboard focus is.
    pub fn focused(&mut self) -> Option<Rectangle> {
        let mut focused = Focused::default();
        self.ui.operate(self.renderer, &mut focused);
        focused.found
    }

    /// Whether the widget that has focus is the one that says `label`.
    pub fn is_focused(&mut self, label: &str) -> bool {
        match (self.focused(), self.find(label)) {
            (Some(focused), Some(text)) => focused.contains(text.center()),
            _ => false,
        }
    }

    /// Every text of the view that ends with an ellipsis when it lacks
    /// room (`basic::truncated`): what it says and what it shows of it.
    pub fn fitted(&mut self) -> Vec<crate::ui::widgets::basic::truncate::Fitted> {
        let mut cut = Cut(vec![]);
        self.ui.operate(self.renderer, &mut cut);
        cut.0
    }

    /// What every text of the view says, in layout order: the rows of a
    /// widget that draws its own text among them.
    pub fn texts(&mut self) -> Vec<String> {
        let mut said = Said(vec![]);
        self.ui.operate(self.renderer, &mut said);
        said.0
    }

    /// How far each scrollable of the view is scrolled, in layout order.
    pub fn scrolled(&mut self) -> Vec<Vector> {
        let mut scrolled = Scrolled(vec![]);
        self.ui.operate(self.renderer, &mut scrolled);
        scrolled.0
    }
}
