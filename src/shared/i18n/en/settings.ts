/** English text of the `settings` namespace; every key starts with `settings.`. */
export default {
  'settings.nav': 'Settings',
  'settings.title': 'Settings',
  'settings.appearance.title': 'Appearance',
  'settings.appearance.theme': 'Theme',
  'settings.theme.system': 'Match the system',
  'settings.theme.light': 'Light',
  'settings.theme.dark': 'Dark',
  'settings.appearance.language': 'Language',
  'settings.language.system': 'Match the system ({language})',
  'settings.desktop.title': 'Desktop',
  'settings.desktop.description': 'How Homeport runs on this computer.',
  'settings.desktop.closeToTray': 'Close to the system tray',
  'settings.desktop.closeToTrayHint':
    'Closing the window keeps Homeport and its Host connections running in the tray. Quit from the tray icon. On desktops that hide tray icons, such as GNOME without an extension, the icon may not show.',
  'settings.desktop.noTray':
    'This desktop has no system tray Homeport can use, so closing the window quits it.',
  'settings.desktop.startAtLogin': 'Start at login',
  'settings.desktop.startAtLoginHint':
    'Homeport starts when you log in to this computer, so it watches your Hosts from the start.',
  'settings.desktop.startHidden': 'Start hidden in the tray',
  'settings.desktop.startHiddenHint':
    'When it starts at login, Homeport stays in the system tray without opening its window.',
  'settings.desktop.startHiddenNoTray':
    'Needs a system tray to open the window from, which this desktop doesn’t have.',
  'settings.notifications.title': 'Notifications',
  'settings.notifications.description':
    'Homeport tells you, even with its window closed to the tray, when:',
  'settings.notifications.containerDied': 'A container stops with an error or runs out of memory',
  'settings.notifications.containerUnhealthy': 'A container turns unhealthy',
  'settings.notifications.hostUnreachable': 'A connected host becomes unreachable',
  'settings.notifications.hostBack': 'An unreachable host is back',
  'settings.machine.title': 'This machine',
  'settings.machine.description': 'What Homeport can use here.',
  'settings.machine.keychain': 'OS keychain',
  'settings.machine.keychainAvailable':
    'Available — SSH keys, certificates, stack environments and registry logins are stored encrypted.',
  'settings.machine.keychainUnavailable':
    'Unavailable — SSH keys, TLS certificates, stack environments and registry logins can’t be stored.',
  'settings.machine.keychainEntryMissing':
    'Entry missing — the keychain has no entry for the key Homeport’s secrets were sealed with, so they can’t be opened or stored.',
  'settings.machine.startOverDescription':
    'If Homeport is using another keychain than before, switch back to it and restart Homeport. If the entry is gone for good, start over with a new key.',
  'settings.machine.startOver': 'Start over with a new key',
  'settings.machine.startOverTitle': 'Start over with a new key?',
  'settings.machine.startOverConfirmDescription':
    'A new key can’t open what was sealed with the missing one: you’ll re-enter the credentials of each host, cluster, registry and repository, and each stack’s environment, or restore a backup.',
  'settings.machine.compose': 'Compose CLI',
  'settings.machine.composeMissing': 'Stacks can be started and stopped, but not deployed.',
  'settings.machine.stackDeploy': 'Docker CLI, for Swarms',
  'settings.machine.stackDeployMissing':
    'Stacks can’t be deployed to a Swarm; everything else about Swarms works.',
  'settings.machine.git': 'git',
  'settings.machine.helm': 'Helm CLI',
  'settings.sync.title': 'Synced Stacks',
  'settings.sync.interval': 'Check repositories every',
  'settings.sync.minutes': 'minutes',
  'settings.data.title': 'Data',
  'settings.data.description': 'What Homeport keeps on this computer.',
  'settings.data.backup': 'Back up…',
  'settings.data.restore': 'Restore…',
  'settings.data.backupHint':
    'Save every Host, Stack, registry login, repository and setting, with their secrets, to one passphrase-protected file, and restore it on this or another computer.',
  'settings.backup.title': "Back up Homeport's data",
  'settings.backup.description':
    "The file holds every Host, Cluster, Stack, registry login, repository and setting, and their keys, certificates, tokens, passwords and environments. Anyone with it and its passphrase can use them, so keep both safe. Engine events and deploy history aren't included.",
  'settings.backup.passphrase': 'Passphrase',
  'settings.backup.passphraseHint': "At least {min} characters. It can't be recovered.",
  'settings.backup.confirmation': 'Passphrase again',
  'settings.backup.problem.short': 'Use at least {min} characters.',
  'settings.backup.problem.mismatch': 'The two passphrases differ.',
  'settings.backup.start': 'Back up…',
  'settings.backup.saveTitle': 'Save the backup as',
  'settings.backup.fileType': 'Homeport backup',
  'settings.backup.done':
    'Backed up {hosts, plural, one {# Host} other {# Hosts}} and {stacks, plural, one {# Stack} other {# Stacks}}',
  'settings.backup.withoutSecrets':
    "These were backed up without their secrets, which couldn't be opened:",
  'settings.restore.pick': 'Restore from',
  'settings.restore.title': "Restore Homeport's data",
  'settings.restore.from': 'From {path}',
  'settings.restore.open': 'Open',
  'settings.restore.madeBy': 'Backed up by Homeport {version} on {date}. It holds:',
  'settings.restore.holds': 'What the backup holds',
  'settings.restore.hosts': '{count, plural, one {# Host} other {# Hosts}}',
  'settings.restore.clusters': '{count, plural, one {# Cluster} other {# Clusters}}',
  'settings.restore.stacks': '{count, plural, one {# Stack} other {# Stacks}}',
  'settings.restore.registries':
    '{count, plural, one {# registry login} other {# registry logins}}',
  'settings.restore.repositories': '{count, plural, one {# repository} other {# repositories}}',
  'settings.restore.settings': 'Your settings',
  'settings.restore.withoutSecrets':
    "These were backed up without their secrets, which couldn't be opened then. They come back without them, replacing the secrets they have here, and registry logins among them don't come back at all:",
  'settings.restore.withoutSecretsList': 'Backed up without their secrets',
  'settings.restore.replaces':
    "Restoring replaces every Host, Stack, registry login, repository and setting you have now, and forgets the Engine events and deploy history. A copy of the current data is kept in Homeport's data folder first.",
  'settings.restore.confirm': 'Replace everything',
  'settings.restore.done':
    'Restored {hosts, plural, one {# Host} other {# Hosts}} and {stacks, plural, one {# Stack} other {# Stacks}}',
  'settings.data.keepEvents': 'Keep Engine events for',
  'settings.data.days': 'days',
  'settings.data.keepEventsHint':
    'Each Host keeps at most 10,000 events for the Events page. Older ones are removed every hour.',
  'settings.machine.gitMissing':
    'Synced Stacks need it; every other kind of Stack works without it.',
  'settings.machine.helmMissing':
    'Helm releases can’t be listed or installed; everything else about Clusters works.',
  'settings.about.tagline': 'Every Docker Engine, on one board.',
  'settings.about.openLogFolder': 'Open log folder',
  'settings.about.licenses': 'Licenses',
  'settings.about.exportDiagnostics': 'Export diagnostics…',
  'settings.diagnostics.title': 'Export diagnostics',
  'settings.diagnostics.description':
    'A zip file to attach to a bug report: a summary of versions and of the kinds of hosts and clusters you use, and Homeport’s logs with secrets removed. Nothing is sent anywhere: you decide whether to share the file.',
  'settings.diagnostics.includeNames': 'Include names and addresses',
  'settings.diagnostics.includeNamesHelp':
    'When off, hosts and clusters appear as host-1, cluster-1… in the summary and in the logs.',
  'settings.diagnostics.files': 'Files in the zip',
  'settings.diagnostics.save': 'Save…',
  'settings.diagnostics.saveTitle': 'Save diagnostics',
  'settings.diagnostics.fileType': 'Zip archive',
  'settings.diagnostics.done': 'Diagnostics saved to {path}.',
  'settings.licenses.title': 'Licenses',
  'settings.licenses.description':
    'Homeport is free software under the GNU Affero General Public License, version 3, built on the open-source packages below.',
  'settings.licenses.source':
    'The source code of this version is published under the same licence.',
  'settings.licenses.sourceLink': 'Source of v{version}',
  'settings.licenses.thirdParty': 'Third-party software',
  'settings.licenses.missing.title': 'Not included in this build',
  'settings.licenses.missing.description':
    'Release builds bundle them; run <code>{command}</code> to generate them for a development build.',
  'settings.palette.group': 'Settings',
  'settings.palette.themeLight': 'Theme: Light',
  'settings.palette.themeDark': 'Theme: Dark',
  'settings.palette.themeSystem': 'Theme: Match the system',
} as const;
