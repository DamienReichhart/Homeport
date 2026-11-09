/** German text of the `events` namespace. */
import type en from '../en/events';
import type { Translation } from '../types';

export default {
  'events.nav': 'Ereignisse',
  'events.title': 'Ereignisse',
  'events.description': 'Was die Engine gemeldet hat, über Neustarts hinweg aufbewahrt.',
  'events.descriptionKept':
    'Was die Engine gemeldet hat, {days, plural, one {# Tag} other {# Tage}} lang aufbewahrt, bis zu 10.000.',
  'events.clear': 'Verlauf löschen',
  'events.clear.title': 'Ereignisverlauf löschen?',
  'events.clear.description':
    'Alle für diesen Host aufbewahrten Ereignisse werden vergessen. Neue erscheinen weiterhin; die der Engine selbst sind nicht betroffen.',
  'events.clear.confirm': 'Verlauf löschen',
  'events.cleared': 'Ereignisverlauf gelöscht',
  'events.loadOlder': 'Ältere Ereignisse laden',
  'events.search': 'Ereignisse durchsuchen',
  'events.type': 'Typ',
  'events.allTypes': 'Alle Typen',
  'events.empty.title': 'Noch keine Ereignisse',
  'events.empty.description': 'Starten oder stoppen Sie einen Container, dann erscheint er hier.',
  'events.column.time': 'Zeit',
  'events.column.type': 'Typ',
  'events.column.action': 'Aktion',
  'events.column.object': 'Objekt',
  'events.column.stack': 'Stack',
} satisfies Translation<typeof en>;
