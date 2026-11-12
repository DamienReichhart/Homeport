//! What a page is given: the app's state, the data layer, what is selected
//! and where the window is, and a way to ask the shell for what a page
//! can't do itself (go somewhere, watch a resource, say that something
//! failed).

use std::{cell::RefCell, sync::Arc};

use iced::Task;

use homeport_clusters::service::ClusterView;
use homeport_hosts::service::HostView;

use super::World;
use crate::{
    app::AppState,
    data::{self, Data, Key, Query},
    i18n::Direction,
    ui::{
        shell::{
            gates::Subject,
            pickers,
            registry::{self, Feature},
            router::{Location, Request},
            selection::Selection,
        },
        theme::Element,
        widgets::feedback::{
            toast::{self, Said},
            Surfaced,
        },
    },
};

/// Starts a read of the data layer.
pub(in crate::ui::shell) type Start =
    Box<dyn FnOnce(&mut Data, &Arc<AppState>) -> Task<data::Message>>;

/// Something a page asked of the shell during its `update`.
pub(in crate::ui::shell) enum Effect {
    /// Go to a page.
    Open(Request),
    /// Go back to where the window was.
    Back,
    /// Show another Host or Cluster.
    Select(Selection),
    /// Read a resource, and keep it fresh while the page shows.
    Watch(Key, Start),
    /// Everything under a key is out of date.
    Invalidate(Key),
    /// A failure, said in passing, under what failed when it has a name.
    Failed(Option<Said>, Surfaced),
    /// What an action did, said in passing.
    Said(toast::Kind, Said),
    /// A link for the system browser.
    OpenUrl(String),
    /// A value for the data layer under this key, in place of a read.
    Put(Key, Box<dyn FnOnce(&mut Data)>),
    /// What the location carries from now on.
    Keep {
        query: Option<String>,
        params: Vec<(String, String)>,
    },
    /// The Details of a failure, asked for.
    Details(Surfaced),
    /// Text for the clipboard.
    Copy(String),
    /// What was done to the namespace picker a page shows.
    Picker(pickers::Message),
}

/// What a page asked for, until the shell does it.
#[derive(Default)]
pub struct Effects(RefCell<Vec<Effect>>);

impl Effects {
    fn push(&self, effect: Effect) {
        self.0.borrow_mut().push(effect);
    }

    pub(in crate::ui::shell) fn is_empty(&self) -> bool {
        self.0.borrow().is_empty()
    }

    pub(in crate::ui::shell) fn take(&self) -> Vec<Effect> {
        std::mem::take(&mut self.0.borrow_mut())
    }
}

/// A page's view of the app. Everything is borrowed from the shell for
/// the length of one `update` or one `view`.
#[derive(Clone, Copy)]
pub struct Context<'a> {
    /// The handle the use cases take.
    pub state: &'a Arc<AppState>,
    /// The data layer, to read what the page watches.
    pub data: &'a Data,
    /// The Hosts, the Clusters, what is selected and what is remembered.
    pub world: &'a World,
    /// The navigation that led here: the route, the action to start and
    /// the search to start with.
    pub location: &'a Location,
    /// What the page is about: what its gate let through.
    pub subject: &'a Subject,
    /// The direction the language reads in.
    pub direction: Direction,
    pub(in crate::ui::shell) pickers: &'a pickers::State,
    pub(in crate::ui::shell) effects: &'a Effects,
}

impl<'a> Context<'a> {
    /// The feature whose page this is, as the registry has it.
    pub fn feature(&self) -> &'static Feature {
        registry::feature(self.location.route.feature())
    }

    /// The selected Host, `None` when a Cluster or nothing is selected.
    pub fn host(&self) -> Option<&'a HostView> {
        self.world.host()
    }

    /// The selected Cluster, `None` when a Host or nothing is selected.
    pub fn cluster(&self) -> Option<&'a ClusterView> {
        self.world.cluster()
    }

    /// Opens a page (`navigate(path)`, `ctx.open(path, action)`).
    pub fn open(&self, request: impl Into<Request>) {
        self.effects.push(Effect::Open(request.into()));
    }

    /// Goes back to where the window was (`navigate(-1)`).
    pub fn back(&self) {
        self.effects.push(Effect::Back);
    }

    /// Shows another Host or Cluster.
    pub fn select(&self, selection: Selection) {
        self.effects.push(Effect::Select(selection));
    }

    /// Starts showing `query`'s resource: it is read unless a recent read
    /// is cached, kept fresh, and let go of when the page closes. Read it
    /// in `view` with `ctx.data.read(&key)`.
    pub fn watch<T: Send + Sync + 'static>(&self, query: Query<T>) {
        let key = query.key().clone();
        self.effects.push(Effect::Watch(
            key,
            Box::new(move |data, state| data.watch(state, &query)),
        ));
    }

    /// Everything under `scope` is read again: what an action changed.
    pub fn invalidate(&self, scope: Key) {
        self.effects.push(Effect::Invalidate(scope));
    }

    /// Says a failure in a toast, which leads to its Details.
    pub fn failed(&self, surfaced: Surfaced) {
        self.effects.push(Effect::Failed(None, surfaced));
    }

    /// The same under a title that names what failed, when the message
    /// alone wouldn't say: one container among those an action was asked
    /// for (`toastError(error, title)`).
    pub fn failed_of(
        &self,
        title: impl Fn() -> String + Send + Sync + 'static,
        surfaced: Surfaced,
    ) {
        self.effects
            .push(Effect::Failed(Some(Said::new(title)), surfaced));
    }

    /// Says what an action did in a toast (`toast.success`). `say` is asked
    /// each time the toast is drawn, so it reads in the language of the
    /// moment: `ctx.done(move || t!("containers.done.stop", count = count))`.
    pub fn done(&self, say: impl Fn() -> String + Send + Sync + 'static) {
        self.effects
            .push(Effect::Said(toast::Kind::Success, Said::new(say)));
    }

    /// Opens `url` in the system browser.
    pub fn open_url(&self, url: impl Into<String>) {
        self.effects.push(Effect::OpenUrl(url.into()));
    }

    /// Files `value` under `key` as if it had just been read: what an
    /// action answered is what a read would (`setQueryData`).
    pub fn put<T: Send + Sync + 'static>(&self, key: Key, value: T) {
        let filed = key.clone();
        self.effects.push(Effect::Put(
            filed,
            Box::new(move |data| data.set(&key, value)),
        ));
    }

    /// Keeps `query` and `params` in the page's location, so the page finds
    /// them when the window comes back to it: a list's filters
    /// (`setSearchParams(.., { replace: true })`). The page stays as it is.
    pub fn keep(&self, query: Option<String>, params: Vec<(String, String)>) {
        self.effects.push(Effect::Keep { query, params });
    }

    /// Opens the Details dialog of a failure shown in place.
    pub fn details(&self, surfaced: Surfaced) {
        self.effects.push(Effect::Details(surfaced));
    }

    /// Puts `text` on the clipboard.
    pub fn copy(&self, text: impl Into<String>) {
        self.effects.push(Effect::Copy(text.into()));
    }

    /// The picker of the Namespace a Cluster page shows, for its header:
    /// Kubernetes' or Nomad's by the selected Cluster's kind, `None` when
    /// there is nothing to pick. The page wraps its messages with
    /// `on_event` and hands them back with [`Context::picked`].
    pub fn namespace_picker<Message: Clone + 'a>(
        &self,
        on_event: impl Fn(pickers::Message) -> Message + 'a,
    ) -> Option<Element<'a, Message>> {
        pickers::view::namespace(self.world, self.pickers).map(|picker| picker.map(on_event))
    }

    /// Hands the shell what was done to the namespace picker.
    pub fn picked(&self, message: pickers::Message) {
        self.effects.push(Effect::Picker(message));
    }
}
