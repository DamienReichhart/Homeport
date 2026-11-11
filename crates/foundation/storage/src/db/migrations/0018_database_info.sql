-- What this database asks of the Homeport that opens it.
--
-- `requires` is the oldest Homeport able to open the file. Homeport sets it
-- each time it opens the database, so that an older Homeport which finds a
-- schema newer than it knows can name the version to install instead of
-- failing on it. Older versions read this table, so it never changes shape.
CREATE TABLE database_info (
    key   TEXT PRIMARY KEY NOT NULL,
    value TEXT NOT NULL
) STRICT;
