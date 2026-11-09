/** German text of the `volumes` namespace. */
import type en from '../en/volumes';
import type { Translation } from '../types';

export default {
  'volumes.nav': 'Volumes',
  'volumes.title': 'Volumes',
  'volumes.summary': '{count, plural, one {# Volume} other {# Volumes}} · {unused} unbenutzt',
  'volumes.cleanUp': 'Aufräumen',
  'volumes.cleanUp.anonymous': 'Unbenutzte anonyme Volumes',
  'volumes.cleanUp.all': 'Alle unbenutzten Volumes',
  'volumes.create': 'Volume erstellen',
  'volumes.search': 'Volumes durchsuchen',
  'volumes.empty.title': 'Keine Volumes',
  'volumes.empty.description':
    'Volumes bewahren Containerdaten über Neustarts und Neuerstellungen hinweg.',
  'volumes.column.name': 'Name',
  'volumes.column.stack': 'Stack',
  'volumes.column.driver': 'Treiber',
  'volumes.column.size': 'Größe',
  'volumes.size.label': 'Größe:',
  'volumes.size.measuring': 'Wird gemessen',
  'volumes.size.unknown': 'Der Treiber dieses Volumes meldet seine Größe nicht.',
  'volumes.size.refresh': 'Größen aktualisieren',
  'volumes.column.usedBy': 'Verwendet von',
  'volumes.column.created': 'Erstellt',
  'volumes.unused': 'Unbenutzt',
  'volumes.inspectOne': '{name} untersuchen',
  'volumes.deleteOne': '{name} löschen',
  'volumes.browseOne': 'Dateien von {name} durchsuchen',
  'volumes.browse.title': 'Dateien von {name}',
  'volumes.browse.description':
    'Schreibgeschützt: Nichts hier kann das Volume ändern, auch nicht, während Container es verwenden.',
  'volumes.browse.saveOne': '{name} speichern',
  'volumes.browse.saved': '{name} gespeichert',
  'volumes.browse.savedArchive': '{name} als .tar-Archiv gespeichert',
  'volumes.helper.needed':
    'Homeport öffnet Volumes mit einem kurzlebigen Hilfscontainer, der das Image {image} benötigt (etwa {size}). Dieser Host hat es nicht.',
  'volumes.helper.removed':
    'Homeport pullt es nur mit Ihrer Zustimmung und entfernt es danach wieder, sofern nichts anderes es verwendet.',
  'volumes.helper.pull': '{image} pullen',
  'volumes.helper.pulling': '{image} wird gepullt…',
  'volumes.helper.progress': 'Pull-Fortschritt',
  'volumes.moreActions': 'Weitere Aktionen für {name}',
  'volumes.backupMenu': 'In eine Datei sichern…',
  'volumes.restoreMenu': 'Aus einer Datei wiederherstellen…',
  'volumes.cloneMenu': 'Klonen…',
  'volumes.clone.title': '{volume} klonen',
  'volumes.clone.description':
    'Kopiert seine Dateien mit ihren Besitzern und Berechtigungen in ein neues Volume auf diesem Host. Die Kopie gehört zu keinem Stack.',
  'volumes.clone.name': 'Name der Kopie',
  'volumes.clone.start': 'Klonen',
  'volumes.clone.into': 'Nach {target}',
  'volumes.clone.copying': 'Wird kopiert…',
  'volumes.clone.done': '{source} nach {target} geklont',
  'volumes.restoreFromFile': 'Aus Datei wiederherstellen',
  'volumes.backup.title': '{volume} sichern',
  'volumes.backup.saveAs': '{volume} sichern nach',
  'volumes.backup.to': 'Nach {path}',
  'volumes.backup.done': '{volume} nach {path} gesichert',
  'volumes.restore.pick': 'Volume wiederherstellen aus',
  'volumes.restore.title': 'Volume wiederherstellen',
  'volumes.restore.titleInto': '{volume} wiederherstellen',
  'volumes.restore.from': 'Aus {path}',
  'volumes.restore.into': 'Wiederherstellen in',
  'volumes.restore.newVolume': 'Ein neues Volume',
  'volumes.restore.existingVolume': 'Ein vorhandenes Volume',
  'volumes.restore.volume': 'Volume',
  'volumes.restore.chooseVolume': 'Volume wählen',
  'volumes.restore.nameTaken': 'Ein Volume namens {volume} existiert bereits.',
  'volumes.restore.start': 'Wiederherstellen',
  'volumes.restore.done': '{volume} wiederhergestellt',
  'volumes.restore.cancelled':
    'Die Wiederherstellung von {volume} wurde abgebrochen; das Volume ist möglicherweise teilweise beschrieben.',
  'volumes.transfer.written': 'Geschrieben',
  'volumes.transfer.sent': 'Gesendet',
  'volumes.transfer.archives': 'Tar-Archive',
  'volumes.flow.running.backup':
    '{containers} {count, plural, one {läuft} other {laufen}} und {count, plural, one {verwendet} other {verwenden}} {volume}. Eine Sicherung, während sie hineinschreiben, kann inkonsistent sein, etwa eine Datenbank mitten im Schreibvorgang.',
  'volumes.flow.running.restore':
    '{containers} {count, plural, one {läuft} other {laufen}} und {count, plural, one {verwendet} other {verwenden}} {volume}. Homeport stoppt {count, plural, one {ihn} other {sie}} während der Wiederherstellung und startet {count, plural, one {ihn} other {sie}} danach wieder.',
  'volumes.flow.running.clone':
    '{containers} {count, plural, one {läuft} other {laufen}} und {count, plural, one {verwendet} other {verwenden}} {volume}. Eine Kopie, während sie hineinschreiben, kann inkonsistent sein, etwa eine Datenbank mitten im Schreibvorgang.',
  'volumes.flow.anyway.backup': 'Trotzdem sichern',
  'volumes.flow.anyway.clone': 'Trotzdem kopieren',
  'volumes.flow.stopDuring.backup':
    '{count, plural, one {Ihn} other {Sie}} während der Sicherung stoppen',
  'volumes.flow.stopDuring.restore': 'Stoppen und wiederherstellen',
  'volumes.flow.stopDuring.clone':
    '{count, plural, one {Ihn} other {Sie}} während des Kopierens stoppen',
  'volumes.flow.replace':
    '{volume} enthält bereits Dateien. Die Wiederherstellung löscht sie und legt die der Sicherung an ihre Stelle.',
  'volumes.flow.replaceConfirm': 'Inhalt ersetzen',
  'volumes.usedBy': 'Verwendet von {containers}.',
  'volumes.notUsed': 'Kein Container verwendet dieses Volume.',
  'volumes.createDialog.title': 'Volume erstellen',
  'volumes.createDialog.name': 'Name',
  'volumes.createDialog.driver': 'Treiber',
  'volumes.deleted': '{name} gelöscht',
  'volumes.pruned':
    '{count, plural, one {# Volume} other {# Volumes}} gelöscht, {space} freigegeben',
  'volumes.remove.title': 'Volume {name} löschen?',
  'volumes.remove.warning':
    '<strong>Alle Daten in diesem Volume werden dauerhaft gelöscht.</strong>',
  'volumes.remove.mounted':
    ' Es ist in {containers} eingehängt; entfernen Sie zuerst diese Container.',
  'volumes.remove.confirm': 'Volume und Daten löschen',
  'volumes.prune.allTitle': 'Alle unbenutzten Volumes löschen?',
  'volumes.prune.anonymousTitle': 'Unbenutzte anonyme Volumes löschen?',
  'volumes.prune.allWarning':
    'Die Daten in allen Volumes, die kein Container verwendet, werden dauerhaft gelöscht, benannte Volumes eingeschlossen.',
  'volumes.prune.anonymousWarning':
    'Die Daten in unbenannten Volumes, die kein Container verwendet, werden dauerhaft gelöscht.',
  'volumes.prune.confirm': 'Volumes und Daten löschen',
} satisfies Translation<typeof en>;
