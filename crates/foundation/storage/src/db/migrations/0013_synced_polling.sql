-- Polling Synced Stacks while Homeport runs, and deploying new commits
-- automatically when asked (ADR 0014). Nothing polls while Homeport is closed.
ALTER TABLE synced_stacks ADD COLUMN poll INTEGER NOT NULL DEFAULT 1;
ALTER TABLE synced_stacks ADD COLUMN auto_deploy INTEGER NOT NULL DEFAULT 0;
ALTER TABLE synced_stacks ADD COLUMN last_polled_at TEXT;
-- The AppError of the last failed poll, as JSON; NULL once a poll succeeds.
ALTER TABLE synced_stacks ADD COLUMN last_poll_error TEXT;
-- The fetched commit changes the compose file's folder since the deployed one.
ALTER TABLE synced_stacks ADD COLUMN update_available INTEGER NOT NULL DEFAULT 0;
-- The commit whose automatic deploy failed, so it isn't tried again, and why.
ALTER TABLE synced_stacks ADD COLUMN auto_deploy_failed_commit TEXT;
ALTER TABLE synced_stacks ADD COLUMN auto_deploy_error TEXT;
-- Whether a deploy was started by auto-deploy rather than by the user.
ALTER TABLE stack_deploys ADD COLUMN automatic INTEGER NOT NULL DEFAULT 0;
