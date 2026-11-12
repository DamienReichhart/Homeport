//! Shortcut keys as `keys.test.ts` held them, and the keymap's resolution.

use iced::keyboard::{key::Named, Key, Modifiers};

use super::{
    keymap::{grouped, keymap, resolve, Command, Group},
    keys::{fires_at, format, matches, parse, Press, Target},
};
use crate::{
    test_support::snapshot::in_language,
    ui::shell::{
        registry::{nav, Capability, Run, Scope, FEATURES},
        router::Route,
    },
};

fn press(key: &str) -> Press {
    Press::of(key)
}

fn ctrl(key: &str) -> Press {
    Press {
        ctrl: true,
        ..Press::of(key)
    }
}

fn meta(key: &str) -> Press {
    Press {
        meta: true,
        ..Press::of(key)
    }
}

#[test]
fn mod_is_command_on_macos_and_ctrl_elsewhere() {
    let k = parse("Mod+K");
    assert!(matches(&k, &meta("k"), true));
    assert!(!matches(&k, &ctrl("k"), true));
    assert!(matches(&k, &ctrl("k"), false));
    assert!(!matches(&k, &meta("k"), false));
    let shifted = Press {
        shift: true,
        ..ctrl("K")
    };
    assert!(!matches(&k, &shifted, false));
}

#[test]
fn a_shortcut_needs_exactly_the_modifiers_it_names() {
    let n = parse("n");
    assert!(matches(&n, &press("n"), false));
    assert!(!matches(&n, &ctrl("n"), false));
    let capital = Press {
        shift: true,
        ..press("N")
    };
    assert!(!matches(&n, &capital, false));
    let up = parse("Alt+ArrowUp");
    let alt_up = Press {
        alt: true,
        ..press("ArrowUp")
    };
    assert!(matches(&up, &alt_up, false));
    assert!(!matches(&up, &press("ArrowUp"), false));
}

#[test]
fn symbols_match_whatever_shift_the_layout_needs() {
    let question = Press {
        shift: true,
        ..press("?")
    };
    assert!(matches(&parse("?"), &question, false));
    assert!(matches(&parse("/"), &press("/"), false));
    assert!(matches(&parse("Mod+,"), &meta(","), true));
    assert!(matches(&parse("Mod+1"), &ctrl("1"), false));
}

#[test]
fn the_keys_shown_are_those_of_the_platform() {
    assert_eq!(format("Mod+K", true), ["⌘", "K"]);
    assert_eq!(format("Mod+K", false), ["Ctrl", "K"]);
    assert_eq!(format("Alt+ArrowDown", false), ["Alt", "↓"]);
    assert_eq!(format("Alt+ArrowDown", true), ["⌥", "↓"]);
    assert_eq!(format("?", false), ["?"]);
}

#[test]
fn plain_keys_type_in_fields_and_the_terminal_keeps_its_keys() {
    let (n, k) = (parse("n"), parse("Mod+K"));
    assert!(fires_at(&n, Target::Page));
    assert!(!fires_at(&n, Target::Text));
    assert!(!fires_at(&n, Target::Dialog));
    assert!(fires_at(&k, Target::Text));
    assert!(!fires_at(&k, Target::Terminal));
    assert!(!fires_at(&parse("Alt+ArrowUp"), Target::Text));
}

#[test]
fn a_press_is_read_from_what_iced_reports() {
    let k = Press::from_iced(&Key::Character("k".into()), Modifiers::CTRL).unwrap();
    assert_eq!(k, ctrl("k"));
    let up = Press::from_iced(&Key::Named(Named::ArrowUp), Modifiers::ALT).unwrap();
    assert!(matches(&parse("Alt+ArrowUp"), &up, false));
    assert_eq!(
        Press::from_iced(&Key::Unidentified, Modifiers::empty()),
        None
    );
}

fn docker_entries() -> Vec<nav::Entry> {
    let all = nav::entries_of(FEATURES);
    nav::visible(&all, Some(Scope::Host), |c| c == Capability::Docker, None)
}

#[test]
fn the_keymap_opens_pages_by_number_and_settings_with_mod_comma() {
    let map = keymap(&docker_entries(), &Route::Containers, true);
    let run = |press: &Press| resolve(&map, press, false, Target::Page).map(|b| &b.command);
    assert_eq!(
        run(&ctrl("4")),
        Some(&Command::Run(Run::Open(Route::Images, None)))
    );
    assert_eq!(
        run(&ctrl(",")),
        Some(&Command::Run(Run::Open(Route::Settings, None)))
    );
    assert_eq!(run(&ctrl("k")), Some(&Command::TogglePalette));
    let alt_down = Press {
        alt: true,
        ..press("ArrowDown")
    };
    assert_eq!(run(&alt_down), Some(&Command::StepHost(1)));
    assert_eq!(run(&ctrl("r")), Some(&Command::Refresh));
    assert_eq!(run(&press("x")), None);
}

#[test]
fn a_page_s_keys_work_on_it_only_and_not_while_typing() {
    let on_containers = keymap(&docker_entries(), &Route::Containers, true);
    let n = resolve(&on_containers, &press("n"), false, Target::Page).unwrap();
    assert_eq!(n.command, Command::Run(Run::Request("run")));
    assert_eq!(n.group, Group::ThisPage);
    assert!(resolve(&on_containers, &press("n"), false, Target::Text).is_none());
    assert!(resolve(&on_containers, &press("/"), false, Target::Dialog).is_none());
    // Mod+K still opens the palette from a field.
    assert!(resolve(&on_containers, &ctrl("k"), false, Target::Text).is_some());
    let on_settings = keymap(&docker_entries(), &Route::Settings, true);
    assert!(resolve(&on_settings, &press("n"), false, Target::Page).is_none());
}

#[test]
fn refresh_needs_a_selected_host() {
    let map = keymap(&docker_entries(), &Route::Settings, false);
    assert!(resolve(&map, &ctrl("r"), false, Target::Page).is_none());
}

#[test]
fn the_sheet_lists_the_active_shortcuts_under_their_headings() {
    let _english = in_language("en");
    let map = keymap(&docker_entries(), &Route::Containers, true);
    let groups = grouped(&map);
    let headings: Vec<String> = groups.iter().map(|(group, _)| group.label()).collect();
    assert_eq!(headings, ["Go to", "General", "Hosts", "This page"]);
    let labels = |group: Group| -> Vec<String> {
        let (_, list) = groups.iter().find(|(g, _)| *g == group).unwrap();
        list.iter().map(|b| b.says.text()).collect()
    };
    assert!(labels(Group::Navigation).contains(&"Go to Images".to_owned()));
    assert_eq!(
        labels(Group::ThisPage),
        ["Search this page", "Run container"]
    );
    assert!(labels(Group::Hosts).contains(&"Next host".to_owned()));
    assert_eq!(
        labels(Group::General),
        [
            "Open Settings",
            "Open the command palette",
            "Show keyboard shortcuts"
        ]
    );
    // Once the page is left, its shortcuts are gone.
    let map = keymap(&docker_entries(), &Route::Settings, true);
    assert!(!grouped(&map).iter().any(|(g, _)| *g == Group::ThisPage));
}
