use std::path::Path;

/// Written by `bun run licenses` (the `beforeBuildCommand` of `tauri build`)
/// and bundled from `bundle > resources`.
const THIRD_PARTY_LICENSES: &str = "licenses/THIRD_PARTY_LICENSES.txt";

fn main() {
    // An empty resource directory is fine for tauri-build (which also watches
    // it for changes), a missing one is not.
    std::fs::create_dir_all("licenses").expect("failed to create src-tauri/licenses");
    if std::env::var("PROFILE").as_deref() == Ok("release")
        && !Path::new(THIRD_PARTY_LICENSES).exists()
    {
        panic!("{THIRD_PARTY_LICENSES} is missing: run `bun run licenses` before a release build");
    }
    tauri_build::build()
}
