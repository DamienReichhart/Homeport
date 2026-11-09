/** Brazilian Portuguese text of the `events` namespace. */
import type en from '../en/events';
import type { Translation } from '../types';

export default {
  'events.nav': 'Eventos',
  'events.title': 'Eventos',
  'events.description': 'O que o Engine informou, mantido entre reinicializações.',
  'events.descriptionKept':
    'O que o Engine informou, mantido por {days, plural, one {# dia} other {# dias}}, até 10.000.',
  'events.clear': 'Limpar histórico',
  'events.clear.title': 'Limpar o histórico de eventos?',
  'events.clear.description':
    'Todos os eventos mantidos para este host são esquecidos. Novos continuam aparecendo; os do próprio Engine não são afetados.',
  'events.clear.confirm': 'Limpar histórico',
  'events.cleared': 'Histórico de eventos limpo',
  'events.loadOlder': 'Carregar eventos mais antigos',
  'events.search': 'Pesquisar eventos',
  'events.type': 'Tipo',
  'events.allTypes': 'Todos os tipos',
  'events.empty.title': 'Nenhum evento ainda',
  'events.empty.description': 'Inicie ou pare um contêiner e ele aparece aqui.',
  'events.column.time': 'Hora',
  'events.column.type': 'Tipo',
  'events.column.action': 'Ação',
  'events.column.object': 'Objeto',
  'events.column.stack': 'Stack',
} satisfies Translation<typeof en>;
