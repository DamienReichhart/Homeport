# Stack deploys pull their images through the Engine API first

Homeport stores registry logins and hands them to the Engine for every pull it makes through the Engine API (pull dialog, Run dialog). Managed Stacks deploy with the `docker compose` CLI instead (ADR 0001), a separate program that only knows the logins in the user's own `~/.docker/config.json`, and logins must never be written to disk in clear (ADR 0002). So before `up` and in place of `compose pull`, Homeport reads the Stack's services from `docker compose config --format json` and pulls their images itself, through the Host's existing connection, with the stored login of each image's registry. `up` then finds every image already on the Host and pulls nothing; when a service asks for `pull_policy: always`, Homeport pulls it and runs `up --pull never`, so the CLI doesn't pull it again with no login. The user's own `docker login` keeps working: an image Homeport has no login for is pulled anonymously first, and if the registry refuses it, compose is never asked to try.

## Considered Options

- **A credential helper Homeport serves**: a temporary `DOCKER_CONFIG` holding only `{"credsStore": "homeport"}` and a `docker-credential-homeport` program that asks Homeport over a private 0600 socket, like the compose proxy. It would also cover the base images of `build:`. Rejected for now: Homeport would have to ship and find a second executable on three systems, keep the user's `cli-plugins` folder visible through `cliPluginsExtraDirs`, merge the user's own logins and helpers into the temporary config, and keep the helper working from a GUI launch on macOS and Windows. All of that for builds, which Managed Stacks refuse anyway.
- **Leave it to the user's `docker login`**: nothing to build, but stored logins would work everywhere except Stacks, the place most long-running services come from.

## Consequences

- Nothing secret reaches the CLI: its environment and the files Homeport creates for the run hold no login. The pulls show in the deploy dialog's output, naming the login used ("as alice on ghcr.io"), never the secret.
- **Pull** on a Managed Stack no longer runs `compose pull`: it pulls each service's image through the Engine API, which is what it did anyway, with the stored logins.
- `config --format json` runs the CLI once more per deploy, locally and without reaching the Host.
- `up --pull never` needs a Compose v2 recent enough to have `--pull` on `up`; it's passed only when a service uses `pull_policy: always`.
- A service whose image only exists on the Host (built there by hand, for example) isn't pulled when the Host has it, as compose's default `missing` policy does.
- Linked Stacks (ADR 0008) allow `build:`, whose `FROM` images are pulled by the CLI or BuildKit with the user's own logins. When they arrive, either their `FROM` images are pre-pulled the same way (the classic builder then finds them), or this decision is revisited for the credential helper.
