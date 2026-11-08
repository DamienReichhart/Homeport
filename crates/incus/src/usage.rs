//! What uses an object, read from the API paths of its `used_by`.

use serde::Serialize;

/// One user of an object: an Instance, a profile, a network…
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct IncusUser {
    /// `instance`, `profile`, `network`, `volume`, `image`, `project`, or
    /// the collection's own name for anything else.
    pub kind: String,
    pub name: String,
    /// The project it's in, when Incus says.
    pub project: Option<String>,
}

fn decode(text: &str) -> String {
    let bytes = text.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' {
            if let Some(byte) = text
                .get(i + 1..i + 3)
                .and_then(|hex| u8::from_str_radix(hex, 16).ok())
            {
                out.push(byte);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// The users `used_by` names, skipping snapshots of what's already listed.
pub(crate) fn users(used_by: &[String]) -> Vec<IncusUser> {
    used_by
        .iter()
        .filter_map(|path| {
            let (path, query) = path.split_once('?').unwrap_or((path, ""));
            let project = query
                .split('&')
                .find_map(|pair| pair.strip_prefix("project="))
                .map(decode);
            let segments: Vec<String> = path
                .trim_start_matches("/1.0/")
                .split('/')
                .filter(|s| !s.is_empty())
                .map(decode)
                .collect();
            let (kind, name) = match segments.as_slice() {
                [collection, name] => (
                    match collection.as_str() {
                        "instances" => "instance",
                        "profiles" => "profile",
                        "networks" => "network",
                        "images" => "image",
                        "projects" => "project",
                        other => other,
                    },
                    name.clone(),
                ),
                // /1.0/storage-pools/<pool>/volumes/<type>/<name>
                [pool_collection, _, volumes, _, name]
                    if pool_collection == "storage-pools" && volumes == "volumes" =>
                {
                    ("volume", name.clone())
                }
                _ => return None,
            };
            Some(IncusUser {
                kind: kind.to_owned(),
                name,
                project,
            })
        })
        .collect()
}

/// The users' names for a message: "web, db".
pub(crate) fn names(users: &[IncusUser]) -> String {
    users
        .iter()
        .map(|u| u.name.as_str())
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn used_by_paths_name_their_users() {
        let used_by: Vec<String> = [
            "/1.0/profiles/default",
            "/1.0/instances/t1?project=lab",
            "/1.0/instances/t1/snapshots/snap0",
            "/1.0/storage-pools/default/volumes/custom/my%20data",
        ]
        .map(String::from)
        .to_vec();
        let found = users(&used_by);
        assert_eq!(found.len(), 3, "{found:?}");
        assert_eq!(found[0].kind, "profile");
        assert_eq!(found[1].project.as_deref(), Some("lab"));
        assert_eq!(found[2].kind, "volume");
        assert_eq!(found[2].name, "my data");
        assert_eq!(names(&found), "default, t1, my data");
    }
}
