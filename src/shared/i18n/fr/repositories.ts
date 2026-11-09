/** French text of the `repositories` namespace. */
import type en from '../en/repositories';
import type { Translation } from '../types';

export default {
  'repositories.title': 'Dépôts',
  'repositories.description':
    'Dépôts git depuis lesquels les stacks synchronisées sont déployées, sur chaque hôte. Homeport ne fait que les lire.',
  'repositories.add': 'Ajouter un dépôt',
  'repositories.empty':
    'Aucun dépôt pour l’instant. Ajoutez-en un pour créer une stack synchronisée.',
  'repositories.manageOne': 'Gérer l’accès à {repository}',
  'repositories.removeOne': 'Supprimer {repository}',
  'repositories.remove.title': 'Supprimer {repository} ?',
  'repositories.remove.description':
    'Homeport oublie sa clé de déploiement ou son jeton et supprime son clone sur cet ordinateur. Le dépôt lui-même n’est pas touché. Retirez aussi la clé de déploiement du dépôt sur votre forge.',
  'repositories.noKeychain':
    'Les clés de déploiement et les jetons sont chiffrés avec une clé conservée dans le trousseau du système, et aucun n’est disponible : seuls des dépôts publics peuvent donc être ajoutés.',
  'repositories.public': 'Un dépôt public : Homeport le lit sans identifiants.',

  'repositories.added': '{repository} ajouté',
  'repositories.removed': '{repository} supprimé',
  'repositories.keyRotated':
    'Nouvelle clé de déploiement créée. Ajoutez-la au dépôt à la place de l’ancienne.',
  'repositories.tokenReplaced': 'Jeton remplacé',
  'repositories.hostKeyReset':
    'Homeport approuvera la clé que le serveur présentera la prochaine fois.',

  'repositories.auth.withKey': 'Avec une clé de déploiement, sur {host}',
  'repositories.auth.withToken': 'En tant que {username} avec un jeton, sur {host}',
  'repositories.auth.public': 'Public, sur {host}',

  'repositories.dialog.addTitle': 'Ajouter un dépôt',
  'repositories.dialog.description':
    'Homeport le clone sur cet ordinateur pour en déployer des stacks synchronisées. Il ne fait jamais de commit ni de push.',
  'repositories.dialog.url': 'URL du dépôt',
  'repositories.dialog.auth': 'Accès',
  'repositories.dialog.authKey': 'Clé de déploiement',
  'repositories.dialog.authToken': 'Jeton',
  'repositories.dialog.authNone': 'Public',
  'repositories.dialog.authKeyHelp':
    'Homeport crée une clé pour ce dépôt uniquement. Vous l’ajoutez au dépôt avec un accès en lecture ; vos propres clés SSH ne sont jamais utilisées.',
  'repositories.dialog.authTokenHelp':
    'Un jeton d’accès autorisé à lire le contenu du dépôt, envoyé via HTTPS. Chiffré sur cet ordinateur et plus jamais affiché.',
  'repositories.dialog.authNoneHelp': 'Pour un dépôt public via HTTPS. Rien n’est enregistré.',
  'repositories.dialog.username': 'Nom d’utilisateur',
  'repositories.dialog.token': 'Jeton d’accès',
  'repositories.dialog.add': 'Ajouter',
  'repositories.dialog.addWithKey': 'Ajouter et créer une clé de déploiement',

  'repositories.missing.url': 'Saisissez l’URL du dépôt.',
  'repositories.missing.validUrl':
    'Saisissez une URL SSH ou HTTPS, comme git@github.com:owner/repo.git ou https://gitlab.com/owner/repo.git.',
  'repositories.missing.auth': 'Choisissez comment Homeport accède à ce dépôt.',
  'repositories.missing.username': 'Saisissez le nom d’utilisateur auquel appartient le jeton.',
  'repositories.missing.token': 'Saisissez le jeton d’accès.',

  'repositories.deployKey.title': 'Clé de déploiement',
  'repositories.deployKey.github':
    'GitHub : dans le dépôt, <strong>Settings → Deploy keys → Add deploy key</strong>.',
  'repositories.deployKey.gitlab':
    'GitLab : dans le projet, <strong>Settings → Repository → Deploy keys</strong>.',
  'repositories.deployKey.gitea':
    'Gitea et Forgejo : dans le dépôt, <strong>Settings → Deploy keys → Add deploy key</strong>.',
  'repositories.deployKey.readOnly':
    'Laissez l’accès en écriture désactivé : Homeport ne fait que lire. Testez ensuite l’accès ci-dessous.',
  'repositories.deployKey.fingerprint': 'Empreinte : <mono>{fingerprint}</mono>',
  'repositories.rotate.button': 'Nouvelle clé',
  'repositories.rotate.title': 'Créer une nouvelle clé de déploiement ?',
  'repositories.rotate.description':
    'La clé actuelle cesse de fonctionner pour Homeport. Ajoutez la nouvelle au dépôt, et retirez-en l’ancienne.',
  'repositories.rotate.confirm': 'Créer une nouvelle clé',

  'repositories.token.replace': 'Remplacer le jeton',
  'repositories.token.save': 'Enregistrer le jeton',

  'repositories.hostKey.trusted': 'Clé approuvée de {host} : <mono>{fingerprint}</mono>',
  'repositories.hostKey.reset': 'Réinitialiser',
  'repositories.hostKey.resetTitle': 'Réinitialiser la clé approuvée de {host} ?',
  'repositories.hostKey.resetDescription':
    'Ne le faites que si {host} a changé sa clé SSH et que vous avez vérifié la nouvelle empreinte sur son site web. Homeport approuvera la clé, quelle qu’elle soit, présentée la prochaine fois.',

  'repositories.test.button': 'Tester l’accès',
  'repositories.test.testing': 'Test en cours…',
  'repositories.test.success':
    'Homeport peut le lire : {branches, plural, one {# branche} other {# branches}} et {tags, plural, one {# tag} other {# tags}}.',
} satisfies Translation<typeof en>;
