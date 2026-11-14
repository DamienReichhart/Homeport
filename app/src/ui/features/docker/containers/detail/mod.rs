//! A container's page (`ui/detail/ContainerDetailPage.tsx`): its header
//! with what can be done to it, and its tabs (`tabs/`, drawn by `view/`).
//! What only this page reads is in `queries`; what the tabs work out of
//! the Engine's answers, in `model`.
//!
//! # Parity with the React page
//!
//! | React | Here | |
//! | --- | --- | --- |
//! | Route `containers/:id/:tab?`, the logs when no tab or an unknown one | `Route::Container`, `Tab::of` | done |
//! | A tab change replaces the address (`navigate(.., { replace: true })`) | `ctx.keep_route`: the route changes, the history doesn't | done |
//! | The id may be the start of one (`id.startsWith`) | `State::find`: the id, else the name, else a start no other id shares; a tab's route then names the whole id | differs: an ambiguous start finds none (React took the first), a name is found too |
//! | Back: to the Board when opened from it (`state.from`), else to the list | `CAME_FROM` parameter; Back is `ctx.back()` then | done |
//! | Loading: a spinner while the list is read | `spinner` | done |
//! | Gone: "This container no longer exists." and "Back to containers" | the same, Details too | done |
//! | List that could not be read | React said the container was gone; here the failure, with Retry | differs |
//! | Header: Back, status dot, name, Stack badge; image, "newer image" mark, status | `view::header` | done |
//! | Stop or Start, Restart, Pause or Resume, Remove (confirmed, then the list) | `Actions::header` (shared actions, `ctx.act`) | done |
//! | More: Rename | the shared Rename dialog | done |
//! | More: Edit, Duplicate, Commit | in the menu, disabled: the Run dialog's ticket (#259) | not done (#259) |
//! | More: Export (save dialog, then progress, Cancel, Close, toast) | `actions::export` | done |
//! | — | More: Kill (confirmed), as in the list's menu | added |
//! | Tabs Logs, Inspect, Health (only with a healthcheck), Exec, Files, Stats, Processes | `Tab::shown` | done |
//! | Logs: history (100, 1 000, 10 000, all), since (any, 5 min, 1 h, 24 h), timestamps, stdout, stderr, follow | kit log viewer, options asked of the Engine | done |
//! | Logs: an option the Engine applies opens the stream again on an empty view | `Logs::restart`; a late chunk of the stream before is ignored | done |
//! | Logs: find, previous, next, clear, download (save dialog, "Logs saved") | the viewer's own; the save through `ctx.platform` and `ctx.act` | done |
//! | Logs: end of the stream; an error written among the lines | "end of logs"; the error over the lines with Retry | differs (kit) |
//! | Logs: a stopped container's logs | shown, with their end | done |
//! | Inspect: Details (container, health and its last 5 runs, environment, mounts, ports, networks, labels) and Raw JSON | `model::inspect`, kit `key_value`, tables and JSON tree | done |
//! | Inspect: read again when the state changes; failure with Retry; spinner | state in the key; `error_message`; `spinner` | done |
//! | Health: status, command, timing, failing streak; last results with exit, time, duration, folded output | `view::health` | done |
//! | Health: read again when the list's status changes, and once per check interval | status in the key; a timer while the tab shows | done |
//! | Health: "no healthcheck", "no check has run", failure with Retry | the same | done |
//! | Exec: a terminal with several shells | stand-in: "Nothing here." (running), "Start the container…" (stopped) | not done (#259) |
//! | Files: browse, refresh, download (save dialog, name or name.tar), "Upload here" (open dialog), toasts | kit file browser, `ctx.platform`, `ctx.act` | done |
//! | Files: a stopped container | "Start the container to browse its files." | done |
//! | — | Files: transfer's progress (files put, bytes come) and Cancel; a failure per file; a download written whole or not at all | added |
//! | Stats: CPU, memory, network, disk charts of the last 5 minutes, the figure of the moment | kit charts (`chart::State::counter` for the rates) | done |
//! | Stats: a stopped container | "Start the container to see its resource usage." | done |
//! | Processes: top under the Engine's titles, every 5 s; stopped; failure with Retry | kit table, a timer while the tab shows | done |
//! | Palette commands, shortcuts | none of the page's own in React; the feature's (`containers:run`, `n`, `/`) are the list's | done |
//!
//! # What differs from the React page, on purpose
//!
//! - A list that could not be read is said, with Retry, where React said
//!   the container no longer existed.
//! - Kill is in the header's menu, as in the list's; React's page had it
//!   only in the list.
//! - A tab that is left lets go of its stream and its timers, and starts
//!   over when it comes back, as a React tab was unmounted and mounted.
//!   A log stream that ended (the container stopped, or was restarted) is
//!   opened again once a list read since says the container runs; the
//!   end itself asks for that list, since a restart is running to
//!   running. React's stayed on "end of logs".
//! - The health and processes timers run only while their tab shows (a
//!   subscription of the tab); React's `refetchInterval` did the same by
//!   unmounting.
//! - Files go one at a time, so each failure is its own (one toast, every
//!   file that failed in its Details), and the transfer shows its count
//!   and can be stopped: Cancel stops before the next file, never in one,
//!   so no file is left cut short in the container, and what went in is
//!   said. A download shows its bytes (of how many, for a file) and Cancel
//!   drops it at once, leaving no file. React sent the files in one
//!   archive and had neither progress nor Cancel.
//! - A transfer or an export goes on to its end when the page is left,
//!   and the shell says how it ended; React's export stopped when its
//!   dialog went with the page.
//! - Two cards a row from a window 1 024 wide (`lg`), taken from the
//!   window's size, as the layout has no breakpoints. The Health tab's
//!   healthcheck stands over its results, each as wide as the page: in a
//!   third of the width (React's `lg` grid) a command wrapped over four
//!   lines.
//! - A healthcheck's timing is written in the syntax `docker run` reads
//!   (`30s`, `1m30s`), laid out as such; how long a check took is said in
//!   the language's units and decimal sign. React wrote both in English.
//! - The charts of the Stats tab say a stream that could not start; React
//!   showed empty charts.
//! - Technical text (names, images, paths, ids, values, labels, outputs)
//!   is laid out left to right in both directions (`basic::technical`).
//! - Environment values are shown as React showed them, unmasked: the
//!   React page had no masking, and the raw view showed them too.

mod message;
mod model;
mod queries;
mod state;
mod tabs;
mod update;
mod view;

#[cfg(test)]
mod tests;

use iced::Subscription;

use self::state::Tab;
pub use self::{
    message::Message,
    state::State,
    update::{open, refresh, request, update},
    view::{dialog, view},
};
use crate::ui::shell::Context;

/// What the tab that shows follows: the container's logs or its stats,
/// while it is there. A tab that is left, a page that is left and a
/// container that is gone let go of their stream, which cancels it.
pub fn subscription(state: &State, ctx: &Context<'_>) -> Subscription<Message> {
    let Some(container) = state.container(ctx) else {
        return Subscription::none();
    };
    let stream = match state.tab {
        Tab::Logs => state
            .logs
            .subscription(ctx.state, &state.host_id, &container.id)
            .map(Message::Logs),
        // Only a running container has stats (`useStats`'s `running`).
        Tab::Stats if container.state == "running" => state
            .stats
            .subscription(ctx.state, &state.host_id, &container.id)
            .map(Message::Stats),
        // Read again while the tab shows: every five seconds, and once
        // per check.
        Tab::Processes if container.state == "running" => iced::time::every(tabs::processes::EVERY)
            .map(|_| Message::Processes(tabs::processes::Message::Poll)),
        Tab::Health => {
            let read = state.health_read(&container, ctx);
            let health = read.data().and_then(|health| health.as_ref().as_ref());
            iced::time::every(tabs::health::every(health))
                .map(|_| Message::Health(tabs::health::Message::Poll))
        }
        _ => Subscription::none(),
    };
    // The window's size lays the page out: how tall the logs are, how
    // many charts a row holds.
    let resized = iced::window::resize_events().map(|(_, size)| Message::Resized(size));
    Subscription::batch([stream, resized])
}
