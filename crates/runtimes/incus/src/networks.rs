//! Incus's networks: their addresses, what uses them and, for managed
//! ones, their DHCP leases; managed bridges can be created and deleted.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use homeport_core::error::{AppError, AppResult, ErrorCode};
use homeport_transport::http::Request;

use crate::{
    api::client::{scoped, segment, Incus},
    instances::usage::{self, IncusUser},
    messages as msg,
};

/// One network.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct IncusNetwork {
    pub name: String,
    /// `bridge`, `macvlan`, `ovn`, `physical`, `loopback`…
    #[serde(rename = "type")]
    pub kind: String,
    /// Whether Incus made and runs it, or it's the machine's own.
    pub managed: bool,
    pub description: String,
    pub status: String,
    /// Its configured IPv4 and IPv6 addresses: "10.42.71.1/24".
    pub ipv4: Option<String>,
    pub ipv6: Option<String>,
    /// Whether it masquerades outgoing IPv4 traffic.
    pub nat: bool,
    pub config: BTreeMap<String, String>,
    pub used_by: Vec<IncusUser>,
}

/// A network's live state and DHCP leases.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct IncusNetworkState {
    /// `up` or `down`.
    pub state: String,
    pub hwaddr: String,
    pub mtu: i64,
    /// Its addresses on the host, with their prefix: "10.42.71.1/24".
    pub addresses: Vec<String>,
    pub leases: Vec<IncusLease>,
}

/// An address the network's DHCP server handed out, or holds for its gateway.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct IncusLease {
    pub hostname: String,
    pub hwaddr: String,
    pub address: String,
    /// `dynamic`, `static` or `gateway`.
    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Deserialize)]
struct RawNetwork {
    name: String,
    #[serde(rename = "type", default)]
    kind: String,
    #[serde(default)]
    managed: bool,
    #[serde(default)]
    description: String,
    #[serde(default)]
    status: String,
    #[serde(default)]
    config: BTreeMap<String, String>,
    #[serde(default)]
    used_by: Option<Vec<String>>,
}

#[derive(Deserialize, Default)]
struct RawState {
    #[serde(default)]
    state: String,
    #[serde(default)]
    hwaddr: String,
    #[serde(default)]
    mtu: i64,
    #[serde(default)]
    addresses: Vec<RawAddress>,
}

#[derive(Deserialize)]
struct RawAddress {
    #[serde(default)]
    address: String,
    #[serde(default)]
    netmask: String,
}

#[derive(Deserialize)]
struct RawLease {
    #[serde(default)]
    hostname: String,
    #[serde(default)]
    hwaddr: String,
    #[serde(default)]
    address: String,
    #[serde(rename = "type", default)]
    kind: String,
}

/// An address setting that means "none".
fn configured(value: Option<&String>) -> Option<String> {
    value.filter(|v| !v.is_empty() && *v != "none").cloned()
}

impl RawNetwork {
    fn into_network(self) -> IncusNetwork {
        IncusNetwork {
            ipv4: configured(self.config.get("ipv4.address")),
            ipv6: configured(self.config.get("ipv6.address")),
            nat: self.config.get("ipv4.nat").is_some_and(|v| v == "true"),
            used_by: usage::users(&self.used_by.unwrap_or_default()),
            name: self.name,
            kind: self.kind,
            managed: self.managed,
            description: self.description,
            status: self.status,
            config: self.config,
        }
    }
}

fn path(name: &str) -> String {
    format!("/1.0/networks/{}", segment(name))
}

/// The networks `project` sees, managed ones first.
pub async fn list(incus: &Incus, project: &str) -> AppResult<Vec<IncusNetwork>> {
    let raw: Vec<RawNetwork> = incus
        .sync(Request::get(scoped("/1.0/networks?recursion=1", project)))
        .await?;
    let mut networks: Vec<IncusNetwork> = raw.into_iter().map(RawNetwork::into_network).collect();
    networks.sort_by(|a, b| (!a.managed, &a.name).cmp(&(!b.managed, &b.name)));
    Ok(networks)
}

/// The network `name`'s live state and, for a managed one, its leases.
pub async fn state(incus: &Incus, project: &str, name: &str) -> AppResult<IncusNetworkState> {
    let raw: RawState = incus
        .sync(Request::get(scoped(
            &format!("{}/state", path(name)),
            project,
        )))
        .await
        .map_err(|e| not_found(e, name))?;
    let leases: Vec<RawLease> = if incus.server().has("network_leases") {
        incus
            .sync(Request::get(scoped(
                &format!("{}/leases", path(name)),
                project,
            )))
            .await
            .unwrap_or_default()
    } else {
        Vec::new()
    };
    Ok(IncusNetworkState {
        state: raw.state,
        hwaddr: raw.hwaddr,
        mtu: raw.mtu,
        addresses: raw
            .addresses
            .into_iter()
            .map(|a| format!("{}/{}", a.address, a.netmask))
            .collect(),
        leases: leases
            .into_iter()
            .map(|l| IncusLease {
                hostname: l.hostname,
                hwaddr: l.hwaddr,
                address: l.address,
                kind: l.kind,
            })
            .collect(),
    })
}

/// What the New bridge dialog asks for.
#[derive(Debug, Clone, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct BridgeRequest {
    pub name: String,
    /// `auto`, `none`, or an address with its prefix: "10.0.5.1/24".
    pub ipv4: String,
    pub ipv6: String,
    pub nat: bool,
    pub description: String,
}

/// The body of `POST /1.0/networks` for `request`.
pub(crate) fn bridge_body(request: &BridgeRequest) -> serde_json::Value {
    let or_auto = |v: &str| {
        let v = v.trim();
        if v.is_empty() {
            "auto".to_owned()
        } else {
            v.to_owned()
        }
    };
    let ipv4 = or_auto(&request.ipv4);
    let mut config = serde_json::json!({
        "ipv4.address": ipv4,
        "ipv6.address": or_auto(&request.ipv6),
    });
    if ipv4 != "none" {
        config["ipv4.nat"] = request.nat.to_string().into();
    }
    serde_json::json!({
        "name": request.name.trim(),
        "type": "bridge",
        "description": request.description.trim(),
        "config": config,
    })
}

/// Makes a managed bridge.
pub async fn create_bridge(incus: &Incus, project: &str, request: &BridgeRequest) -> AppResult<()> {
    let name = request.name.trim();
    // Linux interface names: at most 15 bytes, no slashes or spaces.
    if name.is_empty() || name.len() > 15 || name.contains(['/', ' ', ':']) {
        return Err(AppError::validation(
            msg::NETWORK_NAME_INVALID.with("name", name),
        ));
    }
    let request_body =
        Request::post(scoped("/1.0/networks", project)).json(&bridge_body(request))?;
    incus
        .sync::<serde_json::Value>(request_body)
        .await
        .map(drop)
}

/// Deletes the managed network `name`; one in use is refused, naming its users.
pub async fn delete(incus: &Incus, project: &str, name: &str) -> AppResult<()> {
    let raw: RawNetwork = incus
        .sync(Request::get(scoped(&path(name), project)))
        .await
        .map_err(|e| not_found(e, name))?;
    if !raw.managed {
        return Err(AppError::new(
            ErrorCode::Conflict,
            msg::NETWORK_UNMANAGED.with("name", name),
        ));
    }
    let users = usage::users(&raw.used_by.unwrap_or_default());
    if !users.is_empty() {
        return Err(AppError::new(
            ErrorCode::InUse,
            msg::IN_USE
                .with("name", name)
                .with("users", usage::names(&users)),
        )
        .with_hint(msg::NETWORK_IN_USE_HINT));
    }
    incus
        .sync::<serde_json::Value>(Request::delete(scoped(&path(name), project)))
        .await
        .map(drop)
}

fn not_found(error: AppError, name: &str) -> AppError {
    if error.is(ErrorCode::NotFound) {
        AppError::new(
            ErrorCode::NotFound,
            msg::NETWORK_NOT_FOUND.with("name", name),
        )
        .with_detail(error.detail.unwrap_or(error.message))
    } else {
        error
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn networks_read_their_addresses_and_users() {
        // From GET /1.0/networks?recursion=1 on Incus 6.0.
        let raw: RawNetwork = serde_json::from_value(serde_json::json!({
            "config": { "ipv4.address": "10.42.71.1/24", "ipv4.nat": "true", "ipv6.address": "none" },
            "description": "",
            "name": "incusbr0",
            "type": "bridge",
            "used_by": ["/1.0/profiles/default", "/1.0/instances/t1"],
            "managed": true,
            "status": "Created"
        }))
        .unwrap();
        let network = raw.into_network();
        assert_eq!(network.ipv4.as_deref(), Some("10.42.71.1/24"));
        assert_eq!(network.ipv6, None);
        assert!(network.nat && network.managed);
        assert_eq!(usage::names(&network.used_by), "default, t1");
    }

    #[test]
    fn a_new_bridge_defaults_to_automatic_addresses() {
        let request = BridgeRequest {
            name: "lab0".into(),
            ipv4: String::new(),
            ipv6: "none".into(),
            nat: true,
            description: String::new(),
        };
        assert_eq!(
            bridge_body(&request)["config"],
            serde_json::json!({ "ipv4.address": "auto", "ipv6.address": "none", "ipv4.nat": "true" })
        );
        let no_v4 = BridgeRequest {
            ipv4: "none".into(),
            ..request
        };
        assert!(bridge_body(&no_v4)["config"].get("ipv4.nat").is_none());
    }

    #[tokio::test]
    #[ignore = "needs a local Incus"]
    async fn incus_live_bridges_are_created_and_deleted() {
        let incus = crate::instances::tests::live().await;
        let name = format!("hp{}", std::process::id() % 100_000);
        let request = BridgeRequest {
            name: name.clone(),
            ipv4: "10.250.13.1/24".into(),
            ipv6: "none".into(),
            nat: true,
            description: "made by a test".into(),
        };
        create_bridge(&incus, "default", &request).await.unwrap();
        let listed = list(&incus, "default").await.unwrap();
        let made = listed.iter().find(|n| n.name == name).unwrap();
        assert_eq!(made.ipv4.as_deref(), Some("10.250.13.1/24"));
        let live = state(&incus, "default", &name).await.unwrap();
        assert!(
            live.addresses.iter().any(|a| a == "10.250.13.1/24"),
            "{live:?}"
        );
        assert!(live.leases.iter().any(|l| l.kind == "gateway"), "{live:?}");

        // incusbr0 is used by the default profile.
        let used = list(&incus, "default")
            .await
            .unwrap()
            .into_iter()
            .find(|n| n.managed && !n.used_by.is_empty());
        if let Some(used) = used {
            let refused = delete(&incus, "default", &used.name).await.unwrap_err();
            assert_eq!(refused.code, ErrorCode::InUse, "{refused:?}");
        }
        delete(&incus, "default", &name).await.unwrap();
    }
}
