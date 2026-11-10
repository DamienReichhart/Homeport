//! Every Stack of a Host, from its containers and Homeport's records, with
//! Linked Stacks found along the way (ADR 0008).

use std::collections::BTreeMap;

use homeport_core::error::AppResult;
use homeport_engine::containers::Container;

use crate::{
    linked::{detect, files, FileCache, LinkedStore},
    stack::{derive, Stack, StackRecord},
    store::StackStore,
    synced::SyncedStore,
};

/// The Stacks of the Host `host_id`, given its `containers`. A Stack found
/// running from compose files on this machine is recorded as Linked the
/// first time it's seen; a Linked Stack whose files are gone is listed as
/// External, with why, until they come back.
pub fn list(
    host_id: &str,
    containers: &[Container],
    managed: &StackStore,
    linked: &LinkedStore,
    synced: &SyncedStore,
    cache: &FileCache,
) -> AppResult<Vec<Stack>> {
    let mut records: Vec<StackRecord> = managed
        .list(host_id)?
        .into_iter()
        .map(|s| StackRecord::managed(s.id, s.name))
        .collect();
    // A Synced Stack runs from its worktree on this machine; it's never Linked.
    records.extend(
        synced
            .list(host_id)?
            .into_iter()
            .map(|s| StackRecord::synced(s.id, s.name).with_update_available(s.update_available)),
    );
    let mut known = linked.list(host_id)?;

    let mut projects: BTreeMap<&str, Vec<&Container>> = BTreeMap::new();
    for c in containers {
        if let Some(project) = c.stack.as_deref() {
            projects.entry(project).or_default().push(c);
        }
    }
    for (name, members) in projects {
        if records.iter().any(|r| r.name == name) || known.iter().any(|l| l.name == name) {
            continue;
        }
        let Some(origin) = detect::origin(&members) else {
            continue;
        };
        if !detect::matches(cache, &origin, &members) {
            continue;
        }
        let fingerprint = files::files_fingerprint(&origin.config_files);
        match linked.create(
            host_id,
            name,
            &origin.working_dir,
            &origin.config_files,
            &fingerprint,
        ) {
            Ok(stack) => known.push(stack),
            // Listed as External this time; the next refresh tries again.
            Err(e) => log::warn!("couldn't record linked stack {name}: {e}"),
        }
    }

    records.extend(known.into_iter().map(|l| {
        let missing = !detect::files_present(cache, &l.config_files);
        StackRecord::linked(l.id, l.name, l.working_dir).with_files_missing(missing)
    }));
    Ok(derive(containers, &records))
}

#[cfg(test)]
mod tests;
