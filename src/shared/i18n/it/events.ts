/** Italian text of the `events` namespace. */
import type en from '../en/events';
import type { Translation } from '../types';

export default {
  'events.nav': 'Eventi',
  'events.title': 'Eventi',
  'events.description': 'Ciò che l’Engine ha segnalato, conservato anche dopo i riavvii.',
  'events.descriptionKept':
    'Ciò che l’Engine ha segnalato, conservato per {days, plural, one {# giorno} other {# giorni}}, fino a 10.000 eventi.',
  'events.clear': 'Cancella la cronologia',
  'events.clear.title': 'Cancellare la cronologia degli eventi?',
  'events.clear.description':
    'Tutti gli eventi conservati per questo host vengono dimenticati. Quelli nuovi continuano ad apparire; quelli dell’Engine non vengono toccati.',
  'events.clear.confirm': 'Cancella la cronologia',
  'events.cleared': 'Cronologia degli eventi cancellata',
  'events.loadOlder': 'Carica eventi precedenti',
  'events.search': 'Cerca negli eventi',
  'events.type': 'Tipo',
  'events.allTypes': 'Tutti i tipi',
  'events.empty.title': 'Ancora nessun evento',
  'events.empty.description': 'Avvia o arresta un container e comparirà qui.',
  'events.column.time': 'Ora',
  'events.column.type': 'Tipo',
  'events.column.action': 'Azione',
  'events.column.object': 'Oggetto',
  'events.column.stack': 'Stack',
} satisfies Translation<typeof en>;
