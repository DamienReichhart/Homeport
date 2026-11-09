---
title: Quadlet-Units
description: Die Quadlet-Units eines Podman-Hosts mit ihrem systemd-Status auflisten, ihre Dateien lesen, sie installieren, bearbeiten und entfernen und ihre Services starten oder stoppen.
---

**Quadlet** ist die Art, wie Podman langlebige Services unter systemd ausführt. Eine Quadlet-Unit ist eine Datei – `web.container`, `shop.pod`, `cache.volume`, `lan.network`, `app.kube`, `base.image`, `api.build` –, die der Generator von systemd in einen Service umwandelt, `web.service`, der den Container beim Booten startet und neu startet, wenn er stoppt. Rootful Podman hält seine Units in `/etc/containers/systemd/`; rootless Podman hält die jedes Benutzers in `~/.config/containers/systemd/`.

Die Seite **Quadlet** erscheint in der Seitenleiste, solange ein Podman-Host ausgewählt ist. Sie läuft über Podmans eigene API auf dem Socket, den der Host ohnehin verwendet, und zum Starten und Stoppen über `systemctl`, wie [ADR 0019](https://github.com/DamienReichhart/Homeport/blob/main/docs/adr/0019-quadlet-units-through-podmans-api-and-systemctl.md) es festlegt.

## Was jedes Podman und jede Verbindung erlaubt

Podmans API hat die Quadlet-Unterstützung über zwei Releases hinweg erhalten, und sie startet oder stoppt nie den Service einer Unit. Was Homeport tun kann, hängt daher vom Podman des Hosts ab und davon, wie Homeport ihn erreicht:

| Aktion                                      | Podman            | Verbindung                              |
| ------------------------------------------- | ----------------- | --------------------------------------- |
| Units mit ihrem Status auflisten            | 5.7 und neuer     | jede                                    |
| Units über ihre Container auflisten         | 4.0 bis 5.6       | jede                                    |
| Die Datei einer Unit lesen                  | 5.8 und neuer     | jede                                    |
| Units installieren, bearbeiten, entfernen   | 5.8 und neuer     | jede                                    |
| Einen Service starten, stoppen, neu starten | jedes mit Quadlet | SSH oder ein lokaler Socket unter Linux |

Bei Podman vor 5.7 werden Units über das Label `PODMAN_SYSTEMD_UNIT` gefunden, das Podman an die von ihnen erstellten Container setzt; eine Unit erscheint daher erst, wenn sie einen Container erstellt hat, benannt nach ihrem Service. Die Seite sagt das über der Liste.

Starten und Stoppen brauchen `systemctl`, das Homeport dort ausführt, wo Podman läuft: über die SSH-Sitzung eines SSH-Hosts oder auf diesem Computer bei einem lokalen Socket unter Linux. Ein TCP- oder TLS-Host oder ein lokaler Socket unter macOS oder Windows (wo Podman in einer virtuellen Maschine läuft) kann alles andere; die Schaltflächen sind mit dem Grund deaktiviert.

## Die Seite Quadlet

| Spalte        | Zeigt                                                                                                                                                 |
| ------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Unit**      | Den Dateinamen der Unit und die Anwendung, mit der sie installiert wurde.                                                                             |
| **Art**       | `container`, `pod`, `volume`, `network`, `kube`, `image`, `build` oder `artifact`.                                                                    |
| **Service**   | Den systemd-Service, den sie erzeugt, oder **noch nicht erzeugt**, bis systemd sie geladen hat.                                                       |
| **Status**    | **Läuft**, **Wird gestartet**, **Wird gestoppt**, **Gestoppt**, **Fehlgeschlagen**, **Nicht von systemd geladen** oder **Vorlage**, wie systemd sagt. |
| **Container** | Wie viele Container sie erstellt hat.                                                                                                                 |

Jede Zeile hat **Starten** für eine gestoppte Unit oder **Stoppen** für eine laufende. Wählen Sie eine Unit aus, um ihre Seite zu öffnen. Die Liste aktualisiert sich von selbst, wenn ihre Container starten und stoppen.

Eine Unit mit **Nicht von systemd geladen** ist installiert, aber der Generator von systemd hat sie übersprungen: meist wegen eines Fehlers in der Datei. Auf dem Host sagen dann `systemctl --user daemon-reload` (oder `systemctl daemon-reload` bei rootful Podman) und `journalctl`, warum.

Auf der Seite **Container** zeigt ein Container, den eine Unit erstellt hat, den Service der Unit neben seinem Namen; wählen Sie ihn aus, um die Unit zu öffnen.

## Die Seite einer Unit

- Die Kopfzeile hat **Starten**, **Stoppen** und **Neu starten**, **Bearbeiten** und **Entfernen**.
- **Service** zeigt die systemd-Unit, ob sie eine System-Unit oder eine Benutzer-Unit (`systemctl --user`) ist, wo ihre Datei auf dem Host liegt und ihre Anwendung und ihren Pod, wenn sie welche hat.
- **Unit-Datei** zeigt die Datei, wie Podman sie liest, mit Hervorhebung.
- **Container** listet die Container auf, die sie erstellt hat; jeder öffnet seine Container-Seite.

Wer den Container einer Unit von der Seite Container aus stoppt, kämpft gegen systemd, das ihn unter `Restart=` wieder startet. Stoppen Sie stattdessen die Unit.

## Eine Unit installieren

**Neue Unit** (oder `n` auf der Seite Quadlet oder **Neue Unit** in der Befehlspalette) öffnet einen Editor mit einem Ausgangspunkt für die gewählte Art oder mit einer Datei, die Sie mit **Datei öffnen…** von diesem Computer öffnen. Benennen Sie sie mit ihrer Endung, etwa `web.container`; die Datei braucht den Abschnitt ihrer Art, `[Container]` für eine `.container`.

**Dateien hinzufügen…** legt weitere Dateien neben die Unit – ein `Containerfile` für eine `.build`, das YAML einer `.kube`, eine Env-Datei – in einen nach der Unit benannten Anwendungsordner, wie es `podman quadlet install` tut.

Podman schreibt die Dateien dorthin, wo die Units seines Benutzers liegen, und lädt systemd neu, sodass der Service sofort existiert. Das Installieren startet ihn nicht: Wählen Sie **Starten**, sobald er da ist. Eine Unit, deren Name vergeben ist, wird abgelehnt; bearbeiten Sie stattdessen diese.

## Bearbeiten und Entfernen

**Bearbeiten** öffnet die Datei der Unit. **Speichern** schreibt sie an Ort und Stelle und lädt systemd neu; die daneben installierten Dateien bleiben. Ein laufender Service behält seine alten Einstellungen, bis Sie ihn mit **Neu starten** neu starten.

**Entfernen** löscht die Datei der Unit und lädt systemd neu. **Zuerst ihren laufenden Service stoppen** ist standardmäßig eingeschaltet; ohne diese Option lehnt Podman es ab, eine laufende Unit zu entfernen. Container, Volumes und Images, die die Unit erstellt hat, bleiben erhalten.

## Was Homeport auf dem Host ausführt

Für **Starten**, **Stoppen** und **Neu starten**, und für nichts anderes, führt Homeport einen Befehl auf dem Server des Hosts über SSH aus (oder auf diesem Computer bei einem lokalen Socket), mit den Rechten des SSH-Benutzers:

```bash
systemctl --user start 'web.service'
```

`--user` steht dort für die Units eines rootless Podman und fehlt bei denen eines rootful Podman; `start` wird für diese Aktionen zu `stop` oder `restart`. Homeport prüft den Namen des Service und setzt ihn in Anführungszeichen und verwendet nie `sudo`. Daher:

- **System-Units** (rootful Podman) brauchen einen SSH-Benutzer, der sie verwalten darf, etwa `root`.
- **Benutzer-Units** (rootless Podman) brauchen die systemd-Instanz des Benutzers. Damit sie ohne Anmeldesitzung läuft und die Units beim Booten starten, aktivieren Sie einmalig Lingering, als dieser Benutzer auf dem Host:

```bash
loginctl enable-linger "$USER"
```

Die [Sicherheitsseite](/docs/reference/security/#what-homeport-runs-on-your-hosts) listet jeden Befehl auf, den Homeport auf einem Host ausführt.
