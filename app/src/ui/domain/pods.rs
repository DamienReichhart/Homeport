//! The Podman pod a container is in (`core/host/api/pods.ts`), so a list
//! of containers can say which pod each belongs to.

use std::collections::HashMap;

use homeport_engine::podman::pods::Pod;

/// The pod a container belongs to, by name and id.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PodRef<'a> {
    pub id: &'a str,
    pub name: &'a str,
}

/// Each container of `pods`, by id, with the pod it is in.
pub fn by_container(pods: &[Pod]) -> HashMap<&str, PodRef<'_>> {
    let mut index = HashMap::new();
    for pod in pods {
        for container in &pod.containers {
            let pod = PodRef {
                id: &pod.id,
                name: &pod.name,
            };
            index.insert(container.id.as_str(), pod);
        }
    }
    index
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use homeport_engine::podman::pods::PodContainer;

    use super::*;

    fn pod(id: &str, containers: &[&str]) -> Pod {
        Pod {
            id: id.to_owned(),
            name: format!("{id}-name"),
            status: "running".to_owned(),
            created: String::new(),
            infra_id: None,
            labels: BTreeMap::new(),
            networks: vec![],
            containers: containers
                .iter()
                .map(|c| PodContainer {
                    id: (*c).to_owned(),
                    name: (*c).to_owned(),
                    state: "running".to_owned(),
                    infra: false,
                })
                .collect(),
        }
    }

    #[test]
    fn it_finds_the_pod_of_each_container() {
        let pods = [pod("p1", &["a", "b"]), pod("p2", &["c"])];
        let index = by_container(&pods);
        let p1 = PodRef {
            id: "p1",
            name: "p1-name",
        };
        assert_eq!(index.get("b"), Some(&p1));
        assert_eq!(index.get("c").map(|pod| pod.name), Some("p2-name"));
        assert_eq!(index.get("z"), None);
    }
}
