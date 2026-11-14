//! The fields of the Run dialog as the page keeps them (`RunFormFields`,
//! `AdvancedOptions` and its sections): the form as typed, what its
//! fields of several lines hold, the state of its selects, and which
//! section of the Advanced area shows. Run, Duplicate and Edit show the
//! same fields; what they do with them is theirs.

use std::sync::Arc;

use iced::widget::text_editor::{Action, Content};

use homeport_engine::{images::Image, networks::NetworkView, podman::pods::Pod};

use super::model::{
    advanced::{self, DeviceRow, GpuMode, HealthMode},
    restart_label, Errors, MountRow, PortRow, RunForm, RESTART,
};
pub use super::parts::{Caps, Choice, Hint, Line, Rows, Section, Switch, Text};
use crate::{
    data::{keys, Resource},
    t,
    ui::{
        shell::Context,
        widgets::form::{select, suggest},
    },
};

#[derive(Debug, Clone)]
pub enum Message {
    Typed(Line, String),
    Text(Text, Action),
    Choose(Choice, select::Message),
    Switch(Switch, bool),
    Add(Rows),
    Remove(Rows, usize),
    Section(Section),
    /// Enter, or a comma, in a list of capabilities: what was typed joins.
    AddCapability(Caps),
    /// Its × on a capability.
    RemoveCapability(Caps, usize),
    /// Backspace in the empty field: the last goes.
    RemoveLastCapability(Caps),
    /// Enter in a field: the form is sent, as a form's `onSubmit`.
    Submit,
    /// A field with suggestions.
    Suggest(Hint, suggest::Message),
}

/// The log drivers most Hosts have (`LoggingField`'s `DRIVERS`); any
/// other name can be typed.
pub const DRIVERS: [&str; 5] = ["json-file", "local", "journald", "syslog", "none"];

/// The lists of the fields with suggestions.
#[derive(Debug, Default)]
struct Hints {
    image: suggest::State,
    cap_add: suggest::State,
    cap_drop: suggest::State,
    driver: suggest::State,
}

/// What the fields of several lines hold.
#[derive(Debug)]
struct Editors {
    env: Content,
    labels: Content,
    dns: Content,
    dns_search: Content,
    extra_hosts: Content,
    log_options: Content,
}

impl Editors {
    fn of(form: &RunForm) -> Self {
        let a = &form.advanced;
        Self {
            env: Content::with_text(&form.env),
            labels: Content::with_text(&a.labels),
            dns: Content::with_text(&a.identity.dns),
            dns_search: Content::with_text(&a.identity.dns_search),
            extra_hosts: Content::with_text(&a.identity.extra_hosts),
            log_options: Content::with_text(&a.logging.options),
        }
    }
}

/// The selects' states.
#[derive(Debug, Default)]
struct Selects {
    network: select::State,
    restart: select::State,
    pod: select::State,
    protocols: Vec<select::State>,
    gpus: select::State,
    health: select::State,
}

/// The protocols a port is published over: words of no language.
pub const PROTOCOLS: [&str; 2] = ["tcp", "udp"];

/// What the Run dialog's fields keep.
#[derive(Debug)]
pub struct Form {
    pub host_id: String,
    pub form: RunForm,
    editors: Editors,
    selects: Selects,
    /// Edit keeps the name; Rename changes it.
    pub lock_name: bool,
    /// A new container on a Podman Host may go into one of its pods.
    pub offers_pods: bool,
    pub advanced_open: bool,
    pub section: Section,
    /// What is being typed into the lists of capabilities.
    pub drafts: [String; 2],
    hints: Hints,
}

/// What a draw needs of the Host's lists.
pub struct Lists {
    pub images: Resource<Arc<Vec<Image>>>,
    pub networks: Resource<Arc<Vec<NetworkView>>>,
    pub pods: Resource<Arc<Vec<Pod>>>,
}

impl Form {
    /// The fields showing `form`, on `host_id`. The Advanced area is open
    /// when something in it is set.
    pub fn new(host_id: &str, form: RunForm) -> Self {
        let mut selects = Selects::default();
        selects
            .protocols
            .resize_with(form.ports.len(), Default::default);
        Self {
            host_id: host_id.to_owned(),
            editors: Editors::of(&form),
            advanced_open: form.has_advanced(),
            form,
            selects,
            lock_name: false,
            offers_pods: false,
            section: Section::Resources,
            drafts: Default::default(),
            hints: Hints::default(),
        }
    }

    /// What a field with suggestions offers (React's `<datalist>`): the
    /// Host's image tags, the capabilities not chosen, the usual drivers.
    pub fn suggestions(&self, hint: Hint, lists: &Lists) -> Vec<String> {
        match hint {
            Hint::Image => lists
                .images
                .data()
                .map(|images| images.iter().flat_map(|i| i.tags.iter().cloned()).collect())
                .unwrap_or_default(),
            Hint::Capability(caps) => {
                let security = &self.form.advanced.security;
                let chosen = match caps {
                    Caps::Add => &security.cap_add,
                    Caps::Drop => &security.cap_drop,
                };
                homeport_engine::containers::run::CAPABILITIES
                    .iter()
                    .filter(|name| !chosen.iter().any(|c| c == *name))
                    .map(|name| (*name).to_owned())
                    .collect()
            }
            Hint::LogDriver => DRIVERS.map(str::to_owned).to_vec(),
        }
    }

    /// The list of a field with suggestions.
    pub fn hint(&self, hint: Hint) -> &suggest::State {
        let h = &self.hints;
        match hint {
            Hint::Image => &h.image,
            Hint::Capability(Caps::Add) => &h.cap_add,
            Hint::Capability(Caps::Drop) => &h.cap_drop,
            Hint::LogDriver => &h.driver,
        }
    }

    /// The field a hint is of, and what it holds.
    fn hinted(&self, hint: Hint) -> (Line, String) {
        match hint {
            Hint::Image => (Line::Image, self.form.image.clone()),
            Hint::Capability(caps) => (Line::Capability(caps), self.drafts[caps as usize].clone()),
            Hint::LogDriver => (Line::LogDriver, self.form.advanced.logging.driver.clone()),
        }
    }

    fn suggested(
        &mut self,
        hint: Hint,
        message: suggest::Message,
        ctx: &Context<'_>,
    ) -> (iced::Task<Message>, bool) {
        let options = self.suggestions(hint, &self.lists(ctx));
        let (line, value) = self.hinted(hint);
        let h = &mut self.hints;
        let state = match hint {
            Hint::Image => &mut h.image,
            Hint::Capability(Caps::Add) => &mut h.cap_add,
            Hint::Capability(Caps::Drop) => &mut h.cap_drop,
            Hint::LogDriver => &mut h.driver,
        };
        let (task, event) = state.update(message, &value, &options);
        match event {
            Some(suggest::Event::Input(text)) => self.typed(line, text),
            // Enter adds the capability typed (`TokenInput`); elsewhere it
            // sends the form.
            Some(suggest::Event::Submit) => match hint {
                Hint::Capability(caps) => self.add_capability(caps),
                Hint::Image | Hint::LogDriver => return (task, true),
            },
            None => {}
        }
        (task, false)
    }

    /// The Host's images, networks and pods, as the data layer has them.
    pub fn lists(&self, ctx: &Context<'_>) -> Lists {
        Lists {
            images: ctx.data.read(&keys::host::images(&self.host_id)),
            networks: ctx.data.read(&keys::host::networks(&self.host_id)),
            pods: ctx.data.read(&keys::host::pods(&self.host_id)),
        }
    }

    pub fn errors(&self) -> Errors {
        self.form.errors()
    }

    /// Whether Run (or Apply) can be pressed: an image, and nothing that
    /// can't be read.
    pub fn is_ready(&self) -> bool {
        !self.form.image.trim().is_empty() && self.errors().is_empty()
    }

    /// The Advanced area shows: opened, or one of its fields fails.
    pub fn shows_advanced(&self) -> bool {
        self.advanced_open || !self.errors().advanced.is_empty()
    }

    /// Whether the container goes into a pod: its ports and network are
    /// the pod's, and their fields don't show.
    pub fn in_pod(&self) -> bool {
        self.offers_pods && !self.form.pod.is_empty()
    }

    pub fn editor(&self, which: Text) -> &Content {
        let e = &self.editors;
        match which {
            Text::Env => &e.env,
            Text::Labels => &e.labels,
            Text::Dns => &e.dns,
            Text::DnsSearch => &e.dns_search,
            Text::ExtraHosts => &e.extra_hosts,
            Text::LogOptions => &e.log_options,
        }
    }

    pub fn select(&self, which: Choice) -> &select::State {
        static CLOSED: std::sync::LazyLock<select::State> =
            std::sync::LazyLock::new(select::State::default);
        let s = &self.selects;
        match which {
            Choice::Network => &s.network,
            Choice::Restart => &s.restart,
            Choice::Pod => &s.pod,
            Choice::Protocol(row) => s.protocols.get(row).unwrap_or(&CLOSED),
            Choice::Gpus => &s.gpus,
            Choice::Health => &s.health,
        }
    }

    /// The options of a select, and which is chosen.
    pub fn options(&self, which: Choice, lists: &Lists) -> (Vec<String>, Option<usize>) {
        let form = &self.form;
        let at = |values: &[String], value: &str| values.iter().position(|v| v == value);
        match which {
            Choice::Network => {
                let names = network_names(form, lists);
                let chosen = if form.network.is_empty() {
                    Some(0)
                } else {
                    at(&names, &form.network).map(|i| i + 1)
                };
                let mut labels = vec![t!("containers.run.defaultNetwork")];
                labels.extend(names);
                (labels, chosen)
            }
            Choice::Restart => (
                RESTART.iter().map(|p| restart_label(*p)).collect(),
                RESTART.iter().position(|p| *p == form.restart),
            ),
            Choice::Pod => {
                let names = pod_names(lists);
                let chosen = if form.pod.is_empty() {
                    Some(0)
                } else {
                    at(&names, &form.pod).map(|i| i + 1)
                };
                let mut labels = vec![t!("containers.run.noPod")];
                labels.extend(names);
                (labels, chosen)
            }
            Choice::Protocol(row) => {
                let labels: Vec<String> = PROTOCOLS.iter().map(|p| (*p).to_owned()).collect();
                let chosen = form.ports.get(row).and_then(|p| at(&labels, &p.protocol));
                (labels, chosen)
            }
            Choice::Gpus => (
                GpuMode::ALL.iter().map(|m| m.label()).collect(),
                GpuMode::ALL
                    .iter()
                    .position(|m| *m == form.advanced.security.gpus.mode),
            ),
            Choice::Health => (
                HealthMode::ALL.iter().map(|m| m.label()).collect(),
                HealthMode::ALL
                    .iter()
                    .position(|m| *m == form.advanced.healthcheck.mode),
            ),
        }
    }

    /// What `message` changes; what it started (focus that leaves a
    /// field), and whether it sends the form (Enter).
    pub fn update(&mut self, message: Message, ctx: &Context<'_>) -> (iced::Task<Message>, bool) {
        match message {
            Message::Suggest(hint, message) => return self.suggested(hint, message, ctx),
            Message::Submit => return (iced::Task::none(), true),
            Message::Typed(line, value) => self.typed(line, value),
            Message::Text(which, action) => self.edit(which, action),
            Message::Choose(which, message) => self.choose(which, message, ctx),
            Message::Switch(which, on) => self.switch(which, on),
            Message::Add(rows) => self.add(rows),
            Message::Remove(rows, index) => self.remove(rows, index),
            Message::Section(section) => self.section = section,
            Message::AddCapability(caps) => self.add_capability(caps),
            Message::RemoveCapability(caps, index) => {
                let list = self.capabilities(caps);
                if index < list.len() {
                    list.remove(index);
                }
            }
            Message::RemoveLastCapability(caps) => {
                if self.drafts[caps as usize].is_empty() {
                    self.capabilities(caps).pop();
                }
            }
        }
        (iced::Task::none(), false)
    }

    fn capabilities(&mut self, caps: Caps) -> &mut Vec<String> {
        let security = &mut self.form.advanced.security;
        match caps {
            Caps::Add => &mut security.cap_add,
            Caps::Drop => &mut security.cap_drop,
        }
    }

    /// What was typed joins its list, as the list writes names, once.
    fn add_capability(&mut self, caps: Caps) {
        let draft = std::mem::take(&mut self.drafts[caps as usize]);
        let typed = draft.trim();
        let value = advanced::capability(typed.strip_suffix(',').unwrap_or(typed));
        let list = self.capabilities(caps);
        if !value.is_empty() && !list.contains(&value) {
            list.push(value);
        }
    }

    fn typed(&mut self, line: Line, value: String) {
        let a = &mut self.form.advanced;
        let r = &mut a.resources;
        let h = &mut a.healthcheck;
        let field: &mut String = match line {
            Line::Image => &mut self.form.image,
            Line::Name => &mut self.form.name,
            Line::Command => &mut self.form.command,
            Line::HostPort(i) => match self.form.ports.get_mut(i) {
                Some(row) => &mut row.host_port,
                None => return,
            },
            Line::ContainerPort(i) => match self.form.ports.get_mut(i) {
                Some(row) => &mut row.container_port,
                None => return,
            },
            Line::Source(i) => match self.form.mounts.get_mut(i) {
                Some(row) => &mut row.source,
                None => return,
            },
            Line::Target(i) => match self.form.mounts.get_mut(i) {
                Some(row) => &mut row.target,
                None => return,
            },
            Line::Cpus => &mut r.cpus,
            Line::CpuShares => &mut r.cpu_shares,
            Line::Memory => &mut r.memory,
            Line::MemoryReservation => &mut r.memory_reservation,
            Line::MemorySwap => &mut r.memory_swap,
            Line::HostDevice(i) | Line::ContainerDevice(i) | Line::Permissions(i) => {
                let Some(row) = a.security.devices.get_mut(i) else {
                    return;
                };
                match line {
                    Line::HostDevice(_) => &mut row.host_path,
                    Line::ContainerDevice(_) => &mut row.container_path,
                    _ => &mut row.permissions,
                }
            }
            Line::GpuCount => &mut a.security.gpus.count,
            Line::GpuIds => &mut a.security.gpus.ids,
            Line::HealthCommand => &mut h.command,
            Line::Interval => &mut h.interval,
            Line::Timeout => &mut h.timeout,
            Line::Retries => &mut h.retries,
            Line::StartPeriod => &mut h.start_period,
            Line::Hostname => &mut a.identity.hostname,
            Line::Domainname => &mut a.identity.domainname,
            Line::LogDriver => &mut a.logging.driver,
            Line::Capability(caps) => {
                // A comma ends a name, as Enter does.
                let ends = value.ends_with(',');
                self.drafts[caps as usize] = value;
                if ends {
                    self.add_capability(caps);
                }
                return;
            }
        };
        *field = value;
    }

    fn edit(&mut self, which: Text, action: Action) {
        let edits = action.is_edit();
        let e = &mut self.editors;
        let a = &mut self.form.advanced;
        let (content, field) = match which {
            Text::Env => (&mut e.env, &mut self.form.env),
            Text::Labels => (&mut e.labels, &mut a.labels),
            Text::Dns => (&mut e.dns, &mut a.identity.dns),
            Text::DnsSearch => (&mut e.dns_search, &mut a.identity.dns_search),
            Text::ExtraHosts => (&mut e.extra_hosts, &mut a.identity.extra_hosts),
            Text::LogOptions => (&mut e.log_options, &mut a.logging.options),
        };
        content.perform(action);
        if edits {
            *field = content.text();
        }
    }

    fn choose(&mut self, which: Choice, message: select::Message, ctx: &Context<'_>) {
        let lists = self.lists(ctx);
        let (labels, _) = self.options(which, &lists);
        let s = &mut self.selects;
        let state = match which {
            Choice::Network => &mut s.network,
            Choice::Restart => &mut s.restart,
            Choice::Pod => &mut s.pod,
            Choice::Protocol(row) => match s.protocols.get_mut(row) {
                Some(state) => state,
                None => return,
            },
            Choice::Gpus => &mut s.gpus,
            Choice::Health => &mut s.health,
        };
        let Some(index) = state.update(message, &labels) else {
            return;
        };
        let form = &mut self.form;
        match which {
            // The first option of these two is none.
            Choice::Network => {
                form.network = network_names(form, &lists)
                    .get(index.wrapping_sub(1))
                    .cloned()
                    .unwrap_or_default();
            }
            Choice::Pod => {
                form.pod = pod_names(&lists)
                    .get(index.wrapping_sub(1))
                    .cloned()
                    .unwrap_or_default();
            }
            Choice::Restart => form.restart = RESTART[index.min(RESTART.len() - 1)],
            Choice::Protocol(row) => {
                if let Some(port) = form.ports.get_mut(row) {
                    PROTOCOLS[index.min(1)].clone_into(&mut port.protocol);
                }
            }
            Choice::Gpus => form.advanced.security.gpus.mode = GpuMode::ALL[index.min(3)],
            Choice::Health => form.advanced.healthcheck.mode = HealthMode::ALL[index.min(2)],
        }
    }

    fn switch(&mut self, which: Switch, on: bool) {
        match which {
            Switch::AutoRemove => {
                // A container removed when it exits isn't restarted.
                self.form.auto_remove = on;
                if on {
                    self.form.restart = RESTART[0];
                }
            }
            Switch::ReadOnly(i) => {
                if let Some(row) = self.form.mounts.get_mut(i) {
                    row.read_only = on;
                }
            }
            Switch::Privileged => self.form.advanced.security.privileged = on,
            Switch::Advanced => self.advanced_open = on,
        }
    }

    fn add(&mut self, rows: Rows) {
        match rows {
            Rows::Ports => {
                self.form.ports.push(PortRow::default());
                self.selects.protocols.push(select::State::default());
            }
            Rows::Mounts => self.form.mounts.push(MountRow::default()),
            Rows::Devices => self
                .form
                .advanced
                .security
                .devices
                .push(DeviceRow::default()),
        }
    }

    fn remove(&mut self, rows: Rows, index: usize) {
        fn take<T>(list: &mut Vec<T>, index: usize) {
            if index < list.len() {
                list.remove(index);
            }
        }
        match rows {
            Rows::Ports => {
                take(&mut self.form.ports, index);
                take(&mut self.selects.protocols, index);
            }
            Rows::Mounts => take(&mut self.form.mounts, index),
            Rows::Devices => take(&mut self.form.advanced.security.devices, index),
        }
    }
}

/// The networks a container can be put on: the Host's but `bridge` (the
/// default), and the one the form names if it is not among them (`host`,
/// `none`, a network that is gone), so it still shows as chosen.
fn network_names(form: &RunForm, lists: &Lists) -> Vec<String> {
    let mut names: Vec<String> = lists
        .networks
        .data()
        .map(|list| list.iter().map(|n| n.name.clone()).collect())
        .unwrap_or_default();
    names.retain(|name| name != "bridge");
    if !form.network.is_empty() && !names.contains(&form.network) {
        names.push(form.network.clone());
    }
    names
}

fn pod_names(lists: &Lists) -> Vec<String> {
    lists
        .pods
        .data()
        .map(|list| list.iter().map(|p| p.name.clone()).collect())
        .unwrap_or_default()
}
