use std::path::Path;

/// Written by `bun run licenses` (the `beforeBuildCommand` of `tauri build`)
/// and bundled from `bundle > resources`.
const THIRD_PARTY_LICENSES: &str = "licenses/THIRD_PARTY_LICENSES.txt";

/// The Windows application manifest: Common Controls v6, which the dialog
/// plugin's TaskDialogIndirect needs.
const WINDOWS_MANIFEST: &str = "windows-app-manifest.xml";

fn main() {
    // An empty resource directory is fine for tauri-build (which also watches
    // it for changes), a missing one is not.
    std::fs::create_dir_all("licenses").expect("failed to create src-tauri/licenses");
    if std::env::var("PROFILE").as_deref() == Ok("release")
        && !Path::new(THIRD_PARTY_LICENSES).exists()
    {
        panic!("{THIRD_PARTY_LICENSES} is missing: run `bun run licenses` before a release build");
    }

    let mut attributes = tauri_build::Attributes::new();
    if std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc") {
        // tauri-build puts the manifest in the app's resources only, so test
        // binaries start without Common Controls v6 and Windows refuses to
        // load them (STATUS_ENTRYPOINT_NOT_FOUND). Have the linker embed it
        // in everything it links instead, as Tauri's testing guide suggests.
        attributes = attributes
            .windows_attributes(tauri_build::WindowsAttributes::new_without_app_manifest());
        let manifest = std::env::current_dir()
            .expect("the build script runs in src-tauri")
            .join(WINDOWS_MANIFEST);
        println!("cargo:rerun-if-changed={WINDOWS_MANIFEST}");
        println!("cargo:rustc-link-arg=/MANIFEST:EMBED");
        println!("cargo:rustc-link-arg=/MANIFESTINPUT:{}", manifest.display());
        // Turn linker warnings (a manifest that didn't embed) into errors.
        println!("cargo:rustc-link-arg=/WX");
    }
    tauri_build::try_build(attributes).expect("failed to run tauri-build");
}
