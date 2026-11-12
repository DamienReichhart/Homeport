//! What a gate's buttons do: go somewhere, connect again, show why, and
//! add what the welcome found on this machine.

use std::sync::Arc;

use iced::Task;

use crate::{
    api,
    app::AppState,
    data::{action::Finished, keys, Data},
    ui::{
        shell::{
            gates::{self, view::Action as GateAction, Verdict},
            page::Pages,
            selection::Selection,
            status, Event, Message, Shell,
        },
        widgets::feedback::Surfaced,
    },
};

impl<P: Pages> Shell<P> {
    pub(super) fn gate(
        &mut self,
        action: GateAction,
        data: &mut Data,
    ) -> Task<Message<P::Message>> {
        let route = self.location().route.clone();
        let verdict = gates::decide(&route, &self.world);
        let error = match &verdict {
            Verdict::HostDown(view) => status::host::error(&view.status),
            Verdict::ClusterDown(view) => view.status.error(),
            Verdict::ClustersFailed(error) | Verdict::SwarmFailed(error) => Some(*error),
            _ => None,
        };
        match action {
            GateAction::Go(route) => self.go(route.into(), data),
            GateAction::Details => {
                if let Some(error) = error {
                    let surfaced = Surfaced::at(error.clone(), None, jiff::Timestamp::now());
                    self.events.push(Event::Details(surfaced));
                }
                Task::none()
            }
            GateAction::Copy(command) => {
                self.events.push(Event::Copy(command));
                Task::none()
            }
            GateAction::AddEngine(socket) => {
                let Some(input) = gates::welcome::host_input(&self.world, &socket) else {
                    return Task::none();
                };
                let add = |state: Arc<AppState>| async move {
                    let view = api::hosts::hosts_create(&state, input).await?;
                    Ok(Some(Selection::Host { id: view.host.id }))
                };
                self.adding.start(&self.state, add).map(Message::Added)
            }
            GateAction::AddCluster(context) => {
                let Some(choice) = gates::welcome::context_choice(&self.world, &context) else {
                    return Task::none();
                };
                let add = |state: Arc<AppState>| async move {
                    let added = api::clusters::clusters_import(&state, vec![choice], None).await?;
                    let first = added.into_iter().next();
                    Ok(first.map(|view| Selection::Cluster {
                        id: view.cluster.id,
                    }))
                };
                self.adding.start(&self.state, add).map(Message::Added)
            }
            GateAction::WhyNot(socket) => {
                let problem = gates::welcome::engines(&self.world)
                    .into_iter()
                    .find(|engine| engine.socket_path == socket)
                    .and_then(|engine| engine.problem.clone());
                if let Some(error) = problem {
                    let surfaced = Surfaced::at(error, None, jiff::Timestamp::now());
                    self.events.push(Event::Details(surfaced));
                }
                Task::none()
            }
            GateAction::Retry => match verdict {
                Verdict::HostDown(view) => {
                    let id = view.host.id.clone();
                    let reconnect = |state: Arc<AppState>| async move {
                        api::hosts::hosts_reconnect(&state, id).await.map(|_| ())
                    };
                    self.reconnect
                        .start(&self.state, reconnect)
                        .map(Message::Reconnected)
                }
                Verdict::ClusterDown(view) => {
                    let id = view.cluster.id.clone();
                    let reconnect = |state: Arc<AppState>| async move {
                        api::clusters::clusters_reconnect(&state, id)
                            .await
                            .map(|_| ())
                    };
                    self.reconnect
                        .start(&self.state, reconnect)
                        .map(Message::Reconnected)
                }
                Verdict::ClustersFailed(_) => data
                    .invalidate(&self.state, &keys::cluster::list())
                    .map(Message::Data),
                Verdict::SwarmFailed(_) => {
                    let key = keys::swarm::summary(route.swarm_id().unwrap_or_default());
                    data.invalidate(&self.state, &key).map(Message::Data)
                }
                _ => Task::none(),
            },
        }
    }

    /// What was found on this machine was added, or couldn't be.
    pub(super) fn added(
        &mut self,
        finished: Finished<Option<Selection>>,
        data: &mut Data,
    ) -> Task<Message<P::Message>> {
        self.adding.finish(finished);
        let added = self.adding.done().cloned().flatten();
        let failed = self.adding.error().cloned();
        self.adding.reset();
        if let Some(error) = failed {
            self.events
                .push(Event::Failed(None, Surfaced::new(error, "add_local")));
            return Task::none();
        }
        // The lists have it now, and what was found is found again.
        let stale = [
            keys::host::list(),
            keys::cluster::list(),
            keys::host::local_engines(),
            keys::cluster::local_clusters(),
        ];
        let mut tasks: Vec<_> = stale
            .iter()
            .map(|key| data.invalidate(&self.state, key).map(Message::Data))
            .collect();
        tasks.extend(added.map(|selection| self.select(selection)));
        Task::batch(tasks)
    }
}
