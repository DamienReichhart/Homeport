-- Every Host gains the kind of Engine it points to. Until now every Host was
-- a Docker Engine (or Podman through its Docker-compatible API), so stored
-- ones become Docker-compatible and nothing else about them changes.
ALTER TABLE hosts ADD COLUMN engine TEXT NOT NULL DEFAULT 'docker'
    CHECK (engine IN ('docker', 'containerd', 'incus'));
