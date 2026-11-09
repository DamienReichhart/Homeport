---
title: Lokale Engines
description: Mit Docker Engine, Docker Desktop, Colima, OrbStack, Rancher Desktop oder Podman auf diesem Rechner verbinden.
---

Ein Host mit **Lokaler Socket** spricht mit einer Engine auf diesem Rechner über ihren Unix-Socket oder unter Windows über eine Named Pipe. Er braucht keine Anmeldedaten und keinen Schlüsselbund.

## Automatische Erkennung

Homeport sucht an den üblichen Orten nach laufenden Engines und listet die, die antworten, unter **Auf diesem Rechner gefunden** auf, auf dem Willkommensbildschirm und auf der Seite **Host hinzufügen**. Wählen Sie daneben **Hinzufügen**, um einen nach ihr benannten Host zu erstellen.

| Laufzeitumgebung         | macOS und Linux                                                                                                      | Windows                           |
| ------------------------ | -------------------------------------------------------------------------------------------------------------------- | --------------------------------- |
| Docker Engine            | `/var/run/docker.sock`                                                                                               | —                                 |
| Docker Engine (rootless) | `$XDG_RUNTIME_DIR/docker.sock`                                                                                       | —                                 |
| Docker Desktop           | `~/.docker/desktop/docker.sock`, `~/.docker/run/docker.sock`                                                         | `//./pipe/docker_engine`          |
| Colima                   | `~/.colima/default/docker.sock`                                                                                      | —                                 |
| OrbStack                 | `~/.orbstack/run/docker.sock`                                                                                        | —                                 |
| Rancher Desktop          | `~/.rd/docker.sock`                                                                                                  | —                                 |
| Podman                   | `$XDG_RUNTIME_DIR/podman/podman.sock`, `/run/podman/podman.sock`; [Maschinen unter macOS](#podman-machines-on-macos) | `//./pipe/podman-machine-default` |

Jeder Kandidat wird geprüft, indem die Engine nach ihrer Version gefragt wird, sodass nur laufende Engines aufgelistet werden. Führen mehrere Pfade zum selben Socket – unter macOS verweist `/var/run/docker.sock` meist auf Docker Desktop, OrbStack oder Rancher Desktop –, wird die Engine einmal aufgelistet, unter dem zuerst gefundenen Namen.

### Podman-Maschinen unter macOS

Unter macOS führt Podman seine Engine in einer VM aus (`podman machine`), deren Socket sich mit jeder Maschine ändert. Homeport fragt die `podman`-CLI nach dem Socket jeder laufenden Maschine (`podman machine list` und `podman machine inspect`) und listet jede als **Podman (_Maschinenname_)** auf, zum Beispiel **Podman (podman-machine-default)**. Gestoppte Maschinen werden nicht aufgelistet; starten Sie eine mit `podman machine start` und öffnen Sie **Host hinzufügen** erneut.

Wenn Homeport `podman` nicht ausführen kann – es ist weder in seinem PATH noch in `/opt/homebrew/bin` oder `/usr/local/bin` –, sucht es stattdessen dort, wo Podman die Sockets ablegt: `$TMPDIR/podman/<machine>-api.sock` für Podman 5 und `~/.local/share/containers/podman/machine/…/podman.sock` für Podman 4. Wenn `podman-mac-helper` `/var/run/docker.sock` mit einer Maschine verknüpft hat, wird diese Maschine einmal aufgelistet, unter ihrem Podman-Namen.

## Einen Socket von Hand hinzufügen

Wird Ihre Engine nicht erkannt – ein anderes Colima-Profil als `default`, eine Podman-Maschine unter Windows mit einem anderen Namen als dem Standard oder ein eigener Pfad –, fügen Sie sie selbst hinzu:

1. Wählen Sie **Host hinzufügen…** im Host-Umschalter.
2. Wählen Sie **Lokaler Socket**.
3. Geben Sie unter **Name** einen Namen und unter **Socket-Pfad** den Pfad ein: einen Unix-Socket wie `/run/user/1000/docker.sock` oder unter Windows eine Named Pipe wie `//./pipe/docker_engine`.
4. Wählen Sie **Host hinzufügen**.

Um den Socket zu finden, den Ihre Docker CLI verwendet, führen Sie Folgendes aus:

```sh
docker context inspect --format '{{.Endpoints.docker.Host}}'
```

Für eine Podman-Maschine:

```sh
podman machine inspect --format '{{.ConnectionInfo.PodmanSocket.Path}}'
```

## Berechtigungen unter Linux

Der Standard-Socket `/var/run/docker.sock` gehört der Gruppe `docker`. Ist Ihr Benutzer nicht darin, zeigt der Host **Nicht erreichbar** mit _Permission denied_. Fügen Sie sich hinzu und melden Sie sich erneut an:

```sh
sudo usermod -aG docker $USER
```

Die Mitgliedschaft in der Gruppe `docker` ist auf diesem Rechner gleichbedeutend mit root; rootless Docker oder Podman vermeiden das, falls es Ihnen wichtig ist.
