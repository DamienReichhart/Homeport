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
} as const;
