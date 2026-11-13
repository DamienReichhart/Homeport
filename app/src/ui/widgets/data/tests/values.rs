//! Labelled values: where the labels and the values stand, in both
//! directions, and what stands for a value there is none of.

use crate::{
    test_support::{snapshot::in_language, stage::Stage},
    ui::{
        theme::Element,
        widgets::data::{key_value, key_value::NONE, Item},
    },
};

fn list<'a>(label: &str) -> Element<'a, ()> {
    key_value(vec![
        Item::text("Image", "nginx:1.27"),
        Item::text(label, "the value of the long one"),
        Item::maybe("Health", None),
        Item::maybe("Restarts", Some("3".to_owned())),
    ])
}

#[test]
fn the_values_start_in_one_column_after_the_longest_label() {
    let _english = in_language("en");
    let mut stage = Stage::new((700.0, 300.0));
    stage.frame(list("Restart policy"), |frame| {
        let [image, long, label, name] = [
            "nginx:1.27",
            "the value of the long one",
            "Restart policy",
            "Image",
        ]
        .map(|text| frame.find(text).unwrap_or_else(|| panic!("no {text}")));
        // One column for the values, after the longest label and the gap.
        assert_eq!(image.x, long.x);
        assert!(
            label.x + label.width <= long.x - 24.0,
            "{label:?} then {long:?}"
        );
        assert!(
            long.x - (label.x + label.width) <= 26.0,
            "as wide as the longest label"
        );
        // The labels start where the list does, each on its value's line.
        assert_eq!(name.x, label.x);
        assert_eq!(name.y, image.y);
        // A value there is none of is a dash; one there is, itself.
        assert!(frame.texts().contains(&NONE.to_owned()));
        assert!(frame.find("3").is_some());
    });
}

/// `max-content` in React let one long label push every value out of
/// view: here the labels take 240 px at the most and wrap past it.
#[test]
fn a_label_longer_than_240_px_wraps_and_the_values_keep_their_room() {
    let _english = in_language("en");
    let long = "A label so long that it would take the whole width of the list if it were let";
    let mut stage = Stage::new((700.0, 300.0));
    stage.frame(list(long), |frame| {
        let label = frame.find(long).expect("the label");
        let value = frame.find("the value of the long one").expect("the value");
        assert!(label.width <= 240.0, "{}", label.width);
        assert!(label.height > 20.0, "it wraps: {}", label.height);
        assert_eq!(value.x, label.x + 240.0 + 24.0);
    });
}

#[test]
fn in_arabic_the_labels_are_on_the_right_and_the_values_before_them() {
    let _arabic = in_language("ar");
    let mut stage = Stage::new((700.0, 300.0));
    stage.frame(list("Restart policy"), |frame| {
        let label = frame.find("Restart policy").expect("the label");
        let value = frame.find("the value of the long one").expect("the value");
        assert!(
            value.x + value.width <= label.x - 24.0,
            "{value:?} then {label:?}"
        );
        // The labels end at the list's right edge.
        assert!((label.x + label.width - 700.0).abs() <= 1.0, "{label:?}");
    });
}
