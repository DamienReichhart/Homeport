/** English text of the `app` namespace; every key starts with `app.`. */
export default {
  'app.nav.main': 'Main',
  'app.keychainMissing':
    'No OS keychain is available, so SSH keys, TLS certificates and stack environments can’t be stored. Local, TCP and ssh-agent hosts still work.',
  'app.crash.title': 'Something went wrong',
  'app.crash.description':
    'Homeport hit an unexpected problem on this page. Reloading usually helps; if it keeps happening, copy the report from Details into an issue.',
  'app.crash.reload': 'Reload',
  'app.crash.backToOverview': 'Back to overview',
  'app.shortcuts.title': 'Keyboard shortcuts',
  'app.shortcuts.description':
    'The shortcuts that work here. Keys like N and / don’t fire while you type in a field.',
  'app.shortcuts.general': 'General',
  'app.shortcuts.navigation': 'Go to',
  'app.shortcuts.hosts': 'Hosts',
  'app.shortcuts.help': 'Show keyboard shortcuts',
  'app.shortcuts.settings': 'Open Settings',
  'app.shortcuts.goTo': 'Go to {page}',
  'app.shortcuts.previousHost': 'Previous host',
  'app.shortcuts.nextHost': 'Next host',
  'app.shortcuts.refresh': 'Refresh this host’s lists',
} as const;
