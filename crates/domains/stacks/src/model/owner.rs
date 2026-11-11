//! Where a Managed, Linked or Synced Stack is deployed: a Host's Compose
//! projects, or a Swarm's swarm stacks (ADR 0017).

use serde::{Deserialize, Serialize};

/// A Stack's Host or Swarm, by ID.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, specta::Type)]
#[serde(tag = "kind", content = "id", rename_all = "camelCase")]
pub enum StackOwner {
    Host(String),
    Swarm(String),
}

impl StackOwner {
    pub fn id(&self) -> &str {
        match self {
            Self::Host(id) | Self::Swarm(id) => id,
        }
    }

    pub fn host_id(&self) -> Option<&str> {
        match self {
            Self::Host(id) => Some(id),
            Self::Swarm(_) => None,
        }
    }

    pub fn swarm_id(&self) -> Option<&str> {
        match self {
            Self::Swarm(id) => Some(id),
            Self::Host(_) => None,
        }
    }

    pub fn is_swarm(&self) -> bool {
        matches!(self, Self::Swarm(_))
    }

    /// The column a table keys this kind of owner by.
    pub(crate) fn column(&self) -> &'static str {
        match self {
            Self::Host(_) => "host_id",
            Self::Swarm(_) => "swarm_id",
        }
    }

    /// The owner of a row from its `host_id` and `swarm_id` columns, exactly
    /// one of which is set (the tables check it).
    pub(crate) fn from_columns(
        host_id: Option<String>,
        swarm_id: Option<String>,
    ) -> rusqlite::Result<Self> {
        match (host_id, swarm_id) {
            (Some(host), None) => Ok(Self::Host(host)),
            (None, Some(swarm)) => Ok(Self::Swarm(swarm)),
            _ => Err(rusqlite::Error::InvalidColumnType(
                1,
                "host_id".into(),
                rusqlite::types::Type::Null,
            )),
        }
    }
}

impl From<&str> for StackOwner {
    fn from(host_id: &str) -> Self {
        Self::Host(host_id.to_owned())
    }
}

impl From<&String> for StackOwner {
    fn from(host_id: &String) -> Self {
        Self::Host(host_id.clone())
    }
}

impl From<&StackOwner> for StackOwner {
    fn from(owner: &StackOwner) -> Self {
        owner.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn owners_serialize_with_their_kind() {
        assert_eq!(
            serde_json::to_value(StackOwner::Swarm("s1".into())).unwrap(),
            serde_json::json!({ "kind": "swarm", "id": "s1" })
        );
        assert_eq!(StackOwner::from("h1").host_id(), Some("h1"));
        assert!(StackOwner::from_columns(None, None).is_err());
        assert!(StackOwner::from_columns(Some("h".into()), Some("s".into())).is_err());
    }
}
