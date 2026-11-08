//! Each Host's Engine events, kept in the database so the Events page still
//! has them after Homeport restarts.
//!
//! Events arrive in bursts (a Stack starting sends dozens), so they are
//! written in batches by a thread of their own: at most a second late, or
//! as soon as [`BATCH`] are waiting. Recording never waits for the database.
//! A Host keeps at most [`CAP_PER_HOST`] events, and none older than the
//! retention the user sets ([`EventStore::prune`]).

use std::{
    collections::HashSet,
    sync::{mpsc, Arc, Mutex},
    time::{Duration, Instant},
};

use rusqlite::{params, OptionalExtension};
use serde::{Deserialize, Serialize};

use homeport_core::error::AppResult;
use homeport_storage::Db;

use crate::registry::EngineEvent;

/// How many events a Host keeps at most, whatever the retention.
pub const CAP_PER_HOST: u32 = 10_000;
/// How many events one page of the Events page lists.
pub const PAGE: u32 = 1_000;
/// How many waiting events make the writer write at once.
const BATCH: usize = 200;
/// How long an event waits at most before it is written.
const FLUSH_EVERY: Duration = Duration::from_secs(1);

/// Where a page of older events starts: the oldest event already listed.
/// Events are ordered by time, then by these fields, which together tell
/// any two events of a Host apart.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct EventCursor {
    pub time_nano: i64,
    pub kind: String,
    pub action: String,
    pub actor_id: String,
}

enum Command {
    Record(String, EngineEvent),
    Flush(tokio::sync::oneshot::Sender<()>),
}

pub struct EventStore {
    db: Arc<Db>,
    writer: Mutex<mpsc::Sender<Command>>,
}

impl EventStore {
    /// The store over `db`, with its writer thread running.
    pub fn new(db: Arc<Db>) -> Arc<Self> {
        let (tx, rx) = mpsc::channel();
        let writer_db = db.clone();
        std::thread::Builder::new()
            .name("engine-events".into())
            .spawn(move || write_batches(&writer_db, &rx))
            .expect("start the engine events writer");
        Arc::new(Self {
            db,
            writer: Mutex::new(tx),
        })
    }

    /// Queues `event` of Host `host_id` for writing; never blocks.
    pub fn record(&self, host_id: &str, event: &EngineEvent) {
        self.send(Command::Record(host_id.to_owned(), event.clone()));
    }

    /// Waits until every event recorded so far is written.
    pub async fn flush(&self) {
        let (ack, done) = tokio::sync::oneshot::channel();
        self.send(Command::Flush(ack));
        let _ = done.await;
    }

    fn send(&self, command: Command) {
        if self
            .writer
            .lock()
            .expect("events writer lock poisoned")
            .send(command)
            .is_err()
        {
            log::warn!("the engine events writer stopped; an event wasn't kept");
        }
    }

    /// Up to `limit` of a Host's events, the latest ones, or the latest
    /// before `before`; oldest first.
    pub fn page(
        &self,
        host_id: &str,
        limit: u32,
        before: Option<&EventCursor>,
    ) -> AppResult<Vec<EngineEvent>> {
        let mut events = self.db.with(|c| {
            let mut stmt = c.prepare_cached(
                "SELECT time_nano, kind, action, actor_id, attributes FROM engine_events
                 WHERE host_id = ?1
                   AND (?2 IS NULL OR (time_nano, kind, action, actor_id) < (?2, ?3, ?4, ?5))
                 ORDER BY time_nano DESC, kind DESC, action DESC, actor_id DESC
                 LIMIT ?6",
            )?;
            let rows = stmt.query_map(
                params![
                    host_id,
                    before.map(|b| b.time_nano),
                    before.map(|b| &b.kind),
                    before.map(|b| &b.action),
                    before.map(|b| &b.actor_id),
                    limit,
                ],
                |r| {
                    let time_nano: i64 = r.get(0)?;
                    let attributes: String = r.get(4)?;
                    Ok(EngineEvent {
                        time: time_nano.div_euclid(1_000_000_000),
                        time_nano,
                        kind: r.get(1)?,
                        action: r.get(2)?,
                        actor_id: r.get(3)?,
                        attributes: serde_json::from_str(&attributes).unwrap_or_default(),
                    })
                },
            )?;
            rows.collect::<rusqlite::Result<Vec<_>>>()
        })?;
        events.reverse();
        Ok(events)
    }

    /// When a Host's latest kept event happened, in nanoseconds.
    pub fn last_time_nano(&self, host_id: &str) -> AppResult<Option<i64>> {
        self.db.with(|c| {
            c.query_row(
                "SELECT max(time_nano) FROM engine_events WHERE host_id = ?1",
                [host_id],
                |r| r.get(0),
            )
            .optional()
            .map(Option::flatten)
        })
    }

    /// Removes the events older than `keep`, and each Host's beyond the
    /// latest [`CAP_PER_HOST`]. Returns how many it removed.
    pub fn prune(&self, keep: Duration) -> AppResult<usize> {
        let cutoff = now_nano() - i64::try_from(keep.as_nanos()).unwrap_or(i64::MAX);
        self.db.with(|c| {
            let old = c.execute("DELETE FROM engine_events WHERE time_nano < ?1", [cutoff])?;
            let over = c.execute(
                "DELETE FROM engine_events WHERE rowid IN (
                     SELECT rowid FROM (
                         SELECT rowid, row_number() OVER (
                             PARTITION BY host_id ORDER BY time_nano DESC
                         ) AS n FROM engine_events
                     ) WHERE n > ?1
                 )",
                [CAP_PER_HOST],
            )?;
            Ok(old + over)
        })
    }

    /// Forgets every event of a Host.
    pub fn clear(&self, host_id: &str) -> AppResult<()> {
        self.db.with(|c| {
            c.execute("DELETE FROM engine_events WHERE host_id = ?1", [host_id])
                .map(drop)
        })
    }
}

/// Now, in nanoseconds since the Unix epoch.
pub fn now_nano() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| i64::try_from(d.as_nanos()).unwrap_or(i64::MAX))
}

/// The writer thread: collects recorded events and writes them a batch at a
/// time, until the store is dropped.
fn write_batches(db: &Db, commands: &mpsc::Receiver<Command>) {
    let mut batch: Vec<(String, EngineEvent)> = Vec::new();
    let mut due: Option<Instant> = None;
    loop {
        let command = match due {
            Some(due) => commands.recv_timeout(due.saturating_duration_since(Instant::now())),
            None => commands
                .recv()
                .map_err(|_| mpsc::RecvTimeoutError::Disconnected),
        };
        match command {
            Ok(Command::Record(host, event)) => {
                due.get_or_insert_with(|| Instant::now() + FLUSH_EVERY);
                batch.push((host, event));
                if batch.len() < BATCH {
                    continue;
                }
            }
            Ok(Command::Flush(ack)) => {
                write(db, &mut batch);
                due = None;
                let _ = ack.send(());
                continue;
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {}
            Err(mpsc::RecvTimeoutError::Disconnected) => return write(db, &mut batch),
        }
        write(db, &mut batch);
        due = None;
    }
}

/// Writes `batch` in one transaction, skipping events already kept (a
/// reconnection asks the Engine again from the last one) and those of Hosts
/// removed meanwhile, then trims each Host it touched to the cap.
fn write(db: &Db, batch: &mut Vec<(String, EngineEvent)>) {
    if batch.is_empty() {
        return;
    }
    let hosts: HashSet<&str> = batch.iter().map(|(host, _)| host.as_str()).collect();
    let result = db.with(|c| {
        let tx = c.transaction()?;
        {
            let mut insert = tx.prepare_cached(
                "INSERT OR IGNORE INTO engine_events
                     (host_id, time_nano, kind, action, actor_id, attributes)
                 SELECT ?1, ?2, ?3, ?4, ?5, ?6 WHERE EXISTS (SELECT 1 FROM hosts WHERE id = ?1)",
            )?;
            for (host, e) in batch.iter() {
                let attributes =
                    serde_json::to_string(&e.attributes).unwrap_or_else(|_| "{}".into());
                insert.execute(params![
                    host,
                    e.time_nano,
                    e.kind,
                    e.action,
                    e.actor_id,
                    attributes
                ])?;
            }
            let mut trim = tx.prepare_cached(
                "DELETE FROM engine_events WHERE rowid IN (
                     SELECT rowid FROM engine_events WHERE host_id = ?1
                     ORDER BY time_nano DESC LIMIT -1 OFFSET ?2
                 )",
            )?;
            for host in &hosts {
                trim.execute(params![host, CAP_PER_HOST])?;
            }
        }
        tx.commit()
    });
    if let Err(e) = result {
        log::warn!("couldn't keep {} engine events: {e}", batch.len());
    }
    batch.clear();
}

/// The Engine's `since` for events after `time_nano`: seconds with nanoseconds.
pub fn since(time_nano: i64) -> String {
    format!(
        "{}.{:09}",
        time_nano.div_euclid(1_000_000_000),
        time_nano.rem_euclid(1_000_000_000)
    )
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::store::{HostDraft, HostStore};
    use homeport_core::host::HostConnection;
    use homeport_storage::SecretBox;

    fn setup() -> (Arc<Db>, Arc<EventStore>, String) {
        let db = Arc::new(Db::open_in_memory().unwrap());
        let hosts = HostStore::new(db.clone(), Arc::new(SecretBox::from_key(&[3; 32])));
        let host = hosts
            .create(HostDraft {
                name: "local".into(),
                connection: HostConnection::Local {
                    socket_path: "/var/run/docker.sock".into(),
                },
                engine: crate::EngineKind::Docker,
                secrets: None,
            })
            .unwrap()
            .id;
        (db.clone(), EventStore::new(db), host)
    }

    fn event(time_nano: i64, action: &str) -> EngineEvent {
        EngineEvent {
            time: time_nano / 1_000_000_000,
            time_nano,
            kind: "container".into(),
            action: action.into(),
            actor_id: "abc".into(),
            attributes: HashMap::from([("name".into(), "web".into())]),
        }
    }

    #[tokio::test]
    async fn events_are_kept_once_in_order() {
        let (_, store, host) = setup();
        let now = now_nano();
        for (t, action) in [(now - 30, "start"), (now - 20, "die"), (now - 30, "start")] {
            store.record(&host, &event(t, action));
        }
        // Two events in the same nanosecond are told apart by what they are.
        store.record(&host, &event(now - 20, "stop"));
        store.flush().await;

        let page = store.page(&host, PAGE, None).unwrap();
        let actions: Vec<_> = page.iter().map(|e| e.action.as_str()).collect();
        assert_eq!(actions, ["start", "die", "stop"]);
        assert_eq!(page[0].attributes["name"], "web");
        assert_eq!(store.last_time_nano(&host).unwrap(), Some(now - 20));
    }

    #[tokio::test]
    async fn pages_go_back_from_a_cursor() {
        let (_, store, host) = setup();
        let now = now_nano();
        for i in 0..5 {
            store.record(&host, &event(now - 100 + i, &format!("a{i}")));
        }
        store.flush().await;
        let latest = store.page(&host, 2, None).unwrap();
        assert_eq!(
            latest.iter().map(|e| &e.action).collect::<Vec<_>>(),
            ["a3", "a4"]
        );
        let oldest = &latest[0];
        let cursor = EventCursor {
            time_nano: oldest.time_nano,
            kind: oldest.kind.clone(),
            action: oldest.action.clone(),
            actor_id: oldest.actor_id.clone(),
        };
        let older = store.page(&host, 10, Some(&cursor)).unwrap();
        assert_eq!(
            older.iter().map(|e| &e.action).collect::<Vec<_>>(),
            ["a0", "a1", "a2"]
        );
    }

    #[tokio::test]
    async fn bursts_are_written_without_a_flush() {
        let (_, store, host) = setup();
        let now = now_nano();
        for i in 0..BATCH as i64 {
            store.record(&host, &event(now - 1_000 + i, "start"));
        }
        for _ in 0..100 {
            if store.page(&host, PAGE, None).unwrap().len() == BATCH {
                return;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        panic!("a full batch wasn't written at once");
    }

    #[tokio::test]
    async fn retention_and_the_cap_bound_the_history() {
        let (_, store, host) = setup();
        let day = 86_400 * 1_000_000_000_i64;
        let now = now_nano();
        store.record(&host, &event(now - 40 * day, "old"));
        store.record(&host, &event(now - day, "recent"));
        store.flush().await;
        assert_eq!(store.prune(Duration::from_secs(30 * 86_400)).unwrap(), 1);
        let left = store.page(&host, PAGE, None).unwrap();
        assert_eq!(
            left.iter().map(|e| &e.action).collect::<Vec<_>>(),
            ["recent"]
        );

        for i in 0..i64::from(CAP_PER_HOST) + 5 {
            store.record(&host, &event(now - 100_000 + i, "burst"));
        }
        store.flush().await;
        let kept = store.page(&host, CAP_PER_HOST + 10, None).unwrap();
        assert_eq!(kept.len(), CAP_PER_HOST as usize);
        assert_eq!(kept.last().unwrap().action, "burst");
    }

    #[tokio::test]
    async fn removing_a_host_removes_its_events_and_ignores_late_ones() {
        let (db, store, host) = setup();
        store.record(&host, &event(now_nano(), "start"));
        store.flush().await;
        db.with(|c| c.execute("DELETE FROM hosts WHERE id = ?1", [&host]))
            .unwrap();
        assert!(store.page(&host, PAGE, None).unwrap().is_empty());
        // An event still on its way when the Host was removed is dropped.
        store.record(&host, &event(now_nano(), "die"));
        store.flush().await;
        assert!(store.page(&host, PAGE, None).unwrap().is_empty());
    }

    #[tokio::test]
    async fn clearing_forgets_a_hosts_events() {
        let (_, store, host) = setup();
        store.record(&host, &event(now_nano(), "start"));
        store.flush().await;
        store.clear(&host).unwrap();
        assert!(store.page(&host, PAGE, None).unwrap().is_empty());
        assert_eq!(store.last_time_nano(&host).unwrap(), None);
    }

    #[test]
    fn since_keeps_the_nanoseconds() {
        assert_eq!(since(1_700_000_000_000_000_042), "1700000000.000000042");
    }
}
