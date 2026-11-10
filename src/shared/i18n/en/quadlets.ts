/** English text of the `quadlets` namespace (Podman's Quadlet units); every key starts with `quadlets.`. */
export default {
  'quadlets.nav': 'Quadlet',
  'quadlets.title': 'Quadlet units',
  'quadlets.summary': '{count, plural, one {# unit} other {# units}}',
  'quadlets.search': 'Search units',
  'quadlets.empty': 'No Quadlet units',
  'quadlets.emptyDescription':
    'Quadlet units are files Podman turns into systemd services, so containers start with the host and systemd keeps them running. Install one to see it here.',
  'quadlets.new': 'New unit',
  'quadlets.edit': 'Edit',
  'quadlets.notGenerated': 'not generated yet',
  'quadlets.gone': '{name} isn’t installed any more',
  'quadlets.column.name': 'Unit',
  'quadlets.column.kind': 'Kind',
  'quadlets.column.service': 'Service',
  'quadlets.column.status': 'Status',
  'quadlets.column.containers': 'Containers',
  'quadlets.status.running': 'Running',
  'quadlets.status.starting': 'Starting',
  'quadlets.status.stopping': 'Stopping',
  'quadlets.status.stopped': 'Stopped',
  'quadlets.status.failed': 'Failed',
  'quadlets.status.notLoaded': 'Not loaded by systemd',
  'quadlets.status.template': 'Template',
  'quadlets.action.start': 'Start',
  'quadlets.action.stop': 'Stop',
  'quadlets.action.restart': 'Restart',
  'quadlets.actionOne.start': 'Start {name}',
  'quadlets.actionOne.stop': 'Stop {name}',
  'quadlets.actionOne.restart': 'Restart {name}',
  'quadlets.done.start': '{name} started',
  'quadlets.done.stop': '{name} stopped',
  'quadlets.done.restart': '{name} restarted',
  'quadlets.installed': '{name} installed',
  'quadlets.saved': '{name} saved',
  'quadlets.removed': '{name} removed',
  'quadlets.blocked.noCommands':
    'Homeport starts and stops units with systemctl, which it can only run over SSH or for a Podman on this computer.',
  'quadlets.blocked.notLoaded':
    'systemd hasn’t generated this unit’s service, so there’s nothing to start.',
  'quadlets.blocked.podmanTooOld':
    'Reading, installing, editing and removing units needs Podman 5.8 or later; this host runs Podman {version}.',
  'quadlets.blocked.noFile':
    'This unit is only known from its containers, so its file can’t be changed.',
  'quadlets.note.labels':
    'Podman {version} can’t list Quadlet units, so these come from the containers they created. Their files, and units that created no container, show on Podman 5.7 and later.',
  'quadlets.note.readOnly':
    'Podman {version} lists units but can’t read, install, edit or remove them; that needs Podman 5.8 or later.',
  'quadlets.note.noCommands':
    'Starting and stopping units needs this host to be reached over SSH, or a Podman on this computer.',
  'quadlets.detail.service': 'Service',
  'quadlets.detail.unit': 'systemd unit',
  'quadlets.detail.scope': 'Runs as',
  'quadlets.detail.path': 'File',
  'quadlets.detail.app': 'Application',
  'quadlets.detail.pod': 'Pod',
  'quadlets.detail.containers':
    '{count, plural, =0 {Containers} one {# container} other {# containers}}',
  'quadlets.detail.noContainers': 'It hasn’t created a container, or its service is stopped.',
  'quadlets.scope.user': 'A user unit (systemctl --user)',
  'quadlets.scope.system': 'A system unit',
  'quadlets.file.title': 'Unit file',
  'quadlets.file.unknown': 'This unit is only known from its containers’ label.',
  'quadlets.file.tooOld':
    'Podman {version} can’t read unit files; showing them needs Podman 5.8 or later.',
  'quadlets.form.newTitle': 'New Quadlet unit',
  'quadlets.form.newDescription':
    'Podman writes the file where its user’s units go and reloads systemd. Start the unit once it’s installed.',
  'quadlets.form.editTitle': 'Edit {name}',
  'quadlets.form.editDescription':
    'Saving replaces the file and reloads systemd; restart the unit for a running service to use it. Files installed beside it stay.',
  'quadlets.form.name': 'File name',
  'quadlets.form.kind': 'Kind',
  'quadlets.form.openFile': 'Open file…',
  'quadlets.form.openTitle': 'Open a Quadlet unit file',
  'quadlets.form.unitFiles': 'Quadlet unit files',
  'quadlets.form.content': 'Unit file',
  'quadlets.form.extra': 'Files beside it',
  'quadlets.form.extraHint':
    'A Containerfile, kube YAML or env file the unit uses, installed in its application folder.',
  'quadlets.form.extraTitle': 'Choose files to install beside the unit',
  'quadlets.form.addExtra': 'Add files…',
  'quadlets.form.removeExtra': 'Remove {name}',
  'quadlets.form.install': 'Install',
  'quadlets.form.save': 'Save',
  'quadlets.form.overwrite': 'Overwrite',
  'quadlets.form.nameInvalid':
    'Name it like web.container, ending in .container, .pod, .volume, .network, .kube, .image, .build or .artifact.',
  'quadlets.form.sectionMissing': 'The file needs a {section} section for Quadlet to read it.',
  'quadlets.form.extraInvalid':
    'Each file beside the unit needs its own plain name, without folders.',
  'quadlets.remove.title': 'Remove {name}?',
  'quadlets.remove.description':
    'Podman deletes the unit file and reloads systemd. Containers, volumes and images it created stay.',
  'quadlets.remove.stop': 'Stop its service if it’s running',
  'quadlets.remove.stopRunning': 'Stop its running service first',
};
