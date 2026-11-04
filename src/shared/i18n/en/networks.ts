/** English text of the `networks` namespace; every key starts with `networks.`. */
export default {
  'networks.nav': 'Networks',
  'networks.title': 'Networks',
  'networks.summary': '{count, plural, one {# network} other {# networks}}',
  'networks.cleanUp': 'Clean up',
  'networks.create': 'Create network',
  'networks.search': 'Search networks',
  'networks.empty': 'No networks',

  'networks.column.name': 'Name',
  'networks.column.stack': 'Stack',
  'networks.column.driver': 'Driver',
  'networks.column.subnet': 'Subnet',
  'networks.column.containers': 'Containers',
  'networks.builtIn': 'built-in',
  'networks.internal': 'internal',
  'networks.inspectOne': 'Inspect {name}',
  'networks.removeOne': 'Remove {name}',

  'networks.remove.title': 'Remove network {name}?',
  'networks.remove.attached':
    'It still has {count, plural, one {# attached container} other {# attached containers}}; disconnect them first.',
  'networks.remove.unused':
    'No container uses it. Containers that reference it by name will fail to start.',
  'networks.prune.title': 'Remove unused networks?',
  'networks.prune.description': 'Every custom network no container is attached to is removed.',

  'networks.connected': 'Connected',
  'networks.disconnected': 'Disconnected',
  'networks.removed': 'Removed {name}',
  'networks.pruned': 'Removed {count, plural, one {# network} other {# networks}}',

  'networks.createDialog.title': 'Create a network',
  'networks.createDialog.name': 'Name',
  'networks.createDialog.driver': 'Driver',
  'networks.createDialog.subnet': 'Subnet',
  'networks.createDialog.gateway': 'Gateway',
  'networks.createDialog.automatic': 'automatic',
  'networks.createDialog.internal': 'Internal — no access to outside networks',
  'networks.createDialog.attachable': 'Attachable',

  'networks.dialog.containers': 'Containers',
  'networks.dialog.raw': 'Raw JSON',
  'networks.dialog.noContainer': 'No container is attached.',
  'networks.dialog.disconnect': 'Disconnect',
  'networks.dialog.containerToConnect': 'Container to connect',
  'networks.dialog.connectPlaceholder': 'Connect a container…',
  'networks.dialog.connect': 'Connect',
} as const;
