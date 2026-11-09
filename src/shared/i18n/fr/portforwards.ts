/** French text of the `portforwards` namespace. */
import type en from '../en/portforwards';
import type { Translation } from '../types';

export default {
  'portforwards.nav': 'Redirections de port',
  'portforwards.title': 'Redirections de port',
  'portforwards.summary': '{count, plural, one {# redirection} other {# redirections}}',
  'portforwards.empty.title': 'Aucune redirection de port',
  'portforwards.empty.description':
    'Redirigez un port depuis la page d’un Pod ou d’un Service pour l’atteindre depuis cet ordinateur. Les redirections prennent fin quand Homeport se ferme.',
  'portforwards.column.local': 'Adresse locale',
  'portforwards.column.target': 'Redirige vers',
  'portforwards.column.namespace': 'Namespace',
  'portforwards.column.cluster': 'Cluster',
  'portforwards.column.connections': 'Connexions',
  'portforwards.forward': 'Rediriger…',
  'portforwards.forwardPort': 'Rediriger le port {port}',
  'portforwards.open': 'Ouvrir',
  'portforwards.stop': 'Arrêter',
  'portforwards.stopFailed': 'Homeport n’a pas pu arrêter la redirection',
  'portforwards.started': 'Redirection de {address} vers {target}:{port}',
  'portforwards.failed': 'Une redirection de port a échoué',
  'portforwards.ended': 'Une redirection de port a pris fin',
  'portforwards.dialog.title': 'Rediriger {target}:{port}',
  'portforwards.dialog.description':
    'Dans le namespace {namespace}. Accédez-y par le port local tant que la redirection est active.',
  'portforwards.dialog.localPort': 'Port local',
  'portforwards.dialog.anyPort': 'N’importe quel port libre',
  'portforwards.dialog.loopback':
    'Seul cet ordinateur peut se connecter : la redirection écoute sur 127.0.0.1 (et ::1).',
  'portforwards.dialog.submit': 'Rediriger',
} satisfies Translation<typeof en>;
