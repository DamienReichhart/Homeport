-- The Compose profiles a Managed or Linked Stack deploys, as a JSON array of
-- names. Every existing Stack deploys only its services without a profile,
-- as before.
ALTER TABLE stacks ADD COLUMN profiles TEXT NOT NULL DEFAULT '[]';
ALTER TABLE linked_stacks ADD COLUMN profiles TEXT NOT NULL DEFAULT '[]';
