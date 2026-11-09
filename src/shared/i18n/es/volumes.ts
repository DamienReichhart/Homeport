/** Spanish text of the `volumes` namespace. */
import type en from '../en/volumes';
import type { Translation } from '../types';

export default {
  'volumes.nav': 'Volúmenes',
  'volumes.title': 'Volúmenes',
  'volumes.summary': '{count, plural, one {# volumen} other {# volúmenes}} · {unused} sin usar',
  'volumes.cleanUp': 'Limpiar',
  'volumes.cleanUp.anonymous': 'Volúmenes anónimos sin usar',
  'volumes.cleanUp.all': 'Todos los volúmenes sin usar',
  'volumes.create': 'Crear volumen',
  'volumes.search': 'Buscar volúmenes',
  'volumes.empty.title': 'No hay volúmenes',
  'volumes.empty.description':
    'Los volúmenes conservan los datos de los contenedores entre reinicios y recreaciones.',
  'volumes.column.name': 'Nombre',
  'volumes.column.stack': 'Stack',
  'volumes.column.driver': 'Driver',
  'volumes.column.size': 'Tamaño',
  'volumes.size.label': 'Tamaño:',
  'volumes.size.measuring': 'Midiendo',
  'volumes.size.unknown': 'El driver de este volumen no informa de su tamaño.',
  'volumes.size.refresh': 'Actualizar tamaños',
  'volumes.column.usedBy': 'Usado por',
  'volumes.column.created': 'Creado',
  'volumes.unused': 'Sin usar',
  'volumes.inspectOne': 'Inspeccionar {name}',
  'volumes.deleteOne': 'Eliminar {name}',
  'volumes.browseOne': 'Explorar los archivos de {name}',
  'volumes.browse.title': 'Archivos de {name}',
  'volumes.browse.description':
    'Solo lectura: nada de lo que hay aquí puede cambiar el volumen, aunque lo estén usando contenedores.',
  'volumes.browse.saveOne': 'Guardar {name}',
  'volumes.browse.saved': 'Se guardó {name}',
  'volumes.browse.savedArchive': 'Se guardó {name} como archivo .tar',
  'volumes.helper.needed':
    'Homeport abre los volúmenes con un contenedor auxiliar de corta duración, que necesita la imagen {image} (unos {size}). Este host no la tiene.',
  'volumes.helper.removed':
    'Homeport solo la descarga si aceptas, y la vuelve a eliminar al terminar, salvo que otra cosa la use.',
  'volumes.helper.pull': 'Descargar {image}',
  'volumes.helper.pulling': 'Descargando {image}…',
  'volumes.helper.progress': 'Progreso de la descarga',
  'volumes.moreActions': 'Más acciones para {name}',
  'volumes.backupMenu': 'Hacer copia de seguridad en un archivo…',
  'volumes.restoreMenu': 'Restaurar desde un archivo…',
  'volumes.cloneMenu': 'Clonar…',
  'volumes.clone.title': 'Clonar {volume}',
  'volumes.clone.description':
    'Copia sus archivos, con sus propietarios y permisos, en un volumen nuevo de este host. La copia no pertenece a ningún stack.',
  'volumes.clone.name': 'Nombre de la copia',
  'volumes.clone.start': 'Clonar',
  'volumes.clone.into': 'En {target}',
  'volumes.clone.copying': 'Copiando…',
  'volumes.clone.done': 'Se clonó {source} en {target}',
  'volumes.restoreFromFile': 'Restaurar desde archivo',
  'volumes.backup.title': 'Hacer copia de seguridad de {volume}',
  'volumes.backup.saveAs': 'Guardar copia de seguridad de {volume} en',
  'volumes.backup.to': 'En {path}',
  'volumes.backup.done': 'Se guardó la copia de seguridad de {volume} en {path}',
  'volumes.restore.pick': 'Restaurar un volumen desde',
  'volumes.restore.title': 'Restaurar un volumen',
  'volumes.restore.titleInto': 'Restaurar {volume}',
  'volumes.restore.from': 'Desde {path}',
  'volumes.restore.into': 'Restaurar en',
  'volumes.restore.newVolume': 'Un volumen nuevo',
  'volumes.restore.existingVolume': 'Un volumen existente',
  'volumes.restore.volume': 'Volumen',
  'volumes.restore.chooseVolume': 'Elige un volumen',
  'volumes.restore.nameTaken': 'Ya existe un volumen llamado {volume}.',
  'volumes.restore.start': 'Restaurar',
  'volumes.restore.done': 'Se restauró {volume}',
  'volumes.restore.cancelled':
    'Se canceló la restauración de {volume}; puede que el volumen se haya escrito en parte.',
  'volumes.transfer.written': 'Escrito',
  'volumes.transfer.sent': 'Enviado',
  'volumes.transfer.archives': 'Archivos tar',
  'volumes.flow.running.backup':
    '{containers} {count, plural, one {está} other {están}} en ejecución y {count, plural, one {usa} other {usan}} {volume}. Una copia de seguridad hecha mientras escriben en él puede ser incoherente, como una base de datos sorprendida a mitad de una escritura.',
  'volumes.flow.running.restore':
    '{containers} {count, plural, one {está} other {están}} en ejecución y {count, plural, one {usa} other {usan}} {volume}. Homeport {count, plural, one {lo detiene durante la restauración y lo vuelve a iniciar} other {los detiene durante la restauración y los vuelve a iniciar}} después.',
  'volumes.flow.running.clone':
    '{containers} {count, plural, one {está} other {están}} en ejecución y {count, plural, one {usa} other {usan}} {volume}. Una copia hecha mientras escriben en él puede ser incoherente, como una base de datos sorprendida a mitad de una escritura.',
  'volumes.flow.anyway.backup': 'Hacer la copia de seguridad de todos modos',
  'volumes.flow.anyway.clone': 'Copiar de todos modos',
  'volumes.flow.stopDuring.backup':
    '{count, plural, one {Detenerlo} other {Detenerlos}} durante la copia de seguridad',
  'volumes.flow.stopDuring.restore': 'Detener y restaurar',
  'volumes.flow.stopDuring.clone':
    '{count, plural, one {Detenerlo} other {Detenerlos}} durante la copia',
  'volumes.flow.replace':
    '{volume} ya contiene archivos. Al restaurar se borran y se ponen en su lugar los de la copia de seguridad.',
  'volumes.flow.replaceConfirm': 'Sustituir su contenido',
  'volumes.usedBy': 'Usado por {containers}.',
  'volumes.notUsed': 'Ningún contenedor usa este volumen.',
  'volumes.createDialog.title': 'Crear un volumen',
  'volumes.createDialog.name': 'Nombre',
  'volumes.createDialog.driver': 'Driver',
  'volumes.deleted': 'Se eliminó {name}',
  'volumes.pruned':
    '{count, plural, one {Se eliminó # volumen} other {Se eliminaron # volúmenes}}, {space} recuperados',
  'volumes.remove.title': '¿Eliminar el volumen {name}?',
  'volumes.remove.warning':
    '<strong>Todos los datos de este volumen se eliminan de forma permanente.</strong>',
  'volumes.remove.mounted': ' Lo tienen montado {containers}; elimina antes esos contenedores.',
  'volumes.remove.confirm': 'Eliminar volumen y datos',
  'volumes.prune.allTitle': '¿Eliminar todos los volúmenes sin usar?',
  'volumes.prune.anonymousTitle': '¿Eliminar los volúmenes anónimos sin usar?',
  'volumes.prune.allWarning':
    'Los datos de todos los volúmenes que no usa ningún contenedor se eliminan de forma permanente, incluidos los volúmenes con nombre.',
  'volumes.prune.anonymousWarning':
    'Los datos de los volúmenes sin nombre que no usa ningún contenedor se eliminan de forma permanente.',
  'volumes.prune.confirm': 'Eliminar volúmenes y datos',
} satisfies Translation<typeof en>;
