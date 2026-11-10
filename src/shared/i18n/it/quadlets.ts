/** Italian text of the `quadlets` namespace. */
import type en from '../en/quadlets';
import type { Translation } from '../types';

export default {
  'quadlets.nav': 'Quadlet',
  'quadlets.title': 'Unità Quadlet',
  'quadlets.summary': '{count, plural, one {# unità} other {# unità}}',
  'quadlets.search': 'Cerca unità',
  'quadlets.empty': 'Nessuna unità Quadlet',
  'quadlets.emptyDescription':
    'Le unità Quadlet sono file che Podman trasforma in servizi systemd, così i container si avviano con l’host e systemd li mantiene in esecuzione. Installane una per vederla qui.',
  'quadlets.new': 'Nuova unità',
  'quadlets.edit': 'Modifica',
  'quadlets.notGenerated': 'non ancora generato',
  'quadlets.gone': '{name} non è più installata',
  'quadlets.column.name': 'Unità',
  'quadlets.column.kind': 'Tipo',
  'quadlets.column.service': 'Servizio',
  'quadlets.column.status': 'Stato',
  'quadlets.column.containers': 'Container',
  'quadlets.status.running': 'In esecuzione',
  'quadlets.status.starting': 'Avvio in corso',
  'quadlets.status.stopping': 'Arresto in corso',
  'quadlets.status.stopped': 'Arrestato',
  'quadlets.status.failed': 'Non riuscito',
  'quadlets.status.notLoaded': 'Non caricato da systemd',
  'quadlets.status.template': 'Modello',
  'quadlets.action.start': 'Avvia',
  'quadlets.action.stop': 'Arresta',
  'quadlets.action.restart': 'Riavvia',
  'quadlets.actionOne.start': 'Avvia {name}',
  'quadlets.actionOne.stop': 'Arresta {name}',
  'quadlets.actionOne.restart': 'Riavvia {name}',
  'quadlets.done.start': '{name} avviata',
  'quadlets.done.stop': '{name} arrestata',
  'quadlets.done.restart': '{name} riavviata',
  'quadlets.installed': '{name} installata',
  'quadlets.saved': '{name} salvata',
  'quadlets.removed': '{name} rimossa',
  'quadlets.blocked.noCommands':
    'Homeport avvia e arresta le unità con systemctl, che può eseguire solo tramite SSH o per un Podman su questo computer.',
  'quadlets.blocked.notLoaded':
    'systemd non ha generato il servizio di questa unità, quindi non c’è nulla da avviare.',
  'quadlets.blocked.podmanTooOld':
    'Leggere, installare, modificare e rimuovere unità richiede Podman 5.8 o successivo; questo host esegue Podman {version}.',
  'quadlets.blocked.noFile':
    'Questa unità è nota solo tramite i suoi container, quindi il suo file non può essere modificato.',
  'quadlets.note.labels':
    'Podman {version} non può elencare le unità Quadlet, quindi queste provengono dai container che hanno creato. I loro file, e le unità che non hanno creato alcun container, compaiono con Podman 5.7 e successivi.',
  'quadlets.note.readOnly':
    'Podman {version} elenca le unità ma non può leggerle, installarle, modificarle o rimuoverle; per farlo serve Podman 5.8 o successivo.',
  'quadlets.note.noCommands':
    'Per avviare e arrestare le unità, questo host deve essere raggiunto tramite SSH, oppure deve trattarsi di un Podman su questo computer.',
  'quadlets.detail.service': 'Servizio',
  'quadlets.detail.unit': 'Unità systemd',
  'quadlets.detail.scope': 'Eseguita come',
  'quadlets.detail.path': 'File',
  'quadlets.detail.app': 'Applicazione',
  'quadlets.detail.pod': 'Pod',
  'quadlets.detail.containers':
    '{count, plural, =0 {Container} one {# container} other {# container}}',
  'quadlets.detail.noContainers':
    'Non ha creato alcun container, oppure il suo servizio è arrestato.',
  'quadlets.scope.user': 'Un’unità utente (systemctl --user)',
  'quadlets.scope.system': 'Un’unità di sistema',
  'quadlets.file.title': 'File dell’unità',
  'quadlets.file.unknown': 'Questa unità è nota solo tramite l’etichetta dei suoi container.',
  'quadlets.file.tooOld':
    'Podman {version} non può leggere i file delle unità; per mostrarli serve Podman 5.8 o successivo.',
  'quadlets.form.newTitle': 'Nuova unità Quadlet',
  'quadlets.form.newDescription':
    'Podman scrive il file dove vanno le unità del suo utente e ricarica systemd. Avvia l’unità una volta installata.',
  'quadlets.form.editTitle': 'Modifica {name}',
  'quadlets.form.editDescription':
    'Il salvataggio sostituisce il file e ricarica systemd; riavvia l’unità perché un servizio in esecuzione lo usi. I file installati accanto restano.',
  'quadlets.form.name': 'Nome del file',
  'quadlets.form.kind': 'Tipo',
  'quadlets.form.openFile': 'Apri file…',
  'quadlets.form.openTitle': 'Apri un file di unità Quadlet',
  'quadlets.form.unitFiles': 'File di unità Quadlet',
  'quadlets.form.content': 'File dell’unità',
  'quadlets.form.extra': 'File accanto',
  'quadlets.form.extraHint':
    'Un Containerfile, uno YAML kube o un file env usato dall’unità, installato nella sua cartella dell’applicazione.',
  'quadlets.form.extraTitle': 'Scegli i file da installare accanto all’unità',
  'quadlets.form.addExtra': 'Aggiungi file…',
  'quadlets.form.removeExtra': 'Rimuovi {name}',
  'quadlets.form.install': 'Installa',
  'quadlets.form.save': 'Salva',
  'quadlets.form.overwrite': 'Sovrascrivi',
  'quadlets.form.nameInvalid':
    'Chiamala come web.container, con estensione .container, .pod, .volume, .network, .kube, .image, .build o .artifact.',
  'quadlets.form.sectionMissing':
    'Il file deve avere una sezione {section} perché Quadlet possa leggerlo.',
  'quadlets.form.extraInvalid':
    'Ogni file accanto all’unità deve avere un nome semplice proprio, senza cartelle.',
  'quadlets.remove.title': 'Rimuovere {name}?',
  'quadlets.remove.description':
    'Podman elimina il file dell’unità e ricarica systemd. I container, i volumi e le immagini che ha creato restano.',
  'quadlets.remove.stop': 'Arresta il suo servizio se è in esecuzione',
  'quadlets.remove.stopRunning': 'Arresta prima il suo servizio in esecuzione',
} satisfies Translation<typeof en>;
