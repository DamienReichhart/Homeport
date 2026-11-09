/** French text of the `volumes` namespace. */
import type en from '../en/volumes';
import type { Translation } from '../types';

export default {
  'volumes.nav': 'Volumes',
  'volumes.title': 'Volumes',
  'volumes.summary': '{count, plural, one {# volume} other {# volumes}} · {unused} inutilisé(s)',
  'volumes.cleanUp': 'Nettoyer',
  'volumes.cleanUp.anonymous': 'Volumes anonymes inutilisés',
  'volumes.cleanUp.all': 'Tous les volumes inutilisés',
  'volumes.create': 'Créer un volume',
  'volumes.search': 'Rechercher des volumes',
  'volumes.empty.title': 'Aucun volume',
  'volumes.empty.description':
    'Les volumes conservent les données des conteneurs entre les redémarrages et les recréations.',
  'volumes.column.name': 'Nom',
  'volumes.column.stack': 'Stack',
  'volumes.column.driver': 'Pilote',
  'volumes.column.size': 'Taille',
  'volumes.size.label': 'Taille :',
  'volumes.size.measuring': 'Mesure en cours',
  'volumes.size.unknown': 'Le pilote de ce volume n’indique pas sa taille.',
  'volumes.size.refresh': 'Actualiser les tailles',
  'volumes.column.usedBy': 'Utilisé par',
  'volumes.column.created': 'Créé',
  'volumes.unused': 'Inutilisé',
  'volumes.inspectOne': 'Inspecter {name}',
  'volumes.deleteOne': 'Supprimer {name}',
  'volumes.browseOne': 'Parcourir les fichiers de {name}',
  'volumes.browse.title': 'Fichiers de {name}',
  'volumes.browse.description':
    'Lecture seule : rien ici ne peut modifier le volume, même pendant que des conteneurs l’utilisent.',
  'volumes.browse.saveOne': 'Enregistrer {name}',
  'volumes.browse.saved': '{name} enregistré',
  'volumes.browse.savedArchive': '{name} enregistré en archive .tar',
  'volumes.helper.needed':
    'Homeport ouvre les volumes avec un conteneur auxiliaire éphémère, qui a besoin de l’image {image} (environ {size}). Cet hôte ne l’a pas.',
  'volumes.helper.removed':
    'Homeport ne la récupère que si vous acceptez, et la supprime une fois terminé, sauf si autre chose l’utilise.',
  'volumes.helper.pull': 'Récupérer {image}',
  'volumes.helper.pulling': 'Récupération de {image}…',
  'volumes.helper.progress': 'Progression de la récupération',
  'volumes.moreActions': 'Plus d’actions pour {name}',
  'volumes.backupMenu': 'Sauvegarder dans un fichier…',
  'volumes.restoreMenu': 'Restaurer depuis un fichier…',
  'volumes.cloneMenu': 'Cloner…',
  'volumes.clone.title': 'Cloner {volume}',
  'volumes.clone.description':
    'Copie ses fichiers, avec leurs propriétaires et leurs permissions, dans un nouveau volume sur cet hôte. La copie n’appartient à aucune stack.',
  'volumes.clone.name': 'Nom de la copie',
  'volumes.clone.start': 'Cloner',
  'volumes.clone.into': 'Dans {target}',
  'volumes.clone.copying': 'Copie en cours…',
  'volumes.clone.done': '{source} cloné dans {target}',
  'volumes.restoreFromFile': 'Restaurer depuis un fichier',
  'volumes.backup.title': 'Sauvegarder {volume}',
  'volumes.backup.saveAs': 'Sauvegarder {volume} dans',
  'volumes.backup.to': 'Vers {path}',
  'volumes.backup.done': '{volume} sauvegardé dans {path}',
  'volumes.restore.pick': 'Restaurer un volume depuis',
  'volumes.restore.title': 'Restaurer un volume',
  'volumes.restore.titleInto': 'Restaurer {volume}',
  'volumes.restore.from': 'Depuis {path}',
  'volumes.restore.into': 'Restaurer dans',
  'volumes.restore.newVolume': 'Un nouveau volume',
  'volumes.restore.existingVolume': 'Un volume existant',
  'volumes.restore.volume': 'Volume',
  'volumes.restore.chooseVolume': 'Choisir un volume',
  'volumes.restore.nameTaken': 'Un volume nommé {volume} existe déjà.',
  'volumes.restore.start': 'Restaurer',
  'volumes.restore.done': '{volume} restauré',
  'volumes.restore.cancelled':
    'La restauration de {volume} a été annulée ; le volume est peut-être partiellement écrit.',
  'volumes.transfer.written': 'Écrit',
  'volumes.transfer.sent': 'Envoyé',
  'volumes.transfer.archives': 'Archives tar',
  'volumes.flow.running.backup':
    '{containers} {count, plural, one {est en cours d’exécution et utilise} other {sont en cours d’exécution et utilisent}} {volume}. Une sauvegarde faite pendant qu’ils y écrivent peut être incohérente, par exemple une base de données surprise en pleine écriture.',
  'volumes.flow.running.restore':
    '{containers} {count, plural, one {est en cours d’exécution et utilise} other {sont en cours d’exécution et utilisent}} {volume}. Homeport {count, plural, one {l’arrête} other {les arrête}} pendant la restauration et {count, plural, one {le redémarre} other {les redémarre}} ensuite.',
  'volumes.flow.running.clone':
    '{containers} {count, plural, one {est en cours d’exécution et utilise} other {sont en cours d’exécution et utilisent}} {volume}. Une copie faite pendant qu’ils y écrivent peut être incohérente, par exemple une base de données surprise en pleine écriture.',
  'volumes.flow.anyway.backup': 'Sauvegarder quand même',
  'volumes.flow.anyway.clone': 'Copier quand même',
  'volumes.flow.stopDuring.backup':
    '{count, plural, one {L’arrêter} other {Les arrêter}} pendant la sauvegarde',
  'volumes.flow.stopDuring.restore': 'Arrêter et restaurer',
  'volumes.flow.stopDuring.clone':
    '{count, plural, one {L’arrêter} other {Les arrêter}} pendant la copie',
  'volumes.flow.replace':
    '{volume} contient déjà des fichiers. La restauration les supprime et met ceux de la sauvegarde à leur place.',
  'volumes.flow.replaceConfirm': 'Remplacer son contenu',
  'volumes.usedBy': 'Utilisé par {containers}.',
  'volumes.notUsed': 'Aucun conteneur n’utilise ce volume.',
  'volumes.createDialog.title': 'Créer un volume',
  'volumes.createDialog.name': 'Nom',
  'volumes.createDialog.driver': 'Pilote',
  'volumes.deleted': '{name} supprimé',
  'volumes.pruned':
    '{count, plural, one {# volume supprimé} other {# volumes supprimés}}, {space} récupérés',
  'volumes.remove.title': 'Supprimer le volume {name} ?',
  'volumes.remove.warning':
    '<strong>Toutes les données de ce volume sont définitivement supprimées.</strong>',
  'volumes.remove.mounted': ' Il est monté par {containers} ; supprimez d’abord ces conteneurs.',
  'volumes.remove.confirm': 'Supprimer le volume et les données',
  'volumes.prune.allTitle': 'Supprimer tous les volumes inutilisés ?',
  'volumes.prune.anonymousTitle': 'Supprimer les volumes anonymes inutilisés ?',
  'volumes.prune.allWarning':
    'Les données de tous les volumes qu’aucun conteneur n’utilise sont définitivement supprimées, volumes nommés compris.',
  'volumes.prune.anonymousWarning':
    'Les données des volumes sans nom qu’aucun conteneur n’utilise sont définitivement supprimées.',
  'volumes.prune.confirm': 'Supprimer les volumes et les données',
} satisfies Translation<typeof en>;
