//! What a folder's listing is made of, and the path over it
//! (`fileBrowser.model.ts`).

/// What an entry of a folder is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kind {
    Directory,
    File,
    Symlink,
    Other,
}

/// One entry of a folder: `FileEntry`, without the crate that lists it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub name: String,
    pub path: String,
    pub kind: Kind,
    pub size: u64,
    /// Unix time in seconds.
    pub modified: i64,
    /// `ls`-style permissions, e.g. `-rw-r--r--`.
    pub mode: String,
}

/// `/a/b/c` → `[("/", "/"), ("a", "/a"), ("b", "/a/b"), ("c", "/a/b/c")]`:
/// each folder on the way, and the path that leads to it.
pub fn crumbs(path: &str) -> Vec<(String, String)> {
    let parts: Vec<&str> = path.split('/').filter(|part| !part.is_empty()).collect();
    let mut crumbs = vec![("/".to_owned(), "/".to_owned())];
    crumbs.extend(parts.iter().enumerate().map(|(index, part)| {
        (
            (*part).to_owned(),
            format!("/{}", parts[..=index].join("/")),
        )
    }));
    crumbs
}
