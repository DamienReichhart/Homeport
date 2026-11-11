//! Messages with tags (`Use <mono>{ $name }</mono>`): the text split into
//! spans the window styles, since Fluent itself knows no markup.

/// How a span of a message is shown.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tag {
    /// `<strong>`: emphasised.
    Strong,
    /// `<mono>` and `<code>`: in the monospace font.
    Mono,
    /// `<link>`: opens what the page says it opens.
    Link,
}

/// A run of a message's text, and the tag it sits in, if any.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    pub text: String,
    pub tag: Option<Tag>,
}

/// Stands for `<` in an argument's value while the message is split, so a
/// container named `<b>` is text, not markup.
const ESCAPED: char = '\u{E000}';

/// An argument's value, made safe to sit in a message with tags.
pub fn escape(value: &str) -> String {
    value.replace('<', &ESCAPED.to_string())
}

fn tag(name: &str) -> Option<Tag> {
    match name {
        "strong" => Some(Tag::Strong),
        "mono" | "code" => Some(Tag::Mono),
        "link" => Some(Tag::Link),
        _ => None,
    }
}

/// Splits a rendered message at its tags. A tag the app doesn't know, or one
/// that never closes, stays as text.
pub fn spans(message: &str) -> Vec<Span> {
    let mut spans: Vec<Span> = vec![];
    let mut push = |text: &str, tag: Option<Tag>| {
        if text.is_empty() {
            return;
        }
        let text = text.replace(ESCAPED, "<");
        match spans.last_mut() {
            Some(last) if last.tag == tag => last.text.push_str(&text),
            _ => spans.push(Span { text, tag }),
        }
    };
    let mut rest = message;
    while let Some(open) = rest.find('<') {
        let after = &rest[open + 1..];
        let known = after.find('>').and_then(|end| {
            let name = &after[..end];
            let kind = tag(name)?;
            let body = &after[end + 1..];
            let close = body.find(&format!("</{name}>"))?;
            Some((kind, &body[..close], &body[close + name.len() + 3..]))
        });
        match known {
            Some((kind, inside, following)) => {
                push(&rest[..open], None);
                push(inside, Some(kind));
                rest = following;
            }
            None => {
                push(&rest[..=open], None);
                rest = after;
            }
        }
    }
    push(rest, None);
    spans
}

#[cfg(test)]
mod tests {
    use super::*;

    fn span(text: &str, tag: Option<Tag>) -> Span {
        Span {
            text: text.to_owned(),
            tag,
        }
    }

    #[test]
    fn splits_a_message_at_its_tags() {
        assert_eq!(
            spans("Use <mono>docker ps</mono> or <strong>stop</strong>."),
            [
                span("Use ", None),
                span("docker ps", Some(Tag::Mono)),
                span(" or ", None),
                span("stop", Some(Tag::Strong)),
                span(".", None),
            ]
        );
        assert_eq!(spans("<code>x</code>"), [span("x", Some(Tag::Mono))]);
        assert_eq!(spans("plain"), [span("plain", None)]);
        assert_eq!(spans(""), []);
    }

    #[test]
    fn keeps_what_is_not_markup_as_text() {
        assert_eq!(spans("a < b"), [span("a < b", None)]);
        assert_eq!(spans("<b>bold</b>"), [span("<b>bold</b>", None)]);
        assert_eq!(spans("<mono>open"), [span("<mono>open", None)]);
    }

    #[test]
    fn an_argument_cannot_bring_markup() {
        let message = format!("Remove <strong>{}</strong>?", escape("<mono>x</mono>"));
        assert_eq!(
            spans(&message),
            [
                span("Remove ", None),
                span("<mono>x</mono>", Some(Tag::Strong)),
                span("?", None),
            ]
        );
    }
}
