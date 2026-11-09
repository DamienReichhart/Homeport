/** French text of the `images` namespace. */
import type en from '../en/images';
import type { Translation } from '../types';

export default {
  'images.nav': 'Images',
  'images.title': 'Images',
  'images.summary': '{count, plural, one {# image} other {# images}} · {size}',
  'images.cleanUp': 'Nettoyer',
  'images.cleanUp.dangling': 'Images orphelines',
  'images.cleanUp.all': 'Toutes les images inutilisées',
  'images.saveSelected':
    '{count, plural, one {Enregistrer l’image…} other {Enregistrer # images…}}',
  'images.load': 'Charger…',
  'images.importTar': 'Importer depuis un tar…',
  'images.build': 'Construire',
  'images.pull': 'Récupérer une image',
  'images.search': 'Rechercher des images',
  'images.empty.title': 'Aucune image',
  'images.empty.description': 'Récupérez-en une depuis un registre pour commencer.',

  'images.column.name': 'Nom',
  'images.column.id': 'ID',
  'images.column.created': 'Créée',
  'images.column.size': 'Taille',
  'images.column.status': 'État',
  'images.moreTags': '{count, plural, one {+# autre tag} other {+# autres tags}}',
  'images.inUse': 'Utilisée · {count}',
  'images.unused': 'Inutilisée',
  'images.runOne': 'Exécuter {name}',
  'images.action.inspect': 'Inspecter',
  'images.action.tag': 'Taguer',
  'images.action.push': 'Pousser',
  'images.action.save': 'Enregistrer…',

  'images.remove.title': 'Supprimer {name} ?',
  'images.remove.everyName':
    'Cela supprime tous les noms de l’image ({names}) et ses couches que rien d’autre n’utilise.',
  'images.remove.description':
    'L’image et les couches qu’aucune autre image n’utilise sont supprimées. Elle pourra être récupérée à nouveau plus tard.',
  'images.prune.allTitle': 'Supprimer toutes les images inutilisées ?',
  'images.prune.danglingTitle': 'Supprimer les images orphelines ?',
  'images.prune.allDescription':
    'Toutes les images qu’aucun conteneur n’utilise sont supprimées, y compris celles avec un tag.',
  'images.prune.danglingDescription':
    'Les images sans tag laissées par les builds et les récupérations sont supprimées.',

  'images.removed': '{name} supprimée',
  'images.pruned':
    '{count, plural, one {# image supprimée} other {# images supprimées}}, {space} récupérés',
  'images.pulled': '{reference} récupérée',
  'images.pushed': '{reference} poussée',
  'images.built': '{name} construite',
  'images.builtUnnamed': 'Image construite',
  'images.saved': '{path} enregistré',
  'images.loadedOne': '{name} chargée',
  'images.loaded': '{count, plural, one {# image chargée} other {# images chargées}}',
  'images.imported': '{name} importée',

  'images.files.save': 'Enregistrer les images sous',
  'images.files.load': 'Charger des images depuis',
  'images.files.import': 'Importer un système de fichiers depuis',
  'images.files.imageArchives': 'Archives d’images',
  'images.files.imageArchive': 'Archive d’image',
  'images.files.filesystemArchives': 'Archives de système de fichiers',

  'images.buildDialog.title': 'Construire une image',
  'images.buildDialog.description':
    'À partir d’un dossier de cet ordinateur. Homeport l’envoie à l’hôte, sans ce que son .dockerignore exclut, et le moteur de l’hôte la construit.',
  'images.buildDialog.built': '✓ Construite',
  'images.buildDialog.folder': 'Dossier',
  'images.buildDialog.chooseFolder': 'Choisir le dossier à construire',
  'images.buildDialog.choose': 'Choisir…',
  'images.buildDialog.dockerfile': 'Dockerfile',
  'images.buildDialog.target': 'Étape cible',
  'images.buildDialog.targetPlaceholder': 'La dernière étape',
  'images.buildDialog.names': 'Noms',
  'images.buildDialog.buildArgs': 'Arguments de build',
  'images.buildDialog.buildArgsPlaceholder': 'VERSION=1.0, un par ligne',
  'images.buildDialog.noCache': 'Ne pas utiliser le cache de build',
  'images.buildDialog.pull': 'Récupérer des images de base plus récentes',
  'images.buildForm.noFolder': 'Choisissez le dossier à construire.',
  'images.buildForm.argNeedsName': '« {line} » doit avoir un nom avant « = ».',

  'images.hub.unavailable':
    'Impossible de rechercher sur Docker Hub pour le moment : {error} Un nom complet comme nginx:1.27 peut toujours être récupéré.',
  'images.hub.noMatch': 'Rien sur Docker Hub ne correspond à « {term} ».',
  'images.hub.results': 'Résultats de Docker Hub',
  'images.hub.result': '{name}, {stars} étoiles',
  'images.hub.resultOfficial': '{name}, image officielle, {stars} étoiles',
  'images.hub.official': 'Officielle',
  'images.hub.backToSearch': 'Rechercher',
  'images.hub.tagCount': '{count, plural, one {# tag} other {# tags}}',
  'images.hub.filterTags': 'Filtrer les tags',
  'images.hub.tags': 'Tags',
  'images.hub.noTagMatch': 'Aucun tag ne correspond à « {filter} ».',
  'images.hub.moreTags': 'Plus de tags',

  'images.importDialog.title': 'Importer un système de fichiers comme image',
  'images.importDialog.description':
    '{path} devient une image à une seule couche, sans historique ni paramètres propres ; donnez-lui une commande ci-dessous pour la rendre exécutable.',
  'images.importDialog.name': 'Nom de l’image',
  'images.importDialog.changes': 'Modifications',
  'images.importDialog.changesHint':
    'Instructions Dockerfile, une par ligne : CMD, ENTRYPOINT, ENV, EXPOSE, LABEL, ONBUILD, USER, VOLUME ou WORKDIR.',
  'images.importDialog.import': 'Importer',

  'images.inspect.layers': 'Couches',
  'images.inspect.raw': 'JSON brut',
  'images.inspect.created': 'Créée',
  'images.inspect.instruction': 'Instruction',
  'images.inspect.size': 'Taille',

  'images.pullDialog.title': 'Récupérer une image',
  'images.pullDialog.description':
    'Recherchez sur Docker Hub par nom, ou saisissez toute référence que l’hôte peut atteindre, par ex. nginx:1.27 ou ghcr.io/org/app:v2.',
  'images.pullDialog.placeholder': 'postgres, ou nginx:1.27',
  'images.pullDialog.reference': 'Référence de l’image',
  'images.pullDialog.pull': 'Récupérer',
  'images.done': 'Terminé.',

  'images.pushDialog.title': 'Pousser {name}',
  'images.pushDialog.description':
    'Vers le registre par lequel commence son nom, par ex. ghcr.io/org/app:v2, ou vers Docker Hub pour des noms comme org/app:v2.',
  'images.pushDialog.target': 'Nom à pousser',
  'images.pushDialog.push': 'Pousser',
  'images.pushDialog.tagsFirst': 'L’image reçoit d’abord ce nom, comme avec Taguer.',
  'images.pushDialog.uploading': 'Envoi {current} / {total}',
  'images.push.pushed': '{pushed, plural, one {# couche poussée} other {# couches poussées}}',
  'images.push.pushedAndExisting':
    '{pushed, plural, one {# couche poussée} other {# couches poussées}}, {existing} déjà dans le registre',

  'images.tagDialog.title': 'Taguer {name}',
  'images.tagDialog.newName': 'Nouveau nom',
  'images.tagDialog.add': 'Ajouter le tag',

  'images.transfer.savingOne': 'Enregistrement de {name}',
  'images.transfer.saving': 'Enregistrement de {count, plural, one {# image} other {# images}}',
  'images.transfer.to': 'Vers {path}',
  'images.transfer.from': 'Depuis {path}',
  'images.transfer.written': 'Écrit',
  'images.transfer.sent': 'Envoyé',
  'images.transfer.loading': 'Chargement des images',
  'images.transfer.importing': 'Importation de {reference}',
  'images.transfer.doneWith': 'Terminé : {images}',
  'images.palette.pull': 'Récupérer une image',
} satisfies Translation<typeof en>;
