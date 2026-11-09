/** Italian text of the `images` namespace. */
import type en from '../en/images';
import type { Translation } from '../types';

export default {
  'images.nav': 'Immagini',
  'images.title': 'Immagini',
  'images.summary': '{count, plural, one {# immagine} other {# immagini}} · {size}',
  'images.cleanUp': 'Ripulisci',
  'images.cleanUp.dangling': 'Immagini orfane',
  'images.cleanUp.all': 'Tutte le immagini inutilizzate',
  'images.saveSelected': '{count, plural, one {Salva immagine…} other {Salva # immagini…}}',
  'images.load': 'Carica…',
  'images.importTar': 'Importa da tar…',
  'images.build': 'Compila',
  'images.pull': 'Scarica immagine',
  'images.search': 'Cerca immagini',
  'images.empty.title': 'Nessuna immagine',
  'images.empty.description': 'Scaricane una da un registro per iniziare.',

  'images.column.name': 'Nome',
  'images.column.id': 'ID',
  'images.column.created': 'Creata',
  'images.column.size': 'Dimensione',
  'images.column.status': 'Stato',
  'images.moreTags': '{count, plural, one {+# altro tag} other {+# altri tag}}',
  'images.inUse': 'In uso · {count}',
  'images.unused': 'Inutilizzata',
  'images.runOne': 'Esegui {name}',
  'images.action.inspect': 'Ispeziona',
  'images.action.tag': 'Aggiungi tag',
  'images.action.push': 'Invia',
  'images.action.save': 'Salva…',

  'images.remove.title': 'Rimuovere {name}?',
  'images.remove.everyName':
    'Vengono rimossi tutti i nomi dell’immagine ({names}) e i suoi layer non usati da nient’altro.',
  'images.remove.description':
    'Vengono eliminati l’immagine e i layer non usati da altre immagini. Potrai scaricarla di nuovo in seguito.',
  'images.prune.allTitle': 'Rimuovere tutte le immagini inutilizzate?',
  'images.prune.danglingTitle': 'Rimuovere le immagini orfane?',
  'images.prune.allDescription':
    'Viene eliminata ogni immagine non usata da alcun container, comprese quelle con tag.',
  'images.prune.danglingDescription':
    'Vengono eliminate le immagini senza tag lasciate da build e download.',

  'images.removed': '{name} rimossa',
  'images.pruned':
    '{count, plural, one {# immagine rimossa} other {# immagini rimosse}}, {space} recuperati',
  'images.pulled': '{reference} scaricata',
  'images.pushed': '{reference} inviata',
  'images.built': '{name} compilata',
  'images.builtUnnamed': 'Immagine compilata',
  'images.saved': '{path} salvato',
  'images.loadedOne': '{name} caricata',
  'images.loaded': '{count, plural, one {# immagine caricata} other {# immagini caricate}}',
  'images.imported': '{name} importata',

  'images.files.save': 'Salva le immagini come',
  'images.files.load': 'Carica le immagini da',
  'images.files.import': 'Importa un filesystem da',
  'images.files.imageArchives': 'Archivi di immagini',
  'images.files.imageArchive': 'Archivio di immagine',
  'images.files.filesystemArchives': 'Archivi di filesystem',

  'images.buildDialog.title': 'Compila un’immagine',
  'images.buildDialog.description':
    'Da una cartella di questo computer. Homeport la invia all’host, tralasciando ciò che esclude il suo .dockerignore, e l’Engine dell’host la compila.',
  'images.buildDialog.built': '✓ Compilata',
  'images.buildDialog.folder': 'Cartella',
  'images.buildDialog.chooseFolder': 'Scegli la cartella da compilare',
  'images.buildDialog.choose': 'Scegli…',
  'images.buildDialog.dockerfile': 'Dockerfile',
  'images.buildDialog.target': 'Stage di destinazione',
  'images.buildDialog.targetPlaceholder': 'L’ultimo stage',
  'images.buildDialog.names': 'Nomi',
  'images.buildDialog.buildArgs': 'Argomenti di build',
  'images.buildDialog.buildArgsPlaceholder': 'VERSION=1.0, uno per riga',
  'images.buildDialog.noCache': 'Non usare la cache di build',
  'images.buildDialog.pull': 'Scarica immagini di base più recenti',
  'images.buildForm.noFolder': 'Scegli la cartella da compilare.',
  'images.buildForm.argNeedsName': '«{line}» richiede un nome prima di «=».',

  'images.hub.unavailable':
    'Al momento non è possibile cercare su Docker Hub: {error} Un nome completo come nginx:1.27 viene comunque scaricato.',
  'images.hub.noMatch': 'Niente su Docker Hub corrisponde a «{term}».',
  'images.hub.results': 'Risultati di Docker Hub',
  'images.hub.result': '{name}, {stars} stelle',
  'images.hub.resultOfficial': '{name}, immagine ufficiale, {stars} stelle',
  'images.hub.official': 'Ufficiale',
  'images.hub.backToSearch': 'Cerca',
  'images.hub.tagCount': '{count, plural, one {# tag} other {# tag}}',
  'images.hub.filterTags': 'Filtra i tag',
  'images.hub.tags': 'Tag',
  'images.hub.noTagMatch': 'Nessun tag corrisponde a «{filter}».',
  'images.hub.moreTags': 'Altri tag',

  'images.importDialog.title': 'Importa un filesystem come immagine',
  'images.importDialog.description':
    '{path} diventa un’immagine a un solo layer, senza cronologia né impostazioni proprie; indica un comando qui sotto per renderla eseguibile.',
  'images.importDialog.name': 'Nome dell’immagine',
  'images.importDialog.changes': 'Modifiche',
  'images.importDialog.changesHint':
    'Istruzioni Dockerfile, una per riga: CMD, ENTRYPOINT, ENV, EXPOSE, LABEL, ONBUILD, USER, VOLUME o WORKDIR.',
  'images.importDialog.import': 'Importa',

  'images.inspect.layers': 'Layer',
  'images.inspect.raw': 'JSON originale',
  'images.inspect.created': 'Creato',
  'images.inspect.instruction': 'Istruzione',
  'images.inspect.size': 'Dimensione',

  'images.pullDialog.title': 'Scarica un’immagine',
  'images.pullDialog.description':
    'Cerca su Docker Hub per nome, o digita qualsiasi riferimento raggiungibile dall’host, per esempio nginx:1.27 o ghcr.io/org/app:v2.',
  'images.pullDialog.placeholder': 'postgres, o nginx:1.27',
  'images.pullDialog.reference': 'Riferimento dell’immagine',
  'images.pullDialog.pull': 'Scarica',
  'images.done': 'Fatto.',

  'images.pushDialog.title': 'Invia {name}',
  'images.pushDialog.description':
    'Al registro con cui inizia il nome, per esempio ghcr.io/org/app:v2, o a Docker Hub per nomi come org/app:v2.',
  'images.pushDialog.target': 'Nome da inviare',
  'images.pushDialog.push': 'Invia',
  'images.pushDialog.tagsFirst':
    'All’immagine viene prima assegnato questo nome, come fa Aggiungi tag.',
  'images.pushDialog.uploading': 'Caricamento {current} / {total}',
  'images.push.pushed': '{pushed, plural, one {# layer inviato} other {# layer inviati}}',
  'images.push.pushedAndExisting':
    '{pushed, plural, one {# layer inviato} other {# layer inviati}}, {existing} già nel registro',

  'images.tagDialog.title': 'Aggiungi tag a {name}',
  'images.tagDialog.newName': 'Nuovo nome',
  'images.tagDialog.add': 'Aggiungi tag',

  'images.transfer.savingOne': 'Salvataggio di {name}',
  'images.transfer.saving': 'Salvataggio di {count, plural, one {# immagine} other {# immagini}}',
  'images.transfer.to': 'In {path}',
  'images.transfer.from': 'Da {path}',
  'images.transfer.written': 'Scritti',
  'images.transfer.sent': 'Inviati',
  'images.transfer.loading': 'Caricamento delle immagini',
  'images.transfer.importing': 'Importazione di {reference}',
  'images.transfer.doneWith': 'Fatto: {images}',
  'images.palette.pull': 'Scarica un’immagine',
} satisfies Translation<typeof en>;
