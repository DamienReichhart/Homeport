//! The Run dialog (`ui/dialogs/RunContainerDialog.tsx` and `run/`):
//! create and start a container from an image, or from another
//! container's settings (Duplicate, Edit). What `@/features/containers`
//! exported to the Images and pods pages, so it is the domain's.
//!
//! - [`model`]: the form as typed, what can't be read in it, and the Run
//!   request it makes: the crate's `RunSpec`, as the Tauri command took;
//! - [`form`]: the fields as the page keeps them (Run, Duplicate and
//!   Edit show the same), drawn by `view`;
//! - [`dialog`]: the Run dialog, for Run and Duplicate.
//!
//! `Dialog::run(host, image, pod, ..)` is what the Images page's Run and a
//! pod's "Run a container in it" will open (their tickets): the image and
//! the pod filled in.
//!
//! # Parity with the React dialog
//!
//! | React | Here | |
//! | --- | --- | --- |
//! | Image (focused), Name ("Generated if empty"), Command (monospaced) | `view::fields` | done |
//! | "Pulled as … on …" / "Pulled anonymously" under an image the Host doesn't have | `registries_login_for`, watched per image | done |
//! | Image's suggestions: the Host's tags (`datalist`) | none | not done: the kit has no field of free text with suggestions |
//! | Pod (Podman Host, not a copy): None or a pod; its network and ports are then the pod's, said under it | `Choice::Pod`, ports and network hidden | done |
//! | Ports: host, container, tcp/udp, ×, "Add port"; rows left empty dropped | `view/rows.rs`, `RunForm::to_spec` | done |
//! | Volumes: source, container path, read-only, ×, "Add volume" | the same | done |
//! | Environment, one `KEY=value` a line, `#` comments dropped | textarea, `lines` | done |
//! | Network: Default (bridge), the Host's but `bridge`, one named that is gone | `form::network_names` | done |
//! | Restart policy, disabled while "Remove when it exits" is on, which sets it to Never | `Switch::AutoRemove` | done |
//! | Advanced, closed unless something in it is set, opened by a field it can't read; a tab per section, a dot on a section that fails | `collapsible`, kit tabs | differs: the dot is in the tab's text, not red |
//! | Resources: CPUs, shares, memory, reservation, swap (`-1`), each explained when it can't be read | `model::advanced` (`parse.ts`) | done |
//! | Labels, DNS, search domains, extra hosts, log options: a line each | textareas | done |
//! | Capabilities added and dropped: Enter or a comma adds, Backspace on empty removes, `cap_x` → `X`, × on each | `rows::capabilities` | done; their suggestions (`datalist`) not done, as the image's |
//! | Privileged, with its red warning | `feedback::alert` | done |
//! | Devices (host, container, permissions `rwm`), GPUs: none, all, a number, chosen | the same | done |
//! | Healthcheck: the image's, a command (interval, timeout, retries, start period), none | the same | done |
//! | Hostname, domain name | the same | done |
//! | Log driver (suggested drivers), options | the same | done; the drivers' suggestions not done, as the image's |
//! | Run disabled without an image, while running, or with a field it can't read; "Starting…" | `Form::is_ready` | done |
//! | Run: `containers_run` (with the pod), "Container started", the list read again, the logs of the new container | `ctx.act`, `Step::Started` | done |
//! | A refusal said in the dialog (`ErrorMessage`) | `error_message`, Details | done |
//! | Enter in a field runs (`<form onSubmit>`) | `on_submit` on every field of one line | done |
//! | Duplicate: the original's settings, `<name>-copy`, the host ports both publish said | `Dialog::duplicate`, `shared_host_ports` | done |
//!
//! # What differs from the React dialog, on purpose
//!
//! - A port that is not a number from 1 to 65 535 is said under its row
//!   and Run waits; React sent it, and Tauri refused the request (or, for
//!   a host port, let the Engine pick one).
//! - Closed while the Engine creates the container, the dialog is gone:
//!   "Container started" is said and the list read again, but the window
//!   stays where it is; React's callback still went to the logs.
//! - The dialog scrolls inside the window with its buttons; React's
//!   scrolled within 90 % of it.

pub mod dialog;
pub mod form;
pub mod model;
mod parts;
mod view;

pub use self::{
    dialog::{Dialog, Step},
    form::Form,
    view::fields,
};

#[cfg(all(test, unix))]
mod tests;
