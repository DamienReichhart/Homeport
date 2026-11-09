/** French text of the `registries` namespace. */
import type en from '../en/registries';
import type { Translation } from '../types';

export default {
  'registries.title': 'Registres',
  'registries.description':
    'Identifiants des registres de conteneurs, utilisés pour récupérer des images privées, pousser et construire sur chaque hôte.',
  'registries.add': 'Ajouter des identifiants',
  'registries.noKeychain.title': 'Aucun trousseau du système',
  'registries.noKeychain.description':
    'Les mots de passe et jetons des registres sont chiffrés avec une clé conservée dans le trousseau du système, et aucun n’est disponible. Les images peuvent toujours être récupérées de façon anonyme.',
  'registries.empty':
    'Aucun identifiant pour l’instant : les images sont récupérées de façon anonyme.',
  'registries.editOne': 'Modifier les identifiants de {registry}',
  'registries.removeOne': 'Supprimer les identifiants de {registry}',
  'registries.remove.title': 'Supprimer les identifiants de {registry} ?',
  'registries.remove.description':
    'Les images de {registry} seront récupérées de façon anonyme jusqu’à ce que des identifiants soient de nouveau ajoutés.',

  'registries.saved': 'Identifiants de {registry} enregistrés',
  'registries.removed': 'Identifiants de {registry} supprimés',

  'registries.dialog.editTitle': 'Modifier les identifiants de {registry}',
  'registries.dialog.addTitle': 'Ajouter des identifiants de registre',
  'registries.dialog.description':
    'Utilisés pour récupérer, pousser et construire sur chaque hôte. Le mot de passe ou le jeton est chiffré sur cet ordinateur et n’est plus jamais affiché.',
  'registries.dialog.address': 'Registre',
  'registries.dialog.username': 'Nom d’utilisateur',
  'registries.dialog.secret': 'Mot de passe ou jeton d’accès',
  'registries.dialog.keepSecret': 'Laissez vide pour conserver celui enregistré',
  'registries.dialog.accepted': '{registry} a accepté ces identifiants.',
  'registries.dialog.checking': 'Vérification…',
  'registries.dialog.check': 'Vérifier les identifiants',

  'registries.missing.address': 'Saisissez l’adresse du registre.',
  'registries.missing.username': 'Saisissez le nom d’utilisateur.',
  'registries.missing.secret': 'Saisissez le mot de passe ou le jeton d’accès.',

  'registries.login.as': 'en tant que {username} sur {registry}',
  'registries.login.anonymously': 'de façon anonyme',
  'registries.login.pulledAs': 'Récupérée en tant que {username} sur {registry}',
  'registries.login.pulledAnonymously': 'Récupérée de façon anonyme',
  'registries.login.pushedAs': 'Poussée en tant que {username} sur {registry}',
  'registries.login.pushedAnonymously': 'Poussée de façon anonyme',

  'registries.noLogin.title': 'Aucun identifiant pour {registry}',
  'registries.noLogin.description':
    'La plupart des registres refusent les envois anonymes. <link>Ajoutez d’abord des identifiants dans Paramètres → Registres</link>.',
} satisfies Translation<typeof en>;
