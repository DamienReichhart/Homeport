//! What a JSON document is read as, where each of its nodes is, what is
//! written back, and what opening, closing and searching do.

use crate::{
    test_support::tasks::messages,
    ui::widgets::{
        data::json::{
            shown, Copy, Document, Event, Invalid, Kind, Message, Name, State, Why, SHOWN,
        },
        gallery::data::{large, INSPECT},
    },
};

fn document(text: &str) -> Document {
    Document::parse(text).unwrap_or_else(|error| panic!("{text}: {error}"))
}

/// The node at `path`, found by its path: the two must agree.
fn at(document: &Document, path: &str) -> usize {
    (0..document.len())
        .find(|index| document.path(*index).as_deref() == Some(path))
        .unwrap_or_else(|| panic!("nothing is at {path}"))
}

#[test]
fn a_document_is_its_nodes_in_the_order_it_is_written() {
    let document = document(r#"{"zeta": 1, "alpha": [true, null, "x"], "mid": {}}"#);
    let names: Vec<_> = document
        .nodes()
        .iter()
        .map(|node| node.name.clone())
        .collect();
    // Not sorted: `zeta` stays before `alpha`.
    assert_eq!(
        names,
        [
            Name::Root,
            Name::Member("zeta".to_owned()),
            Name::Member("alpha".to_owned()),
            Name::Item(0),
            Name::Item(1),
            Name::Item(2),
            Name::Member("mid".to_owned()),
        ]
    );
    let kinds: Vec<_> = document.nodes().iter().map(|node| node.kind).collect();
    assert_eq!(
        kinds,
        [
            Kind::Object(3),
            Kind::Number,
            Kind::Array(3),
            Kind::Bool,
            Kind::Null,
            Kind::String,
            Kind::Object(0),
        ]
    );
    let node = |index: usize| document.node(index).unwrap();
    assert_eq!((node(0).end, node(2).end, node(6).end), (7, 6, 7));
    assert_eq!((node(3).depth, node(3).parent), (2, 2));
    assert_eq!(node(5).text, r#""x""#);
    assert_eq!(document.lineage(4), [0, 2, 4]);
}

#[test]
fn scalars_are_documents_and_what_is_no_json_is_refused() {
    for (text, kind) in [
        ("42", Kind::Number),
        ("-1.5e3", Kind::Number),
        ("\"a\"", Kind::String),
        ("null", Kind::Null),
        ("false", Kind::Bool),
    ] {
        let document = document(text);
        assert_eq!((document.len(), document.node(0).unwrap().kind), (1, kind));
    }
    for wrong in [
        "",
        "{",
        "[1,]",
        "{\"a\":1} trailing",
        "{'a':1}",
        "01",
        "1.",
        ".5",
        "1e",
        "-",
        "+1",
        "tru",
        "nul",
        "[1 2]",
        "{\"a\" 1}",
        "{\"a\":}",
        "{,}",
        "\"a",
        "\"\\x\"",
        "\"line\nbreak\"",
        "[1,,2]",
        "NaN",
    ] {
        assert!(Document::parse(wrong).is_err(), "{wrong:?} was read");
    }
    // Where it stopped, and whether the text just ended there.
    assert_eq!(
        Document::parse("[1, x]"),
        Err(Invalid {
            offset: 4,
            why: Why::Unexpected
        })
    );
    assert_eq!(
        Document::parse("{\"a\": [1").unwrap_err().why,
        Why::Unfinished
    );
}

/// A number is what the document wrote, digit for digit: none is read
/// as a float and written back as another.
#[test]
fn a_number_is_shown_and_copied_as_it_was_written() {
    for written in [
        "18446744073709551616",
        "123456789012345678901234567890",
        "1e400",
        "-1e-400",
        "1e2",
        "1E+2",
        "-0",
        "-0.0",
        "0.1",
        "1.50",
        "-1.5e3",
        "0",
        "9007199254740993",
    ] {
        let document = document(written);
        assert_eq!(document.node(0).unwrap().text, written);
        assert_eq!(document.pretty(0), written);
        let inside = Document::parse(&format!("{{\"n\": [{written}]}}")).unwrap();
        assert_eq!(inside.node(2).unwrap().text, written);
        assert_eq!(inside.pretty(1), format!("[\n  {written}\n]"));
    }
}

/// Nothing recurses: a document nests as deep as its text goes.
#[test]
fn a_document_nests_as_deep_as_it_is_written() {
    for deep in [127, 128, 1000, 200_000] {
        let text = format!("{}1{}", "[".repeat(deep), "]".repeat(deep));
        let document =
            Document::parse(&text).unwrap_or_else(|error| panic!("{deep} deep: {error}"));
        assert_eq!(document.len(), deep + 1);
        let innermost = document.node(deep).unwrap();
        assert_eq!(
            (innermost.depth as usize, innermost.text.as_str()),
            (deep, "1")
        );
        assert_eq!(document.node(0).unwrap().end as usize, deep + 1);
    }
    // Written back and read again, it is itself; its path is as long.
    let text = format!("{}null{}", "{\"a\":".repeat(300), "}".repeat(300));
    let document = document(&text);
    assert_eq!(Document::parse(&document.pretty(0)).unwrap(), document);
    assert_eq!(document.path(300).unwrap(), ".a".repeat(300));
    // A text that opens and never closes is unfinished, not a crash.
    assert_eq!(
        Document::parse(&"[".repeat(100_000)).unwrap_err().why,
        Why::Unfinished
    );
}

/// A name written twice is there twice, in order, as the document has it.
#[test]
fn a_name_written_twice_is_shown_twice() {
    let document = document(r#"{"a": 1, "a": 2}"#);
    assert_eq!(document.len(), 3);
    let text = |at: usize| document.node(at).unwrap().text.clone();
    assert_eq!((text(1), text(2)), ("1".to_owned(), "2".to_owned()));
    assert_eq!(document.path(1), document.path(2));
    assert_eq!(document.pretty(0), "{\n  \"a\": 1,\n  \"a\": 2\n}");
}

/// A character that draws nothing, or turns the text after it around,
/// is shown as its escape: a value cannot be made to read as another.
#[test]
fn what_cannot_be_seen_is_shown_as_its_escape() {
    // Drawn as it stands, "user" + U+202E + "nimda" would read "useradmin".
    assert_eq!(shown("\"user\u{202e}nimda\""), "\"user\\u202enimda\"");
    assert_eq!(
        shown("a\u{200b}b\u{feff}c\u{2066}d\u{7f}"),
        "a\\u200bb\\ufeffc\\u2066d\\u007f"
    );
    // What is seen stays: other scripts, an emoji, a space.
    assert_eq!(shown("مرحبا 🚀 é"), "مرحبا 🚀 é");
    // And the document keeps the character itself, for the clipboard.
    let document = document("[\"a\u{202e}b\"]");
    assert_eq!(document.pretty(1), "\"a\u{202e}b\"");
}

#[test]
fn a_path_says_where_a_node_is_as_jq_writes_it() {
    let document = document(
        r#"{"ports": [{"name": "http"}], "labels": {"a.b": 1, "with \"quote\"": 2, "": 3, "9lives": 4, "ключ": 5}, "ok_1": null}"#,
    );
    let paths: Vec<_> = (0..document.len())
        .filter_map(|index| document.path(index))
        .collect();
    assert_eq!(
        paths,
        [
            ".",
            ".ports",
            ".ports[0]",
            ".ports[0].name",
            ".labels",
            r#".labels["a.b"]"#,
            r#".labels["with \"quote\""]"#,
            r#".labels[""]"#,
            r#".labels["9lives"]"#,
            r#".labels["ключ"]"#,
            ".ok_1",
        ]
    );
    // An array at the root, and a member of the root that needs quotes.
    assert_eq!(document_paths("[[1]]"), [".", ".[0]", ".[0][0]"]);
    assert_eq!(document_paths(r#"{"a b": 1}"#), [".", r#".["a b"]"#]);
    // A node that is not there is nowhere, not at the root.
    assert_eq!(document.path(999), None);
}

fn document_paths(text: &str) -> Vec<String> {
    let document = document(text);
    (0..document.len())
        .filter_map(|index| document.path(index))
        .collect()
}

/// What `JSON.stringify(value, null, 2)` wrote, which the copy button
/// copied: the document read back from it is the same document.
#[test]
fn a_node_is_written_back_on_indented_lines() {
    let text = r#"{"a": [1, {"b": null}, []], "c": {}, "d": "x\ny", "e": {"f": [true]}}"#;
    let document = document(text);
    let expected = "{\n  \"a\": [\n    1,\n    {\n      \"b\": null\n    },\n    []\n  ],\n  \"c\": {},\n  \"d\": \"x\\ny\",\n  \"e\": {\n    \"f\": [\n      true\n    ]\n  }\n}";
    assert_eq!(document.pretty(0), expected);
    assert_eq!(Document::parse(&document.pretty(0)).unwrap(), document);
    // A part of it, from its own margin.
    assert_eq!(
        document.pretty(at(&document, ".a[1]")),
        "{\n  \"b\": null\n}"
    );
    assert_eq!(document.pretty(at(&document, ".d")), "\"x\\ny\"");
    assert_eq!(document.pretty(at(&document, ".c")), "{}");
    assert_eq!(document.pretty(999), "");
    // The sample, which has every kind of value: read back, it is itself.
    let inspect = Document::parse(INSPECT).unwrap();
    assert_eq!(Document::parse(&inspect.pretty(0)).unwrap(), inspect);
    // And what `serde_json` itself writes of it, for the same value.
    let value: serde_json::Value = serde_json::from_str(text).unwrap();
    assert_eq!(Document::of(&value).unwrap().len(), document.len());
}

#[test]
fn a_value_too_long_for_a_row_is_cut_between_characters() {
    assert_eq!(shown("short"), "short");
    let long = "é".repeat(SHOWN + 10);
    let cut = shown(&long);
    assert_eq!(cut.chars().count(), SHOWN + 1);
    assert!(cut.ends_with('…'));
    // Exactly as many as fit: whole.
    let fits = "ع".repeat(SHOWN);
    assert_eq!(shown(&fits), fits);
}

#[test]
fn an_object_closes_over_what_it_holds_and_opens_again() {
    let mut state = State::new(document(r#"{"a": {"b": [1, 2]}, "c": 3}"#));
    // The root, a, b, its two items, c.
    assert_eq!(state.rows(), 6);
    let a = u32::try_from(at(state.document(), ".a")).unwrap();
    state.toggle(a);
    assert_eq!(state.rows(), 3, "a hides b and its items");
    assert!(!state.is_open(a as usize));
    state.toggle(a);
    assert_eq!(state.rows(), 6, "what was open inside is open again");
    // A scalar and an empty object have nothing to close.
    let c = u32::try_from(at(state.document(), ".c")).unwrap();
    state.toggle(c);
    assert_eq!(state.rows(), 6);
    // Closing the root leaves its row.
    state.toggle(0);
    assert_eq!(state.rows(), 1);
}

#[test]
fn a_large_document_starts_with_two_levels_open() {
    let state = State::new(document(&large(1000)));
    assert_eq!(state.document().len(), 1 + 1000 * 8);
    // The array and its thousand objects, each closed.
    assert_eq!(state.rows(), 1001);
    let small = State::new(document(&large(10)));
    assert_eq!(small.rows(), small.document().len());
}

/// Runs what an update asked for to its end, answers fed back.
async fn settle(state: &mut State, message: Message) -> Vec<Event> {
    let mut events = vec![];
    let mut pending = vec![message];
    while let Some(message) = pending.pop() {
        let (task, event) = state.update(message);
        events.extend(event);
        pending.extend(messages(task).await);
    }
    events
}

#[tokio::test(start_paused = true)]
async fn a_search_finds_names_and_values_whatever_the_case_and_opens_what_holds_them() {
    let mut state = State::new(document(&large(1000)));
    assert_eq!(state.matches(), (None, 0));
    settle(&mut state, Message::Typed("CONTAINER-99".to_owned())).await;
    // container-99 and container-990 to -999.
    assert_eq!(state.matches(), (Some(0), 11));
    assert!(!state.is_searching());
    // The first match is inside a closed object: it shows now.
    let first = at(state.document(), ".[99].name");
    assert!(state.is_open(at(state.document(), ".[99]")));
    assert_eq!(state.rows(), 1001 + 5);
    assert_eq!(state.marked(), Some(u32::try_from(first).unwrap()));
    // Next goes on and round; Previous comes back.
    settle(&mut state, Message::Next).await;
    assert_eq!(state.matches().0, Some(1));
    assert!(state.is_open(at(state.document(), ".[990]")));
    settle(&mut state, Message::Previous).await;
    settle(&mut state, Message::Previous).await;
    assert_eq!(state.matches().0, Some(10));
    // A member's name matches too.
    settle(&mut state, Message::Typed("Running".to_owned())).await;
    assert_eq!(state.matches().1, 1000);
    // Text in another script, in another case.
    let mut state = State::new(document(r#"{"Ключ": "ЗНАЧЕНИЕ", "other": 1}"#));
    settle(&mut state, Message::Typed("значение".to_owned())).await;
    assert_eq!(state.matches(), (Some(0), 1));
    settle(&mut state, Message::Typed("ключ".to_owned())).await;
    assert_eq!(state.matches(), (Some(0), 1));
    // Nothing matches; then the field is emptied.
    settle(&mut state, Message::Typed("absent".to_owned())).await;
    assert_eq!(state.matches(), (None, 0));
    settle(&mut state, Message::Next).await;
    assert_eq!(state.matches(), (None, 0));
    settle(&mut state, Message::Typed(String::new())).await;
    assert_eq!((state.matches(), state.marked()), ((None, 0), None));
}

#[tokio::test(start_paused = true)]
async fn a_query_typed_over_another_drops_what_the_first_was_reading() {
    let mut state = State::new(document(&large(10_000)));
    // The first slice of a search, whose next is still to come.
    let (task, _) = state.update(Message::Typed("container-1".to_owned()));
    let paused = messages(task).await;
    let (task, _) = state.update(paused[0].clone());
    assert!(state.is_searching(), "80,001 nodes are more than a slice");
    let stale = messages(task).await;
    // Another query before it: the old slice's answer reads nothing.
    settle(&mut state, Message::Typed("name".to_owned())).await;
    let found = state.matches().1;
    assert_eq!(found, 10_000);
    let (task, _) = state.update(stale[0].clone());
    assert!(messages(task).await.is_empty());
    assert_eq!(state.matches().1, found);
}

#[tokio::test(start_paused = true)]
async fn the_menu_copies_where_a_node_is_or_what_it_holds() {
    let mut state = State::new(document(
        r#"{"ports": [{"name": "http"}], "big": 18446744073709551616}"#,
    ));
    let node = u32::try_from(at(state.document(), ".ports[0]")).unwrap();
    let copied = settle(&mut state, Message::Copy(Copy::Path(node))).await;
    assert_eq!(copied, [Event::Copy(".ports[0]".to_owned())]);
    let copied = settle(&mut state, Message::Copy(Copy::Value(node))).await;
    assert_eq!(
        copied,
        [Event::Copy("{\n  \"name\": \"http\"\n}".to_owned())]
    );
    // A number goes to the clipboard as the document wrote it.
    let big = u32::try_from(at(state.document(), ".big")).unwrap();
    let copied = settle(&mut state, Message::Copy(Copy::Value(big))).await;
    assert_eq!(copied, [Event::Copy("18446744073709551616".to_owned())]);
}

/// The whole document is written out by a task, not in `update`: the
/// update that is told to copy gives back no text, only work to run.
#[tokio::test(start_paused = true)]
async fn the_document_is_written_out_for_the_clipboard_off_the_update() {
    let mut state = State::new(document(&large(1000)));
    let (task, event) = state.update(Message::Copy(Copy::Document));
    assert_eq!(event, None, "the text was made in the update");
    assert!(state.is_copied(Copy::Document));
    let mut written = vec![];
    for message in messages(task).await {
        written.extend(state.update(message).1);
    }
    assert_eq!(written, [Event::Copy(state.document().pretty(0))]);
    assert!(!state.is_copied(Copy::Document));
}

/// What a string says is what is searched, not how JSON writes it.
#[tokio::test(start_paused = true)]
async fn a_search_looks_in_what_a_string_says() {
    let text = r#"{"path": "C:\\Users\\me", "quote": "a\"b", "plain": "ab", "spaced": "a  b", "n": 12, "tab": "x\ty"}"#;
    let mut state = State::new(document(text));
    let found = |state: &State| state.matches().1;
    settle(&mut state, Message::Typed(r"C:\Users".to_owned())).await;
    assert_eq!(found(&state), 1, "a backslash is one character");
    settle(&mut state, Message::Typed("a\"b".to_owned())).await;
    assert_eq!(found(&state), 1, "a quote in a value is found");
    // A quote alone is in one value, not around every string.
    settle(&mut state, Message::Typed("\"".to_owned())).await;
    assert_eq!(found(&state), 1);
    // Spaces are part of what is typed: two of them are in one value.
    settle(&mut state, Message::Typed("  ".to_owned())).await;
    assert_eq!(found(&state), 1);
    settle(&mut state, Message::Typed("x\ty".to_owned())).await;
    assert_eq!(found(&state), 1);
    // Numbers as they are written; an object's brackets are no value.
    settle(&mut state, Message::Typed("12".to_owned())).await;
    assert_eq!(found(&state), 1);
    settle(&mut state, Message::Typed("{".to_owned())).await;
    assert_eq!(found(&state), 0);
}

/// An inspect read again: what the reader closed and opened, and what
/// is searched, stay where the new document still has them.
#[tokio::test(start_paused = true)]
async fn another_document_keeps_what_was_closed_opened_and_searched() {
    let before = r#"{"State": {"Status": "running", "Pid": 1}, "Config": {"Env": ["A=1"]}, "Gone": {"x": 1}}"#;
    let after = r#"{"State": {"Status": "exited", "Pid": 0, "ExitCode": 137}, "Config": {"Env": ["A=1", "B=2"]}, "New": {"y": 2}}"#;
    let mut state = State::new(document(before));
    let node = |state: &State, path: &str| u32::try_from(at(state.document(), path)).unwrap();
    state.toggle(node(&state, ".Config"));
    settle(&mut state, Message::Typed("status".to_owned())).await;
    assert_eq!(state.matches(), (Some(0), 1));
    let closed = state.rows();

    let task = state.set_document(document(after));
    for message in messages(task).await {
        let _ = state.update(message);
    }
    // Config is still closed, State still open with its new member,
    // and what is new is open as a new document's would be.
    assert!(!state.is_open(node(&state, ".Config") as usize));
    assert!(state.is_open(node(&state, ".State") as usize));
    assert!(state.is_open(node(&state, ".New") as usize));
    assert_eq!(state.rows(), closed + 1, "one more member of State");
    // The search ran again, on the new document.
    assert_eq!(state.matches(), (Some(0), 1));
    assert_eq!(state.marked(), Some(node(&state, ".State.Status")));
    assert_eq!(state.document().node(2).unwrap().text, "\"exited\"");
    // A large document that was never touched stays as it started.
    let mut large_one = State::new(document(&large(1000)));
    let _ = large_one.set_document(document(&large(1001)));
    assert_eq!(large_one.rows(), 1002);
}
