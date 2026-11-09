/** French text of the `updates` namespace. */
import type en from '../en/updates';
import type { Translation } from '../types';

export default {
  'updates.title': 'Mises à jour',
  'updates.description':
    'Homeport peut rechercher une nouvelle version au démarrage et toutes les quelques heures.',
  'updates.automatic': 'Rechercher automatiquement les mises à jour',
  'updates.checking': 'Vérification…',
  'updates.checkNow': 'Vérifier maintenant',
  'updates.status.available': 'Homeport {version} est disponible (vous avez la {current}).',
  'updates.status.latest': 'Homeport {current} est la dernière version.',

  'updates.notice.label': 'Mise à jour disponible',
  'updates.notice.available': '<strong>Homeport {version}</strong> est disponible.',
  'updates.notice.availablePackageManager':
    '<strong>Homeport {version}</strong> est disponible. Mettez-le à jour avec votre gestionnaire de paquets :',
  'updates.notice.dismiss': 'Ignorer',

  'updates.copyCommand': 'Copier la commande',
  'updates.whatsNew': 'Nouveautés',
  'updates.install': 'Installer et redémarrer',
  'updates.download': 'Télécharger',
  'updates.progress.downloading': 'Téléchargement…',
  'updates.progress.downloadingPercent': 'Téléchargement… {percent} %',
  'updates.progress.installing': 'Installation…',

  'updates.method.inApp':
    'Les mises à jour sont téléchargées, vérifiées avec la clé de signature de Homeport et installées quand vous le décidez.',
  'updates.method.scoop': 'Installé avec Scoop : les mises à jour viennent du bucket Homeport.',
  'updates.method.aur':
    'Installé depuis l’AUR (homeport-bin) : votre assistant AUR installe les mises à jour.',
  'updates.method.packageManager':
    'Installé avec {manager} : les mises à jour viennent du dépôt de paquets Homeport.',
  'updates.method.manual':
    'Ceci est une version de développement : téléchargez les nouvelles versions depuis le site web.',
  'updates.palette.check': 'Rechercher des mises à jour',
} satisfies Translation<typeof en>;
