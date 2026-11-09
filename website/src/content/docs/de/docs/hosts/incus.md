---
title: Incus
description: Mit Incus auf diesem Rechner, über SSH oder über HTTPS mit einem Trust-Token verbinden und das Projekt wählen, das seine Seiten zeigen.
---

**Incus** führt Systemcontainer und virtuelle Maschinen aus, die **Instanzen** aus [der Anleitung](/docs/guides/instances/). Homeport verbindet sich damit über seine eigene REST-API, wie [ADR 0021](https://github.com/DamienReichhart/Homeport/blob/main/docs/adr/0021-incus-over-its-rest-api.md) festlegt. Ein Incus-Host erhält eigene Seiten in der Seitenleiste: **Instanzen**, **Images**, **Profile**, **Speicher** und **Netzwerke**. Die Seiten von Docker, Stacks und das Board werden dafür nicht angeboten.

## Einen Incus-Host hinzufügen

Wählen Sie **Incus** im Feld **Engine** des Formulars und dann, wie er erreicht wird:

| Verbindung         | Wann                                                          | Was sie braucht                                                        |
| ------------------ | ------------------------------------------------------------- | ---------------------------------------------------------------------- |
| **Lokaler Socket** | Incus läuft auf diesem Computer (Linux)                       | Ihren Benutzer in der Gruppe `incus-admin` oder `incus` (siehe unten)  |
| **SSH**            | Incus läuft auf einem Server, an dem Sie sich anmelden können | den SSH-Benutzer in der Gruppe `incus-admin` oder `incus`              |
| **TLS** (HTTPS)    | Incus lauscht im Netzwerk, oder SSH ist nicht erlaubt         | ein Trust-Token oder ein Client-Zertifikat, dem Incus bereits vertraut |

Einfaches **TCP** ist ausgegraut: Incus bietet seine API nicht ohne TLS an. Homeport verbindet sich mit **Incus 6.0** (dem Release mit Langzeitunterstützung) und neuer.

### Seine Sockets

| Socket                            | Wer ihn verwenden darf            | Was er erreicht                                                          |
| --------------------------------- | --------------------------------- | ------------------------------------------------------------------------ |
| `/var/lib/incus/unix.socket`      | die Gruppe `incus-admin`          | alles: jedes Projekt, jeden Speicherpool und jedes Netzwerk              |
| `/var/lib/incus/unix.socket.user` | die Gruppe `incus` (`incus-user`) | nur das eigene Projekt des Benutzers, bei der ersten Verwendung erstellt |

Unter Linux listet **Auf diesem Rechner gefunden** beide auf, wenn sie antworten, den eingeschränkten als **Incus (restricted to your project)**. Ein Socket, den Sie noch nicht verwenden dürfen, wird mit dem Befehl aufgelistet, der das behebt:

```bash
sudo usermod -aG incus-admin "$USER"
```

Gruppenänderungen gelten für neue Anmeldungen: Melden Sie sich ab und wieder an (oder verbinden Sie sich über SSH neu), bevor Sie den Host hinzufügen. Ein gestopptes Incus nennt die Units, die es starten:

```bash
sudo systemctl enable --now incus.socket incus.service
```

### Über HTTPS mit einem Trust-Token

Incus lauscht im Netzwerk, sobald `core.https_address` gesetzt ist. Auf dem Incus-Server:

```bash
incus config set core.https_address :8443
incus config trust add homeport
```

Der zweite Befehl gibt ein **Trust-Token** aus: eine Zeile aus Buchstaben und Ziffern. Wählen Sie in Homeport **Incus** und **TLS**, behalten Sie **Trust-Token** bei und fügen Sie es ein. Homeport zeigt, was es liest (den Namen, für den es gilt, die Adressen des Servers und den Fingerabdruck seines Zertifikats sowie, wann es abläuft), und dann bei **Hinzufügen**:

1. erstellt es sein eigenes Client-Zertifikat, einen im Arbeitsspeicher erzeugten ECDSA-P-384-Schlüssel;
2. probiert es die Adressen des Tokens der Reihe nach und behält die erste, deren Zertifikat den vom Token genannten Fingerabdruck hat; jede andere wird abgelehnt, mit beiden Fingerabdrücken;
3. sendet es das Token mit seinem Zertifikat und prüft, dass Incus ihm nun vertraut.

Der Host erscheint dann in `incus config trust list` unter dem Namen des Tokens. Ein Token funktioniert einmal und läuft ab; ein verwendetes, widerrufenes oder abgelaufenes wird mit dem Befehl abgelehnt, der ein neues erstellt. Um Homeport später den Zugriff zu entziehen, entfernen Sie dort sein Zertifikat:

```bash
incus config trust list
incus config trust remove <fingerprint>
```

**Vorhandenes Zertifikat** nimmt ein Client-Zertifikat und einen Schlüssel, denen Incus bereits vertraut (`incus config trust add-certificate`), mit dem eigenen Zertifikat von Incus, dem vertraut werden soll (`/var/lib/incus/server.crt` oder in einem Cluster `cluster.crt`).

### Das Zertifikat des Servers ist fixiert

Das Zertifikat von Incus ist selbstsigniert, daher gibt es keine CA, gegen die es geprüft werden könnte. Homeport bewahrt das Zertifikat, dem es vertraut hat, beim Host auf und akzeptiert danach genau dieses, verglichen über seinen SHA-256-Fingerabdruck, unter welcher Adresse es auch erreicht wird. Wird Incus neu installiert oder sein Zertifikat erneuert, wird der Host mit dem erwarteten und dem erhaltenen Fingerabdruck abgelehnt: Prüfen Sie den neuen auf dem Server mit `incus info` und fügen Sie den Host dann mit einem neuen Token erneut hinzu. Der Client-Schlüssel und beide Zertifikate werden wie jedes andere Geheimnis verschlüsselt gespeichert; siehe [Sicherheit](/docs/reference/security/#incus-zertifikate).

## Projekte

Incus bewahrt Instanzen, Images, Profile, benutzerdefinierte Volumes und Netzwerke in **Projekten** auf. Die Seiten zeigen jeweils ein Projekt, gewählt unter dem Host-Umschalter, der jedes mit der Anzahl seiner Instanzen und Images auflistet. Homeport merkt sich die Wahl für jeden Host zwischen den Starts. Bis Sie eines wählen, zeigt ein Host `default` oder auf dem eingeschränkten Socket sein einziges Projekt. Speicherpools gehören dem ganzen Server; ihre Volumes werden für das ausgewählte Projekt angezeigt.

## Ereignisse

Die Seiten aktualisieren sich anhand der Lebenszyklus-Ereignisse von Incus, in jedem Projekt: eine Instanz, die startet oder stoppt, ein erstellter Snapshot, ein hinzugefügtes Image, ein geändertes Profil oder Netzwerk. Sie werden auch auf der Seite **Ereignisse** aufbewahrt. Incus bewahrt keinen Verlauf davon auf, daher wird nicht nachgetragen, was geschah, während Homeport geschlossen war.

Solange die Verbindung besteht, fragt Homeport Incus alle 15 Sekunden mit `GET /1.0` ab. Ein gestopptes Incus macht den Host **Nicht erreichbar**, und Homeport verbindet sich von selbst neu, wenn es wieder antwortet.

## Was Homeport auf dem Host tut

Auf einem Incus-Host läuft nichts außer Incus selbst: Konsolen und Terminals laufen über die eigenen WebSockets von Incus, und entfernte Images werden von Incus heruntergeladen. Homeport liest die Image-Liste eines entfernten Image-Servers von diesem Computer aus, für die Image-Auswahl.
