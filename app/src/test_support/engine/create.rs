//! What the fake Engine answers for a container that is made, changed or
//! kept as an image: an image's inspect (the image is there, so nothing
//! is pulled), a container created (its request kept as it came), started,
//! stopped, renamed and removed, what changed in its files, and a commit.
//! A test reads back what each was asked: [`Engine::created`] is the
//! create request's body, as the Engine received it.

use std::sync::atomic::Ordering;

use super::{wire::query_value, Connection, Engine, Request};

/// A container the fake was asked to create.
#[derive(Debug, Clone, PartialEq)]
pub struct Created {
    /// The `name` of the request's query, if it had one.
    pub name: Option<String>,
    /// The request's body: `ContainerCreateBody` as the Engine reads it.
    pub body: serde_json::Value,
}

impl Engine {
    /// The containers it was asked to create, in order.
    pub fn created(&self) -> Vec<Created> {
        self.shared.created.lock().unwrap().clone()
    }

    /// The commits it was asked for: each request's query.
    pub fn commits(&self) -> Vec<String> {
        self.shared.commits.lock().unwrap().clone()
    }

    /// A container's inspect document says `document` from now on (the
    /// sample's running `web` until then).
    pub fn inspects_as(&self, document: serde_json::Value) {
        *self.shared.inspect.lock().unwrap() = Some(document.to_string());
    }

    /// `docker diff` lists this many changed paths from now on.
    pub fn changed_paths(&self, count: usize) {
        self.shared.changes.store(count, Ordering::SeqCst);
    }

    /// Creating a container is refused with `message` from now on, as
    /// the Engine refuses an option it can't apply.
    pub fn refuse_create(&self, message: &str) {
        *self.shared.refused_create.lock().unwrap() = Some(message.to_owned());
    }
}

/// What an image's inspect says, whichever image is asked for.
pub const IMAGE_INSPECTED: &str = r#"{"Id":"sha256:1111","RepoTags":["nginx:latest"],"Config":{}}"#;

impl Connection {
    /// `GET /images/{name}/json`: every image is there.
    pub(super) async fn inspect_image(&mut self) -> bool {
        self.json("200 OK", IMAGE_INSPECTED).await
    }

    /// `POST /containers/create`: kept, and answered with a new id.
    pub(super) async fn create_container(&mut self, request: &Request) -> bool {
        let refused = self.shared.refused_create.lock().unwrap().clone();
        if let Some(message) = refused {
            let body = serde_json::json!({ "message": message }).to_string();
            return self.json("400 Bad Request", &body).await;
        }
        let body = serde_json::from_slice(&request.body).unwrap_or(serde_json::Value::Null);
        let created = super::create::Created {
            name: query_value(&request.query, "name"),
            body,
        };
        let id = {
            let mut list = self.shared.created.lock().unwrap();
            list.push(created);
            format!("new-{}", list.len())
        };
        let answer = format!(r#"{{"Id":"{id}","Warnings":[]}}"#);
        self.json("201 Created", &answer).await
    }

    /// Start, stop, rename, remove: done.
    pub(super) async fn done(&mut self) -> bool {
        self.answer("204 No Content", &[], b"").await
    }

    /// `GET /containers/{id}/changes`: as many files as the test said.
    pub(super) async fn changes(&mut self) -> bool {
        let count = self.shared.changes.load(Ordering::SeqCst);
        let changes: Vec<String> = (0..count)
            .map(|n| format!(r#"{{"Path":"/etc/changed-{n}","Kind":0}}"#))
            .collect();
        self.json("200 OK", &format!("[{}]", changes.join(",")))
            .await
    }

    /// `POST /commit`: kept, and answered with the image's id.
    pub(super) async fn commit(&mut self, request: &Request) -> bool {
        let query = request.query.clone();
        self.shared.commits.lock().unwrap().push(query);
        self.json("201 Created", r#"{"Id":"sha256:2222"}"#).await
    }
}
