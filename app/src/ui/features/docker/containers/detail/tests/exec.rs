//! The Exec tab against the fake Engine: a shell opens when the tab
//! shows, takes keys, prints, says how it ended; other terminals open with
//! their command and user and close; leaving the tab closes them all. And
//! the recorded transcripts of real programs (vim, an htop-like frame,
//! top, tmux: `app/tests/fixtures/terminal`) go through the page's real
//! session, from the Engine's socket to the grid, their queries answered
//! back to the Engine.

use std::{path::PathBuf, time::Duration};

use alacritty_terminal::{
    index::{Column, Line},
    term::cell::Flags,
};
use homeport_engine::images::freshness::{Freshness, FreshnessResult};
use iced::Task;
use tokio::task::JoinHandle;

use super::*;
use crate::{
    test_support::{
        engine::{Engine, Stream},
        fixtures::{connected, host_view},
        following::Following,
        snapshot::{assert_snapshot, in_language},
        tasks::messages,
    },
    ui::{
        features::docker::containers::detail::{
            subscription,
            tabs::exec::{Draft, Message as Exec},
        },
        shell::{gates::Subject, selection::Selection},
        theme::Theme,
        widgets::terminal,
    },
};

/// A container's page on the fake Engine, whose tasks run beside it:
/// a terminal's writer runs until its session ends, so nothing waits on
/// a task here.
struct Terminals {
    rig: Rig,
    engine: Engine,
    following: Following<Message>,
    running: Vec<JoinHandle<Vec<Message>>>,
}

impl Terminals {
    fn at(tab: &str) -> Self {
        let mut bench = Bench::at(Route::Containers);
        let (engine, id) = bench.app.runtime.block_on(async {
            let engine = Engine::start();
            let id = engine.host(&bench.app.state).await;
            (engine, id)
        });
        let mut host = host_view("fake", connected());
        host.host.id.clone_from(&id);
        bench.subject = Subject::Host {
            host_id: id.clone(),
        };
        bench.world.selection = Some(Selection::Host { id: id.clone() });
        bench.set_hosts(vec![host]);
        bench.data.set(&keys::host::containers(&id), sample());
        bench.history.push(route(WEB, Some(tab)).into());
        let mut terminals = Self {
            rig: Rig::open_on(bench),
            engine,
            following: Following::default(),
            running: vec![],
        };
        terminals.sync();
        terminals
    }

    fn sync(&mut self) {
        let follows = subscription(&self.rig.state, &self.rig.bench.ctx());
        let _runtime = self.rig.bench.app.runtime.enter();
        self.following.sync(follows);
    }

    /// Sends `message`; what it started runs beside the page.
    fn send(&mut self, message: Message) {
        let runtime = self.rig.bench.app.runtime.handle().clone();
        let _inside = runtime.enter();
        let task: Task<Message> = update(&mut self.rig.state, message, &self.rig.bench.ctx());
        self.running.push(runtime.spawn(messages(task)));
        let _ = refresh(&mut self.rig.state, &self.rig.bench.ctx());
        self.sync();
    }

    fn exec(&mut self, message: Exec) {
        self.send(Message::Exec(message));
    }

    /// Hands the page what its streams and its tasks send, until `done`
    /// or three seconds.
    fn until(&mut self, done: impl Fn(&Self) -> bool) -> bool {
        for _ in 0..150 {
            if done(self) {
                return true;
            }
            let finished: Vec<_> = self
                .running
                .extract_if(.., |task| task.is_finished())
                .collect();
            for task in finished {
                let answers = self
                    .rig
                    .bench
                    .app
                    .runtime
                    .block_on(task)
                    .unwrap_or_default();
                for answer in answers {
                    self.send(answer);
                }
            }
            let next = self.following.next(Duration::from_millis(20));
            if let Some(message) = self.rig.bench.app.runtime.block_on(next) {
                self.send(message);
            }
        }
        done(self)
    }

    fn terminal(&self, key: u32) -> &terminal::State {
        let shell = self.rig.state.exec.shells.iter().find(|s| s.key == key);
        &shell.expect("the terminal is open").terminal
    }

    /// What the rows of a terminal's screen say, their blanks off.
    fn screen(&self, key: u32) -> Vec<String> {
        let state = self.terminal(key);
        let grid = state.term().grid();
        let (cols, rows) = state.size();
        (0..rows)
            .map(|row| {
                let line = Line(i32::try_from(row).unwrap());
                let text: String = (0..cols).map(|col| grid[line][Column(col)].c).collect();
                text.trim_end().to_owned()
            })
            .collect()
    }

    fn flags(&self, key: u32, row: i32, col: usize) -> Flags {
        self.terminal(key).term().grid()[Line(row)][Column(col)].flags
    }

    fn says(&self, key: u32, text: &str) -> bool {
        self.screen(key).iter().any(|row| row.contains(text))
    }
}

fn fixture(name: &str) -> Vec<u8> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/terminal")
        .join(name);
    std::fs::read(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

#[test]
fn a_shell_opens_with_the_tab_takes_keys_and_says_how_it_ended() {
    let _english = in_language("en");
    let mut live = Terminals::at("exec");
    assert!(live.until(|live| live.engine.open(Stream::Exec) == 1));
    let created = live.engine.execs();
    // A shell (no command), a terminal of the grid's size.
    assert!(created[0].contains(r#""Tty":true"#), "{}", created[0]);
    assert!(
        created[0].contains(r#""ConsoleSize":[24,80]"#),
        "{}",
        created[0]
    );
    live.exec(Exec::Terminal(
        1,
        terminal::Message::Input("ls\r".to_owned()),
    ));
    assert!(live.until(|live| live.engine.typed().contains("ls\r")));
    live.engine.print("README.md\r\n# ");
    assert!(live.until(|live| live.says(1, "README.md")));
    live.engine.exit_with(0);
    live.engine.exit();
    assert!(live.until(|live| live.says(1, "[process exited with code 0]")));
    assert!(live.rig.state.exec.shells[0].failure.is_none());
    // Keys typed after the end go nowhere, quietly.
    live.exec(Exec::Terminal(
        1,
        terminal::Message::Input("pwd\r".to_owned()),
    ));
    assert!(!live.engine.typed().contains("pwd"));
}

#[test]
fn a_container_without_the_shell_says_so_over_the_terminal() {
    let _english = in_language("en");
    let mut live = Terminals::at("exec");
    assert!(live.until(|live| live.engine.open(Stream::Exec) == 1));
    live.engine.exit_with(127);
    live.engine.exit();
    assert!(live.until(|live| live.rig.state.exec.shells[0].failure.is_some()));
    let failure = live.rig.state.exec.shells[0].failure.as_ref().unwrap();
    assert_eq!(failure.error.code, homeport_core::error::ErrorCode::NoShell);
    assert!(live.says(1, "[process exited with code 127]"));
}

#[test]
fn another_terminal_runs_its_command_as_its_user_and_closes() {
    let _english = in_language("en");
    let mut live = Terminals::at("exec");
    assert!(live.until(|live| live.engine.open(Stream::Exec) == 1));
    live.exec(Exec::New);
    assert!(live.rig.shows("Open"));
    live.exec(Exec::Typed(Draft::Command, "psql".to_owned()));
    live.exec(Exec::Typed(Draft::User, "postgres".to_owned()));
    live.exec(Exec::Open);
    assert!(live.until(|live| live.engine.open(Stream::Exec) == 2));
    let created = live.engine.execs();
    assert!(created[1].contains(r#""psql""#), "{}", created[1]);
    assert!(
        created[1].contains(r#""User":"postgres""#),
        "{}",
        created[1]
    );
    // It shows, under its command; one opened without is numbered after
    // the highest open (`Math.max(..keys) + 1`): "Shell 2" again.
    assert_eq!(live.rig.state.exec.active, 2);
    assert!(live.rig.shows("psql") && live.rig.shows("Shell"));
    live.exec(Exec::Close(2));
    assert!(live.until(|live| live.engine.open(Stream::Exec) == 1));
    assert_eq!(live.rig.state.exec.active, 1);
    // What was typed stays for the next (React kept its draft).
    assert_eq!(live.rig.state.exec.draft.command, "psql");
    live.exec(Exec::Typed(Draft::Command, String::new()));
    live.exec(Exec::Open);
    assert!(live.rig.shows("Shell 2"));
    live.exec(Exec::Close(1));
    live.exec(Exec::Close(2));
    assert!(live.until(|live| live.engine.open(Stream::Exec) == 0));
    assert!(live.rig.shows("No terminal open."));
}

#[test]
fn leaving_the_tab_closes_every_terminal_and_coming_back_starts_over() {
    let mut live = Terminals::at("exec");
    assert!(live.until(|live| live.engine.open(Stream::Exec) == 1));
    live.exec(Exec::Open);
    assert!(live.until(|live| live.engine.open(Stream::Exec) == 2));
    live.send(Message::Tab(Tab::Logs));
    assert!(live.until(|live| live.engine.open(Stream::Exec) == 0));
    live.send(Message::Tab(Tab::Exec));
    assert!(live.until(|live| live.engine.open(Stream::Exec) == 1));
    assert_eq!(
        live.rig.state.exec.shells.len(),
        1,
        "one shell, as at first"
    );
    assert_eq!(live.engine.execs().len(), 3);
}

#[test]
fn a_stopped_container_asks_to_be_started_and_opens_nothing() {
    let _english = in_language("en");
    let mut live = Terminals::at("exec");
    assert!(live.until(|live| live.engine.open(Stream::Exec) == 1));
    let mut stopped = sample();
    "exited".clone_into(&mut stopped[0].state);
    let host = live.rig.state.host_id.clone();
    live.rig
        .bench
        .data
        .set(&keys::host::containers(&host), stopped);
    let _ = refresh(&mut live.rig.state, &live.rig.bench.ctx());
    live.sync();
    assert!(live.until(|live| live.engine.open(Stream::Exec) == 0));
    assert!(live.rig.shows("Start the container to open a terminal."));
}

#[test]
fn vim_through_the_page_draws_its_screen_and_is_answered() {
    let mut live = Terminals::at("exec");
    assert!(live.until(|live| live.engine.open(Stream::Exec) == 1));
    live.engine.print_bytes(&fixture("vim.bin"));
    assert!(live.until(|live| live.says(1, "\"~/hello.py\" 13L, 234B")));
    let rows = live.screen(1);
    assert_eq!(rows[0], "  1 #!/usr/bin/env python3");
    assert_eq!(rows[6], "  7 def greet(name: str) -> str:");
    assert!(rows[22].starts_with("~/hello.py") && rows[22].ends_with("All"));
    assert!(live.flags(1, 22, 0).contains(Flags::INVERSE | Flags::BOLD));
    // vim's questions (where the cursor is, which terminal, its colours)
    // were answered back to the program, through the Engine, in the
    // light theme's colours: the terminal was told it before it drew.
    let answers = "\x1b[2;2R\x1b[3;1R\x1b[>0;2600;1c\x1b]10;rgb:1414/1414/1414\x07\x1b]11;rgb:ffff/ffff/ffff\x07";
    assert!(
        live.until(|live| live.engine.typed().contains(answers)),
        "{:?}",
        live.engine.typed()
    );
}

#[test]
fn an_htop_frame_top_and_tmux_through_the_page_draw_as_recorded() {
    let mut live = Terminals::at("exec");
    assert!(live.until(|live| live.engine.open(Stream::Exec) == 1));
    live.engine.print_bytes(&fixture("meters.bin"));
    assert!(live.until(|live| live.says(1, "F1Help  F10Quit")));
    let rows = live.screen(1);
    assert!(rows[0].starts_with("  0[||||||||||||||||||||||") && rows[0].contains("66.0%]"));
    assert!(rows[4].starts_with(" Mem["));
    assert!(live.flags(1, 8, 0).contains(Flags::INVERSE));
    // A wide character takes its two cells.
    let wide = rows[10].find('日').map(|at| rows[10][..at].chars().count());
    let wide = wide.expect("the wide row");
    assert!(live.flags(1, 10, wide).contains(Flags::WIDE_CHAR));
    // htop leaves its screen; top draws on the main one.
    live.engine.print("\x1b[?1049l");
    live.engine.print_bytes(&fixture("top.bin"));
    assert!(live.until(|live| live.screen(1)[0].starts_with("top - ")));
    assert!(live.screen(1)[1].starts_with("Tasks:  80 total,"));
    live.engine.print_bytes(&fixture("tmux.bin"));
    assert!(live.until(|live| live.screen(1)[23].starts_with("[hp] 0:shell*")));
}

#[test]
fn the_exec_tab_looks_as_react_s() {
    let mut live = Terminals::at("exec");
    assert!(live.until(|live| live.engine.open(Stream::Exec) == 1));
    live.exec(Exec::Typed(Draft::Command, "psql".to_owned()));
    live.exec(Exec::Open);
    assert!(live.until(|live| live.engine.open(Stream::Exec) == 2));
    live.engine.print("postgres=# select 1;\r\n ?column?\r\n----------\r\n        1\r\n(1 row)\r\n\r\npostgres=# ");
    assert!(live.until(|live| live.says(2, "(1 row)")));
    // A newer image is published for it: the header says so.
    let newer = FreshnessResult {
        reference: live.rig.container().image,
        image_id: "sha256:abc".to_owned(),
        freshness: Freshness::Newer,
    };
    let host = live.rig.state.host_id.clone();
    live.rig
        .bench
        .data
        .set(&keys::host::freshness(&host), vec![newer]);
    let ctx = live.rig.bench.ctx();
    for language in ["en", "ar"] {
        let name = format!("container-exec-{language}");
        assert_snapshot(&name, language, &Theme::Light, SIZE, || {
            view(&live.rig.state, &ctx)
        });
    }
    assert_snapshot("container-exec-dark-en", "en", &Theme::Dark, SIZE, || {
        view(&live.rig.state, &ctx)
    });
}
