-- Each Host's Engine events, kept across restarts for the Events page.
-- Removing a Host removes its events. An event is identified by when it
-- happened (to the nanosecond), what it is, and what it happened to; the
-- unique index also serves listing a Host's events by time.
CREATE TABLE engine_events (
    host_id    TEXT NOT NULL REFERENCES hosts (id) ON DELETE CASCADE,
    time_nano  INTEGER NOT NULL,
    kind       TEXT NOT NULL,
    action     TEXT NOT NULL,
    actor_id   TEXT NOT NULL,
    attributes TEXT NOT NULL,   -- JSON object of the actor's attributes
    UNIQUE (host_id, time_nano, kind, action, actor_id)
) STRICT;
