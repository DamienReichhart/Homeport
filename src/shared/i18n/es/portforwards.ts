/** Spanish text of the `portforwards` namespace. */
import type en from '../en/portforwards';
import type { Translation } from '../types';

export default {
  'portforwards.nav': 'Reenvíos de puerto',
  'portforwards.title': 'Reenvíos de puerto',
  'portforwards.summary': '{count, plural, one {# reenvío} other {# reenvíos}}',
  'portforwards.empty.title': 'No hay reenvíos de puerto',
  'portforwards.empty.description':
    'Reenvía un puerto desde la página de un Pod o de un Service para alcanzarlo desde este equipo. Los reenvíos terminan cuando Homeport se cierra.',
  'portforwards.column.local': 'Dirección local',
  'portforwards.column.target': 'Reenvía a',
  'portforwards.column.namespace': 'Namespace',
  'portforwards.column.cluster': 'Clúster',
  'portforwards.column.connections': 'Conexiones',
  'portforwards.forward': 'Reenviar…',
  'portforwards.forwardPort': 'Reenviar el puerto {port}',
  'portforwards.open': 'Abrir',
  'portforwards.stop': 'Detener',
  'portforwards.stopFailed': 'Homeport no pudo detener el reenvío',
  'portforwards.started': 'Reenviando {address} a {target}:{port}',
  'portforwards.failed': 'Falló un reenvío de puerto',
  'portforwards.ended': 'Terminó un reenvío de puerto',
  'portforwards.dialog.title': 'Reenviar {target}:{port}',
  'portforwards.dialog.description':
    'En el namespace {namespace}. Alcánzalo en el puerto local mientras el reenvío esté activo.',
  'portforwards.dialog.localPort': 'Puerto local',
  'portforwards.dialog.anyPort': 'Cualquier puerto libre',
  'portforwards.dialog.loopback':
    'Solo este equipo puede conectarse: el reenvío escucha en 127.0.0.1 (y ::1).',
  'portforwards.dialog.submit': 'Reenviar',
} satisfies Translation<typeof en>;
