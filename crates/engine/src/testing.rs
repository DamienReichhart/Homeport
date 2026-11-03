//! Helpers for integration tests against the local Engine. Tests only touch
//! resources they create, named `homeport-test-*`, and remove them after.

use bollard::{
    models::ContainerCreateBody,
    query_parameters::{
        CreateContainerOptionsBuilder, CreateImageOptionsBuilder, RemoveContainerOptionsBuilder,
    },
    Docker,
};
use futures_util::TryStreamExt;

pub const IMAGE: &str = "alpine:3.22";

pub fn docker() -> Docker {
    Docker::connect_with_local_defaults().expect("local Engine socket")
}

pub fn unique(prefix: &str) -> String {
    format!(
        "homeport-test-{prefix}-{}",
        &uuid::Uuid::new_v4().simple().to_string()[..8]
    )
}

pub async fn ensure_image(docker: &Docker) {
    if docker.inspect_image(IMAGE).await.is_ok() {
        return;
    }
    docker
        .create_image(
            Some(
                CreateImageOptionsBuilder::default()
                    .from_image(IMAGE)
                    .build(),
            ),
            None,
            None,
        )
        .try_collect::<Vec<_>>()
        .await
        .expect("pull test image");
}

/// Creates (not starts) a container running `cmd`, removed when the guard drops.
pub async fn container(docker: &Docker, name: &str, cmd: &[&str]) -> Guard {
    ensure_image(docker).await;
    let created = docker
        .create_container(
            Some(CreateContainerOptionsBuilder::default().name(name).build()),
            ContainerCreateBody {
                image: Some(IMAGE.into()),
                cmd: Some(cmd.iter().map(|s| s.to_string()).collect()),
                labels: Some([("homeport.test".to_string(), "1".to_string())].into()),
                ..Default::default()
            },
        )
        .await
        .expect("create test container");
    Guard {
        docker: docker.clone(),
        id: created.id,
    }
}

pub struct Guard {
    docker: Docker,
    pub id: String,
}

impl Drop for Guard {
    fn drop(&mut self) {
        let docker = self.docker.clone();
        let id = self.id.clone();
        // Best effort: the test runtime may already be shutting down.
        std::thread::spawn(move || {
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(async move {
                    let _ = docker
                        .remove_container(
                            &id,
                            Some(
                                RemoveContainerOptionsBuilder::default()
                                    .force(true)
                                    .v(true)
                                    .build(),
                            ),
                        )
                        .await;
                })
        })
        .join()
        .ok();
    }
}
