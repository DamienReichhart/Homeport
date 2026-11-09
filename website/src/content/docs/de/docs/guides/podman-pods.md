---
title: Podman-Pods
description: Podman-Pods auflisten, starten, stoppen und entfernen, einen mit seinen veröffentlichten Ports erstellen und Container darin ausführen.
---

Ein **Pod** ist Podmans Art, Container zu gruppieren: Sie teilen sich ein Netzwerk (und ihren Hostnamen und IPC), erreichen sich also gegenseitig über `localhost`, und sie starten und stoppen gemeinsam. Jeder Pod hat einen **Infra-Container**, der das Geteilte und die Ports hält, die der Pod veröffentlicht. Homeport zeigt Pods auf Hosts, deren Engine Podman ist, über Podmans eigene API auf dem Socket, den der Host ohnehin verwendet; auf dem Host wird nichts installiert.

Die Seite **Pods** erscheint in der Seitenleiste nur, solange ein Podman-Host ausgewählt ist. Auf jeder anderen Docker-kompatiblen Engine fehlt sie, und ein Link darauf sagt, warum. Unter [Engine-Arten](/docs/hosts/#engine-kinds) steht, wie Homeport Podman erkennt.

## Die Seite Pods

| Spalte        | Zeigt                                                                                                                         |
| ------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| **Name**      | Den Namen des Pods, mit seinem Status als Punkt.                                                                              |
| **Status**    | **Läuft**, **Läuft teilweise** (einige Container gestoppt), **Pausiert**, **Gestoppt**, **Erstellt** oder **Fehlgeschlagen**. |
| **Container** | Wie viele seiner eigenen Container laufen, ohne den Infra-Container.                                                          |
| **Netzwerke** | Die Netzwerke, in denen der Pod ist.                                                                                          |
| **Erstellt**  | Wann er erstellt wurde.                                                                                                       |

Jede Zeile hat die Aktion, die zu ihrem Status passt – **Starten** für einen gestoppten Pod, **Stoppen** für einen laufenden – und **Entfernen**. Wählen Sie einen Pod aus, um seine Seite zu öffnen. Die Liste aktualisiert sich von selbst, wenn die Container des Pods starten und stoppen, ob aus Homeport oder aus einem Terminal.

Auf der Seite **Container** zeigt ein Container, der in einem Pod läuft, den Namen des Pods neben seinem eigenen; wählen Sie den Namen aus, um den Pod zu öffnen.

## Die Seite eines Pods

Die Kopfzeile hat **Container ausführen** und die Aktionen des Pods: **Starten**, **Stoppen**, **Neu starten**, **Pausieren**, **Fortsetzen** und **Entfernen**, wie es sein Status zulässt. Jede wirkt auf alle Container des Pods auf einmal, wie es `podman pod start` und seine Verwandten tun.

- **Container** listet die Container des Pods auf, seinen Infra-Container zuletzt. Wählen Sie einen aus, um seine Container-Seite mit Logs, Terminal und Dateien zu öffnen.
- **Einstellungen** zeigt die Ports, die der Pod veröffentlicht, die Namespaces, die sich seine Container teilen, seine Netzwerke, seinen Hostnamen und seine Labels sowie den Befehl `podman pod create`, mit dem er erstellt wurde, wenn Podman ihn aufbewahrt hat.

## Einen Pod entfernen

**Entfernen** fragt zuerst nach und sagt, wie viele Container mit dem Pod gehen. Ein laufender Pod wird zuerst gestoppt: Die Schaltfläche lautet dann **Stoppen und entfernen**. Die Volumes des Pods bleiben erhalten.

## Einen Pod erstellen

**Neuer Pod** (oder `n` auf der Seite Pods oder **Neuer Pod** in der Befehlspalette) fragt nach:

- einem **Name**: Buchstaben, Ziffern, `_`, `.` und `-`, beginnend mit einem Buchstaben oder einer Ziffer;
- **Ports**, die veröffentlicht werden, Host-Port zu Container-Port, wie im Dialog zum Ausführen. Lassen Sie den Host-Port leer, damit Podman einen freien wählt;
- **Labels**, ein `key=value` pro Zeile.

Die Ports gehören dem Pod: Sie werden an seinem Infra-Container veröffentlicht und erreichen den Container im Pod, der auf diesem Port lauscht. Die Seite des neuen Pods öffnet sich, bereit, Container darin auszuführen. Ein Pod hat keine eigenen Container, bis Sie einen ausführen; Podman startet ihn mit dem ersten.

## Einen Container in einem Pod ausführen

Auf einem Podman-Host hat der Dialog zum **Ausführen** ein Feld **Pod**, und die Schaltfläche **Container ausführen** eines Pods öffnet den Dialog mit diesem Pod vorausgewählt. Ein Container in einem Pod:

- teilt sich das Netzwerk des Pods, kann also keine Ports veröffentlichen und keinem anderen Netzwerk beitreten: Der Dialog blendet diese Felder aus, sobald ein Pod gewählt ist;
- verwendet den Hostnamen und die DNS-Einstellungen des Pods, daher lehnt Homeport eigene ab;
- behält alles andere, was der Dialog zum Ausführen bietet: Befehl, Umgebung, Mounts, Neustartrichtlinie, Limits, Capabilities, Geräte, Healthcheck und Logging.

Duplizieren und Bearbeiten verschieben einen Container nie in einen Pod: Podman kann einen bestehenden Container nicht in einen verschieben.

## Was Homeport braucht

Pods funktionieren mit Podman 4.0 und neuer, über jede Art von Verbindung – lokaler Socket, SSH, TLS oder TCP –, da sie über denselben Socket laufen wie alles andere.
