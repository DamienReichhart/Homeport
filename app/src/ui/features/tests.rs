//! The table of pages against the registry: every feature that has a
//! route has a page, under the name the registry gives it.

use std::collections::HashSet;

use super::PAGES;
use crate::ui::shell::registry::{FeatureId, FEATURES};

#[test]
fn every_feature_with_a_route_has_a_page() {
    let pages: HashSet<FeatureId> = PAGES.iter().copied().collect();
    assert_eq!(pages.len(), PAGES.len(), "a feature is listed twice");
    for feature in FEATURES {
        assert_eq!(
            pages.contains(&feature.id),
            !feature.routes.is_empty(),
            "{}",
            feature.id.name()
        );
    }
}
