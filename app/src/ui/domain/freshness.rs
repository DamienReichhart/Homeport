//! Whether a newer image was published under the tag a container or an
//! image has (`core/host/model/freshness.ts`), the mark that says so, and
//! the button that asks every registry again (`CheckNewerImages`).

use std::collections::HashMap;

use iced::Task;

use homeport_engine::{
    containers::Container,
    images::freshness::{Freshness, FreshnessResult},
};

use crate::{
    api::engine::images::images_freshness,
    data::{action::Finished, keys, Action},
    t,
    ui::{
        shell::Context,
        theme::{icon, Element, Icon},
        widgets::{
            basic::button::{self, Variant},
            feedback::Surfaced,
            overlay::{tooltip, Side},
        },
    },
};

/// The freshness of each reference and image pair, for quick lookups.
#[derive(Debug, Default)]
pub struct Index<'a>(HashMap<(&'a str, &'a str), Freshness>);

/// What a check found, by reference and by the image that runs under it.
pub fn index(results: Option<&[FreshnessResult]>) -> Index<'_> {
    Index(
        results
            .unwrap_or_default()
            .iter()
            .map(|result| {
                let pair = (result.reference.as_str(), result.image_id.as_str());
                (pair, result.freshness)
            })
            .collect(),
    )
}

impl Index<'_> {
    /// What is known of `reference` as the image `image_id` has it.
    pub fn of(&self, reference: &str, image_id: &str) -> Freshness {
        self.0
            .get(&(reference, image_id))
            .copied()
            .unwrap_or(Freshness::Unknown)
    }

    /// Whether the registry has a newer image under the tag a container
    /// was created from.
    pub fn of_container(&self, container: &Container) -> Freshness {
        self.of(&container.image, &container.image_id)
    }
}

/// One result for several (a Stack's services): newer if any is newer,
/// unknown only if all are, up to date otherwise.
pub fn aggregate(all: &[Freshness]) -> Freshness {
    if all.contains(&Freshness::Newer) {
        Freshness::Newer
    } else if all.iter().all(|freshness| *freshness == Freshness::Unknown) {
        Freshness::Unknown
    } else {
        Freshness::UpToDate
    }
}

/// The sentence for one or more references that have a newer image.
pub fn newer_label(references: &[&str]) -> String {
    match references {
        [reference] => t!("core.freshness.newerOne", reference = *reference),
        _ => t!(
            "core.freshness.newerMany",
            references = references.join(", ")
        ),
    }
}

/// The "newer image" mark: a small icon whose tooltip says what was
/// published (`NewerImage`).
pub fn newer_mark<'a, Message: Clone + 'a>(label: String) -> Element<'a, Message> {
    tooltip(icon(Icon::CircleArrowUp), label, Side::Top).into()
}

/// How many references have a newer image, each counted once.
fn newer_count(results: &[FreshnessResult]) -> usize {
    let mut newer: Vec<&str> = results
        .iter()
        .filter(|result| result.freshness == Freshness::Newer)
        .map(|result| result.reference.as_str())
        .collect();
    newer.sort_unstable();
    newer.dedup();
    newer.len()
}

/// "Check for newer images": asks every registry again, skipping the
/// cache, and says what it found.
#[derive(Debug, Default)]
pub struct Check(Action<Vec<FreshnessResult>>);

#[derive(Debug, Clone)]
pub enum Message {
    Pressed,
    Checked(Finished<Vec<FreshnessResult>>),
}

impl Check {
    pub fn is_pending(&self) -> bool {
        self.0.is_pending()
    }

    /// `host_id` is the Host whose images are checked.
    pub fn update(&mut self, host_id: &str, message: Message, ctx: &Context<'_>) -> Task<Message> {
        match message {
            Message::Pressed => {
                let host = host_id.to_owned();
                self.0
                    .start(ctx.state, move |state| async move {
                        images_freshness(&state, host, true).await
                    })
                    .map(Message::Checked)
            }
            Message::Checked(finished) => {
                self.0.finish(finished);
                if let Some(results) = self.0.done() {
                    let newer = newer_count(results);
                    // What it answered is what a read would.
                    ctx.put(keys::host::freshness(host_id), results.clone());
                    ctx.done(move || match newer {
                        0 => t!("core.freshness.allUpToDate"),
                        newer => t!("core.freshness.newerCount", count = newer),
                    });
                } else if let Some(error) = self.0.error() {
                    ctx.failed(Surfaced::new(error.clone(), "images_freshness"));
                }
                self.0.reset();
                Task::none()
            }
        }
    }

    /// The button, waiting while a check is on its way.
    pub fn button<'a>(&self) -> Element<'a, Message> {
        let label = if self.is_pending() {
            t!("core.freshness.checking")
        } else {
            t!("core.freshness.check")
        };
        button::labelled(Icon::RefreshCw, label)
            .variant(Variant::Outline)
            .on_press_maybe((!self.is_pending()).then_some(Message::Pressed))
            .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{fixtures::container, snapshot::in_language};

    fn result(reference: &str, image_id: &str, freshness: Freshness) -> FreshnessResult {
        FreshnessResult {
            reference: reference.to_owned(),
            image_id: image_id.to_owned(),
            freshness,
        }
    }

    fn results() -> Vec<FreshnessResult> {
        vec![
            result("nginx:1.27", "sha256:old", Freshness::Newer),
            result("nginx:1.27", "sha256:new", Freshness::UpToDate),
            result("nginx:latest", "sha256:new", Freshness::Newer),
            result("app:dev", "sha256:app", Freshness::Unknown),
        ]
    }

    #[test]
    fn a_container_is_read_by_its_reference_and_the_image_it_runs() {
        let results = results();
        let index = index(Some(&results));
        let c = |image: &str, image_id: &str| {
            let mut c = container("web", None, "running", 0);
            c.image = image.to_owned();
            c.image_id = image_id.to_owned();
            c
        };
        let of = |image, id| index.of_container(&c(image, id));
        assert_eq!(of("nginx:1.27", "sha256:old"), Freshness::Newer);
        assert_eq!(of("nginx:1.27", "sha256:new"), Freshness::UpToDate);
        assert_eq!(of("redis", "sha256:r"), Freshness::Unknown);
        // A check that knows nothing marks nothing.
        let nothing = super::index(None);
        assert_eq!(nothing.of("nginx:1.27", "sha256:old"), Freshness::Unknown);
    }

    #[test]
    fn a_stack_is_aggregated_from_its_services() {
        use Freshness::{Newer, Unknown, UpToDate};
        assert_eq!(aggregate(&[UpToDate, Newer, Unknown]), Newer);
        assert_eq!(aggregate(&[UpToDate, Unknown]), UpToDate);
        assert_eq!(aggregate(&[Unknown, Unknown]), Unknown);
        assert_eq!(aggregate(&[]), Unknown);
    }

    #[test]
    fn it_says_what_was_published_and_counts_each_reference_once() {
        let _english = in_language("en");
        assert_eq!(
            newer_label(&["nginx:1.27"]),
            "A newer nginx:1.27 was published"
        );
        assert_eq!(
            newer_label(&["a:1", "b:2"]),
            "Newer images were published: a:1, b:2"
        );
        let mut found = results();
        found.push(result("nginx:1.27", "sha256:older", Freshness::Newer));
        assert_eq!(newer_count(&found), 2);
    }
}
