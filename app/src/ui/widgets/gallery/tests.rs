//! The kit drawn (in both themes and in Arabic) and driven by the
//! keyboard, through the gallery: the page wired as a real page is.

use iced::{
    keyboard::key::Named,
    widget::{container, Space},
    Length,
};

use super::{about, failure, labels, Gallery, Message};
use crate::{
    test_support::{
        snapshot::{assert_snapshot, assert_snapshot_after, in_language},
        window::{drive, Window},
    },
    ui::{
        theme::{self, Element, Theme},
        widgets::{
            feedback::{
                confirm::{self, Ask},
                confirm_dialog,
                error_details::Details,
                toast::{self, Kind},
                Surfaced, Toasts,
            },
            form::select,
            overlay::{self, menu},
        },
    },
};

/// The three pictures of everything: light, dark, and mirrored.
const LOOKS: [(&str, &str, Theme); 3] = [
    ("light-en", "en", Theme::Light),
    ("dark-en", "en", Theme::Dark),
    ("light-ar", "ar", Theme::Light),
];

/// `part` on the page's colour, with room around it.
fn on_page<'a>(part: Element<'a, Message>) -> Element<'a, Message> {
    container(part)
        .padding(24)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(theme::container::page)
        .into()
}

fn blank<'a>() -> Element<'a, Message> {
    on_page(Space::new().into())
}

/// Sends `act`'s messages to the gallery, as the window would.
fn play(gallery: &mut Gallery, size: (f32, f32), act: impl FnOnce(&mut Window<'_, Message>)) {
    let messages = drive(size, || gallery.view(), act);
    for message in messages {
        let _ = gallery.update(message);
    }
}

fn keys(window: &mut Window<'_, Message>, keys: &[Named]) {
    for key in keys {
        window.tap_key(*key);
    }
}

/// Tall enough for a whole page: a press needs its target in view.
const WINDOW: (f32, f32) = (1100.0, 1400.0);

fn controls() -> Gallery {
    Gallery {
        page: super::Page::Controls,
        ..Gallery::default()
    }
}

#[test]
fn the_feedback_widgets_in_both_themes_and_mirrored() {
    for (look, language, theme) in LOOKS {
        let gallery = Gallery::default();
        let name = format!("widgets-feedback-{look}");
        assert_snapshot(&name, language, &theme, (900.0, 1000.0), || {
            on_page(gallery.feedback())
        });
    }
}

#[test]
fn the_controls_in_both_themes_and_mirrored_with_one_hovered_and_one_focused() {
    for (look, language, theme) in LOOKS {
        let gallery = controls();
        let name = format!("widgets-controls-{look}");
        assert_snapshot_after(
            &name,
            language,
            &theme,
            (1000.0, 1340.0),
            || on_page(gallery.controls()),
            |window| {
                // A press gives "Secondary" focus and a key shows its ring;
                // the pointer then rests on "Outline".
                window.click("Secondary").unwrap();
                window.tap_key(Named::Shift);
                let outline = window.find("Outline").unwrap();
                window.point_at(outline.bounds().center());
            },
        );
    }
}

#[test]
fn the_menu_with_its_submenus_open() {
    for (look, language, theme) in LOOKS {
        let mut gallery = controls();
        gallery.menu.update(menu::Event::Toggle);
        gallery
            .menu
            .update(menu::Event::OpenSub { depth: 0, index: 5 });
        gallery
            .menu
            .update(menu::Event::OpenSub { depth: 1, index: 2 });
        let name = format!("widgets-menu-{look}");
        assert_snapshot(&name, language, &theme, (620.0, 330.0), || {
            on_page(gallery.floating())
        });
    }
}

#[test]
fn the_combobox_open_on_a_search() {
    for (look, language, theme) in LOOKS {
        let mut gallery = controls();
        let labels = labels();
        gallery.combo.update(select::Message::Open, &labels);
        gallery
            .combo
            .update(select::Message::Query("o".to_owned()), &labels);
        gallery.combo.update(select::Message::Step(1), &labels);
        gallery.runtime = Some(3);
        let name = format!("widgets-select-{look}");
        assert_snapshot(&name, language, &theme, (760.0, 300.0), || {
            on_page(gallery.selects())
        });
    }
}

#[test]
fn the_dialogs_and_the_toasts() {
    let at = "2026-03-04T05:06:07Z".parse().unwrap();
    for (look, language, theme) in LOOKS {
        let ask = Ask::new(
            "Remove the volume db-data?",
            "Its data is deleted from prod-eu-1 and can't be brought back.",
            "Remove",
        )
        .destructive()
        .typed("db-data");
        let mut state = confirm::State::default();
        state.update(confirm::Message::Input("db-da".to_owned()), &ask);
        let name = format!("widgets-confirm-{look}");
        assert_snapshot(&name, language, &theme, (560.0, 340.0), || {
            let dialog = confirm_dialog(&ask, &state).map(Message::Confirm);
            overlay::over(blank(), Some(dialog))
        });

        let details = Details::new(Surfaced::at(failure(), Some("containers_list"), at));
        let about = about();
        let name = format!("widgets-details-{look}");
        assert_snapshot(&name, language, &theme, (760.0, 520.0), || {
            let dialog = details.view(Some(&about)).map(Message::Details);
            overlay::over(blank(), Some(dialog))
        });

        let mut toasts = Toasts::default();
        toasts.say_more(
            Kind::Success,
            || "Started web".to_owned(),
            || "On prod-eu-1.".to_owned(),
        );
        toasts.say_acting(
            Kind::Warning,
            || "The disk is nearly full".to_owned(),
            || "Clean up".to_owned(),
            (),
        );
        toasts.failure(Surfaced::at(failure(), Some("containers_act"), at));
        let name = format!("widgets-toasts-{look}");
        assert_snapshot(&name, language, &theme, (480.0, 300.0), || {
            toast::over(blank(), toasts.view().map(|t| t.map(Message::Toast)))
        });
    }
}

#[test]
fn a_select_is_driven_by_the_arrows_home_end_typing_and_enter() {
    let _english = in_language("en");
    let chosen_by = |act: &dyn Fn(&mut Window<'_, Message>)| {
        let mut gallery = controls();
        gallery.select.update(select::Message::Open, &labels());
        play(&mut gallery, WINDOW, |window| act(window));
        (gallery.engine, gallery.select.is_open())
    };
    // It opens on the chosen option, the first: two down is the third.
    let down = chosen_by(&|w| keys(w, &[Named::ArrowDown, Named::ArrowDown, Named::Enter]));
    assert_eq!(down, (Some(2), false));
    // Up from the first goes round to the last; End and Home go to the ends.
    assert_eq!(
        chosen_by(&|w| keys(w, &[Named::ArrowUp, Named::Enter])).0,
        Some(5)
    );
    assert_eq!(
        chosen_by(&|w| keys(w, &[Named::End, Named::Space])).0,
        Some(5)
    );
    let home = chosen_by(&|w| {
        keys(
            w,
            &[Named::End, Named::Home, Named::ArrowDown, Named::Enter],
        )
    });
    assert_eq!(home.0, Some(1));
    // Typing goes to the option that starts with it.
    let typed = chosen_by(&|w| {
        w.typewrite("p");
        keys(w, &[Named::Enter]);
    });
    assert_eq!(typed.0, Some(3));
    // Escape closes and chooses nothing.
    assert_eq!(chosen_by(&|w| keys(w, &[Named::Escape])), (Some(0), false));
}

#[test]
fn the_closed_select_opens_with_a_press_or_an_arrow() {
    let _english = in_language("en");
    let mut gallery = controls();
    play(&mut gallery, WINDOW, |window| {
        window.click("Docker Engine").unwrap();
    });
    assert!(gallery.select.is_open());
    play(&mut gallery, WINDOW, |window| {
        keys(window, &[Named::Escape])
    });
    assert!(!gallery.select.is_open());
    // The press left focus on the button: an arrow opens the list again.
    play(&mut gallery, WINDOW, |window| {
        window.click("Docker Engine").unwrap();
        window.click("Docker Engine").unwrap();
        keys(window, &[Named::ArrowDown]);
    });
    assert!(gallery.select.is_open());
}

#[test]
fn a_combobox_narrows_as_you_type_and_enter_takes_the_highlighted_option() {
    let _english = in_language("en");
    let mut gallery = controls();
    gallery.combo.update(select::Message::Open, &labels());
    // The search field has focus as the list opens.
    play(&mut gallery, WINDOW, |window| {
        window.typewrite("o");
    });
    // "o" leaves Docker Engine, containerd, Podman and Nomad.
    play(&mut gallery, WINDOW, |window| {
        assert!(window.find("Incus").is_err());
        keys(window, &[Named::ArrowDown]);
    });
    play(&mut gallery, WINDOW, |window| {
        keys(window, &[Named::ArrowDown])
    });
    play(&mut gallery, WINDOW, |window| keys(window, &[Named::Enter]));
    assert_eq!(gallery.runtime, Some(3));
    assert!(!gallery.combo.is_open());
    // What matches nothing says so.
    gallery.combo.update(select::Message::Open, &labels());
    gallery
        .combo
        .update(select::Message::Query("zzz".to_owned()), &labels());
    play(&mut gallery, WINDOW, |window| {
        assert!(window.find("Nothing matches.").is_ok());
    });
}

/// What the gallery said it chose, read off its toasts.
fn said(gallery: &Gallery) -> Vec<String> {
    gallery
        .toasts
        .visible()
        .iter()
        .map(|toast| toast.text().0)
        .collect()
}

#[test]
fn a_dropdown_menu_is_driven_by_the_keyboard_through_its_submenus() {
    let _english = in_language("en");
    let open = || {
        let mut gallery = controls();
        gallery.menu.update(menu::Event::Toggle);
        gallery
    };
    // Focus starts on Start; Down is Restart; Enter chooses and closes.
    let mut gallery = open();
    play(&mut gallery, WINDOW, |w| {
        keys(w, &[Named::ArrowDown, Named::Enter])
    });
    assert_eq!(said(&gallery), ["Restart"]);
    assert!(!gallery.menu.is_open());
    // The disabled Stop is passed over; typing "r" twice reaches Remove.
    let mut gallery = open();
    play(&mut gallery, WINDOW, |w| {
        keys(w, &[Named::ArrowDown, Named::ArrowDown, Named::Enter]);
    });
    assert_eq!(said(&gallery), ["Follow"]);
    let mut gallery = open();
    play(&mut gallery, WINDOW, |w| {
        w.typewrite("rr");
        keys(w, &[Named::Enter]);
    });
    assert_eq!(said(&gallery), ["Remove"]);
    // End, Up: Copy. The arrow towards the end opens its submenu, whose
    // first item has focus; the arrow back closes it; Escape closes all.
    let mut gallery = open();
    play(&mut gallery, WINDOW, |w| {
        keys(w, &[Named::End, Named::ArrowUp, Named::ArrowRight]);
    });
    assert!(gallery.menu.is_open() && said(&gallery).is_empty());
    let mut within = open();
    within.menu = gallery.menu.clone();
    play(&mut within, WINDOW, |w| {
        keys(w, &[Named::ArrowDown, Named::Enter])
    });
    assert_eq!(said(&within), ["Id"]);
    assert!(!within.menu.is_open());
    let submenu = gallery.menu.clone();
    play(&mut gallery, WINDOW, |w| keys(w, &[Named::ArrowLeft]));
    assert!(gallery.menu.is_open() && gallery.menu != submenu);
    play(&mut gallery, WINDOW, |w| keys(w, &[Named::Escape]));
    assert!(!gallery.menu.is_open());
}

#[test]
fn a_press_outside_closes_the_menu() {
    let _english = in_language("en");
    let mut gallery = controls();
    play(&mut gallery, WINDOW, |window| {
        window.click("Actions").unwrap();
    });
    assert!(gallery.menu.is_open());
    play(&mut gallery, WINDOW, |window| {
        window.click("Buttons").unwrap();
    });
    assert!(!gallery.menu.is_open());
}

#[test]
fn tabs_follow_the_arrows_and_mirror_in_arabic() {
    let after = |language: &str, pressed: &[Named]| {
        let _language = in_language(language);
        let mut gallery = controls();
        play(&mut gallery, WINDOW, |window| {
            window.click("Logs").unwrap();
            keys(window, pressed);
        });
        gallery.tab
    };
    assert_eq!(after("en", &[]), 1);
    assert_eq!(after("en", &[Named::ArrowRight]), 2);
    assert_eq!(after("en", &[Named::ArrowLeft, Named::ArrowLeft]), 3);
    assert_eq!(after("en", &[Named::End]), 3);
    assert_eq!(after("en", &[Named::End, Named::ArrowRight]), 0);
    assert_eq!(after("en", &[Named::Home]), 0);
    // Right to left, the next tab is on the left.
    assert_eq!(after("ar", &[Named::ArrowLeft]), 2);
    assert_eq!(after("ar", &[Named::ArrowRight]), 0);
}

#[test]
fn a_toggle_waits_for_enter_or_space() {
    let _english = in_language("en");
    let mut gallery = controls();
    play(&mut gallery, WINDOW, |window| {
        window.click("Board").unwrap();
    });
    assert_eq!(gallery.layout, 1);
    // The arrow moves focus only; Space then chooses.
    play(&mut gallery, WINDOW, |window| {
        window.click("Board").unwrap();
        keys(window, &[Named::ArrowLeft]);
    });
    assert_eq!(gallery.layout, 1);
    play(&mut gallery, WINDOW, |window| {
        window.click("Board").unwrap();
        keys(window, &[Named::ArrowLeft, Named::Space]);
    });
    assert_eq!(gallery.layout, 0);
}

#[test]
fn a_dialog_keeps_the_keyboard_and_the_pointer_to_itself() {
    let _english = in_language("en");
    let mut gallery = Gallery::default();
    play(&mut gallery, WINDOW, |window| {
        window.click("Confirm").unwrap();
    });
    assert!(gallery.confirm.is_some());
    // The page under it hears no press…
    play(&mut gallery, WINDOW, |window| {
        let _ = window.click("Success toast");
    });
    assert!(gallery.toasts.is_empty() && gallery.confirm.is_some());
    // … Tab goes round inside it, back to Cancel, which Enter presses.
    play(&mut gallery, WINDOW, |window| {
        keys(window, &[Named::Tab, Named::Tab, Named::Enter]);
    });
    assert!(gallery.confirm.is_none() && gallery.toasts.is_empty());
    // Shift+Tab from Cancel is the action; Escape closes.
    play(&mut gallery, WINDOW, |window| {
        window.click("Confirm").unwrap();
    });
    play(&mut gallery, WINDOW, |window| {
        keys(window, &[Named::Escape])
    });
    assert!(gallery.confirm.is_none());
}

#[test]
fn a_popover_opens_and_closes_on_escape() {
    let _english = in_language("en");
    let mut gallery = controls();
    play(&mut gallery, WINDOW, |window| {
        assert!(window.find("Ports").is_err());
        window.click("Popover").unwrap();
    });
    assert!(gallery.popover);
    play(&mut gallery, WINDOW, |window| {
        assert!(window.find("Ports").is_ok());
        keys(window, &[Named::Escape]);
    });
    assert!(!gallery.popover);
}

#[test]
fn a_field_with_suggestions_offers_them_as_typed_and_keeps_free_text() {
    let _english = in_language("en");
    // A field that holds `start`, clicked, then typed in.
    let typed = |start: &str, act: &dyn Fn(&mut Window<'_, Message>)| {
        let mut gallery = Gallery {
            image: start.to_owned(),
            ..controls()
        };
        let start = start.to_owned();
        play(&mut gallery, WINDOW, |window| {
            window.click(start.as_str()).unwrap();
            act(window);
        });
        gallery
    };
    // The list opens on what holds what is typed; Down twice and Enter
    // take the second.
    let gallery = typed("n", &|w| {
        w.typewrite("gi");
        keys(w, &[Named::ArrowDown, Named::ArrowDown, Named::Enter]);
    });
    assert_eq!(gallery.image, "nginx:latest");
    assert!(!gallery.images.is_open());
    // Free text stays: Enter with nothing highlighted takes nothing.
    let gallery = typed("m", &|w| {
        w.typewrite("y-own:1");
        keys(w, &[Named::Enter]);
    });
    assert_eq!(gallery.image, "my-own:1");
    // Escape closes the list (each frame is drawn anew: the list is
    // open before Escape comes).
    let mut gallery = typed("p", &|w| {
        w.typewrite("o");
    });
    assert!(gallery.images.is_open());
    play(&mut gallery, WINDOW, |window| {
        window.click("po").unwrap();
        keys(window, &[Named::Escape]);
    });
    assert_eq!(gallery.image, "po");
    assert!(!gallery.images.is_open());
    // A press on a suggestion takes it.
    let mut gallery = typed("p", &|w| {
        w.typewrite("o");
    });
    assert!(gallery.images.is_open());
    play(&mut gallery, WINDOW, |window| {
        window.click("postgres:16").unwrap();
    });
    assert_eq!(gallery.image, "postgres:16");
    assert!(!gallery.images.is_open());
}

#[test]
fn a_field_with_suggestions_in_both_themes_and_mirrored() {
    for (look, language, theme) in LOOKS {
        let mut gallery = controls();
        let _ = gallery.update(Message::Image(super::suggest::Message::Typed(
            "nginx".to_owned(),
        )));
        let _ = gallery.update(Message::Image(super::suggest::Message::Step(1)));
        let name = format!("widgets-suggest-{look}");
        assert_snapshot(&name, language, &theme, (420.0, 200.0), || {
            on_page(container(gallery.suggestion()).width(280).into())
        });
    }
}

/// The field with suggestions in a dialog, before another field.
fn in_a_dialog(gallery: &Gallery) -> Element<'_, Message> {
    use crate::ui::widgets::{form::input, overlay::dialog};
    let other = input("", &gallery.name).on_input(Message::Name);
    let content = dialog("Run").push(gallery.suggestion()).push(other);
    overlay::over(
        blank(),
        Some(overlay::modal(content).on_close(Message::Ask(false))),
    )
}

/// Draws the dialog on `stage`, lets `act` use it, and hands what it said
/// to the gallery.
fn on_stage(
    gallery: &mut Gallery,
    stage: &mut crate::test_support::stage::Stage,
    act: impl FnOnce(&mut crate::test_support::stage::Frame<'_, '_, Message>),
) -> Vec<Message> {
    let messages = stage.frame(in_a_dialog(gallery), act);
    for message in messages.clone() {
        let _ = gallery.update(message);
    }
    messages
}

#[test]
fn in_a_dialog_tab_closes_the_suggestions_and_moves_on() {
    use crate::test_support::stage::Stage;
    let _english = in_language("en");
    for back in [false, true] {
        let mut gallery = controls();
        let mut stage = Stage::new((640.0, 480.0));
        // The dialog gives the field the keyboard as it opens.
        on_stage(&mut gallery, &mut stage, |frame| frame.typewrite("ngi"));
        assert!(gallery.images.is_open());
        on_stage(&mut gallery, &mut stage, |frame| {
            if back {
                frame.tab_back();
            } else {
                frame.tab();
            }
        });
        assert!(
            !gallery.images.is_open(),
            "Tab (back: {back}) closes the list"
        );
        on_stage(&mut gallery, &mut stage, |frame| {
            assert!(frame.is_focused("TOKEN=abc"), "focus left the field");
        });
    }
}

#[test]
fn in_a_dialog_the_arrows_enter_and_escape_go_to_the_suggestions_first() {
    use crate::test_support::stage::Stage;
    let _english = in_language("en");
    let mut gallery = controls();
    let mut stage = Stage::new((640.0, 480.0));
    on_stage(&mut gallery, &mut stage, |frame| frame.typewrite("ngi"));
    on_stage(&mut gallery, &mut stage, |frame| {
        frame.tap_key(Named::ArrowDown)
    });
    on_stage(&mut gallery, &mut stage, |frame| {
        frame.tap_key(Named::ArrowDown)
    });
    on_stage(&mut gallery, &mut stage, |frame| {
        frame.tap_key(Named::ArrowUp)
    });
    on_stage(&mut gallery, &mut stage, |frame| {
        frame.tap_key(Named::Enter)
    });
    assert_eq!(gallery.image, "nginx:1.27");
    // Escape: the list first, then the dialog (a value typed whole offers
    // nothing more: back to what holds others).
    gallery.image = "ngi".to_owned();
    on_stage(&mut gallery, &mut stage, |frame| {
        frame.tap_key(Named::ArrowDown)
    });
    assert!(gallery.images.is_open());
    let first = on_stage(&mut gallery, &mut stage, |frame| {
        frame.tap_key(Named::Escape)
    });
    assert!(!gallery.images.is_open());
    assert!(
        !first.iter().any(|m| matches!(m, Message::Ask(false))),
        "the dialog stays"
    );
    let second = on_stage(&mut gallery, &mut stage, |frame| {
        frame.tap_key(Named::Escape)
    });
    assert!(
        second.iter().any(|m| matches!(m, Message::Ask(false))),
        "then it closes"
    );
}

#[test]
fn a_suggestion_taken_with_the_pointer_leaves_the_keyboard_in_the_field() {
    use crate::test_support::stage::Stage;
    let _english = in_language("en");
    let mut gallery = controls();
    let mut stage = Stage::new((640.0, 480.0));
    on_stage(&mut gallery, &mut stage, |frame| frame.typewrite("post"));
    on_stage(&mut gallery, &mut stage, |frame| frame.click("postgres:16"));
    assert_eq!(gallery.image, "postgres:16");
    on_stage(&mut gallery, &mut stage, |frame| {
        assert!(
            frame.is_focused("postgres:16"),
            "the field keeps the keyboard"
        );
    });
}

#[test]
fn keys_that_compose_text_are_the_input_method_s() {
    use crate::test_support::stage::Stage;
    use iced::advanced::input_method;
    let _english = in_language("en");
    let mut gallery = controls();
    let mut stage = Stage::new((640.0, 480.0));
    on_stage(&mut gallery, &mut stage, |frame| frame.typewrite("n"));
    assert!(gallery.images.is_open());
    // Composing (a Japanese word): the arrows and Escape choose among the
    // input method's candidates, not the suggestions.
    let said = on_stage(&mut gallery, &mut stage, |frame| {
        frame.send([iced::Event::InputMethod(input_method::Event::Preedit(
            "に".to_owned(),
            None,
        ))]);
        frame.tap_key(Named::ArrowDown);
        frame.tap_key(Named::Escape);
    });
    assert!(gallery.images.is_open(), "{said:?}");
    let stepped = said.iter().any(|m| {
        matches!(
            m,
            Message::Image(super::suggest::Message::Step(_) | super::suggest::Message::Close)
        )
    });
    assert!(!stepped, "{said:?}");
}

#[test]
fn a_list_of_thousands_draws_a_few_dozen() {
    use super::suggest;
    use crate::test_support::stage::Stage;
    let tags: Vec<String> = (0..5000)
        .map(|n| format!("registry.home/app-{n}:latest"))
        .collect();
    let mut state = suggest::State::default();
    let _ = state.update::<()>(suggest::Message::Typed("app".to_owned()), "", &tags);
    let mut stage = Stage::new((640.0, 480.0));
    stage.frame(
        on_page(suggest::suggest(&state, "app").view().map(Message::Image)),
        |frame| {
            let shown = frame
                .texts()
                .iter()
                .filter(|t| t.starts_with("registry.home/"))
                .count();
            assert!(shown > 0 && shown <= suggest::SHOWN, "{shown} rows");
        },
    );
}

#[test]
fn enter_never_sends_the_form_while_a_suggestion_is_highlighted() {
    use super::suggest::{self, Event};
    let two = vec!["app-1".to_owned(), "app-2".to_owned()];
    let mut state = suggest::State::default();
    let _ = state.update::<()>(suggest::Message::Typed("app".to_owned()), "", &two);
    let _ = state.update::<()>(suggest::Message::Step(1), "app", &two);
    let _ = state.update::<()>(suggest::Message::Step(1), "app", &two);
    // The images are read again meanwhile: one is gone.
    let one = vec!["app-1".to_owned()];
    let (_, event) = state.update::<()>(suggest::Message::Submit, "app", &one);
    assert_eq!(
        event,
        Some(Event::Input("app-2".to_owned())),
        "what was highlighted"
    );
}
