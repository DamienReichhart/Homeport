-- SSH Hosts gain an ordered list of bastions (ADR 0012); every existing
-- one connects directly, so it starts empty. Its trusted key stays in host_key.
UPDATE hosts
SET connection = json_set(connection, '$.bastions', json('[]'))
WHERE json_extract(connection, '$.kind') = 'ssh'
  AND json_type(connection, '$.bastions') IS NULL;
