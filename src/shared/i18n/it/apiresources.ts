/** Italian text of the `apiresources` namespace. */
import type en from '../en/apiresources';
import type { Translation } from '../types';

export default {
  'apiresources.nav': 'Risorse API',
  'apiresources.title': 'Risorse API',
  'apiresources.summary':
    '{count, plural, one {# tipo servito} other {# tipi serviti}} da questo cluster',
  'apiresources.refresh': 'Aggiorna',
  'apiresources.types': 'Tipi di risorsa',
  'apiresources.search': 'Kind o nome breve (pvc)',
  'apiresources.searchObjects': 'Cerca',
  'apiresources.noMatch': 'Nessun tipo corrisponde.',
  'apiresources.coreGroup': 'core',
  'apiresources.pick':
    'Scegli un tipo per elencarne gli oggetti: quelli predefiniti e quelli aggiunti dai tuoi CRD.',
  'apiresources.typeGone': 'Questo cluster non serve (più) quel tipo.',
  'apiresources.cantList': 'Non elencabile',
  'apiresources.notAllowed': 'Le tue credenziali non permettono di elencarlo',
  'apiresources.cantListWhy':
    'Gli oggetti {kind} non possono essere elencati: la sua API permette solo di crearli o leggerli uno alla volta.',
  'apiresources.empty': 'Niente qui.',
  'apiresources.column.name': 'Nome',
  'apiresources.column.namespace': 'Namespace',
  'apiresources.column.age': 'Età',
  'apiresources.tab.overview': 'Panoramica',
  'apiresources.tab.conditions': 'Condizioni',
  'apiresources.tab.events': 'Eventi',
  'apiresources.tab.source': 'YAML',
  'apiresources.detail.namespace': 'Namespace',
  'apiresources.detail.owners': 'Di proprietà di',
  'apiresources.detail.age': 'Età',
  'apiresources.detail.labels': 'Etichette',
  'apiresources.detail.annotations': 'Annotazioni',
  'apiresources.edit': 'Modifica',
  'apiresources.copyYaml': 'Copia YAML',
} satisfies Translation<typeof en>;
