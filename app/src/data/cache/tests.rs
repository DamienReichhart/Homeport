//! The cache's rules, on a clock the tests move and with loaders that are
//! never run: a test answers each read itself.

use std::{sync::Arc, time::Duration};

use tokio::time::Instant;

use homeport_core::error::{AppError, ErrorCode};

use super::{backoff, Cache, Effect};
use crate::data::{keys::host, query::Value, Key, Query, Resource};

const SECOND: Duration = Duration::from_secs(1);

fn containers(host_id: &str) -> Query<Vec<&'static str>> {
    Query::new(host::containers(host_id), |_| async { Ok(vec![]) })
}

fn images(host_id: &str) -> Query<Vec<&'static str>> {
    Query::new(host::images(host_id), |_| async { Ok(vec![]) })
}

fn value(names: &[&'static str]) -> Result<Value, AppError> {
    Ok(Arc::new(names.to_vec()))
}

fn down() -> AppError {
    AppError::new(
        ErrorCode::HostNotConnected,
        homeport_hosts::messages::NOT_CONNECTED_UNKNOWN,
    )
}

/// The reads asked for: key, flight and wait.
fn loads(effects: &[Effect]) -> Vec<(Key, u64, Duration)> {
    effects
        .iter()
        .filter_map(|effect| match effect {
            Effect::Load {
                key, flight, after, ..
            } => Some((key.clone(), *flight, *after)),
            Effect::Abort { .. } => None,
        })
        .collect()
}

fn aborts(effects: &[Effect]) -> Vec<u64> {
    effects
        .iter()
        .filter_map(|effect| match effect {
            Effect::Abort { flight } => Some(*flight),
            Effect::Load { .. } => None,
        })
        .collect()
}

/// The flight of the only read asked for.
fn flight(effects: &[Effect]) -> u64 {
    let loads = loads(effects);
    assert_eq!(loads.len(), 1, "{loads:?}");
    loads[0].1
}

fn read(cache: &Cache, key: &Key) -> Resource<Vec<&'static str>> {
    cache
        .read::<Vec<&'static str>>(key)
        .map(|list| (*list).clone())
}

/// A cache where `h1`'s containers were read at `now` and are shown.
fn loaded(now: Instant) -> (Cache, Key) {
    let mut cache = Cache::default();
    let query = containers("h1");
    let started = cache.watch(&query, now);
    cache.settle(query.key(), flight(&started), value(&["web"]), now);
    (cache, query.key().clone())
}

#[test]
fn a_first_watch_loads_and_its_answer_is_shown() {
    let now = Instant::now();
    let mut cache = Cache::default();
    let query = containers("h1");
    assert_eq!(read(&cache, query.key()), Resource::Idle);

    let started = cache.watch(&query, now);
    assert_eq!(loads(&started), [(query.key().clone(), 1, Duration::ZERO)]);
    assert_eq!(read(&cache, query.key()), Resource::Loading);

    let after = cache.settle(query.key(), 1, value(&["web"]), now);
    assert!(after.is_empty());
    assert_eq!(read(&cache, query.key()), Resource::Loaded(vec!["web"]));
}

#[test]
fn two_pages_share_one_read() {
    let now = Instant::now();
    let mut cache = Cache::default();
    let started = cache.watch(&containers("h1"), now);
    assert_eq!(loads(&started).len(), 1);
    // A second page opens the same list while it is on its way, and a third
    // right after it arrived.
    assert!(cache.watch(&containers("h1"), now).is_empty());
    cache.settle(
        &host::containers("h1"),
        flight(&started),
        value(&["web"]),
        now,
    );
    assert!(cache.watch(&containers("h1"), now + SECOND).is_empty());
    // Another Host's list is another read.
    assert_eq!(loads(&cache.watch(&containers("h2"), now)).len(), 1);
}

#[test]
fn stale_data_is_shown_while_it_is_read_again() {
    let now = Instant::now();
    let (mut cache, key) = loaded(now);
    cache.unwatch(&key, now);

    // Still good 9 s later: the page coming back reads nothing.
    assert!(cache.watch(&containers("h1"), now + 9 * SECOND).is_empty());
    cache.unwatch(&key, now + 9 * SECOND);

    // Out of date after 10 s: shown as it was, and read again.
    let later = now + 10 * SECOND;
    let started = cache.watch(&containers("h1"), later);
    assert_eq!(loads(&started).len(), 1);
    assert_eq!(read(&cache, &key), Resource::Refreshing(vec!["web"]));
    cache.settle(&key, flight(&started), value(&["web", "db"]), later);
    assert_eq!(read(&cache, &key), Resource::Loaded(vec!["web", "db"]));
}

#[test]
fn staleness_follows_the_querys_policy() {
    let now = Instant::now();
    let mut cache = Cache::default();
    let never = || containers("h1").never_stale();
    let started = cache.watch(&never(), now);
    cache.settle(&host::containers("h1"), flight(&started), value(&[]), now);
    cache.unwatch(&host::containers("h1"), now);
    assert!(cache.watch(&never(), now + 200 * SECOND).is_empty());

    let always = || images("h1").always_stale();
    let started = cache.watch(&always(), now);
    cache.settle(&host::images("h1"), flight(&started), value(&[]), now);
    cache.unwatch(&host::images("h1"), now);
    assert_eq!(loads(&cache.watch(&always(), now)).len(), 1);
}

#[test]
fn a_failed_read_is_retried_with_a_growing_wait() {
    assert_eq!(backoff(0), SECOND);
    assert_eq!(backoff(1), 2 * SECOND);
    assert_eq!(backoff(4), 16 * SECOND);
    assert_eq!(backoff(5), 30 * SECOND);
    assert_eq!(backoff(40), 30 * SECOND);

    let now = Instant::now();
    let mut cache = Cache::default();
    let query = containers("h1").retries(2);
    let key = query.key().clone();
    let started = cache.watch(&query, now);
    let id = flight(&started);

    let first = cache.settle(&key, id, Err(down()), now);
    assert_eq!(loads(&first), [(key.clone(), id, SECOND)]);
    assert_eq!(read(&cache, &key), Resource::Loading);
    let second = cache.settle(&key, id, Err(down()), now + SECOND);
    assert_eq!(loads(&second), [(key.clone(), id, 2 * SECOND)]);

    // The last try fails too: the resource is failed, with nothing to keep.
    let third = cache.settle(&key, id, Err(down()), now + 3 * SECOND);
    assert!(third.is_empty());
    assert_eq!(read(&cache, &key), Resource::Failed(down(), None));
}

#[test]
fn a_retry_that_answers_clears_the_failure() {
    let now = Instant::now();
    let mut cache = Cache::default();
    let query = containers("h1");
    let key = query.key().clone();
    let id = flight(&cache.watch(&query, now));
    assert_eq!(loads(&cache.settle(&key, id, Err(down()), now)).len(), 1);
    cache.settle(&key, id, value(&["web"]), now + SECOND);
    assert_eq!(read(&cache, &key), Resource::Loaded(vec!["web"]));
}

#[test]
fn a_query_that_does_not_retry_fails_at_once_and_keeps_its_data() {
    let now = Instant::now();
    let (mut cache, key) = loaded(now);
    cache.unwatch(&key, now);
    let later = now + 20 * SECOND;
    let id = flight(&cache.watch(&containers("h1").no_retry(), later));
    assert!(cache.settle(&key, id, Err(down()), later).is_empty());
    assert_eq!(
        read(&cache, &key),
        Resource::Failed(down(), Some(vec!["web"]))
    );
    // A page opening a failed resource asks again.
    cache.unwatch(&key, later);
    assert_eq!(loads(&cache.watch(&containers("h1"), later)).len(), 1);
}

#[test]
fn invalidating_a_scope_reads_again_what_is_shown() {
    let now = Instant::now();
    let mut cache = Cache::default();
    for query in [containers("h1"), images("h1"), containers("h2")] {
        let id = flight(&cache.watch(&query, now));
        cache.settle(query.key(), id, value(&["one"]), now);
    }
    // Nobody shows h1's images any more.
    cache.unwatch(&host::images("h1"), now);

    let started = cache.invalidate(&host::scope("h1"));
    let keys: Vec<Key> = loads(&started).into_iter().map(|(key, ..)| key).collect();
    assert_eq!(keys, [host::containers("h1")]);
    assert_eq!(
        read(&cache, &host::containers("h1")),
        Resource::Refreshing(vec!["one"])
    );
    // h2 is untouched.
    assert_eq!(
        read(&cache, &host::containers("h2")),
        Resource::Loaded(vec!["one"])
    );
    // The images are read when a page opens them again, however recent.
    assert_eq!(
        read(&cache, &host::images("h1")),
        Resource::Loaded(vec!["one"])
    );
    assert_eq!(loads(&cache.watch(&images("h1"), now)).len(), 1);
}

#[test]
fn invalidating_during_a_read_reads_once_more_after() {
    let now = Instant::now();
    let mut cache = Cache::default();
    let query = containers("h1");
    let key = query.key().clone();
    let first = flight(&cache.watch(&query, now));

    // Three events while the list is on its way ask for one more read.
    for _ in 0..3 {
        assert!(cache.invalidate(&host::scope("h1")).is_empty());
    }
    let again = cache.settle(&key, first, value(&["old"]), now);
    let second = flight(&again);
    assert_ne!(first, second);
    assert_eq!(read(&cache, &key), Resource::Refreshing(vec!["old"]));
    assert!(cache.settle(&key, second, value(&["new"]), now).is_empty());
    assert_eq!(read(&cache, &key), Resource::Loaded(vec!["new"]));
}

#[test]
fn leaving_aborts_the_read_and_ignores_its_answer() {
    let now = Instant::now();
    let mut cache = Cache::default();
    let query = containers("h1");
    let key = query.key().clone();
    let id = flight(&cache.watch(&query, now));
    cache.watch(&query, now);

    // One of the two pages leaves: the other still waits.
    assert!(cache.unwatch(&key, now).is_empty());
    assert_eq!(aborts(&cache.unwatch(&key, now)), [id]);
    assert_eq!(read(&cache, &key), Resource::Idle);

    // The answer of the aborted read, should it still come, changes nothing.
    assert!(cache.settle(&key, id, value(&["late"]), now).is_empty());
    assert_eq!(read(&cache, &key), Resource::Idle);
}

#[test]
fn changing_selection_releases_everything_of_the_host() {
    let now = Instant::now();
    let mut cache = Cache::default();
    let containers_flight = flight(&cache.watch(&containers("h1"), now));
    let images_flight = flight(&cache.watch(&images("h1"), now));
    cache.watch(&images("h1"), now);
    let other = flight(&cache.watch(&containers("h2"), now));

    let mut stopped = aborts(&cache.release(&host::scope("h1"), now));
    stopped.sort_unstable();
    assert_eq!(stopped, [containers_flight, images_flight]);
    // h2's read goes on.
    cache.settle(&host::containers("h2"), other, value(&["web"]), now);
    assert_eq!(
        read(&cache, &host::containers("h2")),
        Resource::Loaded(vec!["web"])
    );
    // Nothing of h1 is polled or read for nobody.
    assert!(cache.invalidate(&host::scope("h1")).is_empty());
}

#[test]
fn a_host_going_down_fails_its_resources_and_keeps_their_data() {
    let now = Instant::now();
    let (mut cache, key) = loaded(now);
    let images_flight = flight(&cache.watch(&images("h1"), now));
    let other = flight(&cache.watch(&containers("h2"), now));
    cache.settle(&host::containers("h2"), other, value(&["db"]), now);

    let stopped = cache.fail(&host::scope("h1"), &down(), now);
    assert_eq!(aborts(&stopped), [images_flight]);
    assert_eq!(
        read(&cache, &key),
        Resource::Failed(down(), Some(vec!["web"]))
    );
    assert_eq!(
        read(&cache, &host::images("h1")),
        Resource::Failed(down(), None)
    );
    assert_eq!(
        read(&cache, &host::containers("h2")),
        Resource::Loaded(vec!["db"])
    );

    // It comes back: everything shown is read again, the old data on screen.
    let started = cache.invalidate(&host::scope("h1"));
    assert_eq!(loads(&started).len(), 2);
    assert_eq!(read(&cache, &key), Resource::Refreshing(vec!["web"]));
    assert_eq!(read(&cache, &host::images("h1")), Resource::Loading);
    let id = loads(&started)
        .into_iter()
        .find(|(k, ..)| *k == key)
        .map(|(_, id, _)| id)
        .unwrap();
    cache.settle(&key, id, value(&["web", "db"]), now + SECOND);
    assert_eq!(read(&cache, &key), Resource::Loaded(vec!["web", "db"]));
}

#[test]
fn a_polled_resource_is_read_again_at_its_interval_while_shown() {
    let now = Instant::now();
    let mut cache = Cache::default();
    assert!(!cache.polls());
    let query = || containers("h1").poll(5 * SECOND);
    let key = host::containers("h1");
    let id = flight(&cache.watch(&query(), now));
    assert!(cache.polls());
    // Not while its first read is on its way.
    assert!(cache.poll(now + 10 * SECOND).is_empty());
    cache.settle(&key, id, value(&["web"]), now + 10 * SECOND);

    assert!(cache.poll(now + 14 * SECOND).is_empty());
    let id = flight(&cache.poll(now + 15 * SECOND));
    // A failure counts as an answer: the next try is an interval later.
    cache.settle(&key, id, Err(down()), now + 15 * SECOND);
    assert!(cache
        .settle(&key, id, Err(down()), now + 16 * SECOND)
        .is_empty());
    assert!(cache.poll(now + 20 * SECOND).is_empty());
    assert_eq!(loads(&cache.poll(now + 21 * SECOND)).len(), 1);

    // Nobody shows it: nothing is polled.
    cache.release(&key, now + 21 * SECOND);
    assert!(!cache.polls());
    assert!(cache.poll(now + 60 * SECOND).is_empty());
}

#[test]
fn what_nobody_shows_is_forgotten_after_a_while() {
    let now = Instant::now();
    let (mut cache, key) = loaded(now);
    cache.unwatch(&key, now);
    cache.poll(now + 299 * SECOND);
    assert!(cache.holds(&key));
    cache.poll(now + 300 * SECOND);
    assert!(!cache.holds(&key));
    assert_eq!(read(&cache, &key), Resource::Idle);

    // A resource kept no time at all is forgotten as its page leaves.
    let query = images("h1").keep(Duration::ZERO);
    let id = flight(&cache.watch(&query, now));
    cache.settle(query.key(), id, value(&["nginx"]), now);
    cache.unwatch(query.key(), now);
    assert!(!cache.holds(query.key()));
    // What is shown is never forgotten.
    let query = containers("h2");
    cache.watch(&query, now);
    cache.poll(now + 3600 * SECOND);
    assert!(cache.holds(query.key()));
}

#[test]
fn a_value_can_be_set_and_changed_by_hand() {
    let now = Instant::now();
    let mut cache = Cache::default();
    let key = host::events("h1");
    // Nothing to change yet.
    assert!(!cache.update::<Vec<&str>>(&key, |list| Some(list.clone())));
    cache.set(&key, vec!["start"], now);
    assert_eq!(read(&cache, &key), Resource::Loaded(vec!["start"]));

    let appended = cache.update::<Vec<&str>>(&key, |list| {
        let mut list = list.clone();
        list.push("die");
        Some(list)
    });
    assert!(appended);
    assert_eq!(read(&cache, &key), Resource::Loaded(vec!["start", "die"]));
    // Leaving it, or naming another type, changes nothing.
    assert!(!cache.update::<Vec<&str>>(&key, |_| None));
    assert!(!cache.update::<u8>(&key, |n| Some(n + 1)));
    assert_eq!(cache.keys_in(&host::scope("h1")), [key]);
}
