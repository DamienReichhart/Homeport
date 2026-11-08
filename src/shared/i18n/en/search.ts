/** English text of the `search` namespace; every key starts with `search.`. */
export default {
  'search.kind.container': 'Containers',
  'search.kind.stack': 'Stacks',
  'search.kind.image': 'Images',
  'search.kind.volume': 'Volumes',
  'search.kind.network': 'Networks',
  'search.onHostState': '{host} · {state, select, running {running} other {stopped}}',
  'search.showAll': 'Show all {count}',
  'search.skipped': '{count, plural, one {# host} other {# hosts}} not searched (not connected)',
} as const;
