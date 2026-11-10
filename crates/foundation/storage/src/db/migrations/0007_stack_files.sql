-- A Managed Stack is made of one or more compose files, merged in order
-- like `docker compose -f a.yaml -f b.yaml`. Each existing Stack's single
-- definition becomes its compose.yaml.
CREATE TABLE stack_files (
    stack_id TEXT NOT NULL REFERENCES stacks (id) ON DELETE CASCADE,
    position INTEGER NOT NULL,
    name     TEXT NOT NULL,
    content  TEXT NOT NULL,
    PRIMARY KEY (stack_id, position),
    UNIQUE (stack_id, name)
) STRICT;

INSERT INTO stack_files (stack_id, position, name, content)
SELECT id, 0, 'compose.yaml', compose FROM stacks;

ALTER TABLE stacks DROP COLUMN compose;
