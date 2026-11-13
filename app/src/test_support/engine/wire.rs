//! What the fake Engine reads and writes on the wire: a query's values,
//! and the tar archives a container's files come and go in.

/// The value of `name` in a query, percent-decoded.
pub(super) fn query_value(query: &str, name: &str) -> Option<String> {
    let value = query
        .split('&')
        .filter_map(|pair| pair.split_once('='))
        .find(|(key, _)| *key == name)?
        .1;
    let mut bytes = vec![];
    let mut rest = value.as_bytes();
    while let Some((first, tail)) = rest.split_first() {
        match (first, tail) {
            (b'%', [high, low, tail @ ..]) => {
                let hex = [*high, *low];
                let hex = std::str::from_utf8(&hex).ok()?;
                bytes.push(u8::from_str_radix(hex, 16).ok()?);
                rest = tail;
            }
            (b'+', _) => {
                bytes.push(b' ');
                rest = tail;
            }
            _ => {
                bytes.push(*first);
                rest = tail;
            }
        }
    }
    String::from_utf8(bytes).ok()
}

/// The names of the entries at the top of a tar archive.
pub(super) fn entry_names(archive: &[u8]) -> Vec<String> {
    let mut archive = tar::Archive::new(archive);
    let Ok(entries) = archive.entries() else {
        return vec![];
    };
    entries
        .filter_map(Result::ok)
        .filter_map(|entry| Some(entry.path().ok()?.display().to_string()))
        .collect()
}

/// What the Engine sends for a path: a tar of the file, or of a folder
/// that holds one file.
pub(super) fn archive_of(name: &str, file: Option<&[u8]>) -> Vec<u8> {
    let mut builder = tar::Builder::new(Vec::new());
    let mut put = |path: String, bytes: &[u8]| {
        let mut header = tar::Header::new_gnu();
        header.set_size(bytes.len() as u64);
        header.set_mode(0o644);
        header.set_cksum();
        let _ = builder.append_data(&mut header, path, bytes);
    };
    match file {
        Some(bytes) => put(name.to_owned(), bytes),
        None => put(format!("{name}/inside.txt"), b"inside\n"),
    }
    builder.into_inner().unwrap_or_default()
}
