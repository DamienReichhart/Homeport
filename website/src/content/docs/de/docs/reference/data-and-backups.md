---
title: Ihre Daten und Sicherungen
description: Wo Homeport seine Daten auf jedem System aufbewahrt, wie Sie sie sichern und wiederherstellen und was Sie erneut eingeben, wenn Geheimnisse verloren sind.
---

Homeport bewahrt, was es weiß, auf Ihrem Computer auf: wie Ihre Hosts und Cluster erreicht werden, die Definitionen und Umgebungen Ihrer Stacks, Ihre Registry-Anmeldungen, Repositorys und Einstellungen. Diese Seite sagt, wo das liegt, was Sie kopieren und wie Sie es zurückbekommen. Wie es geschützt ist, steht unter [Sicherheit und Ihre Daten](/docs/reference/security/).

Was auf Ihren Hosts und Clustern läuft, sind keine Daten von Homeport: Container, Images, Volumes und Kubernetes- oder Nomad-Objekte bleiben, wo sie sind, und keine Sicherung von Homeport enthält sie. Um den Inhalt eines Volumes zu kopieren, [sichern Sie es in eine Datei](/docs/guides/volumes-networks/#sichern-und-wiederherstellen), auf der Seite Volumes.

## Was wo liegt

### Der Datenordner

| System  | Ordner                                            |
| ------- | ------------------------------------------------- |
| Linux   | `~/.local/share/com.homeport.app/`                |
| macOS   | `~/Library/Application Support/com.homeport.app/` |
| Windows | `%APPDATA%\com.homeport.app\`                     |

Unter Linux liegt der Ordner unter `$XDG_DATA_HOME`, wenn diese Variable gesetzt ist. Kann Homeport seine Datenbank nicht öffnen, zeigt sein Fenster den vollständigen Pfad der Datei, mit **Ordner öffnen**.

| Im Ordner                            | Was es ist                                                                                                                                                                                             |
| ------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `homeport.db`                        | Die Datenbank: jeder Host, Cluster, Stack, jede Registry-Anmeldung, jedes Repository und jede Einstellung, der Bereitstellungsverlauf und die Engine-Ereignisse. Geheimnisse darin sind verschlüsselt. |
| `homeport.db-wal`, `homeport.db-shm` | Das Write-Ahead-Log von SQLite und sein Index, vorhanden, solange Homeport läuft. Die letzten Änderungen stehen möglicherweise nur in der `-wal`-Datei, bis Homeport beendet wird.                     |
| `homeport.db.before-restore-<date>`  | Eine Kopie der Datenbank, die direkt vor jeder [Wiederherstellung](#eine-sicherung-wiederherstellen) angelegt wird. Löschen Sie die, die Sie nicht mehr brauchen.                                      |
| `repositories/` und `synced-stacks/` | Die Klone und Arbeitskopien der Repositorys von [synchronisierten Stacks](/docs/guides/synced-stacks/). Homeport erstellt sie aus den Repositorys selbst neu.                                          |

[Was Homeport speichert](/docs/reference/security/#was-homeport-speichert) listet auf, was die Datenbank enthält und welche Teile verschlüsselt sind.

### Der Eintrag im Schlüsselbund

Der Schlüssel, der die Geheimnisse verschlüsselt, liegt nicht im Ordner. Er ist ein Eintrag im Schlüsselbund Ihres Betriebssystems, mit dem Dienst `com.homeport.app` und dem Konto `secrets-key`: in der **Schlüsselbundverwaltung** unter macOS, in **Anmeldeinformationsverwaltung → Windows-Anmeldeinformationen** unter Windows und in **Passwörter und Verschlüsselung** (Seahorse) oder KWallet unter Linux. Er gehört zu Ihrem Benutzerkonto auf diesem Computer.

Die Datenbank und dieser Eintrag gehören zusammen. Eine Datenbank ohne ihren Schlüssel lässt sich trotzdem öffnen, mit jedem Namen, jeder Adresse und jeder Compose-Datei, aber keines ihrer Geheimnisse kann entschlüsselt werden: Hosts und Cluster zeigen **Zugangsdaten verloren**. Deshalb reicht eine Kopie des Ordners nicht, um auf einen anderen Computer umzuziehen, und deshalb ist die [Sicherung in der App](#in-homeport-sichern) die empfohlene.

### Logs und die Position des Fensters

| System  | Logs                                    | Größe und Position des Fensters                                     |
| ------- | --------------------------------------- | ------------------------------------------------------------------- |
| Linux   | `~/.local/share/com.homeport.app/logs/` | `~/.config/com.homeport.app/.window-state.json`                     |
| macOS   | `~/Library/Logs/com.homeport.app/`      | `~/Library/Application Support/com.homeport.app/.window-state.json` |
| Windows | `%LOCALAPPDATA%\com.homeport.app\logs\` | `%APPDATA%\com.homeport.app\.window-state.json`                     |

Keines davon lohnt eine Sicherung. **Einstellungen → Über → Protokollordner öffnen** öffnet die Logs.

### Was Ihnen gehört

- **Die Ordner verknüpfter Stacks.** Homeport merkt sich nur, wo der Ordner und die Compose-Dateien eines [verknüpften Stacks](/docs/guides/stacks/#verknüpfte-stacks) liegen. Der Ordner, seine Compose-Dateien und seine `.env` gehören Ihnen: Sichern Sie sie wie Ihre anderen Projekte, meist mit Git.
- **Ihre kubeconfig, `~/.ssh` und `~/.docker`.** Homeport liest sie, wenn Sie daraus importieren, und ändert sie nie.
- **Die Chart-Repositorys und Anmeldungen von helm.** Sie gehören helm selbst und werden mit Ihrem Terminal geteilt.

## In Homeport sichern

**Einstellungen → Daten → Sichern…** schreibt alles in eine einzige `.homeport`-Datei. Das ist die Sicherung, die Sie bevorzugen sollten, denn sie enthält die Geheimnisse selbst, geschützt durch eine Passphrase statt durch den Schlüsselbund dieses Computers: Sie lässt sich auf jedem Computer wiederherstellen.

1. Öffnen Sie **Einstellungen** und wählen Sie unter **Daten** die Option **Sichern…**.
2. Geben Sie zweimal eine Passphrase mit mindestens 12 Zeichen ein. Homeport speichert sie nie, und nichts kann die Datei ohne sie öffnen.
3. Wählen Sie, wo die Datei gespeichert wird.

Homeport sagt dann, was es gespeichert hat, und nennt alles, was es ohne sein Geheimnis speichern musste, weil dieses Geheimnis auf diesem Computer nicht entschlüsselt werden konnte.

| In der Sicherung                                                                                                     | Nicht in der Sicherung                                                         |
| -------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------ |
| Hosts, mit ihren Schlüsseln, Passphrasen, Zertifikaten, Bastionen und den Serverschlüsseln, denen Sie vertraut haben | Engine-Ereignisse                                                              |
| Cluster, mit ihren Zertifikaten, Tokens und Einstellungen für Exec-Plugins                                           | Bereitstellungsverlauf                                                         |
| Compose-Dateien und Umgebungen verwalteter Stacks                                                                    | Ordner und Dateien verknüpfter Stacks                                          |
| Ordner- und Dateipfade verknüpfter Stacks                                                                            | Klone synchronisierter Stacks (bei der nächsten Synchronisierung neu erstellt) |
| Repository, Branch oder Tag, Synchronisierungseinstellungen und Umgebung synchronisierter Stacks                     | Logs sowie Größe und Position des Fensters                                     |
| Auf einem Swarm bereitgestellte Stacks                                                                               | Alles auf Ihren Hosts und Clustern: Container, Images, Volumes                 |
| Registry-Anmeldungen, mit ihren Passwörtern und Tokens                                                               | Die Chart-Repositorys von helm                                                 |
| Repositorys, mit ihren Deploy-Keys und Tokens                                                                        | Portweiterleitungen, die enden, wenn Homeport beendet wird                     |
| Ihre Einstellungen                                                                                                   |                                                                                |

Die ganze Datei ist mit der Passphrase verschlüsselt, sodass sie ohne diese nichts preisgibt, nicht einmal den Namen eines Hosts. Sie enthält jedes Geheimnis, das Homeport hat: Bewahren Sie sie und ihre Passphrase so sicher auf wie die Schlüssel selbst. [Sicherungen](/docs/reference/security/#sicherungen) beschreibt die Verschlüsselung.

Sichern Sie, bevor Sie eine Vorabversion ausprobieren, bevor Sie Ihr System neu installieren und immer dann, wenn Sie Hosts hinzugefügt haben, die Sie nicht noch einmal einrichten möchten.

### Eine Sicherung wiederherstellen

1. Öffnen Sie auf diesem oder einem anderen Computer **Einstellungen** und wählen Sie unter **Daten** die Option **Wiederherstellen…**.
2. Wählen Sie die `.homeport`-Datei und geben Sie ihre Passphrase ein. Homeport listet auf, was die Datei enthält und welches Homeport sie wann erstellt hat, bevor es etwas ändert.
3. Wählen Sie **Alles ersetzen**.

Die Wiederherstellung ersetzt alle Daten von Homeport durch die der Sicherung; sie führt nichts zusammen. Homeport kopiert zuerst die aktuelle Datenbank nach `homeport.db.before-restore-<date>` im Datenordner und ersetzt dann alles in einem Schritt, sodass ein Fehler Ihre Daten unverändert lässt. Jedes Geheimnis wird erneut mit dem Schlüsselbund-Schlüssel dieses Computers verschlüsselt.

Danach:

- Hosts und Cluster verbinden sich sofort wieder, ohne nach einem Schlüssel, einem Passwort oder einem zu vertrauenden Serverschlüssel zu fragen.
- Ein verknüpfter Stack, dessen Ordner nicht unter demselben Pfad auf diesem Computer liegt, erscheint als extern, bis er dort liegt.
- Synchronisierte Stacks klonen ihr Repository bei ihrer nächsten Synchronisierung neu.
- Die Seite Ereignisse und der Bereitstellungsverlauf beginnen leer.

Eine von einem älteren Homeport erstellte Sicherung lässt sich in einem neueren wiederherstellen. Eine von einem neueren Homeport erstellte wird abgelehnt: Aktualisieren Sie zuerst Homeport.

Um zum Stand vor einer Wiederherstellung zurückzukehren, beenden Sie Homeport, benennen Sie `homeport.db` um, benennen Sie die gewünschte `homeport.db.before-restore-<date>` in `homeport.db` um, löschen Sie eine daneben verbliebene `homeport.db-wal` und `homeport.db-shm` und öffnen Sie Homeport wieder.

### Nur einige Hosts umziehen

**Exportieren** und **Importieren** auf der Seite Hosts ziehen die von Ihnen gewählten Hosts mit ihren Geheimnissen in einer Datei derselben Art um und fügen sie zu den Hosts hinzu, die der andere Computer bereits hat. Siehe [Auf einen anderen Computer umziehen](/docs/reference/security/#auf-einen-anderen-computer-umziehen).

## Die Dateien selbst kopieren

Eine Kopie der Datenbank eignet sich für eine Systemsicherung dieses Computers, etwa mit Time Machine, dem Dateiversionsverlauf oder Ihrem eigenen Skript. Sie behält, was die Sicherung in der App auslässt, die Engine-Ereignisse und den Bereitstellungsverlauf, aber **nicht den Schlüssel aus dem Schlüsselbund**: Mit ihren Geheimnissen lässt sie sich nur für denselben Benutzer auf demselben Computer wiederherstellen, oder auf einem, dessen Schlüsselbund mit ihr wiederhergestellt wurde. Überall sonst kommen alle Namen, Adressen und Compose-Dateien zurück, und die Geheimnisse müssen [erneut eingegeben](#wenn-die-geheimnisse-verloren-sind) werden.

### Während Homeport geschlossen ist

Beenden Sie Homeport, über **Beenden** im Infobereich, wenn es [in den Infobereich schließt](/docs/guides/settings/#desktop), und kopieren Sie dann den ganzen Datenordner. Sobald Homeport beendet ist, enthält `homeport.db` alles, und die `-wal`- und `-shm`-Dateien sind verschwunden.

### Während Homeport läuft

Kopieren Sie `homeport.db` nicht allein, während Homeport läuft: Die letzten Änderungen stehen möglicherweise noch in `homeport.db-wal`, und eine mitten im Schreiben erstellte Kopie kann inkonsistent sein. Lassen Sie stattdessen SQLite eine konsistente Kopie erstellen, mit dem Kommandozeilenwerkzeug `sqlite3` (bei macOS dabei; unter Linux und Windows installieren Sie es aus dem `sqlite3`-Paket Ihrer Distribution oder von [sqlite.org](https://sqlite.org/download.html)):

```sh
# Linux
sqlite3 ~/.local/share/com.homeport.app/homeport.db ".backup '$HOME/homeport-backup.db'"

# macOS
sqlite3 "$HOME/Library/Application Support/com.homeport.app/homeport.db" ".backup '$HOME/homeport-backup.db'"
```

```powershell
# Windows
sqlite3 "$env:APPDATA\com.homeport.app\homeport.db" ".backup '$HOME\homeport-backup.db'"
```

Das Ergebnis ist eine einzige vollständige Datei, ohne `-wal` daneben. Um sie zu prüfen:

```sh
sqlite3 ~/homeport-backup.db "PRAGMA integrity_check;"
```

Es gibt `ok` aus.

### Eine Kopie wiederherstellen

1. Beenden Sie Homeport.
2. Verschieben Sie im Datenordner `homeport.db` beiseite, und löschen Sie `homeport.db-wal` und `homeport.db-shm`, falls vorhanden: Sie gehören zu der Datenbank, die Sie ersetzen.
3. Legen Sie Ihre Kopie dort ab, mit dem Namen `homeport.db`. Haben Sie den ganzen Ordner kopiert, legen Sie stattdessen den Ordner zurück.
4. Öffnen Sie Homeport.

Homeport aktualisiert eine von einer älteren Version erstellte Kopie, wenn es sie öffnet. Eine Kopie, die zuletzt von einem neueren Homeport verwendet wurde, wird unverändert abgelehnt, mit [einer Meldung, die die zu installierende Version nennt](/docs/reference/troubleshooting/#diese-datenbank-wurde-zuletzt-von-homeport--oder-neuer-verwendet).

## Wenn die Geheimnisse verloren sind

Nach einem Zurücksetzen des Schlüsselbunds, einem neuen Benutzerkonto oder einer kopierten Datenbank, die auf einem anderen Computer geöffnet wird, behält Homeport alles, was es noch lesen kann, und markiert, was es nicht lesen kann. Nichts wird gelöscht, und was kein Geheimnis ist (Namen, Adressen, Compose-Dateien, Ordnerpfade), ist unversehrt. Geben Sie erneut ein:

| Was                                        | Wie                                                                                                                                                                                                            |
| ------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **TLS-Hosts**                              | Laden Sie auf der Seite des Hosts das CA-Zertifikat, das Client-Zertifikat und den Client-Schlüssel erneut und wählen Sie **Speichern**.                                                                       |
| **SSH-Hosts mit importiertem Schlüssel**   | Fügen Sie den privaten Schlüssel und seine Passphrase erneut ein und wählen Sie **Speichern**.                                                                                                                 |
| **SSH-Hosts mit einem Homeport-Schlüssel** | Wählen Sie **Schlüssel neu erzeugen** und [autorisieren Sie den neuen Schlüssel](/docs/hosts/ssh/#den-schlüssel-von-homeport-autorisieren) dann auf dem Server. Entfernen Sie dort die alte Zeile `homeport@`. |
| **Incus-Hosts über HTTPS**                 | Entfernen Sie den Host und [fügen Sie ihn mit einem neuen Trust-Token erneut hinzu](/docs/hosts/incus/#über-https-mit-einem-trust-token).                                                                      |
| **Kubernetes-Cluster**                     | **Aus kubeconfig aktualisieren** auf der Seite des Clusters.                                                                                                                                                   |
| **Nomad-Cluster**                          | Fügen Sie das ACL-Token und, falls verwendet, die Zertifikate in den **Einstellungen** des Clusters ein.                                                                                                       |
| **Stack-Umgebungen**                       | Geben Sie die Umgebung eines verwalteten oder synchronisierten Stacks auf seiner Seite erneut ein. Verknüpfte Stacks behalten ihre in der `.env` ihres Ordners, die Homeport nie hatte.                        |
| **Registry-Anmeldungen**                   | Bearbeiten Sie unter **Einstellungen → Registrys** jede Anmeldung und geben Sie ihr Passwort oder Token ein.                                                                                                   |
| **Repositorys**                            | Wählen Sie unter **Einstellungen → Repositorys** **Neuer Schlüssel** und fügen Sie den Schlüssel dem Repository auf Ihrer Forge hinzu, oder wählen Sie **Token ersetzen**.                                     |

Hosts mit lokalem Socket, TCP und ssh-agent enthalten kein Geheimnis und funktionieren weiter. Die im Bereitstellungsverlauf aufgezeichneten Umgebungen können nicht erneut eingegeben werden; die Compose-Dateien des Verlaufs bleiben lesbar.

Meldet Homeport stattdessen, dass der Schlüsselbund-Eintrag fehlt, mit dem seine Geheimnisse versiegelt wurden, hat es sie noch nicht aufgegeben: Wechseln Sie zurück zu dem Schlüsselbund mit dem Eintrag, oder wählen Sie **Mit neuem Schlüssel neu beginnen** unter **Einstellungen → Dieser Rechner**, bevor Sie sie erneut eingeben.

Siehe [Zugangsdaten verloren](/docs/reference/troubleshooting/#zugangsdaten-verloren) in der Fehlerbehebung.

## Ihre Daten entfernen

[Ihre Daten entfernen](/docs/reference/updating/#ihre-daten-entfernen) listet die Ordner und den Eintrag im Schlüsselbund auf, die Sie löschen, und die Schlüssel, die Sie von Ihren Servern entfernen.
