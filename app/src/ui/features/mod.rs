//! The features: one folder per former `src/features/<name>`, grouped by
//! what they are about (`app`, `docker`, `podman`, `containerd`, `incus`,
//! `kubernetes`, `nomad`). Each is a `State`, a `Message` and four
//! functions (`open`, `update`, `view`, `request`, with `refresh` and
//! `subscription` for what changes around it), and never uses another
//! feature.
//!
//! [`Page`] is all of them behind the one type the shell hosts: the table
//! below says which module answers for which feature of the registry, one
//! line per feature.

pub mod app;
pub mod containerd;
pub mod docker;
pub mod incus;
pub mod kubernetes;
pub mod nomad;
pub mod podman;
#[cfg(test)]
mod tests;

use iced::{Subscription, Task};

use super::{
    shell::{
        page::{Context, Pages},
        registry::FeatureId,
    },
    theme::Element,
};

/// Declares [`Page`] and its [`Message`] with a variant per feature, and
/// sends each call of [`Pages`] to the feature's own function.
macro_rules! pages {
    ($($id:ident => $($module:ident)::+,)+) => {
        /// The page that shows: the state of one feature.
        #[derive(Debug)]
        pub enum Page {
            $($id($($module)::+::State),)+
        }

        /// A message of the page that shows.
        #[derive(Debug, Clone)]
        pub enum Message {
            $($id($($module)::+::Message),)+
        }

        /// The features that have a page, for the test that holds this
        /// table to the registry.
        pub const PAGES: &[FeatureId] = &[$(FeatureId::$id,)+];

        impl Pages for Page {
            type Message = Message;

            fn open(ctx: &Context<'_>) -> (Self, Task<Message>) {
                match ctx.location.route.feature() {
                    $(FeatureId::$id => {
                        let (state, task) = $($module)::+::open(ctx);
                        (Self::$id(state), task.map(Message::$id))
                    })+
                    // It has no page: its commands open the Settings.
                    FeatureId::Updates => {
                        let (state, task) = app::settings::open(ctx);
                        (Self::Settings(state), task.map(Message::Settings))
                    }
                }
            }

            fn update(&mut self, message: Message, ctx: &Context<'_>) -> Task<Message> {
                match (self, message) {
                    $((Self::$id(state), Message::$id(message)) => {
                        $($module)::+::update(state, message, ctx).map(Message::$id)
                    })+
                    // A message of a page that was left meanwhile.
                    _ => Task::none(),
                }
            }

            fn view<'a>(&'a self, ctx: &Context<'a>) -> Element<'a, Message> {
                match self {
                    $(Self::$id(state) => $($module)::+::view(state, ctx).map(Message::$id),)+
                }
            }

            fn request(&mut self, action: &str, ctx: &Context<'_>) -> Task<Message> {
                match self {
                    $(Self::$id(state) => {
                        $($module)::+::request(state, action, ctx).map(Message::$id)
                    })+
                }
            }

            fn refresh(&mut self, ctx: &Context<'_>) -> Task<Message> {
                match self {
                    $(Self::$id(state) => {
                        $($module)::+::refresh(state, ctx).map(Message::$id)
                    })+
                }
            }

            fn subscription(&self, ctx: &Context<'_>) -> Subscription<Message> {
                match self {
                    $(Self::$id(state) => {
                        $($module)::+::subscription(state, ctx).map(Message::$id)
                    })+
                }
            }
        }
    };
}

pages! {
    Overview => app::overview,
    Board => app::board,
    Containers => docker::containers,
    Images => docker::images,
    Volumes => docker::volumes,
    Networks => docker::networks,
    Stacks => docker::stacks,
    PodmanPods => podman::podman_pods,
    Quadlets => podman::quadlets,
    ContainerdContainers => containerd::containerd_containers,
    ContainerdImages => containerd::containerd_images,
    ContainerdSnapshots => containerd::containerd_snapshots,
    ContainerdNamespaces => containerd::containerd_namespaces,
    IncusInstances => incus::incus_instances,
    IncusImages => incus::incus_images,
    IncusProfiles => incus::incus_profiles,
    IncusStorage => incus::incus_storage,
    IncusNetworks => incus::incus_networks,
    Swarm => docker::swarm,
    Workloads => kubernetes::workloads,
    Pods => kubernetes::pods,
    Network => kubernetes::network,
    Config => kubernetes::config,
    Nodes => kubernetes::nodes,
    ClusterEvents => kubernetes::cluster_events,
    ApiResources => kubernetes::api_resources,
    Rbac => kubernetes::rbac,
    Apply => kubernetes::apply,
    Helm => kubernetes::helm,
    PortForwards => kubernetes::port_forwards,
    NomadJobs => nomad::nomad_jobs,
    NomadNodes => nomad::nomad_nodes,
    Events => docker::events,
    Settings => app::settings,
    Hosts => app::hosts,
    Clusters => app::clusters,
}
