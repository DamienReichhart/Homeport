/** German text of the `settings` namespace. */
import type en from '../en/settings';
import type { Translation } from '../types';

export default {
  'settings.nav': 'Einstellungen',
  'settings.title': 'Einstellungen',
  'settings.appearance.title': 'Darstellung',
  'settings.appearance.theme': 'Design',
  'settings.theme.system': 'Wie das System',
  'settings.theme.light': 'Hell',
  'settings.theme.dark': 'Dunkel',
  'settings.appearance.language': 'Sprache',
  'settings.language.system': 'Wie das System ({language})',
  'settings.desktop.title': 'Desktop',
  'settings.desktop.description': 'Wie Homeport auf diesem Computer läuft.',
  'settings.desktop.closeToTray': 'In den Infobereich schließen',
  'settings.desktop.closeToTrayHint':
    'Beim Schließen des Fensters laufen Homeport und seine Host-Verbindungen im Infobereich weiter. Beenden Sie es über das Symbol im Infobereich. Auf Desktops, die Infobereich-Symbole ausblenden, etwa GNOME ohne Erweiterung, wird das Symbol möglicherweise nicht angezeigt.',
  'settings.desktop.noTray':
    'Dieser Desktop hat keinen Infobereich, den Homeport nutzen kann, daher beendet das Schließen des Fensters die App.',
  'settings.desktop.startAtLogin': 'Bei der Anmeldung starten',
  'settings.desktop.startAtLoginHint':
    'Homeport startet, wenn Sie sich an diesem Computer anmelden, und überwacht Ihre Hosts so von Anfang an.',
  'settings.desktop.startHidden': 'Verborgen im Infobereich starten',
  'settings.desktop.startHiddenHint':
    'Wenn Homeport bei der Anmeldung startet, bleibt es im Infobereich, ohne sein Fenster zu öffnen.',
  'settings.desktop.startHiddenNoTray':
    'Erfordert einen Infobereich, aus dem das Fenster geöffnet werden kann, und den hat dieser Desktop nicht.',
  'settings.notifications.title': 'Benachrichtigungen',
  'settings.notifications.description':
    'Homeport benachrichtigt Sie, auch wenn sein Fenster in den Infobereich geschlossen ist, wenn:',
  'settings.notifications.containerDied':
    'Ein Container mit einem Fehler stoppt oder keinen Speicher mehr hat',
  'settings.notifications.containerUnhealthy': 'Ein Container fehlerhaft wird',
  'settings.notifications.hostUnreachable': 'Ein verbundener Host nicht mehr erreichbar ist',
  'settings.notifications.hostBack': 'Ein nicht erreichbarer Host wieder da ist',
  'settings.machine.title': 'Dieser Rechner',
  'settings.machine.description': 'Was Homeport hier nutzen kann.',
  'settings.machine.keychain': 'Schlüsselbund des Betriebssystems',
  'settings.machine.keychainAvailable':
    'Verfügbar — SSH-Schlüssel, Zertifikate, Stack-Umgebungen und Registry-Anmeldungen werden verschlüsselt gespeichert.',
  'settings.machine.keychainUnavailable':
    'Nicht verfügbar — SSH-Schlüssel, TLS-Zertifikate, Stack-Umgebungen und Registry-Anmeldungen können nicht gespeichert werden.',
  'settings.machine.compose': 'Compose-CLI',
  'settings.machine.composeMissing':
    'Stacks können gestartet und gestoppt, aber nicht bereitgestellt werden.',
  'settings.machine.stackDeploy': 'Docker-CLI, für Swarms',
  'settings.machine.stackDeployMissing':
    'Stacks können nicht in einem Swarm bereitgestellt werden; alles andere an Swarms funktioniert.',
  'settings.machine.git': 'git',
  'settings.machine.helm': 'Helm-CLI',
  'settings.sync.title': 'Synchronisierte Stacks',
  'settings.sync.interval': 'Repositorys prüfen alle',
  'settings.sync.minutes': 'Minuten',
  'settings.data.title': 'Daten',
  'settings.data.description': 'Was Homeport auf diesem Computer aufbewahrt.',
  'settings.data.backup': 'Sichern…',
  'settings.data.restore': 'Wiederherstellen…',
  'settings.data.backupHint':
    'Speichern Sie alle Hosts, Stacks, Registry-Anmeldungen, Repositorys und Einstellungen samt ihren Geheimnissen in einer passphrasengeschützten Datei und stellen Sie sie auf diesem oder einem anderen Computer wieder her.',
  'settings.backup.title': 'Daten von Homeport sichern',
  'settings.backup.description':
    'Die Datei enthält alle Hosts, Cluster, Stacks, Registry-Anmeldungen, Repositorys und Einstellungen sowie deren Schlüssel, Zertifikate, Tokens, Passwörter und Umgebungen. Wer die Datei und ihre Passphrase hat, kann sie verwenden, bewahren Sie daher beides sicher auf. Engine-Ereignisse und der Bereitstellungsverlauf sind nicht enthalten.',
  'settings.backup.passphrase': 'Passphrase',
  'settings.backup.passphraseHint':
    'Mindestens {min} Zeichen. Sie lässt sich nicht wiederherstellen.',
  'settings.backup.confirmation': 'Passphrase wiederholen',
  'settings.backup.problem.short': 'Verwenden Sie mindestens {min} Zeichen.',
  'settings.backup.problem.mismatch': 'Die beiden Passphrasen unterscheiden sich.',
  'settings.backup.start': 'Sichern…',
  'settings.backup.saveTitle': 'Sicherung speichern unter',
  'settings.backup.fileType': 'Homeport-Sicherung',
  'settings.backup.done':
    '{hosts, plural, one {# Host} other {# Hosts}} und {stacks, plural, one {# Stack} other {# Stacks}} gesichert',
  'settings.backup.withoutSecrets':
    'Diese wurden ohne ihre Geheimnisse gesichert, da diese sich nicht öffnen ließen:',
  'settings.restore.pick': 'Wiederherstellen aus',
  'settings.restore.title': 'Daten von Homeport wiederherstellen',
  'settings.restore.from': 'Aus {path}',
  'settings.restore.open': 'Öffnen',
  'settings.restore.madeBy': 'Gesichert von Homeport {version} am {date}. Sie enthält:',
  'settings.restore.holds': 'Was die Sicherung enthält',
  'settings.restore.hosts': '{count, plural, one {# Host} other {# Hosts}}',
  'settings.restore.clusters': '{count, plural, one {# Cluster} other {# Cluster}}',
  'settings.restore.stacks': '{count, plural, one {# Stack} other {# Stacks}}',
  'settings.restore.registries':
    '{count, plural, one {# Registry-Anmeldung} other {# Registry-Anmeldungen}}',
  'settings.restore.repositories': '{count, plural, one {# Repository} other {# Repositorys}}',
  'settings.restore.settings': 'Ihre Einstellungen',
  'settings.restore.replaces':
    'Die Wiederherstellung ersetzt alle Hosts, Stacks, Registry-Anmeldungen, Repositorys und Einstellungen, die Sie jetzt haben, und vergisst die Engine-Ereignisse und den Bereitstellungsverlauf. Zuvor wird eine Kopie der aktuellen Daten im Datenordner von Homeport abgelegt.',
  'settings.restore.confirm': 'Alles ersetzen',
  'settings.restore.done':
    '{hosts, plural, one {# Host} other {# Hosts}} und {stacks, plural, one {# Stack} other {# Stacks}} wiederhergestellt',
  'settings.data.keepEvents': 'Engine-Ereignisse aufbewahren für',
  'settings.data.days': 'Tage',
  'settings.data.keepEventsHint':
    'Jeder Host bewahrt für die Seite „Ereignisse“ höchstens 10.000 Ereignisse auf. Ältere werden stündlich entfernt.',
  'settings.machine.gitMissing':
    'Synchronisierte Stacks benötigen es; alle anderen Arten von Stacks funktionieren ohne.',
  'settings.machine.helmMissing':
    'Helm-Releases können nicht aufgelistet oder installiert werden; alles andere an Clustern funktioniert.',
  'settings.about.tagline': 'Jede Docker Engine, auf einem Board.',
  'settings.about.openLogFolder': 'Protokollordner öffnen',
  'settings.about.licenses': 'Lizenzen',
  'settings.licenses.title': 'Lizenzen',
  'settings.licenses.description':
    'Homeport ist freie Software unter der GNU Affero General Public License, Version 3, und baut auf den unten aufgeführten Open-Source-Paketen auf.',
  'settings.licenses.source':
    'Der Quellcode dieser Version ist unter derselben Lizenz veröffentlicht.',
  'settings.licenses.sourceLink': 'Quellcode von v{version}',
  'settings.licenses.thirdParty': 'Software von Drittanbietern',
  'settings.licenses.missing.title': 'In diesem Build nicht enthalten',
  'settings.licenses.missing.description':
    'Release-Builds bringen sie mit; führen Sie <code>{command}</code> aus, um sie für einen Entwicklungs-Build zu erzeugen.',
  'settings.palette.group': 'Einstellungen',
  'settings.palette.themeLight': 'Design: Hell',
  'settings.palette.themeDark': 'Design: Dunkel',
  'settings.palette.themeSystem': 'Design: Wie das System',
} satisfies Translation<typeof en>;
