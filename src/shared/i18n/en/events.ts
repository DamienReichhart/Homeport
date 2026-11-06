/** English text of the `events` namespace; every key starts with `events.`. */
export default {
  'events.nav': 'Events',
  'events.title': 'Events',
  'events.description': 'What the Engine reported, kept across restarts.',
  'events.descriptionKept':
    'What the Engine reported, kept for {days, plural, one {# day} other {# days}}, up to 10,000.',
  'events.clear': 'Clear history',
  'events.clear.title': 'Clear the event history?',
  'events.clear.description':
    "Every event kept for this Host is forgotten. New ones keep appearing; the Engine's own aren't affected.",
  'events.clear.confirm': 'Clear history',
  'events.cleared': 'Cleared the event history',
  'events.loadOlder': 'Load older events',
  'events.search': 'Search events',
  'events.type': 'Type',
  'events.allTypes': 'All types',
  'events.empty.title': 'No events yet',
  'events.empty.description': 'Start or stop a container and it shows up here.',
  'events.column.time': 'Time',
  'events.column.type': 'Type',
  'events.column.action': 'Action',
  'events.column.object': 'Object',
  'events.column.stack': 'Stack',
} as const;
