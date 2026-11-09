/** Italian text of the `clusters` namespace. */
import type en from '../en/clusters';
import type { Translation } from '../types';

export default {
  'clusters.title': 'Cluster',
  'clusters.summary': '{count, plural, one {# cluster} other {# cluster}}',
  'clusters.add': 'Aggiungi un cluster',
  'clusters.notFound': 'Questo cluster non esiste più.',
  'clusters.empty.title': 'Ancora nessun cluster',
  'clusters.empty.description':
    'Aggiungi i cluster Kubernetes raggiunti dal tuo kubeconfig. Homeport non installa nulla su di essi.',
  'clusters.column.name': 'Nome',
  'clusters.column.status': 'Stato',
  'clusters.column.kind': 'Tipo',
  'clusters.column.server': 'API',
  'clusters.column.auth': 'Accede con',
  'clusters.palette.add': 'Aggiungi un cluster',
  'clusters.palette.manage': 'Gestisci i cluster',

  'clusters.auth.none': 'Nessuna credenziale',
  'clusters.auth.clientCertificate': 'Certificato client',
  'clusters.auth.token': 'Token',
  'clusters.auth.basic': 'Nome utente e password',
  'clusters.auth.exec': 'Plugin exec ({command})',
  'clusters.auth.oidc': 'OpenID Connect',
  'clusters.auth.aclToken': 'Token ACL',

  'clusters.addPage.description':
    'Ogni contesto Kubernetes che scegli, o ogni agente Nomad che indichi, diventa un cluster. Le sue credenziali sono memorizzate cifrate, mai in chiaro sul disco.',
  'clusters.addPage.fromMachine': 'Dal kubeconfig di questo computer',
  'clusters.addPage.paste': 'Incolla un kubeconfig',
  'clusters.addPage.nomad': 'Nomad',
  'clusters.paste.description':
    'Il kubeconfig viene letto in memoria e mai salvato come file. I file a cui fa riferimento devono essere percorsi completi.',
  'clusters.paste.label': 'Kubeconfig',
  'clusters.paste.read': 'Leggi i contesti',

  'clusters.import.readFrom': 'Letto da {paths}',
  'clusters.import.noContexts': 'Questo kubeconfig non ha contesti',
  'clusters.import.pick': 'Importa {context}',
  'clusters.import.name': 'Nome del cluster creato da {context}',
  'clusters.import.current': 'attuale',
  'clusters.import.added': 'Già aggiunto come {name}.',
  'clusters.import.refresh': 'Aggiornalo',
  'clusters.import.submit':
    '{count, plural, =0 {Importa} one {Importa # cluster} other {Importa # cluster}}',
  'clusters.import.done': '{count, plural, one {# cluster aggiunto} other {# cluster aggiunti}}',
  'clusters.import.problem.empty': 'Dai un nome a ogni cluster.',
  'clusters.import.problem.duplicate': 'Due cluster si chiamerebbero «{name}».',
  'clusters.import.problem.taken': 'Esiste già un cluster chiamato «{name}».',

  'clusters.detail.enabled': 'Attivato',
  'clusters.detail.reconnect': 'Riconnetti',
  'clusters.detail.name': 'Nome',
  'clusters.detail.rename': 'Rinomina',
  'clusters.detail.renamed': 'Cluster rinominato',
  'clusters.connection.title': 'Connessione',
  'clusters.connection.server': 'Server API',
  'clusters.connection.address': 'Indirizzo',
  'clusters.connection.region': 'Regione',
  'clusters.connection.customCa': 'Un proprio certificato CA',
  'clusters.connection.clientCertificate': 'Certificato client',
  'clusters.connection.checkedAs': 'Certificato verificato come {name}',
  'clusters.connection.version': 'Versione',
  'clusters.connection.platform': 'Piattaforma',
  'clusters.connection.namespace': 'Namespace predefinito',
  'clusters.connection.auth': 'Accede con',
  'clusters.connection.tlsServerName': 'Nome del certificato',
  'clusters.connection.tls': 'TLS',
  'clusters.connection.insecure':
    'Il certificato del server non viene verificato (insecure-skip-tls-verify)',
  'clusters.connection.proxy': 'Proxy',
  'clusters.connection.source': 'Importato da',
  'clusters.source.machine': 'Contesto {context} del kubeconfig di questo computer',
  'clusters.source.pasted': 'Contesto {context} di un kubeconfig incollato',

  'clusters.nomad.description':
    'Homeport comunica con l’API HTTP di un agente Nomad, server o client, da questo computer. Sul cluster non viene installato nulla, e qui non serve la CLI nomad.',
  'clusters.nomad.name': 'Nome',
  'clusters.nomad.address': 'Indirizzo',
  'clusters.nomad.addressHelp':
    'L’API HTTP dell’agente, come la fornisce NOMAD_ADDR alla CLI nomad. Porta 4646 se non ne indichi una.',
  'clusters.nomad.region': 'Regione',
  'clusters.nomad.regionHelp': 'Lascia vuoto per usare la regione dell’agente.',
  'clusters.nomad.token': 'Token ACL',
  'clusters.nomad.tokenHelp':
    'Il secret ID del token, quando il cluster ha le ACL attivate. La documentazione di Homeport fornisce una policy per il token.',
  'clusters.nomad.tls': 'Impostazioni TLS',
  'clusters.nomad.tlsHelp':
    'Solo per un agente che serve HTTPS con una propria CA, o che richiede un certificato client. Incolla il contenuto dei file PEM.',
  'clusters.nomad.ca': 'Certificato CA',
  'clusters.nomad.caHelp':
    'La CA che ha firmato il certificato dell’agente, come la indica NOMAD_CACERT. Le CA di questo computer sono attendibili senza di essa.',
  'clusters.nomad.clientCertificate': 'Certificato client',
  'clusters.nomad.clientKey': 'Chiave client',
  'clusters.nomad.tlsServerName': 'Nome del server TLS',
  'clusters.nomad.tlsServerNameHelp':
    'Il nome con cui viene verificato il certificato dell’agente, se diverso da quello dell’indirizzo, come lo imposta NOMAD_TLS_SERVER_NAME.',
  'clusters.nomad.keep': 'Salvato; lascia vuoto per mantenerlo',
  'clusters.nomad.willBeRemoved': 'Verrà rimosso al salvataggio',
  'clusters.nomad.remove': 'Rimuovi',
  'clusters.nomad.keepIt': 'Mantieni',
  'clusters.nomad.test': 'Verifica la connessione',
  'clusters.nomad.testing': 'Verifica in corso…',
  'clusters.nomad.testedVersion': 'Connesso a Nomad {version}, regione {region}.',
  'clusters.nomad.testedNoVersion':
    'Connesso alla regione {region}. Il token non può leggere l’agente, quindi la sua versione non è nota.',
  'clusters.nomad.add': 'Aggiungi cluster',
  'clusters.nomad.added': '{name} aggiunto',
  'clusters.nomad.settings': 'Impostazioni',
  'clusters.nomad.settingsDescription':
    'Il salvataggio riconnette. Il token e la chiave TLS non vengono più mostrati: lasciali vuoti per mantenerli.',
  'clusters.nomad.save': 'Salva le modifiche',
  'clusters.nomad.saved': 'Cluster salvato',

  'clusters.refresh.title': 'Aggiorna dal kubeconfig',
  'clusters.refresh.machine':
    'Legge di nuovo il contesto {context} dal kubeconfig di questo computer, per quando il cluster è stato ricreato o le sue credenziali rinnovate.',
  'clusters.refresh.pasted':
    'Incolla di nuovo il kubeconfig per rileggere il contesto {context}, per quando il cluster è stato ricreato o le sue credenziali rinnovate.',
  'clusters.refresh.submit': 'Aggiorna',
  'clusters.refresh.done': 'Cluster aggiornato dal suo kubeconfig',

  'clusters.remove.title': 'Rimuovi il cluster',
  'clusters.remove.description':
    'Homeport dimentica il cluster e le sue credenziali memorizzate. Non cambia nulla sul cluster né nel tuo kubeconfig.',
  'clusters.remove.confirmTitle': 'Rimuovere {name}?',
  'clusters.remove.confirmDescription': 'Puoi aggiungerlo di nuovo dal suo kubeconfig.',
} satisfies Translation<typeof en>;
