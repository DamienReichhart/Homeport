//! The Hosts page: every Host with its connection status. A placeholder
//! for `src/features/hosts`, which its ticket ports whole (the list's
//! actions, Add host, a Host's settings); it already stands on what a real
//! feature does: the shell's interface, and the Hosts read through the
//! data layer.

use std::sync::Arc;

use iced::{widget::text, Length, Task};

use homeport_hosts::service::HostView;

use crate::{
    data::{keys, resources, Resource},
    t,
    ui::{
        shell::{layout::page_header, status, Context},
        theme::{
            self,
            metrics::{space, text as size},
            Element, Icon,
        },
        widgets::{
            feedback::{
                empty_state, error_message, error_message::Event, spinner, status_dot, Surfaced,
            },
            filling, line, stack,
        },
    },
};

#[derive(Debug, Default)]
pub struct State;

#[derive(Debug, Clone)]
pub enum Message {
    /// What was asked of the failure the page shows.
    Failure(Event),
}

pub fn open(ctx: &Context<'_>) -> (State, Task<Message>) {
    ctx.watch(resources::hosts());
    (State, Task::none())
}

fn hosts(ctx: &Context<'_>) -> Resource<Arc<Vec<HostView>>> {
    ctx.data.read(&keys::host::list())
}

pub fn update(_state: &mut State, message: Message, ctx: &Context<'_>) -> Task<Message> {
    match message {
        Message::Failure(Event::Retry) => ctx.invalidate(keys::host::list()),
        Message::Failure(Event::Details) => {
            if let Some(error) = hosts(ctx).error() {
                ctx.details(Surfaced::new(error.clone(), "hosts_list"));
            }
        }
        Message::Failure(Event::Copy(command)) => ctx.copy(command),
    }
    Task::none()
}

pub fn request(_state: &mut State, _action: &str, _ctx: &Context<'_>) -> Task<Message> {
    Task::none()
}

fn row<'a>(view: &HostView) -> Element<'a, Message> {
    let summary = status::host::describe(&view.status);
    line(vec![
        status_dot(summary.tone),
        filling(text(view.host.name.clone()).size(size::SM)).into(),
        text(summary.label)
            .size(size::SM)
            .style(theme::text::muted)
            .into(),
    ])
    .spacing(space::S2)
    .width(Length::Fill)
    .into()
}

pub fn view<'a>(_state: &'a State, ctx: &Context<'a>) -> Element<'a, Message> {
    let hosts = hosts(ctx);
    let mut parts = vec![page_header(t!("hosts.title"), None, vec![])];
    if let Some(error) = hosts.error() {
        parts.push(error_message(error, true, Message::Failure));
    }
    match hosts.data() {
        None if hosts.is_pending() => parts.push(spinner()),
        Some(list) if !list.is_empty() => {
            parts.push(
                stack(list.iter().map(row).collect())
                    .spacing(space::S2)
                    .into(),
            );
        }
        _ if hosts.error().is_none() => {
            parts.push(empty_state(t!("hosts.empty")).icon(Icon::Server).view());
        }
        _ => {}
    }
    stack(parts).spacing(space::S4).into()
}

#[cfg(test)]
mod tests {
    use homeport_core::error::{AppError, ErrorCode};
    use homeport_hosts::registry::HostStatus;

    use super::*;
    use crate::{
        test_support::{
            fixtures::{connected, host_view},
            snapshot::assert_snapshot,
        },
        ui::{
            shell::{page::testing::Bench, router::Route},
            theme::Theme,
        },
    };

    fn bench(list: Vec<HostView>) -> Bench {
        let mut bench = Bench::at(Route::Hosts);
        bench.data.set(&keys::host::list(), list);
        bench
    }

    fn three() -> Vec<HostView> {
        vec![
            host_view("prod", connected()),
            host_view("staging", HostStatus::Connecting),
            host_view("backup", HostStatus::Disabled),
        ]
    }

    #[test]
    fn opening_reads_the_hosts_through_the_data_layer() {
        let bench = Bench::at(Route::Hosts);
        let _ = open(&bench.ctx());
        assert_eq!(bench.watched(), [keys::host::list()]);
    }

    #[test]
    fn a_failure_leads_to_its_details_and_retry_reads_again() {
        let mut bench = bench(three());
        let locked = AppError::new(ErrorCode::DbLocked, homeport_storage::messages::DB_LOCKED);
        bench.data.fail(&keys::host::list(), &locked);
        let mut state = State;
        let _ = update(&mut state, Message::Failure(Event::Details), &bench.ctx());
        let _ = update(&mut state, Message::Failure(Event::Retry), &bench.ctx());
        assert_eq!(bench.asked(), ["details", "invalidate"]);
    }

    #[test]
    fn the_page_in_english() {
        let bench = bench(three());
        assert_snapshot("hosts-en", "en", &Theme::Light, (360.0, 160.0), || {
            view(&State, &bench.ctx())
        });
    }

    #[test]
    fn the_page_in_arabic_is_mirrored() {
        let bench = bench(three());
        assert_snapshot("hosts-ar", "ar", &Theme::Light, (360.0, 160.0), || {
            view(&State, &bench.ctx())
        });
    }

    #[test]
    fn the_page_with_no_host() {
        let bench = bench(vec![]);
        assert_snapshot(
            "hosts-empty-en",
            "en",
            &Theme::Light,
            (360.0, 220.0),
            || view(&State, &bench.ctx()),
        );
    }
}
