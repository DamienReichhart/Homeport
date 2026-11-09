/** Spanish text of the `events` namespace. */
import type en from '../en/events';
import type { Translation } from '../types';

export default {
  'events.nav': 'Eventos',
  'events.title': 'Eventos',
  'events.description': 'Lo que informó el Engine, conservado entre reinicios.',
  'events.descriptionKept':
    'Lo que informó el Engine, conservado durante {days, plural, one {# día} other {# días}}, hasta 10.000.',
  'events.clear': 'Borrar historial',
  'events.clear.title': '¿Borrar el historial de eventos?',
  'events.clear.description':
    'Se olvidan todos los eventos guardados para este host. Seguirán apareciendo eventos nuevos; los del propio Engine no se ven afectados.',
  'events.clear.confirm': 'Borrar historial',
  'events.cleared': 'Se borró el historial de eventos',
  'events.loadOlder': 'Cargar eventos anteriores',
  'events.search': 'Buscar eventos',
  'events.type': 'Tipo',
  'events.allTypes': 'Todos los tipos',
  'events.empty.title': 'Aún no hay eventos',
  'events.empty.description': 'Inicia o detén un contenedor y aparecerá aquí.',
  'events.column.time': 'Hora',
  'events.column.type': 'Tipo',
  'events.column.action': 'Acción',
  'events.column.object': 'Objeto',
  'events.column.stack': 'Stack',
} satisfies Translation<typeof en>;
