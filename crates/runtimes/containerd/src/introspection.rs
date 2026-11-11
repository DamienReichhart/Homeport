//! What containerd says about itself: its platform and its snapshotters.

use serde::Serialize;

use homeport_core::error::AppResult;

use crate::{
    grpc::{
        api::containerd::services::introspection::v1::{Plugin, PluginsRequest},
        error,
    },
    Containerd,
};

const SNAPSHOTTER: &str = "io.containerd.snapshotter.v1";

/// The snapshotter containerd and nerdctl use when none is named.
const DEFAULT_SNAPSHOTTER: &str = "overlayfs";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ServerInfo {
    /// `linux`, `windows`…
    pub os: String,
    /// `amd64`, `arm64`…
    pub arch: String,
    /// Its CPU variant when it has one, `v8` for arm64.
    pub variant: String,
    /// The snapshotters that loaded, the default first.
    pub snapshotters: Vec<String>,
}

impl ServerInfo {
    /// The snapshotter images are unpacked into and containers use by default.
    pub fn default_snapshotter(&self) -> &str {
        self.snapshotters
            .first()
            .map_or(DEFAULT_SNAPSHOTTER, String::as_str)
    }

    /// `os/arch[/variant]`, as a Platform's name.
    pub fn platform(&self) -> String {
        match self.variant.as_str() {
            "" => format!("{}/{}", self.os, self.arch),
            variant => format!("{}/{}/{variant}", self.os, self.arch),
        }
    }
}

/// Reads containerd's plugins.
pub async fn describe(client: &Containerd) -> AppResult<ServerInfo> {
    let plugins = client
        .introspection()
        .plugins(PluginsRequest::default())
        .await
        .map_err(|s| error::status(&s))?
        .into_inner()
        .plugins;
    Ok(info(&plugins))
}

fn info(plugins: &[Plugin]) -> ServerInfo {
    let platform = plugins
        .iter()
        .filter(|p| p.init_err.is_none())
        .flat_map(|p| p.platforms.first())
        .next();
    let mut snapshotters: Vec<String> = plugins
        .iter()
        .filter(|p| p.r#type == SNAPSHOTTER && p.init_err.is_none())
        .map(|p| p.id.clone())
        .collect();
    let preferred = if platform.is_some_and(|p| p.os == "windows") {
        "windows"
    } else {
        DEFAULT_SNAPSHOTTER
    };
    if let Some(i) = snapshotters.iter().position(|s| s == preferred) {
        let first = snapshotters.remove(i);
        snapshotters.insert(0, first);
    }
    ServerInfo {
        os: platform.map(|p| p.os.clone()).unwrap_or_default(),
        arch: platform.map(|p| p.architecture.clone()).unwrap_or_default(),
        variant: platform.map(|p| p.variant.clone()).unwrap_or_default(),
        snapshotters,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grpc::api::{containerd::types::Platform, google::rpc::Status};

    fn plugin(kind: &str, id: &str, platform: Option<(&str, &str)>, failed: bool) -> Plugin {
        Plugin {
            r#type: kind.into(),
            id: id.into(),
            platforms: platform
                .map(|(os, arch)| Platform {
                    os: os.into(),
                    architecture: arch.into(),
                    ..Default::default()
                })
                .into_iter()
                .collect(),
            init_err: failed.then(|| Status {
                code: 9,
                message: "skip plugin".into(),
                details: vec![],
            }),
            ..Default::default()
        }
    }

    #[test]
    fn the_platform_and_loaded_snapshotters_come_from_the_plugins() {
        let info = info(&[
            plugin(SNAPSHOTTER, "btrfs", Some(("linux", "amd64")), true),
            plugin(SNAPSHOTTER, "native", Some(("linux", "amd64")), false),
            plugin(SNAPSHOTTER, "overlayfs", Some(("linux", "amd64")), false),
            plugin("io.containerd.content.v1", "content", None, false),
        ]);
        assert_eq!(info.platform(), "linux/amd64");
        assert_eq!(info.snapshotters, ["overlayfs", "native"]);
        assert_eq!(info.default_snapshotter(), "overlayfs");
        assert_eq!(
            ServerInfo {
                snapshotters: vec![],
                ..info
            }
            .default_snapshotter(),
            "overlayfs"
        );
    }
}
