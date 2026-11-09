---
title: Engine locali
description: Connettiti a Docker Engine, Docker Desktop, Colima, OrbStack, Rancher Desktop o Podman su questo computer.
---

Un host **Socket locale** comunica con un Engine su questo computer tramite il suo socket Unix, o una named pipe su Windows. Non richiede credenziali né portachiavi.

## Rilevamento automatico

Homeport cerca gli Engine in esecuzione nelle posizioni abituali ed elenca quelli che rispondono sotto **Trovati su questo computer**, nella schermata di benvenuto e nella pagina **Aggiungi host**. Scegli **Aggiungi** accanto a uno per creare un host con il suo nome.

| Runtime                  | macOS e Linux                                                                                                    | Windows                           |
| ------------------------ | ---------------------------------------------------------------------------------------------------------------- | --------------------------------- |
| Docker Engine            | `/var/run/docker.sock`                                                                                           | —                                 |
| Docker Engine (rootless) | `$XDG_RUNTIME_DIR/docker.sock`                                                                                   | —                                 |
| Docker Desktop           | `~/.docker/desktop/docker.sock`, `~/.docker/run/docker.sock`                                                     | `//./pipe/docker_engine`          |
| Colima                   | `~/.colima/default/docker.sock`                                                                                  | —                                 |
| OrbStack                 | `~/.orbstack/run/docker.sock`                                                                                    | —                                 |
| Rancher Desktop          | `~/.rd/docker.sock`                                                                                              | —                                 |
| Podman                   | `$XDG_RUNTIME_DIR/podman/podman.sock`, `/run/podman/podman.sock`; [macchine su macOS](#podman-machines-on-macos) | `//./pipe/podman-machine-default` |

Ogni candidato viene verificato chiedendo all’Engine la sua versione, quindi vengono elencati solo gli Engine in esecuzione. Quando più percorsi portano allo stesso socket — su macOS, `/var/run/docker.sock` di solito punta a Docker Desktop, OrbStack o Rancher Desktop — l’Engine viene elencato una sola volta, con il primo nome trovato.

### Macchine Podman su macOS

Su macOS, Podman esegue il suo Engine in una VM (`podman machine`) il cui socket cambia con ogni macchina. Homeport chiede alla CLI `podman` il socket di ogni macchina in esecuzione (`podman machine list` e `podman machine inspect`) ed elenca ciascuna come **Podman (_nome della macchina_)**, per esempio **Podman (podman-machine-default)**. Le macchine arrestate non vengono elencate; avviane una con `podman machine start` e apri di nuovo **Aggiungi host**.

Se Homeport non riesce a eseguire `podman` — non è nel suo PATH né in `/opt/homebrew/bin` o `/usr/local/bin` — cerca invece dove Podman mette i socket: `$TMPDIR/podman/<machine>-api.sock` per Podman 5, e `~/.local/share/containers/podman/machine/…/podman.sock` per Podman 4. Quando `podman-mac-helper` ha collegato `/var/run/docker.sock` a una macchina, quella macchina viene elencata una sola volta, con il suo nome Podman.

## Aggiungere un socket a mano

Se il tuo Engine non viene rilevato — un profilo Colima diverso da `default`, una macchina Podman su Windows con un nome diverso da quello predefinito, o un percorso personalizzato — aggiungilo tu:

1. Scegli **Aggiungi host…** nel selettore degli host.
2. Seleziona **Socket locale**.
3. Inserisci un **Nome** e il **Percorso del socket**: un socket Unix come `/run/user/1000/docker.sock`, o una named pipe come `//./pipe/docker_engine` su Windows.
4. Scegli **Aggiungi host**.

Per trovare il socket usato dalla tua Docker CLI, esegui:

```sh
docker context inspect --format '{{.Endpoints.docker.Host}}'
```

Per una macchina Podman:

```sh
podman machine inspect --format '{{.ConnectionInfo.PodmanSocket.Path}}'
```

## Permessi su Linux

Il socket predefinito `/var/run/docker.sock` appartiene al gruppo `docker`. Se il tuo utente non ne fa parte, l’host mostra **Non raggiungibile** con _Permission denied_. Aggiungiti e accedi di nuovo:

```sh
sudo usermod -aG docker $USER
```

Far parte del gruppo `docker` equivale ad avere i privilegi di root su quel computer; Docker rootless o Podman lo evitano, se per te è importante.
