/** German text of the `images` namespace. */
import type en from '../en/images';
import type { Translation } from '../types';

export default {
  'images.nav': 'Images',
  'images.title': 'Images',
  'images.summary': '{count, plural, one {# Image} other {# Images}} · {size}',
  'images.cleanUp': 'Aufräumen',
  'images.cleanUp.dangling': 'Verwaiste Images',
  'images.cleanUp.all': 'Alle unbenutzten Images',
  'images.saveSelected': '{count, plural, one {Image speichern…} other {# Images speichern…}}',
  'images.load': 'Laden…',
  'images.importTar': 'Aus tar importieren…',
  'images.build': 'Bauen',
  'images.pull': 'Image pullen',
  'images.search': 'Images durchsuchen',
  'images.empty.title': 'Keine Images',
  'images.empty.description': 'Pullen Sie zum Start eines aus einer Registry.',

  'images.column.name': 'Name',
  'images.column.id': 'ID',
  'images.column.created': 'Erstellt',
  'images.column.size': 'Größe',
  'images.column.status': 'Status',
  'images.moreTags': '+{count} weitere Tags',
  'images.inUse': 'In Verwendung · {count}',
  'images.unused': 'Unbenutzt',
  'images.runOne': '{name} ausführen',
  'images.action.inspect': 'Untersuchen',
  'images.action.tag': 'Taggen',
  'images.action.push': 'Pushen',
  'images.action.save': 'Speichern…',

  'images.remove.title': '{name} entfernen?',
  'images.remove.everyName':
    'Dadurch werden alle Namen des Images ({names}) und seine Layer entfernt, die nichts anderes verwendet.',
  'images.remove.description':
    'Das Image und die Layer, die kein anderes Image verwendet, werden gelöscht. Es kann später erneut gepullt werden.',
  'images.prune.allTitle': 'Alle unbenutzten Images entfernen?',
  'images.prune.danglingTitle': 'Verwaiste Images entfernen?',
  'images.prune.allDescription':
    'Alle Images, die kein Container verwendet, werden gelöscht, getaggte eingeschlossen.',
  'images.prune.danglingDescription':
    'Ungetaggte Images, die Builds und Pulls hinterlassen haben, werden gelöscht.',

  'images.removed': '{name} entfernt',
  'images.pruned': '{count, plural, one {# Image} other {# Images}} entfernt, {space} freigegeben',
  'images.pulled': '{reference} gepullt',
  'images.pushed': '{reference} gepusht',
  'images.built': '{name} gebaut',
  'images.builtUnnamed': 'Image gebaut',
  'images.saved': '{path} gespeichert',
  'images.loadedOne': '{name} geladen',
  'images.loaded': '{count, plural, one {# Image} other {# Images}} geladen',
  'images.imported': '{name} importiert',

  'images.files.save': 'Images speichern unter',
  'images.files.load': 'Images laden aus',
  'images.files.import': 'Dateisystem importieren aus',
  'images.files.imageArchives': 'Image-Archive',
  'images.files.imageArchive': 'Image-Archiv',
  'images.files.filesystemArchives': 'Dateisystem-Archive',

  'images.buildDialog.title': 'Image bauen',
  'images.buildDialog.description':
    'Aus einem Ordner auf diesem Computer. Homeport sendet ihn an den Host, ohne das, was seine .dockerignore ausschließt, und die Engine des Hosts baut ihn.',
  'images.buildDialog.built': '✓ Gebaut',
  'images.buildDialog.folder': 'Ordner',
  'images.buildDialog.chooseFolder': 'Zu bauenden Ordner wählen',
  'images.buildDialog.choose': 'Wählen…',
  'images.buildDialog.dockerfile': 'Dockerfile',
  'images.buildDialog.target': 'Ziel-Stage',
  'images.buildDialog.targetPlaceholder': 'Die letzte Stage',
  'images.buildDialog.names': 'Namen',
  'images.buildDialog.buildArgs': 'Build-Argumente',
  'images.buildDialog.buildArgsPlaceholder': 'VERSION=1.0, eines pro Zeile',
  'images.buildDialog.noCache': 'Build-Cache nicht verwenden',
  'images.buildDialog.pull': 'Neuere Basis-Images pullen',
  'images.buildForm.noFolder': 'Wählen Sie den zu bauenden Ordner.',
  'images.buildForm.argNeedsName': '„{line}“ braucht vor „=“ einen Namen.',

  'images.hub.unavailable':
    'Docker Hub kann gerade nicht durchsucht werden: {error} Ein vollständiger Name wie nginx:1.27 lässt sich trotzdem pullen.',
  'images.hub.noMatch': 'Nichts auf Docker Hub passt zu „{term}“.',
  'images.hub.results': 'Ergebnisse von Docker Hub',
  'images.hub.result': '{name}, {stars} Sterne',
  'images.hub.resultOfficial': '{name}, offizielles Image, {stars} Sterne',
  'images.hub.official': 'Offiziell',
  'images.hub.backToSearch': 'Suche',
  'images.hub.tagCount': '{count, plural, one {# Tag} other {# Tags}}',
  'images.hub.filterTags': 'Tags filtern',
  'images.hub.tags': 'Tags',
  'images.hub.noTagMatch': 'Kein Tag passt zu „{filter}“.',
  'images.hub.moreTags': 'Weitere Tags',

  'images.importDialog.title': 'Dateisystem als Image importieren',
  'images.importDialog.description':
    '{path} wird zu einem Image mit einem Layer, ohne eigene Historie oder Einstellungen; geben Sie ihm unten einen Befehl, damit es ausführbar wird.',
  'images.importDialog.name': 'Image-Name',
  'images.importDialog.changes': 'Änderungen',
  'images.importDialog.changesHint':
    'Dockerfile-Anweisungen, eine pro Zeile: CMD, ENTRYPOINT, ENV, EXPOSE, LABEL, ONBUILD, USER, VOLUME oder WORKDIR.',
  'images.importDialog.import': 'Importieren',

  'images.inspect.layers': 'Layer',
  'images.inspect.raw': 'Rohes JSON',
  'images.inspect.created': 'Erstellt',
  'images.inspect.instruction': 'Anweisung',
  'images.inspect.size': 'Größe',

  'images.pullDialog.title': 'Image pullen',
  'images.pullDialog.description':
    'Durchsuchen Sie Docker Hub nach Namen oder geben Sie eine beliebige Referenz ein, die der Host erreicht, z. B. nginx:1.27 oder ghcr.io/org/app:v2.',
  'images.pullDialog.placeholder': 'postgres oder nginx:1.27',
  'images.pullDialog.reference': 'Image-Referenz',
  'images.pullDialog.pull': 'Pullen',
  'images.done': 'Fertig.',

  'images.pushDialog.title': '{name} pushen',
  'images.pushDialog.description':
    'In die Registry, mit der sein Name beginnt, z. B. ghcr.io/org/app:v2, oder zu Docker Hub bei Namen wie org/app:v2.',
  'images.pushDialog.target': 'Zu pushender Name',
  'images.pushDialog.push': 'Pushen',
  'images.pushDialog.tagsFirst': 'Das Image erhält zuerst diesen Namen, wie bei „Taggen“.',
  'images.pushDialog.uploading': 'Wird hochgeladen {current} / {total}',
  'images.push.pushed': '{pushed, plural, one {# Layer} other {# Layer}} gepusht',
  'images.push.pushedAndExisting':
    '{pushed, plural, one {# Layer} other {# Layer}} gepusht, {existing} bereits in der Registry',

  'images.tagDialog.title': '{name} taggen',
  'images.tagDialog.newName': 'Neuer Name',
  'images.tagDialog.add': 'Tag hinzufügen',

  'images.transfer.savingOne': '{name} wird gespeichert',
  'images.transfer.saving':
    '{count, plural, one {# Image wird} other {# Images werden}} gespeichert',
  'images.transfer.to': 'Nach {path}',
  'images.transfer.from': 'Aus {path}',
  'images.transfer.written': 'Geschrieben',
  'images.transfer.sent': 'Gesendet',
  'images.transfer.loading': 'Images werden geladen',
  'images.transfer.importing': '{reference} wird importiert',
  'images.transfer.doneWith': 'Fertig: {images}',
  'images.palette.pull': 'Image pullen',
} satisfies Translation<typeof en>;
