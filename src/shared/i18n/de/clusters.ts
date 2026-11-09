/** German text of the `clusters` namespace. */
import type en from '../en/clusters';
import type { Translation } from '../types';

export default {
  'clusters.title': 'Cluster',
  'clusters.summary': '{count, plural, one {# Cluster} other {# Cluster}}',
  'clusters.add': 'Cluster hinzufügen',
  'clusters.notFound': 'Dieser Cluster existiert nicht mehr.',
  'clusters.empty.title': 'Noch keine Cluster',
  'clusters.empty.description':
    'Fügen Sie die Kubernetes-Cluster hinzu, die Ihre kubeconfig erreicht. Homeport installiert nichts darauf.',
  'clusters.column.name': 'Name',
  'clusters.column.status': 'Status',
  'clusters.column.kind': 'Art',
  'clusters.column.server': 'API',
  'clusters.column.auth': 'Meldet sich an mit',
  'clusters.palette.add': 'Cluster hinzufügen',
  'clusters.palette.manage': 'Cluster verwalten',

  'clusters.auth.none': 'Keine Zugangsdaten',
  'clusters.auth.clientCertificate': 'Client-Zertifikat',
  'clusters.auth.token': 'Token',
  'clusters.auth.basic': 'Benutzername und Passwort',
  'clusters.auth.exec': 'Exec-Plugin ({command})',
  'clusters.auth.oidc': 'OpenID Connect',
  'clusters.auth.aclToken': 'ACL-Token',

  'clusters.addPage.description':
    'Jeder Kubernetes-Kontext, den Sie wählen, und jeder Nomad-Agent, den Sie angeben, wird zu einem Cluster. Seine Zugangsdaten werden verschlüsselt gespeichert, nie im Klartext auf der Festplatte.',
  'clusters.addPage.fromMachine': 'Aus der kubeconfig dieses Rechners',
  'clusters.addPage.paste': 'kubeconfig einfügen',
  'clusters.addPage.nomad': 'Nomad',
  'clusters.paste.description':
    'Die kubeconfig wird im Arbeitsspeicher gelesen und nie als Datei gespeichert. Dateien, auf die sie verweist, müssen vollständige Pfade sein.',
  'clusters.paste.label': 'Kubeconfig',
  'clusters.paste.read': 'Kontexte lesen',

  'clusters.import.readFrom': 'Gelesen aus {paths}',
  'clusters.import.noContexts': 'Diese kubeconfig hat keine Kontexte',
  'clusters.import.pick': '{context} importieren',
  'clusters.import.name': 'Name des aus {context} erstellten Clusters',
  'clusters.import.current': 'aktuell',
  'clusters.import.added': 'Bereits als {name} hinzugefügt.',
  'clusters.import.refresh': 'Aktualisieren',
  'clusters.import.submit':
    '{count, plural, =0 {Importieren} one {# Cluster importieren} other {# Cluster importieren}}',
  'clusters.import.done':
    '{count, plural, one {# Cluster hinzugefügt} other {# Cluster hinzugefügt}}',
  'clusters.import.problem.empty': 'Geben Sie jedem Cluster einen Namen.',
  'clusters.import.problem.duplicate': 'Zwei Cluster würden „{name}“ heißen.',
  'clusters.import.problem.taken': 'Ein Cluster namens „{name}“ existiert bereits.',

  'clusters.detail.enabled': 'Aktiviert',
  'clusters.detail.reconnect': 'Neu verbinden',
  'clusters.detail.name': 'Name',
  'clusters.detail.rename': 'Umbenennen',
  'clusters.detail.renamed': 'Cluster umbenannt',
  'clusters.connection.title': 'Verbindung',
  'clusters.connection.server': 'API-Server',
  'clusters.connection.address': 'Adresse',
  'clusters.connection.region': 'Region',
  'clusters.connection.customCa': 'Eigenes CA-Zertifikat',
  'clusters.connection.clientCertificate': 'Client-Zertifikat',
  'clusters.connection.checkedAs': 'Zertifikat geprüft als {name}',
  'clusters.connection.version': 'Version',
  'clusters.connection.platform': 'Plattform',
  'clusters.connection.namespace': 'Standard-Namespace',
  'clusters.connection.auth': 'Meldet sich an mit',
  'clusters.connection.tlsServerName': 'Zertifikatsname',
  'clusters.connection.tls': 'TLS',
  'clusters.connection.insecure':
    'Das Zertifikat des Servers wird nicht geprüft (insecure-skip-tls-verify)',
  'clusters.connection.proxy': 'Proxy',
  'clusters.connection.source': 'Importiert aus',
  'clusters.source.machine': 'Kontext {context} der kubeconfig dieses Rechners',
  'clusters.source.pasted': 'Kontext {context} einer eingefügten kubeconfig',

  'clusters.nomad.description':
    'Homeport spricht von diesem Computer aus mit der HTTP-API eines Nomad-Agenten, Server oder Client. Auf dem Cluster wird nichts installiert, und hier wird keine nomad-CLI benötigt.',
  'clusters.nomad.name': 'Name',
  'clusters.nomad.address': 'Adresse',
  'clusters.nomad.addressHelp':
    'Die HTTP-API des Agenten, wie NOMAD_ADDR sie der nomad-CLI angibt. Port 4646, wenn keiner angegeben ist.',
  'clusters.nomad.region': 'Region',
  'clusters.nomad.regionHelp': 'Leer lassen, um die eigene Region des Agenten zu verwenden.',
  'clusters.nomad.token': 'ACL-Token',
  'clusters.nomad.tokenHelp':
    'Die Secret-ID des Tokens, wenn im Cluster ACLs aktiviert sind. Die Dokumentation von Homeport nennt eine Policy dafür.',
  'clusters.nomad.tls': 'TLS-Einstellungen',
  'clusters.nomad.tlsHelp':
    'Nur für einen Agenten, der HTTPS mit einer eigenen CA bereitstellt oder ein Client-Zertifikat verlangt. Fügen Sie den Inhalt der PEM-Dateien ein.',
  'clusters.nomad.ca': 'CA-Zertifikat',
  'clusters.nomad.caHelp':
    'Die CA, die das Zertifikat des Agenten signiert hat, wie NOMAD_CACERT sie angibt. Den CAs dieses Computers wird auch ohne vertraut.',
  'clusters.nomad.clientCertificate': 'Client-Zertifikat',
  'clusters.nomad.clientKey': 'Client-Schlüssel',
  'clusters.nomad.tlsServerName': 'TLS-Servername',
  'clusters.nomad.tlsServerNameHelp':
    'Der Name, gegen den das Zertifikat des Agenten geprüft wird, falls nicht der der Adresse, wie NOMAD_TLS_SERVER_NAME ihn setzt.',
  'clusters.nomad.keep': 'Gespeichert; leer lassen, um es beizubehalten',
  'clusters.nomad.willBeRemoved': 'Wird beim Speichern entfernt',
  'clusters.nomad.remove': 'Entfernen',
  'clusters.nomad.keepIt': 'Beibehalten',
  'clusters.nomad.test': 'Verbindung testen',
  'clusters.nomad.testing': 'Wird getestet…',
  'clusters.nomad.testedVersion': 'Mit Nomad {version} verbunden, Region {region}.',
  'clusters.nomad.testedNoVersion':
    'Mit Region {region} verbunden. Das Token kann den Agenten nicht lesen, daher ist seine Version unbekannt.',
  'clusters.nomad.add': 'Cluster hinzufügen',
  'clusters.nomad.added': '{name} hinzugefügt',
  'clusters.nomad.settings': 'Einstellungen',
  'clusters.nomad.settingsDescription':
    'Beim Speichern wird neu verbunden. Token und TLS-Schlüssel werden nie wieder angezeigt: Lassen Sie sie leer, um sie beizubehalten.',
  'clusters.nomad.save': 'Änderungen speichern',
  'clusters.nomad.saved': 'Cluster gespeichert',

  'clusters.refresh.title': 'Aus kubeconfig aktualisieren',
  'clusters.refresh.machine':
    'Liest den Kontext {context} erneut aus der kubeconfig dieses Rechners, für den Fall, dass der Cluster neu erstellt oder seine Zugangsdaten erneuert wurden.',
  'clusters.refresh.pasted':
    'Fügen Sie die kubeconfig erneut ein, um den Kontext {context} neu zu lesen, für den Fall, dass der Cluster neu erstellt oder seine Zugangsdaten erneuert wurden.',
  'clusters.refresh.submit': 'Aktualisieren',
  'clusters.refresh.done': 'Cluster aus seiner kubeconfig aktualisiert',

  'clusters.remove.title': 'Cluster entfernen',
  'clusters.remove.description':
    'Homeport vergisst den Cluster und seine gespeicherten Zugangsdaten. Am Cluster und in Ihrer kubeconfig ändert sich nichts.',
  'clusters.remove.confirmTitle': '{name} entfernen?',
  'clusters.remove.confirmDescription': 'Sie können ihn aus seiner kubeconfig erneut hinzufügen.',
} satisfies Translation<typeof en>;
