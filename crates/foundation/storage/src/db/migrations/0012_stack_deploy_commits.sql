-- The commit each deploy of a Synced Stack used (ADR 0014); NULL for
-- Managed and Linked Stacks, and for deploys recorded before 0.11.
ALTER TABLE stack_deploys ADD COLUMN commit_hash TEXT;
