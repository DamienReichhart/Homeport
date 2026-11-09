---
title: Einstellungen
description: Design und Sprache, der Infobereich, der Start bei der Anmeldung, Benachrichtigungen, Updates, Registry-Anmeldungen, Repositorys, wie oft synchronisierte Stacks geprüft werden, was dieser Rechner unterstützt, Version und Lizenzen.
---

**Einstellungen** ist der letzte Eintrag in der Seitenleiste.

## Darstellung

**Design** steht standardmäßig auf **Wie das System** und folgt dem hellen oder dunklen Modus Ihres Betriebssystems, wenn er sich ändert. Wählen Sie **Hell** oder **Dunkel**, um es festzulegen.

**Sprache** steht standardmäßig auf **Wie das System**: Homeport zeigt sich in der Sprache Ihres Betriebssystems, wenn es diese hat, und andernfalls auf Englisch. Die Auswahl nennt die gefundene Sprache, etwa **Wie das System (Deutsch)**. Wählen Sie eine Sprache, um stattdessen diese zu verwenden; das ganze Fenster wechselt sofort, ohne Neustart, und ebenso das Menü im Infobereich und die Benachrichtigungen.

Homeport spricht Englisch (English), Französisch (Français), Spanisch (Español), Deutsch, Italienisch (Italiano), brasilianisches Portugiesisch (Português), Russisch (Русский), vereinfachtes Chinesisch (简体中文), Japanisch (日本語), Koreanisch (한국어), Hindi (हिन्दी) und Arabisch (العربية), das es von rechts nach links anzeigt. Regionale Varianten folgen ihrer Sprache: Kanadisches Französisch zeigt Französisch, europäisches Portugiesisch zeigt brasilianisches Portugiesisch. Traditionelles Chinesisch ist noch nicht verfügbar und zeigt Englisch.

Datumsangaben, Zahlen und Größen folgen ebenfalls der Sprache, im regionalen Format Ihres Systems, wenn es eines dafür hat: _1.5 GB_ und _4 Nov 2026_ in britischem Englisch, _1,5 Go_ und _4 nov. 2026_ auf Französisch. Namen, IDs, Befehle, Logs und was Engines und Cluster sagen, bleiben, wie sie sind, und die **Details** eines Fehlers behalten den Originaltext des Systems oder der Bibliothek, die fehlgeschlagen ist, sodass sich ein Fehlerbericht in jeder Sprache gleich liest.

## Desktop

Wie Homeport auf diesem Computer läuft.

- **In den Infobereich schließen** – standardmäßig aus, sodass das Schließen des Fensters Homeport beendet. Schalten Sie es ein, um das Fenster stattdessen im [Infobereich](/docs/guides/overview/#infobereich) zu verbergen: Homeport läuft mit offenen Host-Verbindungen weiter. Holen Sie es über das Symbol im Infobereich zurück oder indem Sie Homeport erneut öffnen; beenden Sie es über **Beenden** im Infobereich. Auf einem Desktop ohne Infobereich ist der Schalter ausgegraut, mit einer Erklärung.
- **Bei der Anmeldung starten** – standardmäßig aus. Schalten Sie es ein, und Homeport startet, wenn Sie sich anmelden, und überwacht Ihre Hosts so von Anfang an. Der Schalter zeigt, was Ihr System eingetragen hat: Entfernen Sie Homeport selbst aus Ihren Anmeldeobjekten, erscheint er als aus. Unter Windows steht es in **Task-Manager → Autostart-Apps**, unter macOS in **Systemeinstellungen → Allgemein → Anmeldeobjekte**, unter Linux in `~/.config/autostart/`. Der Eintrag eines AppImage folgt der Datei, wenn Sie sie verschieben, ab dem nächsten Öffnen.
- **Verborgen im Infobereich starten** – standardmäßig aus und nur verfügbar, solange **Bei der Anmeldung starten** eingeschaltet ist. Wenn Homeport bei der Anmeldung startet, bleibt es im [Infobereich](/docs/guides/overview/#infobereich), ohne sein Fenster zu öffnen. Wenn Sie Homeport selbst öffnen, zeigt es immer das Fenster, und ohne Infobereich öffnet es sich immer.

## Benachrichtigungen

Homeport zeigt eine Benachrichtigung Ihres Betriebssystems, wenn auf einem Host etwas passiert, ob sein Fenster offen, minimiert oder [in den Infobereich geschlossen](#desktop) ist. Jede Art hat ihren eigenen Schalter, alle standardmäßig eingeschaltet, und ein Schalter wirkt sofort:

| Schalter                                                                | Beispiel                                                                                    |
| ----------------------------------------------------------------------- | ------------------------------------------------------------------------------------------- |
| **Ein Container mit einem Fehler stoppt oder keinen Speicher mehr hat** | _web-1 wurde auf prod-server gestoppt (Exit-Code 137)_, _db hat auf … keinen Speicher mehr_ |
| **Ein Container fehlerhaft wird**                                       | _api ist auf prod-server fehlerhaft_                                                        |
| **Ein verbundener Host nicht mehr erreichbar ist**                      | _prod-server ist nicht erreichbar: …_                                                       |
| **Ein nicht erreichbarer Host wieder da ist**                           | _prod-server ist wieder verbunden._                                                         |

Einen Container zu stoppen – mit **Stoppen** in Homeport, `docker stop`, `docker kill` oder `docker compose down` – ist kein Absturz und zeigt daher nichts; ebenso wenig ein Container, der mit Code 0 endet. Ein Host, der sich beim Start von Homeport nicht verbinden lässt, ist auch kein Ausfall: nur einer, der verbunden war. Ein Container, der beim Neustarten feststeckt, oder ein Host, der immer wieder abbricht, benachrichtigt höchstens einmal alle 5 Minuten.

Benachrichtigungen stammen aus den Verbindungen, die Homeport ohnehin hält, daher wird auf Ihren Hosts nichts installiert, und sie hören auf, wenn Homeport beendet wird. Ihr Betriebssystem entscheidet, wie sie aussehen und ob sie erscheinen: Prüfen Sie seine Benachrichtigungseinstellungen für Homeport, wenn keine erscheinen.

## Updates

- **Automatisch nach Updates suchen** – standardmäßig ein: Homeport sucht beim Start und alle 6 Stunden nach einer neuen Version und zeigt eine Leiste oben im Fenster, wenn es eine gibt.
- **Jetzt prüfen** – prüft sofort, auch wenn die automatische Suche aus ist, und sagt, ob Sie die neueste Version haben.

Wenn ein Update verfügbar ist, zeigt die Karte, was Sie tun können; das hängt davon ab, wie Sie Homeport installiert haben:

- **Windows, macOS und das AppImage** – **Installieren und neu starten** lädt das Update herunter, prüft seine Signatur, installiert es und öffnet Homeport wieder.
- **`.deb`- und `.rpm`-Pakete** – Ihr Paketmanager installiert Updates, daher zeigt die Karte den auszuführenden Befehl, mit einer Schaltfläche **Befehl kopieren**.

**Neuerungen** öffnet das [Änderungsprotokoll](/changelog/). Mehr dazu unter [Aktualisieren](/docs/reference/updating/).

## Registrys

Anmeldungen für Container-Registrys – Docker Hub, GitHub Container Registry (GHCR) oder Ihre eigene –, einmal in Homeport hinterlegt und auf jedem Host verwendet: wenn Sie ein Image pullen, einen Container aus einem Image ausführen, das noch nicht da ist, pushen, bauen oder einen verwalteten, verknüpften oder synchronisierten Stack bereitstellen. Es gibt eine Anmeldung pro Registry-Adresse.

- **Anmeldung hinzufügen** fragt nach der **Registry** (`docker.io`, `ghcr.io`, `registry.example.com:5000`), dem **Benutzername** und dem **Passwort oder Zugriffstoken**. GHCR und Docker-Hub-Konten mit Zwei-Faktor-Authentifizierung brauchen ein Zugriffstoken statt des Passworts.
- **Anmeldung prüfen** fragt die Registry von diesem Computer aus, ob sie die Anmeldung akzeptiert, wie es `docker login` tut, ohne etwas zu speichern. Eine abgelehnte Anmeldung wird mit einem Hinweis gemeldet; eine Registry, die Homeport nicht erreichen kann, meldet stattdessen das.
- **Bearbeiten** ändert die Adresse oder den Benutzernamen; lassen Sie das Passwort leer, um das gespeicherte zu behalten. **Entfernen** vergisst die Anmeldung: Images aus dieser Registry werden wieder anonym gepullt.

Docker Hub kann als `docker.io`, `index.docker.io` oder `https://index.docker.io/v1/` eingegeben werden; das ist eine einzige Registry. Ein Image ohne Registry im Namen, etwa `nginx` oder `bitnami/redis`, verwendet die Docker-Hub-Anmeldung.

Passwörter und Tokens werden mit dem Schlüssel aus dem Schlüsselbund des Betriebssystems verschlüsselt und nie wieder angezeigt, protokolliert oder in Fehlerdetails aufgenommen. Ohne Schlüsselbund erklärt die Karte, dass Anmeldungen nicht gespeichert werden können, und Images werden anonym gepullt. Siehe [Sicherheit](/docs/reference/security/).

Homeport liest weder `~/.docker/config.json` noch dessen Credential-Helper.

## Repositorys

Die Git-Repositorys, aus denen [synchronisierte Stacks](/docs/guides/synced-stacks/) bereitgestellt werden, einmal hinterlegt und auf jedem Host verwendbar.

- **Repository hinzufügen** fragt nach der URL des Repositorys und danach, wie Homeport es erreicht: ein **Deploy-Key**, den es für dieses Repository erstellt (SSH-URLs), ein **Token** mit seinem Benutzernamen (HTTPS-URLs) oder **Öffentlich** für eines, das jeder lesen kann.
- Die Zahnrad-Schaltfläche zeigt den Deploy-Key eines Repositorys, mit dem Ort, an dem er auf GitHub, GitLab, Gitea und Forgejo hinzugefügt wird, und **Neuer Schlüssel**, um ihn zu ersetzen; oder **Token ersetzen**. **Zugriff testen** listet seine Branches und Tags von diesem Computer aus auf. Sobald Homeport dem Schlüssel eines SSH-Servers vertraut hat, erscheint dort sein Fingerabdruck, mit **Zurücksetzen** für den Fall, dass die Forge ihn wirklich geändert hat.
- **Entfernen** vergisst das Repository und löscht seinen Klon auf diesem Computer; das wird abgelehnt, solange ein synchronisierter Stack es verwendet.

Schlüssel und Tokens werden mit dem Schlüssel aus dem Schlüsselbund des Betriebssystems verschlüsselt und nie wieder angezeigt, protokolliert oder in Fehlerdetails aufgenommen. Ihre eigene Git-Konfiguration, Ihre Credential-Helper und Ihre SSH-Schlüssel werden nie verwendet. Siehe [Repositorys](/docs/guides/synced-stacks/#repositorys).

## Synchronisierte Stacks

**Repositorys prüfen alle** legt fest, wie viele Minuten Homeport zwischen den Prüfungen der Repositorys synchronisierter Stacks auf neue Commits wartet: standardmäßig 5, mindestens 1. Homeport prüft nur, solange es geöffnet ist; solange es geschlossen ist, wird nichts synchronisiert. Siehe [Abfrage und automatische Bereitstellung](/docs/guides/synced-stacks/#abfrage-und-automatische-bereitstellung).

## Daten

Was Homeport auf diesem Computer aufbewahrt.

### Sichern und Wiederherstellen

**Sichern…** speichert alle Hosts, Cluster, Stacks, Registry-Anmeldungen, Repositorys und Einstellungen mit ihren Schlüsseln, Zertifikaten, Tokens, Passwörtern und Umgebungen in einer einzigen `.homeport`-Datei. Geben Sie zweimal eine Passphrase mit mindestens 12 Zeichen ein: Die ganze Datei wird damit verschlüsselt, und sie lässt sich nicht wiederherstellen. Engine-Ereignisse und der Bereitstellungsverlauf sind nicht enthalten, ebenso wenig etwas auf Ihren Hosts oder Clustern – Container, Images, Volumes und Kubernetes-Ressourcen bleiben, wo sie sind.

**Wiederherstellen…** öffnet eine solche Datei mit ihrer Passphrase und listet auf, was sie enthält: ihre Hosts und Cluster mit Namen und wie viele Stacks, Registry-Anmeldungen und Repositorys. **Alles ersetzen** ersetzt dann alle Daten von Homeport durch die der Sicherung, auf diesem oder einem anderen Computer:

- Alles verbindet sich und stellt sofort bereit: SSH- und TLS-Hosts, Registrys und Repositorys, ohne erneut nach einem Schlüssel oder Passwort zu fragen oder dem Schlüssel eines Servers erneut vertrauen zu müssen.
- Ein [verknüpfter Stack](/docs/guides/stacks/#verknüpfte-stacks), dessen Ordner nicht auf diesem Computer liegt, erscheint als extern, bis der Ordner unter demselben Pfad vorhanden ist.
- [Synchronisierte Stacks](/docs/guides/synced-stacks/) klonen ihr Repository bei ihrer nächsten Synchronisierung erneut.
- Die aktuellen Daten werden zuvor nach `homeport.db.before-restore-<date>` im Datenordner von Homeport kopiert. Die Wiederherstellung einer falschen Datei, einer Datei eines neueren Homeport oder eine, die mittendrin fehlschlägt, ändert nichts.

Um stattdessen einige Hosts zu Ihren vorhandenen hinzuzufügen, verwenden Sie **Exportieren** und **Importieren** auf der Seite Hosts.

### Engine-Ereignisse aufbewahren

**Engine-Ereignisse aufbewahren für** legt fest, wie viele Tage die [Engine-Ereignisse](/docs/guides/overview/#ereignisse) jedes Hosts aufbewahrt werden: standardmäßig 30, von 1 bis 365. Jeder Host bewahrt unabhängig davon höchstens 10.000 Ereignisse auf. Ältere werden beim Start von Homeport und stündlich entfernt, und sofort, wenn Sie die Zahl verringern.

## Dieser Rechner

Was Homeport auf diesem Computer nutzen kann:

- **Schlüsselbund des Betriebssystems** – **Verfügbar**, wenn SSH-Schlüssel, Zertifikate, Stack-Umgebungen und Registry-Anmeldungen verschlüsselt gespeichert werden können; andernfalls **Nicht verfügbar**. Siehe [Voraussetzungen](/docs/requirements/#optional-ein-schlüsselbund-des-betriebssystems).
- **Compose-CLI** – die gefundene Compose-Version, etwa `docker compose v2.29.1`, oder **Nicht gefunden**, wenn Stacks gestartet und gestoppt, aber nicht bereitgestellt werden können. Siehe [Stacks](/docs/guides/stacks/#was-die-bereitstellung-braucht).
- **git** – das gefundene git, etwa `/usr/bin/git 2.43.0`, oder was zu installieren ist, wenn synchronisierte Stacks nicht verwendet werden können. Siehe [Voraussetzungen](/docs/requirements/#optional-git).
- **Helm-CLI** – das gefundene helm, etwa `/usr/local/bin/helm v3.16.2`, oder was für dieses System zu installieren ist, wenn [Helm-Releases](/docs/clusters/helm/) nicht aufgelistet oder installiert werden können. Helm 2 wird als nicht unterstützt gemeldet. Siehe [Voraussetzungen](/docs/requirements/#optional-helm).

Homeport prüft die CLIs bei jedem Öffnen dieser Seite, sodass eine seit dem Start installierte erscheint; beenden Sie Homeport nach der Installation eines Schlüsselbunds und öffnen Sie es erneut.

## Über

Zeigt die Version von Homeport und die Plattform, für die es gebaut wurde, zum Beispiel `v0.2.0 · macos/aarch64`. Geben Sie sie an, wenn Sie [ein Problem melden](https://github.com/DamienReichhart/Homeport/issues).

**Lizenzen** zeigt die Lizenz von Homeport (die GNU AGPL, Version 3), einen Link zum Quellcode der Version, die Sie verwenden, und die Hinweise aller Open-Source-Pakete, mit denen es gebaut ist.
