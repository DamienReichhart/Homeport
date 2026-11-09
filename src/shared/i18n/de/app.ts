/** German text of the `app` namespace. */
import type en from '../en/app';
import type { Translation } from '../types';

export default {
  'app.nav.main': 'Hauptmenü',
  'app.nav.docs': 'Dokumentation',
  'app.nav.github': 'GitHub',
  'app.nav.opensInBrowser': '{label} (wird im Browser geöffnet)',
  'app.keychainMissing':
    'Es ist kein Schlüsselbund des Betriebssystems verfügbar, daher können SSH-Schlüssel, TLS-Zertifikate und Stack-Umgebungen nicht gespeichert werden. Lokale, TCP- und ssh-agent-Hosts funktionieren weiterhin.',
  'app.crash.title': 'Etwas ist schiefgelaufen',
  'app.crash.description':
    'Homeport ist auf dieser Seite auf ein unerwartetes Problem gestoßen. Neu laden hilft meistens; wenn es weiterhin auftritt, kopieren Sie den Bericht aus „Details“ in ein Issue.',
  'app.crash.reload': 'Neu laden',
  'app.crash.backToOverview': 'Zurück zur Übersicht',
  'app.shortcuts.title': 'Tastenkürzel',
  'app.shortcuts.description':
    'Die Tastenkürzel, die hier funktionieren. Tasten wie N und / lösen nichts aus, während Sie in einem Feld tippen.',
  'app.shortcuts.general': 'Allgemein',
  'app.shortcuts.navigation': 'Gehe zu',
  'app.shortcuts.hosts': 'Hosts',
  'app.shortcuts.help': 'Tastenkürzel anzeigen',
  'app.shortcuts.settings': 'Einstellungen öffnen',
  'app.shortcuts.goTo': 'Zu {page} wechseln',
  'app.shortcuts.previousHost': 'Vorheriger Host',
  'app.shortcuts.nextHost': 'Nächster Host',
  'app.shortcuts.refresh': 'Listen dieses Hosts aktualisieren',
  'app.palette.title': 'Befehlspalette',
  'app.palette.description':
    'Zu einer Seite wechseln, den Host wechseln oder einen Befehl ausführen.',
  'app.palette.open': 'Befehlspalette öffnen',
  'app.palette.placeholder': 'Seite, Host oder Befehl eingeben…',
  'app.palette.empty': 'Keine Treffer.',
  'app.palette.recent': 'Zuletzt verwendet',
  'app.palette.goTo': 'Gehe zu',
  'app.palette.hosts': 'Hosts',
  'app.palette.actions': 'Aktionen',
  'app.palette.switchTo': 'Zu {name} wechseln',
} satisfies Translation<typeof en>;
