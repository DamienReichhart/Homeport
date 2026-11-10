-- What each deploy of a Managed or Linked Stack used, for its history and
-- rolling back. The environment is sealed like the Stack's own (ADR 0002).
-- Forgetting the Stack deletes its rows; deleting the Host does too.
CREATE TABLE stack_deploys (
    id          TEXT PRIMARY KEY NOT NULL,
    host_id     TEXT NOT NULL REFERENCES hosts (id) ON DELETE CASCADE,
    stack_id    TEXT NOT NULL, -- of `stacks` or `linked_stacks`, by kind
    stack_kind  TEXT NOT NULL, -- 'managed' or 'linked'
    stack_name  TEXT NOT NULL,
    action      TEXT NOT NULL, -- the compose action that deployed it
    definition  TEXT NOT NULL, -- JSON array of {"name", "content"}, in merge order
    env         BLOB,          -- .env text sealed by crypto::SecretBox; NULL when empty
    profiles    TEXT NOT NULL DEFAULT '[]',
    started_at  TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    finished_at TEXT,          -- NULL while running, or when the run was cancelled
    success     INTEGER,
    exit_code   INTEGER
) STRICT;

CREATE INDEX stack_deploys_by_stack ON stack_deploys (stack_id, started_at);
