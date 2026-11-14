//! What the fields read of the Host and of the registries: its images and
//! networks for the selects and the suggestions, and which login a pull
//! of the image would use (`RegistryLogin`), read again as the image
//! changes. A dialog that closes lets go of them.

use std::sync::Arc;

use homeport_registries::Registry;

use super::Form;
use crate::{
    api::hosts::registries::registries_login_for,
    data::{keys, resources, Query, Resource},
    ui::shell::Context,
};

/// The login a pull of `reference` would use (`registryQueries.loginFor`).
fn login_for(reference: &str) -> Query<Option<Registry>> {
    let reference = reference.to_owned();
    Query::new(keys::stacks::registry_login_for(&reference), move |app| {
        let reference = reference.clone();
        async move { registries_login_for(&app, reference) }
    })
}

impl Form {
    /// Reads what the fields show: the Host's images and networks, and
    /// the login of the image.
    pub fn read(&mut self, ctx: &Context<'_>) {
        ctx.watch(resources::images(&self.host_id));
        ctx.watch(resources::networks(&self.host_id));
        self.read_login(ctx);
    }

    /// Lets go of what [`Form::read`] read: the dialog closed. The pods a
    /// Podman page reads itself are left to it.
    pub fn let_go(&mut self, ctx: &Context<'_>) {
        ctx.unwatch(keys::host::images(&self.host_id));
        ctx.unwatch(keys::host::networks(&self.host_id));
        if let Some(login) = self.login.take() {
            ctx.unwatch(login);
        }
    }

    /// Reads which login a pull of the image would use, once the form
    /// names another image; lets go of the read for the one before.
    pub(super) fn read_login(&mut self, ctx: &Context<'_>) {
        let reference = self.form.image.trim();
        let key = (!reference.is_empty()).then(|| keys::stacks::registry_login_for(reference));
        if key == self.login {
            return;
        }
        if let Some(old) = self.login.take() {
            ctx.unwatch(old);
        }
        if key.is_some() {
            ctx.watch(login_for(reference));
        }
        self.login = key;
    }

    /// The login a pull would use, said for an image the Host doesn't
    /// have, once its images are read.
    pub fn login(&self, ctx: &Context<'_>) -> Option<Resource<Arc<Option<Registry>>>> {
        let image = self.form.image.trim();
        let images = ctx
            .data
            .read::<Vec<homeport_engine::images::Image>>(&keys::host::images(&self.host_id));
        let local = images
            .data()
            .is_none_or(|images| images.iter().any(|i| i.tags.iter().any(|tag| tag == image)));
        let key = self.login.as_ref().filter(|_| !local)?;
        Some(ctx.data.read(key))
    }
}
