/** German text of the `apiresources` namespace. */
import type en from '../en/apiresources';
import type { Translation } from '../types';

export default {
  'apiresources.nav': 'API-Ressourcen',
  'apiresources.title': 'API-Ressourcen',
  'apiresources.summary':
    '{count, plural, one {# Typ} other {# Typen}}, von diesem Cluster bereitgestellt',
  'apiresources.refresh': 'Aktualisieren',
  'apiresources.types': 'Ressourcentypen',
  'apiresources.search': 'Kind oder Kurzname (pvc)',
  'apiresources.searchObjects': 'Suchen',
  'apiresources.noMatch': 'Kein Typ passt.',
  'apiresources.coreGroup': 'core',
  'apiresources.pick':
    'Wählen Sie einen Typ, um seine Objekte aufzulisten: integrierte und solche, die Ihre CRDs hinzufügen.',
  'apiresources.typeGone': 'Dieser Cluster stellt diesen Typ (nicht mehr) bereit.',
  'apiresources.cantList': 'Nicht auflistbar',
  'apiresources.notAllowed': 'Ihre Zugangsdaten dürfen ihn nicht auflisten',
  'apiresources.cantListWhy':
    '{kind}-Objekte lassen sich nicht auflisten: Ihre API erlaubt nur, sie einzeln zu erstellen oder zu lesen.',
  'apiresources.empty': 'Hier ist nichts.',
  'apiresources.column.name': 'Name',
  'apiresources.column.namespace': 'Namespace',
  'apiresources.column.age': 'Alter',
  'apiresources.tab.overview': 'Übersicht',
  'apiresources.tab.conditions': 'Bedingungen',
  'apiresources.tab.events': 'Events',
  'apiresources.tab.source': 'YAML',
  'apiresources.detail.namespace': 'Namespace',
  'apiresources.detail.owners': 'Gehört zu',
  'apiresources.detail.age': 'Alter',
  'apiresources.detail.labels': 'Labels',
  'apiresources.detail.annotations': 'Annotationen',
  'apiresources.edit': 'Bearbeiten',
  'apiresources.copyYaml': 'YAML kopieren',
} satisfies Translation<typeof en>;
