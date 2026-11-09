/** German text of the `updates` namespace. */
import type en from '../en/updates';
import type { Translation } from '../types';

export default {
  'updates.title': 'Updates',
  'updates.description':
    'Homeport kann beim Start und alle paar Stunden nach einer neuen Version suchen.',
  'updates.automatic': 'Automatisch nach Updates suchen',
  'updates.checking': 'Wird geprüft…',
  'updates.checkNow': 'Jetzt prüfen',
  'updates.status.available': 'Homeport {version} ist verfügbar (Sie haben {current}).',
  'updates.status.latest': 'Homeport {current} ist die neueste Version.',

  'updates.notice.label': 'Update verfügbar',
  'updates.notice.available': '<strong>Homeport {version}</strong> ist verfügbar.',
  'updates.notice.availablePackageManager':
    '<strong>Homeport {version}</strong> ist verfügbar. Aktualisieren Sie es mit Ihrem Paketmanager:',
  'updates.notice.dismiss': 'Ausblenden',

  'updates.copyCommand': 'Befehl kopieren',
  'updates.whatsNew': 'Neuerungen',
  'updates.install': 'Installieren und neu starten',
  'updates.download': 'Herunterladen',
  'updates.progress.downloading': 'Wird heruntergeladen…',
  'updates.progress.downloadingPercent': 'Wird heruntergeladen… {percent} %',
  'updates.progress.installing': 'Wird installiert…',

  'updates.method.inApp':
    'Updates werden heruntergeladen, mit dem Signaturschlüssel von Homeport geprüft und installiert, wann Sie es wählen.',
  'updates.method.scoop': 'Mit Scoop installiert: Updates kommen aus dem Homeport-Bucket.',
  'updates.method.aur':
    'Aus dem AUR installiert (homeport-bin): Ihr AUR-Helper installiert Updates.',
  'updates.method.packageManager':
    'Mit {manager} installiert: Updates kommen aus dem Paket-Repository von Homeport.',
  'updates.method.manual':
    'Dies ist ein Entwicklungs-Build: Laden Sie neue Versionen von der Website herunter.',
  'updates.palette.check': 'Nach Updates suchen',
} satisfies Translation<typeof en>;
