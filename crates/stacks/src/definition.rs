//! Checks on a Stack's compose definition and environment.
//!
//! Compose resolves relative paths and reads `env_file`/`build` contexts on
//! the machine running the CLI, which isn't the Host (ADR 0001). A Managed
//! Stack has no folder on this machine, so those are refused or flagged
//! before deploying; each [`SourceKind`] says which rules apply.

use serde::Serialize;
use serde_yaml_ng::{Mapping, Value};

use homeport_core::error::{AppError, AppResult};

use crate::{messages as msg, runtime::SourceKind, store::ComposeFile};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ComposeCheck {
    /// Problems that block deploying.
    pub errors: Vec<String>,
    /// Things that will probably not do what the author expects.
    pub warnings: Vec<String>,
    /// The Compose profiles its services declare, sorted.
    pub profiles: Vec<String>,
}

impl ComposeCheck {
    pub fn is_deployable(&self) -> bool {
        self.errors.is_empty()
    }
}

/// Compose project names: lowercase letters, digits, `-` and `_`, starting
/// with a letter or digit.
pub fn validate_name(name: &str) -> AppResult<()> {
    let mut chars = name.chars();
    let valid_start = chars
        .next()
        .is_some_and(|c| c.is_ascii_lowercase() || c.is_ascii_digit());
    if valid_start
        && chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_')
    {
        Ok(())
    } else {
        Err(AppError::validation(msg::STACK_NAME))
    }
}

/// A Managed Stack's compose files: at least one, each a plain file name
/// ending in `.yaml` or `.yml`, no two alike.
pub fn validate_files(files: &[ComposeFile]) -> AppResult<()> {
    if files.is_empty() {
        return Err(AppError::validation(msg::NO_FILES));
    }
    for (i, file) in files.iter().enumerate() {
        let name = file.name.as_str();
        let plain = !name.is_empty()
            && !name.starts_with('.')
            && !name.contains(['/', '\\', ':'])
            && (name.ends_with(".yaml") || name.ends_with(".yml"));
        if !plain {
            return Err(AppError::validation(msg::FILE_NAME.with("name", name)));
        }
        if files[..i].iter().any(|f| f.name == file.name) {
            return Err(AppError::validation(
                msg::FILE_NAME_TWICE.with("name", name),
            ));
        }
    }
    Ok(())
}

/// What can be found wrong with a definition: the problems that block
/// deploying it, as errors, and the warnings as the editor shows them.
#[derive(Default)]
struct Review {
    blockers: Vec<AppError>,
    warnings: Vec<String>,
}

/// Checks a definition made of `files` (their YAML text, merged in order the
/// way compose merges them) against the rules of `kind`, for a Host.
pub fn check(files: &[&str], kind: SourceKind) -> ComposeCheck {
    check_for(files, kind, false)
}

/// [`check`], for a Swarm when `swarm`: `docker stack deploy`'s rules too.
pub fn check_for(files: &[&str], kind: SourceKind, swarm: bool) -> ComposeCheck {
    let review = review(files, kind, swarm);
    ComposeCheck {
        // The editor lists each blocker with its raw cause, if any.
        errors: review
            .blockers
            .into_iter()
            .map(|e| match e.detail {
                Some(detail) => format!("{} {detail}", e.message),
                None => e.message,
            })
            .collect(),
        warnings: review.warnings,
        profiles: declared_profiles(files),
    }
}

/// The profiles the services of `files` (merged) declare, sorted, once each.
pub fn declared_profiles(files: &[&str]) -> Vec<String> {
    let docs: Vec<Value> = files
        .iter()
        .filter_map(|f| serde_yaml_ng::from_str(f).ok())
        .collect();
    let mut profiles: Vec<String> = merge(&docs)
        .services
        .unwrap_or_default()
        .iter()
        .filter_map(|(_, service)| service.get("profiles").and_then(Value::as_sequence))
        .flatten()
        .filter_map(|p| p.as_str().map(str::to_owned))
        .collect();
    profiles.sort();
    profiles.dedup();
    profiles
}

/// The services of `files` (merged), sorted.
pub fn service_names(files: &[&str]) -> Vec<String> {
    let docs: Vec<Value> = files
        .iter()
        .filter_map(|f| serde_yaml_ng::from_str(f).ok())
        .collect();
    let mut names: Vec<String> = merge(&docs)
        .services
        .unwrap_or_default()
        .into_iter()
        .map(|(name, _)| name)
        .collect();
    names.sort();
    names
}

/// Whether a service of `files` (merged) is built from local files, so a
/// deploy of changed files should rebuild it.
pub fn builds(files: &[&str]) -> bool {
    let docs: Vec<Value> = files
        .iter()
        .filter_map(|f| serde_yaml_ng::from_str(f).ok())
        .collect();
    merge(&docs)
        .services
        .unwrap_or_default()
        .iter()
        .any(|(_, service)| service.contains_key("build"))
}

/// The services of `files` (merged) that sit only behind profiles none of
/// which is `active`, so they don't run.
pub fn services_turned_off(files: &[&str], active: &[String]) -> Vec<String> {
    let docs: Vec<Value> = files
        .iter()
        .filter_map(|f| serde_yaml_ng::from_str(f).ok())
        .collect();
    merge(&docs)
        .services
        .unwrap_or_default()
        .into_iter()
        .filter(|(_, service)| {
            let profiles: Vec<&str> = service
                .get("profiles")
                .and_then(Value::as_sequence)
                .into_iter()
                .flatten()
                .filter_map(Value::as_str)
                .collect();
            !profiles.is_empty() && !profiles.iter().any(|p| active.iter().any(|a| a == p))
        })
        .map(|(name, _)| name)
        .collect()
}

/// Compose profile names: letters, digits, `_`, `.` and `-`, starting with a
/// letter or digit.
pub fn validate_profiles(profiles: &[String]) -> AppResult<()> {
    for name in profiles {
        let mut chars = name.chars();
        let valid = chars.next().is_some_and(|c| c.is_ascii_alphanumeric())
            && chars.all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | '-'));
        if !valid {
            return Err(AppError::validation(msg::PROFILE_NAME.with("name", name)));
        }
    }
    Ok(())
}

/// The first problem that blocks deploying `files` to a Host, if any.
pub fn blocker(files: &[&str], kind: SourceKind) -> Option<AppError> {
    blocker_for(files, kind, false)
}

/// [`blocker`], for a Swarm when `swarm`.
pub fn blocker_for(files: &[&str], kind: SourceKind, swarm: bool) -> Option<AppError> {
    review(files, kind, swarm).blockers.into_iter().next()
}

/// Keys of a service `docker stack deploy` ignores, with what to use instead.
const IGNORED_ON_SWARM: [(&str, &str); 3] = [
    ("container_name", "a Swarm names each task itself"),
    ("restart", "use deploy.restart_policy"),
    (
        "links",
        "services reach each other by name on their networks",
    ),
];

/// Whether the definition's files on this machine are its project files, so
/// `build:`, `env_file:` and relative paths mean what their author meant.
fn has_project_folder(kind: SourceKind) -> bool {
    match kind {
        SourceKind::Managed => false,
        SourceKind::Linked | SourceKind::Synced => true,
    }
}

/// `files` merged the way compose merges them, as far as the checks need:
/// each service's keys, later files overriding earlier ones, so an override
/// can add `ports:` to a service whose `image:` is in the base file.
struct Merged {
    /// `None` when no file has a `services:` section.
    services: Option<Vec<(String, Mapping)>>,
    sections: Vec<(&'static str, Vec<(String, Value)>)>,
}

fn merge(docs: &[Value]) -> Merged {
    let mut services: Option<Vec<(String, Mapping)>> = None;
    for doc in docs {
        let Some(section) = doc.get("services").and_then(Value::as_mapping) else {
            continue;
        };
        let merged = services.get_or_insert_with(Vec::new);
        for (name, service) in section {
            let name = name.as_str().unwrap_or("?").to_owned();
            let keys = service.as_mapping().cloned().unwrap_or_default();
            match merged.iter_mut().find(|(n, _)| *n == name) {
                Some((_, existing)) => existing.extend(keys),
                None => merged.push((name, keys)),
            }
        }
    }
    let sections = ["configs", "secrets"]
        .into_iter()
        .map(|section| {
            let entries = docs
                .iter()
                .filter_map(|d| d.get(section).and_then(Value::as_mapping))
                .flatten()
                .map(|(k, v)| (k.as_str().unwrap_or("?").to_owned(), v.clone()))
                .collect();
            (section, entries)
        })
        .collect();
    Merged { services, sections }
}

fn review(files: &[&str], kind: SourceKind, swarm: bool) -> Review {
    let mut check = Review::default();
    let local = has_project_folder(kind);
    let mut docs = Vec::with_capacity(files.len());
    for yaml in files {
        match serde_yaml_ng::from_str::<Value>(yaml) {
            Ok(doc) => docs.push(doc),
            Err(e) => {
                check
                    .blockers
                    .push(AppError::validation(msg::COMPOSE_UNPARSABLE).with_detail(e));
                return check;
            }
        }
    }
    let merged = merge(&docs);
    let Some(services) = merged.services else {
        check
            .blockers
            .push(AppError::validation(msg::COMPOSE_NO_SERVICES_SECTION));
        return check;
    };
    if services.is_empty() {
        check
            .blockers
            .push(AppError::validation(msg::COMPOSE_NO_SERVICE));
    }

    for (name, service) in &services {
        let name = name.as_str();
        let service = Value::Mapping(service.clone());
        let builds = service.get("build").is_some();
        let has_image = service.get("image").is_some();
        if swarm && builds && !has_image {
            // A Swarm's nodes pull their images; one built here reaches none.
            check.blockers.push(AppError::validation(
                msg::COMPOSE_BUILD_SWARM.with("service", name),
            ));
        } else if !local && builds {
            check.blockers.push(AppError::validation(
                msg::COMPOSE_BUILD.with("service", name),
            ));
        } else if swarm && builds {
            check.warnings.push(format!(
                "Service \"{name}\" has build:, which a Swarm ignores: its nodes pull {} from its registry, so push the image there first.",
                service.get("image").and_then(Value::as_str).unwrap_or("its image")
            ));
        }
        if !has_image && !builds {
            check.blockers.push(AppError::validation(
                msg::COMPOSE_NO_IMAGE.with("service", name),
            ));
        }
        if swarm {
            swarm_warnings(&mut check, name, &service);
        }
        if local {
            continue;
        }
        if service.get("env_file").is_some() {
            check.warnings.push(format!(
                "Service \"{name}\" uses env_file:, which is read on this machine. Put the variables in the Stack's environment instead."
            ));
        }
        if swarm {
            // Already said, as paths on each node.
            continue;
        }
        for volume in service
            .get("volumes")
            .and_then(Value::as_sequence)
            .into_iter()
            .flatten()
        {
            if let Some(source) = bind_source(volume).filter(|s| is_relative(s)) {
                check.warnings.push(format!(
                    "Service \"{name}\" mounts \"{source}\", a path relative to this machine. Use an absolute path on the Host or a named volume."
                ));
            }
        }
    }

    if local {
        return check;
    }
    for (section, entries) in &merged.sections {
        for (name, entry) in entries {
            if entry.get("file").is_none() {
                continue;
            }
            if swarm {
                // A Managed Stack's deploy has no folder to read it from.
                check.blockers.push(AppError::validation(
                    msg::COMPOSE_FILE_OBJECT_SWARM
                        .with("section", *section)
                        .with("name", name),
                ));
            } else {
                check.warnings.push(format!(
                    "{section} \"{name}\" is read from a file on this machine."
                ));
            }
        }
    }
    check
}

/// What `docker stack deploy` does differently with a service of a Swarm:
/// keys it ignores, and bind mounts, which are paths on each node.
fn swarm_warnings(check: &mut Review, name: &str, service: &Value) {
    for (key, instead) in IGNORED_ON_SWARM {
        if service.get(key).is_some() {
            check.warnings.push(format!(
                "Service \"{name}\" sets {key}:, which docker stack deploy ignores: {instead}."
            ));
        }
    }
    let waits = service
        .get("depends_on")
        .and_then(Value::as_mapping)
        .is_some_and(|d| d.values().any(|v| v.get("condition").is_some()));
    if waits {
        check.warnings.push(format!(
            "Service \"{name}\" waits for its depends_on: conditions, which a Swarm ignores: services start in any order, so make it retry until what it needs is up."
        ));
    }
    if service.get("profiles").is_some() {
        check.warnings.push(format!(
            "Service \"{name}\" has profiles:, which docker stack deploy ignores: it's deployed whatever profiles are on."
        ));
    }
    for volume in service
        .get("volumes")
        .and_then(Value::as_sequence)
        .into_iter()
        .flatten()
    {
        if let Some(source) = bind_source(volume) {
            let relative = if is_relative(&source) {
                " A relative path is resolved on this computer first, then looked for on each node."
            } else {
                ""
            };
            check.warnings.push(format!(
                "Service \"{name}\" mounts \"{source}\", a path on whichever node runs its task, which must have it.{relative}"
            ));
        }
    }
}

/// The bind mounts of `files` (merged) whose source is a relative path, by
/// service.
pub fn relative_binds(files: &[&str]) -> Vec<(String, String)> {
    let docs: Vec<Value> = files
        .iter()
        .filter_map(|f| serde_yaml_ng::from_str(f).ok())
        .collect();
    let mut binds = Vec::new();
    for (name, service) in merge(&docs).services.unwrap_or_default() {
        for volume in service
            .get("volumes")
            .and_then(Value::as_sequence)
            .into_iter()
            .flatten()
        {
            if let Some(source) = bind_source(volume).filter(|s| is_relative(s)) {
                binds.push((name.clone(), source));
            }
        }
    }
    binds
}

/// The host side of a service volume, when it's a path rather than a named volume.
fn bind_source(volume: &Value) -> Option<String> {
    match volume {
        Value::String(short) => {
            let source = short.split(':').next()?;
            (source.starts_with(['.', '~', '/'])).then(|| source.to_owned())
        }
        Value::Mapping(_) => {
            let kind = volume.get("type").and_then(Value::as_str);
            let source = volume.get("source").and_then(Value::as_str)?;
            (kind == Some("bind")).then(|| source.to_owned())
        }
        _ => None,
    }
}

fn is_relative(path: &str) -> bool {
    !path.starts_with('/')
}

/// Parses `.env` text into variables for the compose process. Supports
/// comments, `export` prefixes and single or double quotes.
pub fn parse_env(text: &str) -> AppResult<Vec<(String, String)>> {
    let mut vars = Vec::new();
    for (n, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let line = line.strip_prefix("export ").unwrap_or(line);
        let Some((key, value)) = line.split_once('=') else {
            return Err(AppError::validation(
                msg::ENV_NOT_KEY_VALUE.with("line", n + 1),
            ));
        };
        let key = key.trim();
        if key.is_empty() || !key.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            return Err(AppError::validation(msg::ENV_BAD_NAME.with("line", n + 1)));
        }
        let value = value.trim();
        let value = if let Some(inner) = value.strip_prefix('"').and_then(|v| v.strip_suffix('"')) {
            inner.replace("\\n", "\n").replace("\\\"", "\"")
        } else if let Some(inner) = value.strip_prefix('\'').and_then(|v| v.strip_suffix('\'')) {
            inner.to_owned()
        } else {
            // Unquoted values end at an inline comment.
            value.split(" #").next().unwrap_or("").trim_end().to_owned()
        };
        vars.push((key.to_owned(), value));
    }
    Ok(vars)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_managed(yaml: &str) -> ComposeCheck {
        check(&[yaml], SourceKind::Managed)
    }

    #[test]
    fn stack_names_follow_compose_rules() {
        assert!(validate_name("shop-2_prod").is_ok());
        for bad in ["", "Shop", "-shop", "my shop", "café"] {
            assert!(validate_name(bad).is_err(), "{bad}");
        }
    }

    #[test]
    fn compose_file_names_are_plain_and_unique() {
        let f = |n: &str| ComposeFile::new(n, "");
        assert!(validate_files(&[f("compose.yaml"), f("compose.prod.yml")]).is_ok());
        assert!(validate_files(&[]).is_err());
        for bad in [
            "",
            "../x.yaml",
            "dir/x.yaml",
            "x.txt",
            ".hidden.yaml",
            "C:x.yaml",
        ] {
            assert!(validate_files(&[f(bad)]).is_err(), "{bad}");
        }
        assert!(validate_files(&[f("a.yaml"), f("a.yaml")]).is_err());
    }

    #[test]
    fn a_swarm_refuses_local_builds_without_an_image() {
        let built = "services:\n  app:\n    build: .\n";
        let pushed = "services:\n  app:\n    build: .\n    image: registry.example.com/app:1\n";
        for kind in [SourceKind::Managed, SourceKind::Linked, SourceKind::Synced] {
            let refused = check_for(&[built], kind, true);
            assert!(!refused.is_deployable(), "{kind:?}");
            assert!(
                refused.errors[0].contains("pull their images"),
                "{refused:?}"
            );
        }
        // With an image, a Linked Stack builds here and the nodes pull it.
        let linked = check_for(&[pushed], SourceKind::Linked, true);
        assert!(linked.is_deployable(), "{linked:?}");
        assert!(linked.warnings.iter().any(|w| w.contains("push the image")));
        // On a Host, a Linked Stack builds without an image, as before.
        assert!(check_for(&[built], SourceKind::Linked, false).is_deployable());
    }

    #[test]
    fn a_swarm_warns_about_what_stack_deploy_ignores() {
        let yaml = "services:\n  web:\n    image: nginx\n    container_name: web\n    restart: always\n    depends_on:\n      db:\n        condition: service_healthy\n    volumes:\n      - ./site:/usr/share/nginx/html\n      - /srv/data:/data\n  db:\n    image: postgres\n";
        let checked = check_for(&[yaml], SourceKind::Linked, true);
        assert!(checked.is_deployable());
        let has = |needle: &str| checked.warnings.iter().any(|w| w.contains(needle));
        assert!(has("container_name:"));
        assert!(has("restart:"));
        assert!(has("depends_on:"));
        assert!(has("\"./site\", a path on whichever node"));
        assert!(has("\"/srv/data\", a path on whichever node"));
        // None of that is said for a Host.
        assert!(check_for(&[yaml], SourceKind::Linked, false)
            .warnings
            .is_empty());
    }

    #[test]
    fn file_configs_need_a_folder_on_a_swarm() {
        let yaml = "services:\n  web:\n    image: nginx\n    configs: [conf]\nconfigs:\n  conf:\n    file: ./nginx.conf\n";
        assert!(!check_for(&[yaml], SourceKind::Managed, true).is_deployable());
        assert!(check_for(&[yaml], SourceKind::Managed, false).is_deployable());
        assert!(check_for(&[yaml], SourceKind::Linked, true).is_deployable());
        assert!(check_for(&[yaml], SourceKind::Synced, true).is_deployable());
    }

    #[test]
    fn profiles_are_the_union_of_every_service_s() {
        let base = "services:\n  web:\n    image: nginx\n  debug:\n    image: busybox\n    profiles: [debug, tools]\n";
        let extra = "services:\n  worker:\n    image: app\n    profiles: [workers, debug]\n";
        assert_eq!(
            declared_profiles(&[base, extra]),
            ["debug", "tools", "workers"]
        );
        assert_eq!(
            check(&[base], SourceKind::Managed).profiles,
            ["debug", "tools"]
        );
        assert!(declared_profiles(&["services:\n  web:\n    image: x\n"]).is_empty());
        assert!(validate_profiles(&["debug".into(), "a.b-c_1".into()]).is_ok());
        assert!(validate_profiles(&["-x".into()]).is_err());
        assert_eq!(
            services_turned_off(&[base, extra], &[]),
            ["debug", "worker"]
        );
        assert_eq!(
            services_turned_off(&[base, extra], &["tools".into()]),
            ["worker"]
        );
        assert!(validate_profiles(&["a b".into()]).is_err());
    }

    #[test]
    fn plain_image_stacks_pass() {
        let c = check_managed(
            "services:\n  web:\n    image: nginx\n    volumes:\n      - data:/data\n      - /srv/site:/usr/share/nginx/html:ro\nvolumes:\n  data: {}\n",
        );
        assert_eq!(c, ComposeCheck::default());
    }

    #[test]
    fn local_builds_are_refused() {
        let c = check_managed("services:\n  app:\n    build: .\n");
        assert!(!c.is_deployable());
        assert!(c.errors[0].contains("build:"));
    }

    #[test]
    fn paths_read_on_this_machine_are_flagged() {
        let c = check_managed(
            "services:\n  app:\n    image: app\n    env_file: .env\n    volumes:\n      - ./data:/data\n      - type: bind\n        source: ~/conf\n        target: /conf\nsecrets:\n  key:\n    file: ./key.pem\n",
        );
        assert!(c.is_deployable());
        assert_eq!(c.warnings.len(), 4, "{:?}", c.warnings);
    }

    #[test]
    fn malformed_definitions_are_explained() {
        assert!(!check_managed("services: [").is_deployable());
        assert!(!check_managed("version: '3'\n").is_deployable());
        assert!(!check_managed("services:\n  app:\n    ports: []\n").is_deployable());
    }

    #[test]
    fn linked_stacks_may_build_and_use_paths_in_their_folder() {
        let yaml = "services:\n  app:\n    build: .\n    env_file: .env\n    volumes:\n      - ./data:/data\nsecrets:\n  key:\n    file: ./key.pem\n";
        assert_eq!(check(&[yaml], SourceKind::Linked), ComposeCheck::default());
        // The structure is still checked.
        assert!(
            !check(&["services:\n  app:\n    ports: []\n"], SourceKind::Linked).is_deployable()
        );
        assert!(!check(&["version: '3'\n"], SourceKind::Linked).is_deployable());
    }

    #[test]
    fn several_files_are_checked_merged() {
        let base = "services:\n  web:\n    image: nginx\n";
        let prod = "services:\n  web:\n    ports: [\"80:80\"]\n";
        assert!(check(&[prod], SourceKind::Managed).errors[0].contains("needs an image"));
        assert_eq!(
            check(&[base, prod], SourceKind::Managed),
            ComposeCheck::default()
        );
        let extra = "services:\n  worker:\n    command: [\"run\"]\n";
        let c = check(&[base, extra], SourceKind::Managed);
        assert_eq!(c.errors.len(), 1, "{c:?}");
        assert!(c.errors[0].contains("worker"));
        // An override file alone needs no services: section of its own.
        assert_eq!(
            check(&[base, "volumes:\n  data: {}\n"], SourceKind::Managed),
            ComposeCheck::default()
        );
    }

    #[test]
    fn relative_binds_are_listed_by_service() {
        let base = "services:\n  app:\n    image: x\n    volumes:\n      - ./data:/data\n      - /srv/abs:/abs\n      - named:/named\n";
        assert_eq!(
            relative_binds(&[base]),
            [("app".to_owned(), "./data".to_owned())]
        );
    }

    #[test]
    fn env_files_parse_like_compose() {
        let vars = parse_env(
            "# comment\nexport A=1\nB = two words # note\nC=\"quoted # kept\"\nD='single'\nE=\n",
        )
        .unwrap();
        assert_eq!(
            vars,
            [
                ("A".into(), "1".into()),
                ("B".into(), "two words".into()),
                ("C".into(), "quoted # kept".into()),
                ("D".into(), "single".into()),
                ("E".into(), String::new()),
            ]
        );
        assert!(parse_env("not a pair").is_err());
        assert!(parse_env("BAD-NAME=1").is_err());
    }
}
