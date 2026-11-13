//! What can be done to a container from a list: start and stop, its
//! menu, and the confirmations and the dialog they open
//! (`ContainerRowMenu`, `useContainerMutations`, `removeConfirmation`,
//! `RenameDialog`). A page keeps one [`Actions`], hands it its
//! [`Message`]s and lays [`Actions::dialog`] over itself.
//!
//! An action asked for several containers asks each one's Host, all at
//! once; every one is tried whatever the others do. What worked is said
//! in one toast with its count; what failed is said in one toast, as the
//! React app's one `toastError` for the action: the first failure's
//! message, whose Details list every container that failed and why.
//!
//! The actions run through `Context::act`: the toast and the lists to
//! read again are the shell's to report, so they are whether or not the
//! page is still there when the Engine answers.
//!
//! Export saves a container's filesystem to a tar ([`export`]). Edit,
//! Duplicate and Commit are in the menu and can't be chosen: their
//! dialogs are the run ticket's (#259).

pub mod export;
pub mod view;

#[cfg(test)]
mod tests;

use std::{
    collections::{BTreeSet, HashSet},
    future::Future,
    sync::Arc,
};

use iced::{futures::future::join_all, Task};

use homeport_core::error::{AppError, AppResult};
use homeport_engine::containers::{Container, ContainerAction};

use crate::{
    api::engine::containers::{containers_act, containers_remove, containers_rename},
    app::AppState,
    data::keys,
    t,
    ui::{
        shell::{
            router::{Request, Route},
            selection::Selection,
            Context, Outcome as Told,
        },
        widgets::feedback::{
            confirm::{self, Ask, Outcome},
            error_message, Surfaced,
        },
    },
};

/// A container an action is asked for, and the Host it runs on.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Target {
    pub host_id: String,
    pub host_name: String,
    pub id: String,
    pub name: String,
    /// `running`, `paused`, `exited`…
    pub state: String,
}

impl Target {
    pub fn of(host_id: &str, host_name: &str, container: &Container) -> Self {
        Self {
            host_id: host_id.to_owned(),
            host_name: host_name.to_owned(),
            id: container.id.clone(),
            name: container.name.clone(),
            state: container.state.clone(),
        }
    }

    fn is_running(&self) -> bool {
        self.state == "running"
    }

    fn is_paused(&self) -> bool {
        self.state == "paused"
    }

    /// What tells it from every other container of every Host.
    fn key(&self) -> (String, String) {
        (self.host_id.clone(), self.id.clone())
    }
}

/// What the bar over a list offers for the selected containers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bulk {
    Start,
    Stop,
    Restart,
    Remove,
}

/// How each container fared.
pub type Results = Vec<(Target, AppResult<()>)>;

#[derive(Debug, Clone)]
pub enum Message {
    /// Start, stop, restart, pause or resume these containers.
    Act(ContainerAction, Vec<Target>),
    Acted(ContainerAction, Results),
    AskKill(Target),
    /// Remove these, once confirmed. `by_host` names them Host by Host:
    /// a selection that may span Hosts.
    AskRemove {
        targets: Vec<Target>,
        by_host: bool,
    },
    Confirm(confirm::Message),
    Removed(Results),
    AskRename(Target),
    RenameInput(String),
    RenameSubmit,
    /// The Engine answered the rename of this container (by its id).
    Renamed(String, AppResult<()>),
    RenameFailure(error_message::Event),
    /// Escape, Cancel or the scrim of the Rename dialog.
    CloseDialog,
    /// Export this container's filesystem: the save dialog first.
    AskExport(Target),
    /// The save dialog of an export, or the dialog that shows it.
    Export(export::Message),
    /// Go to a tab of the container's page: its logs, its terminal.
    Open(Target, &'static str),
}

/// What a confirmation is about.
#[derive(Debug, Clone, PartialEq, Eq)]
enum What {
    Kill(Target),
    Remove { targets: Vec<Target>, by_host: bool },
}

#[derive(Debug)]
enum Dialog {
    Confirm {
        what: What,
        state: confirm::State,
    },
    Rename {
        target: Target,
        name: String,
        /// The Engine was asked and has not answered.
        pending: bool,
        /// Why the last try failed, shown in place.
        failure: Option<Box<Surfaced>>,
    },
}

/// The state of a list's container actions.
#[derive(Debug, Default)]
pub struct Actions {
    /// The list spans Hosts: a failure's toast names the Host, and opening
    /// a container selects its Host first.
    across_hosts: bool,
    /// The containers an action is on its way for: their button waits.
    pending: HashSet<(String, String)>,
    dialog: Option<Dialog>,
    /// The export the dialog shows, and its save dialog.
    exports: export::Exports,
}

/// The failure of an action asked for several containers, as one: the
/// first one's code, message and hint, and in its detail every container
/// that failed with its Host and why, for the Details dialog and the log.
fn failure(results: &Results, command: &'static str) -> Option<Surfaced> {
    let failed: Vec<(&Target, &AppError)> = results
        .iter()
        .filter_map(|(target, result)| Some((target, result.as_ref().err()?)))
        .collect();
    let (_, first) = failed.first()?;
    let lines: Vec<String> = failed
        .iter()
        .map(|(target, error)| {
            let why = error
                .detail
                .as_deref()
                .map_or_else(String::new, |detail| format!(" ({detail})"));
            format!(
                "{}/{}: {}{why}",
                target.host_name, target.name, error.message
            )
        })
        .collect();
    let mut error = (*first).clone();
    error.detail = Some(lines.join("\n"));
    Some(Surfaced::new(error, command))
}

/// What the shell reports of an action asked for several containers:
/// what worked in one toast, what failed in one, and every Host that was
/// asked reads its containers again.
fn told(
    results: &Results,
    command: &'static str,
    done: impl Fn(usize) -> String + Send + Sync + 'static,
) -> Told {
    let worked = results.iter().filter(|(_, result)| result.is_ok()).count();
    let mut told = Told::quiet();
    if worked > 0 {
        told = told.saying(move || done(worked));
    }
    if let Some(failure) = failure(results, command) {
        told = told.failing(failure);
    }
    let hosts: BTreeSet<&str> = results.iter().map(|(t, _)| t.host_id.as_str()).collect();
    hosts.into_iter().fold(told, |told, host| {
        told.invalidating(keys::host::containers(host))
    })
}

/// What the shell reports of a lifecycle action.
pub(crate) fn acted(action: ContainerAction, results: &Results) -> Told {
    told(results, "containers_act", move |count| {
        view::done(action, count)
    })
}

/// What the shell reports of a removal.
pub(crate) fn removed(results: &Results) -> Told {
    told(results, "containers_remove", |count| {
        t!("containers.done.remove", count = count)
    })
}

/// How many of `results` failed.
fn failures(results: &Results) -> usize {
    results.iter().filter(|(_, result)| result.is_err()).count()
}

impl Actions {
    /// For the list of one Host.
    pub fn of_host() -> Self {
        Self::default()
    }

    /// For a list of every Host's containers.
    pub fn across_hosts() -> Self {
        Self {
            across_hosts: true,
            ..Self::default()
        }
    }

    /// Whether an action is on its way for this container.
    pub fn is_pending(&self, host_id: &str, id: &str) -> bool {
        // Two short strings per row in view: nothing next to drawing it.
        self.pending.contains(&(host_id.to_owned(), id.to_owned()))
    }

    /// Whether a confirmation, the Rename dialog or an export is open.
    pub fn has_dialog(&self) -> bool {
        self.dialog.is_some() || self.exports.shown.is_some()
    }

    /// What the confirmation asks, in the language of the moment.
    fn ask(what: &What) -> Ask {
        match what {
            What::Kill(target) => Ask::new(
                t!("containers.kill.title", name = target.name.as_str()),
                t!("containers.kill.description"),
                t!("containers.action.kill"),
            )
            .destructive(),
            What::Remove { targets, by_host } => {
                let (title, description) = if *by_host {
                    view::removal_by_host(targets)
                } else {
                    view::removal(targets)
                };
                Ask::new(title, description, t!("common.remove")).destructive()
            }
        }
    }

    /// Asks each container's Host to `run` on it, all at once, and gives
    /// how each fared.
    async fn each<F, Fut>(state: Arc<AppState>, targets: Vec<Target>, run: F) -> Results
    where
        F: Fn(Arc<AppState>, Target) -> Fut,
        Fut: Future<Output = AppResult<()>>,
    {
        let all = targets.into_iter().map(|target| {
            let result = run(state.clone(), target.clone());
            async move { (target, result.await) }
        });
        join_all(all).await
    }

    fn act(
        &mut self,
        action: ContainerAction,
        targets: Vec<Target>,
        ctx: &Context<'_>,
    ) -> Task<Message> {
        if targets.is_empty() {
            return Task::none();
        }
        self.pending.extend(targets.iter().map(Target::key));
        let work = Self::each(
            ctx.state.clone(),
            targets,
            move |state, target| async move {
                containers_act(&state, target.host_id, target.id, action).await
            },
        );
        ctx.act(
            work,
            move |results| acted(action, results),
            move |results| Message::Acted(action, results),
        )
    }

    pub fn update(&mut self, message: Message, ctx: &Context<'_>) -> Task<Message> {
        match message {
            Message::Act(action, targets) => return self.act(action, targets, ctx),
            Message::Acted(action, results) => {
                for (target, _) in &results {
                    self.pending.remove(&target.key());
                }
                let failed = failures(&results);
                // Kill was confirmed: its dialog waited for this.
                if let Some(Dialog::Confirm {
                    what: What::Kill(_),
                    state,
                }) = &mut self.dialog
                {
                    if action == ContainerAction::Kill && state.is_busy() {
                        if failed == 0 {
                            self.dialog = None;
                        } else {
                            state.failed();
                        }
                    }
                }
            }
            Message::AskKill(target) => {
                self.dialog = Some(Dialog::Confirm {
                    what: What::Kill(target),
                    state: confirm::State::default(),
                });
            }
            Message::AskRemove { targets, by_host } if !targets.is_empty() => {
                self.dialog = Some(Dialog::Confirm {
                    what: What::Remove { targets, by_host },
                    state: confirm::State::default(),
                });
            }
            Message::AskRemove { .. } => {}
            Message::Confirm(message) => return self.confirm(message, ctx),
            Message::Removed(results) => {
                let failed = failures(&results);
                match &mut self.dialog {
                    // Nothing went: the dialog offers it again.
                    Some(Dialog::Confirm { state, .. }) if failed == results.len() => {
                        state.failed()
                    }
                    Some(Dialog::Confirm { .. }) => self.dialog = None,
                    _ => {}
                }
            }
            Message::AskRename(target) => {
                self.dialog = Some(Dialog::Rename {
                    name: target.name.clone(),
                    target,
                    pending: false,
                    failure: None,
                });
            }
            Message::CloseDialog => {
                if matches!(self.dialog, Some(Dialog::Rename { .. })) {
                    self.dialog = None;
                }
            }
            Message::AskExport(target) => {
                return self.exports.ask(target, ctx).map(Message::Export);
            }
            Message::Export(message) => {
                return self.exports.update(message, ctx).map(Message::Export);
            }
            Message::Open(target, tab) => {
                if self.across_hosts {
                    ctx.select(Selection::Host { id: target.host_id });
                }
                let page = Request::to(Route::Container {
                    id: target.id,
                    tab: Some(tab.to_owned()),
                });
                // Opened from a list of every Host's containers, the
                // page's Back returns there (`navigate(.., { state: { from } })`).
                ctx.open(if self.across_hosts {
                    page.with_param(super::CAME_FROM, "board")
                } else {
                    page
                });
            }
            rename => return self.rename(rename, ctx),
        }
        Task::none()
    }

    fn confirm(&mut self, message: confirm::Message, ctx: &Context<'_>) -> Task<Message> {
        let Some(Dialog::Confirm { what, state }) = &mut self.dialog else {
            return Task::none();
        };
        match state.update(message, &Self::ask(what)) {
            Outcome::Pending => Task::none(),
            Outcome::Cancelled => {
                self.dialog = None;
                Task::none()
            }
            Outcome::Confirmed => match what.clone() {
                What::Kill(target) => self.act(ContainerAction::Kill, vec![target], ctx),
                // Forced, so a running one is stopped first, and with the
                // anonymous volumes, as the dialog says.
                What::Remove { targets, .. } => {
                    let remove = |state: Arc<AppState>, target: Target| async move {
                        containers_remove(&state, target.host_id, target.id, true, true).await
                    };
                    let work = Self::each(ctx.state.clone(), targets, remove);
                    ctx.act(work, removed, Message::Removed)
                }
            },
        }
    }

    fn rename(&mut self, message: Message, ctx: &Context<'_>) -> Task<Message> {
        let Some(Dialog::Rename {
            target,
            name,
            pending,
            failure,
        }) = &mut self.dialog
        else {
            return Task::none();
        };
        match message {
            Message::RenameInput(typed) => *name = typed,
            Message::RenameSubmit if !*pending => {
                let (host, id, name) = (target.host_id.clone(), target.id.clone(), name.clone());
                *pending = true;
                *failure = None;
                let (renamed, scope) = (id.clone(), keys::host::containers(&host));
                let state = ctx.state.clone();
                // The list is read again once the Engine took the name,
                // whether or not the dialog is still open by then.
                return ctx.act(
                    async move { containers_rename(&state, host, id, name).await },
                    move |result| match result {
                        Ok(()) => Told::quiet().invalidating(scope),
                        Err(_) => Told::quiet(),
                    },
                    move |result| Message::Renamed(renamed.clone(), result),
                );
            }
            // The answer of a rename whose dialog was closed, and another
            // opened since, is not this dialog's.
            Message::Renamed(id, result) if id == target.id && *pending => {
                *pending = false;
                match result {
                    Ok(()) => self.dialog = None,
                    Err(error) => {
                        *failure = Some(Box::new(Surfaced::new(error, "containers_rename")));
                    }
                }
            }
            Message::RenameFailure(error_message::Event::Details) => {
                if let Some(failure) = failure {
                    ctx.details((**failure).clone());
                }
            }
            Message::RenameFailure(error_message::Event::Copy(command)) => ctx.copy(command),
            _ => {}
        }
        Task::none()
    }
}
