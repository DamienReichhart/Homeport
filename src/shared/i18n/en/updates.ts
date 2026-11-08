/** English text of the `updates` namespace; every key starts with `updates.`. */
export default {
  'updates.title': 'Updates',
  'updates.description': 'Homeport can look for a new version when it starts and every few hours.',
  'updates.automatic': 'Check for updates automatically',
  'updates.checking': 'Checking…',
  'updates.checkNow': 'Check now',
  'updates.status.available': 'Homeport {version} is available (you have {current}).',
  'updates.status.latest': 'Homeport {current} is the latest version.',

  'updates.notice.label': 'Update available',
  'updates.notice.available': '<strong>Homeport {version}</strong> is available.',
  'updates.notice.availablePackageManager':
    '<strong>Homeport {version}</strong> is available. Update it with your package manager:',
  'updates.notice.dismiss': 'Dismiss',

  'updates.copyCommand': 'Copy command',
  'updates.whatsNew': 'What’s new',
  'updates.install': 'Install and restart',
  'updates.download': 'Download',
  'updates.progress.downloading': 'Downloading…',
  'updates.progress.downloadingPercent': 'Downloading… {percent}%',
  'updates.progress.installing': 'Installing…',

  'updates.method.inApp':
    'Updates are downloaded, checked against Homeport’s signing key and installed when you choose.',
  'updates.method.scoop': 'Installed with Scoop: updates come from the Homeport bucket.',
  'updates.method.aur': 'Installed from the AUR (homeport-bin): your AUR helper installs updates.',
  'updates.method.packageManager':
    'Installed with {manager}: updates come from the Homeport package repository.',
  'updates.method.manual': 'This is a development build: download new versions from the website.',
  'updates.palette.check': 'Check for updates',
} as const;
