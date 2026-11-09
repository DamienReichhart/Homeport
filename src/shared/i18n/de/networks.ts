/** German text of the `networks` namespace. */
import type en from '../en/networks';
import type { Translation } from '../types';

export default {
  'networks.nav': 'Netzwerke',
  'networks.title': 'Netzwerke',
  'networks.summary': '{count, plural, one {# Netzwerk} other {# Netzwerke}}',
  'networks.cleanUp': 'Aufräumen',
  'networks.create': 'Netzwerk erstellen',
  'networks.search': 'Netzwerke durchsuchen',
  'networks.empty': 'Keine Netzwerke',

  'networks.column.name': 'Name',
  'networks.column.stack': 'Stack',
  'networks.column.driver': 'Treiber',
  'networks.column.subnet': 'Subnetz',
  'networks.column.containers': 'Container',
  'networks.builtIn': 'integriert',
  'networks.internal': 'intern',
  'networks.inspectOne': '{name} untersuchen',
  'networks.removeOne': '{name} entfernen',

  'networks.remove.title': 'Netzwerk {name} entfernen?',
  'networks.remove.attached':
    'Es hat noch {count, plural, one {# verbundenen Container} other {# verbundene Container}}; trennen Sie sie zuerst.',
  'networks.remove.unused':
    'Kein Container verwendet es. Container, die es über seinen Namen referenzieren, starten dann nicht mehr.',
  'networks.prune.title': 'Unbenutzte Netzwerke entfernen?',
  'networks.prune.description':
    'Alle benutzerdefinierten Netzwerke, mit denen kein Container verbunden ist, werden entfernt.',

  'networks.connected': 'Verbunden',
  'networks.disconnected': 'Getrennt',
  'networks.removed': '{name} entfernt',
  'networks.pruned': '{count, plural, one {# Netzwerk} other {# Netzwerke}} entfernt',

  'networks.createDialog.title': 'Netzwerk erstellen',
  'networks.createDialog.name': 'Name',
  'networks.createDialog.driver': 'Treiber',
  'networks.createDialog.subnet': 'Subnetz',
  'networks.createDialog.gateway': 'Gateway',
  'networks.createDialog.automatic': 'automatisch',
  'networks.createDialog.internal': 'Intern — kein Zugriff auf externe Netzwerke',
  'networks.createDialog.attachable': 'Anfügbar (attachable)',

  'networks.dialog.containers': 'Container',
  'networks.dialog.raw': 'Rohes JSON',
  'networks.dialog.noContainer': 'Kein Container ist verbunden.',
  'networks.dialog.disconnect': 'Trennen',
  'networks.dialog.containerToConnect': 'Zu verbindender Container',
  'networks.dialog.connectPlaceholder': 'Container verbinden…',
  'networks.dialog.connect': 'Verbinden',
} satisfies Translation<typeof en>;
