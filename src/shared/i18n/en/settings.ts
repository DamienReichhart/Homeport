/** English text of the `settings` namespace; every key starts with `settings.`. */
export default {
  'settings.nav': 'Settings',
  'settings.title': 'Settings',
  'settings.appearance.title': 'Appearance',
  'settings.appearance.theme': 'Theme',
  'settings.theme.system': 'Match the system',
  'settings.theme.light': 'Light',
  'settings.theme.dark': 'Dark',
  'settings.machine.title': 'This machine',
  'settings.machine.description': 'What Homeport can use here.',
  'settings.machine.keychain': 'OS keychain',
  'settings.machine.keychainAvailable':
    'Available — SSH keys, certificates, stack environments and registry logins are stored encrypted.',
  'settings.machine.keychainUnavailable':
    'Unavailable — SSH keys, TLS certificates, stack environments and registry logins can’t be stored.',
  'settings.machine.compose': 'Compose CLI',
  'settings.machine.composeMissing': 'Stacks can be started and stopped, but not deployed.',
  'settings.about.tagline': 'Every Docker Engine, on one board.',
  'settings.about.openLogFolder': 'Open log folder',
  'settings.about.licenses': 'Licenses',
  'settings.licenses.title': 'Licenses',
  'settings.licenses.description':
    'Homeport is free software under the MIT License, built on the open-source packages below.',
  'settings.licenses.thirdParty': 'Third-party software',
  'settings.licenses.missing.title': 'Not included in this build',
  'settings.licenses.missing.description':
    'Release builds bundle them; run <code>{command}</code> to generate them for a development build.',
} as const;
