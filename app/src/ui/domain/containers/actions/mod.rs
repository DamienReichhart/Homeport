//! What can be done to a container from a list: start and stop, its
//! menu, and the confirmations and the dialog they open
//! (`ContainerRowMenu`, `useContainerMutations`, `removeConfirmation`,
//! `RenameDialog`). A page keeps one [`Actions`], hands it its
//! [`Message`]s and lays [`Actions::dialog`] over itself.
//!
//! An action asked for several containers asks each one's Host, all at
//! once; every one is tried whatever the others do. What worked is said
//! in one toast with its count; what failed is named in a toast per
//! failure, the containers a Host refused for the same reason together.
//!
//! Edit, Duplicate, Commit and Export are in the menu and can't be chosen:
//! their dialogs are the detail and run tickets' (#258, #259).

pub mod view;

#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, HashSet};

use iced::{futures::future::join_all, Task};

use homeport_core::error::{AppError, AppResult};
use homeport_engine::containers::{Container, ContainerAction};

use crate::{
    api::engine::containers::{containers_act, containers_remove, containers_rename},
    data::{action::Finished, keys, Action},
    t,
    ui::{
        shell::{router::Route, selection::Selection, Context},
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
    Renamed(Finished<()>),
    RenameFailure(error_message::Event),
    /// Escape, Cancel or the scrim of the Rename dialog.
    CloseDialog,
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
        action: Action<()>,
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
}

/// The names of `targets`, the first few and how many more.
fn names(targets: &[&Target]) -> String {
    const SHOWN: usize = 3;
    let mut names: Vec<&str> = targets
        .iter()
        .take(SHOWN)
        .map(|t| t.name.as_str())
        .collect();
    let more = targets.len().saturating_sub(SHOWN);
    let more = format!("+{more}");
    if targets.len() > SHOWN {
        names.push(&more);
    }
    names.join(", ")
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

    /// Whether a confirmation or the Rename dialog is open.
    pub fn has_dialog(&self) -> bool {
        self.dialog.is_some()
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
    fn each<F, Fut>(ctx: &Context<'_>, targets: Vec<Target>, run: F) -> Task<Results>
    where
        F: Fn(std::sync::Arc<crate::app::AppState>, Target) -> Fut + Send + 'static,
        Fut: std::future::Future<Output = AppResult<()>> + Send + 'static,
    {
        let state = ctx.state.clone();
        Task::perform(
            async move {
                let all = targets.into_iter().map(|target| {
                    let result = run(state.clone(), target.clone());
                    async move { (target, result.await) }
                });
                join_all(all).await
            },
            |results| results,
        )
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
        Self::each(ctx, targets, move |state, target| async move {
            containers_act(&state, target.host_id, target.id, action).await
        })
        .map(move |results| Message::Acted(action, results))
    }

    /// Says what worked in one toast and what failed in one per failure,
    /// and has every Host that was asked read its containers again.
    /// Returns how many failed.
    fn report(
        &self,
        results: &Results,
        command: &'static str,
        done: impl Fn(usize) -> String + Send + Sync + 'static,
        ctx: &Context<'_>,
    ) -> usize {
        let worked = results.iter().filter(|(_, result)| result.is_ok()).count();
        if worked > 0 {
            ctx.done(move || done(worked));
        }
        // Those a Host refused for the same reason are said together.
        let mut failures: BTreeMap<(String, String), (&AppError, Vec<&Target>)> = BTreeMap::new();
        for (target, result) in results {
            if let Err(error) = result {
                let reason = format!("{:?}{}{:?}", error.code, error.message, error.detail);
                failures
                    .entry((target.host_name.clone(), reason))
                    .or_insert((error, vec![]))
                    .1
                    .push(target);
            }
        }
        for ((host, _), (error, targets)) in failures {
            let names = names(&targets);
            let surfaced = Surfaced::new(error.clone(), command);
            if self.across_hosts {
                ctx.failed_of(
                    move || {
                        t!(
                            "board.remove.onHost",
                            host = host.as_str(),
                            names = names.as_str()
                        )
                    },
                    surfaced,
                );
            } else {
                ctx.failed_of(move || names.clone(), surfaced);
            }
        }
        let hosts: HashSet<&str> = results.iter().map(|(t, _)| t.host_id.as_str()).collect();
        for host in hosts {
            ctx.invalidate(keys::host::containers(host));
        }
        results.len() - worked
    }

    pub fn update(&mut self, message: Message, ctx: &Context<'_>) -> Task<Message> {
        match message {
            Message::Act(action, targets) => return self.act(action, targets, ctx),
            Message::Acted(action, results) => {
                for (target, _) in &results {
                    self.pending.remove(&target.key());
                }
                let failed = self.report(
                    &results,
                    "containers_act",
                    move |count| view::done(action, count),
                    ctx,
                );
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
                let failed = self.report(
                    &results,
                    "containers_remove",
                    |count| t!("containers.done.remove", count = count),
                    ctx,
                );
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
                    action: Action::default(),
                    failure: None,
                });
            }
            Message::CloseDialog => {
                if matches!(self.dialog, Some(Dialog::Rename { .. })) {
                    self.dialog = None;
                }
            }
            Message::Open(target, tab) => {
                if self.across_hosts {
                    ctx.select(Selection::Host { id: target.host_id });
                }
                ctx.open(Route::Container {
                    id: target.id,
                    tab: Some(tab.to_owned()),
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
                    Self::each(ctx, targets, |state, target| async move {
                        containers_remove(&state, target.host_id, target.id, true, true).await
                    })
                    .map(Message::Removed)
                }
            },
        }
    }

    fn rename(&mut self, message: Message, ctx: &Context<'_>) -> Task<Message> {
        let Some(Dialog::Rename {
            target,
            name,
            action,
            failure,
        }) = &mut self.dialog
        else {
            return Task::none();
        };
        match message {
            Message::RenameInput(typed) => *name = typed,
            Message::RenameSubmit if !action.is_pending() => {
                let (host, id, name) = (target.host_id.clone(), target.id.clone(), name.clone());
                *failure = None;
                return action
                    .start(ctx.state, move |state| async move {
                        containers_rename(&state, host, id, name).await
                    })
                    .map(Message::Renamed);
            }
            Message::Renamed(finished) => {
                if action.finish(finished) {
                    ctx.invalidate(keys::host::containers(&target.host_id));
                    self.dialog = None;
                } else if let Some(error) = action.error() {
                    *failure = Some(Box::new(Surfaced::new(error.clone(), "containers_rename")));
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
