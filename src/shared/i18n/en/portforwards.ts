/** English text of the `portforwards` namespace (port forwards to a Cluster); every key starts with `portforwards.`. */
export default {
  'portforwards.nav': 'Port forwards',
  'portforwards.title': 'Port forwards',
  'portforwards.summary': '{count, plural, one {# forward} other {# forwards}}',
  'portforwards.empty.title': 'No port forwards',
  'portforwards.empty.description':
    'Forward a port from a Pod’s or a Service’s page to reach it from this computer. Forwards end when Homeport quits.',
  'portforwards.column.local': 'Local address',
  'portforwards.column.target': 'Forwards to',
  'portforwards.column.namespace': 'Namespace',
  'portforwards.column.cluster': 'Cluster',
  'portforwards.column.connections': 'Connections',
  'portforwards.forward': 'Forward…',
  'portforwards.forwardPort': 'Forward port {port}',
  'portforwards.open': 'Open',
  'portforwards.stop': 'Stop',
  'portforwards.stopFailed': 'Homeport couldn’t stop the forward',
  'portforwards.started': 'Forwarding {address} to {target}:{port}',
  'portforwards.failed': 'A port forward failed',
  'portforwards.ended': 'A port forward ended',
  'portforwards.dialog.title': 'Forward {target}:{port}',
  'portforwards.dialog.description':
    'In the namespace {namespace}. Reach it at the local port while the forward runs.',
  'portforwards.dialog.localPort': 'Local port',
  'portforwards.dialog.anyPort': 'Any free port',
  'portforwards.dialog.loopback':
    'Only this computer can connect: the forward listens on 127.0.0.1 (and ::1).',
  'portforwards.dialog.submit': 'Forward',
} as const;
