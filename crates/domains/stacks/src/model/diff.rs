//! What a redeploy would change: the definition that is deployed (the
//! Stack's latest successful deploy) against the one about to be. Values of
//! environment variables are compared here and never returned: only whether
//! each key was added, removed, changed or kept.

use serde::{Deserialize, Serialize};

use crate::{model::definition, store::ComposeFile};

/// What happened to one environment variable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum EnvChange {
    Added,
    Removed,
    Changed,
    Unchanged,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct EnvKeyChange {
    pub key: String,
    pub change: EnvChange,
}

/// One compose file, as deployed and as about to be; `None` on the side
/// that doesn't have it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct FileChange {
    pub name: String,
    pub deployed: Option<String>,
    pub candidate: Option<String>,
}

/// The definition about to be deployed, when it isn't the stored one: the
/// editor's text for a Managed Stack, the edited files for a Linked one.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Candidate {
    pub files: Vec<ComposeFile>,
    /// `None` for a Linked Stack, which has no environment of its own.
    pub env: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct DeployDiff {
    /// Whether Homeport knows what is deployed: false when the Stack was
    /// deployed before it kept a history, or outside it.
    pub known: bool,
    /// When the deployed definition was deployed.
    pub deployed_at: Option<String>,
    /// The candidate's files in order, then those it no longer has.
    pub files: Vec<FileChange>,
    /// The order of the files changed, which changes how they merge.
    pub reordered: bool,
    pub env: Vec<EnvKeyChange>,
    pub deployed_profiles: Vec<String>,
    pub candidate_profiles: Vec<String>,
    /// Nothing at all would change.
    pub unchanged: bool,
}

/// One side of the comparison: compose files, environment and profiles.
pub struct Definition<'a> {
    pub files: &'a [ComposeFile],
    pub env: &'a str,
    pub profiles: &'a [String],
}

/// Compares what is deployed, if Homeport knows, with `candidate`.
pub fn diff(deployed: Option<(&str, Definition<'_>)>, candidate: Definition<'_>) -> DeployDiff {
    let Some((deployed_at, deployed)) = deployed else {
        return DeployDiff {
            known: false,
            deployed_at: None,
            files: candidate
                .files
                .iter()
                .map(|f| FileChange {
                    name: f.name.clone(),
                    deployed: None,
                    candidate: Some(f.content.clone()),
                })
                .collect(),
            reordered: false,
            env: env_changes("", candidate.env),
            deployed_profiles: vec![],
            candidate_profiles: candidate.profiles.to_vec(),
            unchanged: false,
        };
    };
    let mut files: Vec<FileChange> = candidate
        .files
        .iter()
        .map(|f| FileChange {
            name: f.name.clone(),
            deployed: deployed
                .files
                .iter()
                .find(|d| d.name == f.name)
                .map(|d| d.content.clone()),
            candidate: Some(f.content.clone()),
        })
        .collect();
    files.extend(
        deployed
            .files
            .iter()
            .filter(|d| !candidate.files.iter().any(|f| f.name == d.name))
            .map(|d| FileChange {
                name: d.name.clone(),
                deployed: Some(d.content.clone()),
                candidate: None,
            }),
    );
    let names = |fs: &[ComposeFile]| fs.iter().map(|f| f.name.clone()).collect::<Vec<_>>();
    let (before, after) = (names(deployed.files), names(candidate.files));
    let shared = |a: &[String], b: &[String]| -> Vec<String> {
        a.iter().filter(|n| b.contains(n)).cloned().collect()
    };
    let reordered = shared(&before, &after) != shared(&after, &before);
    let env = env_changes(deployed.env, candidate.env);
    let mut deployed_profiles = deployed.profiles.to_vec();
    let mut candidate_profiles = candidate.profiles.to_vec();
    deployed_profiles.sort();
    candidate_profiles.sort();
    let unchanged = !reordered
        && files.iter().all(|f| f.deployed == f.candidate)
        && env.iter().all(|e| e.change == EnvChange::Unchanged)
        && deployed_profiles == candidate_profiles;
    DeployDiff {
        known: true,
        deployed_at: Some(deployed_at.to_owned()),
        files,
        reordered,
        env,
        deployed_profiles,
        candidate_profiles,
        unchanged,
    }
}

/// Each variable of either environment and whether it was added, removed,
/// changed or kept, sorted by name. Unparsable text counts as empty.
pub fn env_changes(deployed: &str, candidate: &str) -> Vec<EnvKeyChange> {
    let parse = |env: &str| definition::parse_env(env).unwrap_or_default();
    let (before, after) = (parse(deployed), parse(candidate));
    let value = |vars: &[(String, String)], key: &str| {
        vars.iter()
            .rev()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.clone())
    };
    let mut keys: Vec<String> = before
        .iter()
        .chain(&after)
        .map(|(k, _)| k.clone())
        .collect();
    keys.sort();
    keys.dedup();
    keys.into_iter()
        .map(|key| {
            let change = match (value(&before, &key), value(&after, &key)) {
                (None, Some(_)) => EnvChange::Added,
                (Some(_), None) => EnvChange::Removed,
                (Some(a), Some(b)) if a != b => EnvChange::Changed,
                _ => EnvChange::Unchanged,
            };
            EnvKeyChange { key, change }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn def<'a>(files: &'a [ComposeFile], env: &'a str) -> Definition<'a> {
        Definition {
            files,
            env,
            profiles: &[],
        }
    }

    #[test]
    fn env_keys_are_compared_without_their_values() {
        let changes = env_changes(
            "PASSWORD=old\nKEPT=1\nGONE=x\n",
            "PASSWORD=new\nKEPT=1\nNEW=y\n",
        );
        let keyed: Vec<(&str, EnvChange)> =
            changes.iter().map(|c| (c.key.as_str(), c.change)).collect();
        assert_eq!(
            keyed,
            [
                ("GONE", EnvChange::Removed),
                ("KEPT", EnvChange::Unchanged),
                ("NEW", EnvChange::Added),
                ("PASSWORD", EnvChange::Changed),
            ]
        );
        let json = serde_json::to_string(&changes).unwrap();
        assert!(!json.contains("old") && !json.contains("new\""), "{json}");
    }

    #[test]
    fn a_changed_tag_shows_in_its_file() {
        let before = [ComposeFile::new("compose.yaml", "image: nginx:1.26\n")];
        let after = [ComposeFile::new("compose.yaml", "image: nginx:1.27\n")];
        let d = diff(Some(("then", def(&before, "A=1"))), def(&after, "A=1"));
        assert!(d.known && !d.unchanged && !d.reordered);
        assert_eq!(d.files[0].deployed.as_deref(), Some("image: nginx:1.26\n"));
        assert_eq!(d.files[0].candidate.as_deref(), Some("image: nginx:1.27\n"));
        assert_eq!(d.env[0].change, EnvChange::Unchanged);
    }

    #[test]
    fn nothing_changed_says_so() {
        let files = [ComposeFile::new("compose.yaml", "services: {}\n")];
        let d = diff(Some(("then", def(&files, "A=1"))), def(&files, "A=1"));
        assert!(d.unchanged);
        // A profile turned on is a change.
        let on = ["debug".to_owned()];
        let d = diff(
            Some(("then", def(&files, "A=1"))),
            Definition {
                profiles: &on,
                ..def(&files, "A=1")
            },
        );
        assert!(!d.unchanged);
    }

    #[test]
    fn added_removed_and_reordered_files_are_told_apart() {
        let a = ComposeFile::new("a.yaml", "a");
        let b = ComposeFile::new("b.yaml", "b");
        let c = ComposeFile::new("c.yaml", "c");
        let d = diff(
            Some(("then", def(&[a.clone(), b.clone()], ""))),
            def(&[b.clone(), a.clone(), c.clone()], ""),
        );
        assert!(d.reordered && !d.unchanged);
        let names: Vec<_> = d.files.iter().map(|f| f.name.as_str()).collect();
        assert_eq!(names, ["b.yaml", "a.yaml", "c.yaml"]);
        assert_eq!(d.files[2].deployed, None);
        let d = diff(Some(("then", def(&[a.clone(), b], ""))), def(&[a], ""));
        assert_eq!(d.files[1].candidate, None);
        assert!(!d.reordered);
    }

    #[test]
    fn an_unknown_deploy_shows_the_new_definition_only() {
        let files = [ComposeFile::new("compose.yaml", "services: {}\n")];
        let d = diff(None, def(&files, "A=1"));
        assert!(!d.known && !d.unchanged);
        assert_eq!(d.files[0].deployed, None);
        assert_eq!(d.env[0].change, EnvChange::Added);
    }
}
