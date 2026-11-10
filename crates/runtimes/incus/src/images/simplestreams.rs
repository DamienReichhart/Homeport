//! The catalogue of a remote image server, read from its simplestreams
//! index from this computer and kept a few minutes (ADR 0021). Incus
//! itself downloads the images.

use std::{
    collections::{BTreeMap, HashMap},
    sync::Mutex,
    time::{Duration, Instant},
};

use serde::{Deserialize, Serialize};

use homeport_core::error::{AppError, AppResult, ErrorCode};

use crate::messages as msg;

/// The public image server Incus's `images:` remote points to.
pub const DEFAULT_SERVER: &str = "https://images.linuxcontainers.org";

/// How long a server's catalogue is kept before it's fetched again.
const KEEP: Duration = Duration::from_secs(10 * 60);

/// One image a remote server publishes for an architecture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct RemoteImage {
    /// Its shortest alias, what `incus launch images:<alias>` takes:
    /// "debian/12".
    pub alias: String,
    /// Every alias, shortest first.
    pub aliases: Vec<String>,
    pub os: String,
    pub release: String,
    /// The release's name when it has one: "bookworm".
    pub release_title: String,
    /// "default", "cloud" (with cloud-init)…
    pub variant: String,
    /// Its architecture as the server names it: "amd64".
    pub arch: String,
    /// Whether it's published as a system container image.
    pub container: bool,
    /// Whether it's published as a virtual machine image.
    pub virtual_machine: bool,
    /// The latest build: "20261108_05:24".
    pub serial: String,
    /// Bytes of the container and virtual machine root filesystems.
    pub container_size: Option<i64>,
    pub virtual_machine_size: Option<i64>,
}

#[derive(Deserialize)]
struct Index {
    #[serde(default)]
    products: BTreeMap<String, Product>,
}

#[derive(Deserialize)]
struct Product {
    #[serde(default)]
    aliases: String,
    #[serde(default)]
    arch: String,
    #[serde(default)]
    os: String,
    #[serde(default)]
    release: String,
    #[serde(default)]
    release_title: String,
    #[serde(default)]
    variant: String,
    #[serde(default)]
    versions: BTreeMap<String, Version>,
}

#[derive(Deserialize)]
struct Version {
    #[serde(default)]
    items: HashMap<String, Item>,
}

#[derive(Deserialize)]
struct Item {
    #[serde(default)]
    ftype: String,
    #[serde(default)]
    size: i64,
}

/// The architecture simplestreams names after Debian's, from the kernel's.
pub fn arch_name(kernel: &str) -> &str {
    match kernel {
        "x86_64" => "amd64",
        "aarch64" => "arm64",
        "armv7l" | "armv8l" => "armhf",
        "i686" => "i386",
        "ppc64le" => "ppc64el",
        other => other,
    }
}

/// Reads an `images.json` index, keeping the images for `arch` (a kernel
/// architecture, as Incus reports it).
fn parse(index: &[u8], arch: &str) -> AppResult<Vec<RemoteImage>> {
    let index: Index = serde_json::from_slice(index).map_err(|e| {
        AppError::new(ErrorCode::RegistryUnreachable, msg::CATALOGUE_UNREADABLE).with_cause(&e)
    })?;
    let wanted = arch_name(arch);
    let mut images: Vec<RemoteImage> = index
        .products
        .into_values()
        .filter(|p| p.arch == wanted)
        .filter_map(|p| {
            let (serial, latest) = p.versions.iter().next_back()?;
            let size_of = |types: &[&str]| {
                latest
                    .items
                    .values()
                    .find(|i| types.contains(&i.ftype.as_str()))
                    .map(|i| i.size)
            };
            let container_size = size_of(&["squashfs", "root.tar.xz"]);
            let virtual_machine_size = size_of(&["disk-kvm.img"]);
            let mut aliases: Vec<String> = p
                .aliases
                .split(',')
                .map(str::trim)
                .filter(|a| !a.is_empty())
                .map(str::to_owned)
                .collect();
            aliases.sort_by(|a, b| a.len().cmp(&b.len()).then_with(|| a.cmp(b)));
            let alias = aliases.first().cloned()?;
            Some(RemoteImage {
                alias,
                aliases,
                os: p.os,
                release: p.release,
                release_title: p.release_title,
                variant: p.variant,
                arch: p.arch,
                container: container_size.is_some(),
                virtual_machine: virtual_machine_size.is_some(),
                serial: serial.clone(),
                container_size,
                virtual_machine_size,
            })
        })
        .collect();
    images.sort_by(|a, b| {
        (a.os.to_lowercase(), &a.release, &a.variant).cmp(&(
            b.os.to_lowercase(),
            &b.release,
            &b.variant,
        ))
    });
    Ok(images)
}

type Cache = HashMap<String, (Instant, Vec<u8>)>;
static CACHE: Mutex<Option<Cache>> = Mutex::new(None);

/// The index of `server`, from the cache while it's fresh.
async fn fetch(server: &str) -> AppResult<Vec<u8>> {
    let server = server.trim_end_matches('/');
    if let Some((at, body)) = CACHE
        .lock()
        .expect("catalogue lock poisoned")
        .get_or_insert_with(HashMap::new)
        .get(server)
    {
        if at.elapsed() < KEEP {
            return Ok(body.clone());
        }
    }
    let unreachable = |e: &dyn std::error::Error| {
        AppError::new(
            ErrorCode::RegistryUnreachable,
            msg::CATALOGUE_UNREACHABLE.with("server", server),
        )
        .with_hint(msg::CATALOGUE_UNREACHABLE_HINT)
        .with_detail(e.to_string())
    };
    if rustls::crypto::CryptoProvider::get_default().is_none() {
        // Another thread may have installed it first; either way one is set.
        let _ = rustls::crypto::ring::default_provider().install_default();
    }
    let client = reqwest::Client::builder()
        .user_agent(concat!("Homeport/", env!("CARGO_PKG_VERSION")))
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| unreachable(&e))?;
    let url = format!("{server}/streams/v1/images.json");
    let body = client
        .get(&url)
        .send()
        .await
        .and_then(reqwest::Response::error_for_status)
        .map_err(|e| unreachable(&e))?
        .bytes()
        .await
        .map_err(|e| unreachable(&e))?
        .to_vec();
    CACHE
        .lock()
        .expect("catalogue lock poisoned")
        .get_or_insert_with(HashMap::new)
        .insert(server.to_owned(), (Instant::now(), body.clone()));
    Ok(body)
}

/// The images `server` publishes for `arch`, the kernel architecture of
/// the Host that will run them.
pub async fn catalogue(server: &str, arch: &str) -> AppResult<Vec<RemoteImage>> {
    parse(&fetch(server).await?, arch)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Four products of images.linuxcontainers.org's index, two builds each.
    const INDEX: &[u8] = include_bytes!("../testdata/simplestreams.json");

    #[test]
    fn images_are_read_for_the_hosts_architecture() {
        let images = parse(INDEX, "x86_64").unwrap();
        assert_eq!(images.len(), 3, "{images:#?}");
        let debian = images.iter().find(|i| i.os == "Debian").unwrap();
        assert_eq!(debian.alias, "debian/12");
        assert_eq!(debian.release_title, "bookworm");
        assert_eq!(debian.arch, "amd64");
        assert!(debian.container && debian.virtual_machine);
        assert!(debian.container_size.is_some_and(|s| s > 0));
        assert_eq!(debian.serial.len(), "20261108_05:24".len());
        let cloud = images.iter().find(|i| i.variant == "cloud").unwrap();
        assert_eq!(cloud.alias, "alpine/3.21/cloud");

        let arm = parse(INDEX, "aarch64").unwrap();
        assert_eq!(arm.len(), 1);
        assert_eq!(arm[0].arch, "arm64");
    }

    #[test]
    fn kernel_architectures_map_to_debians() {
        assert_eq!(arch_name("x86_64"), "amd64");
        assert_eq!(arch_name("aarch64"), "arm64");
        assert_eq!(arch_name("riscv64"), "riscv64");
    }

    #[test]
    fn an_index_that_isnt_json_says_so() {
        let e = parse(b"<html>", "x86_64").unwrap_err();
        assert_eq!(e.code, ErrorCode::RegistryUnreachable);
    }
}
