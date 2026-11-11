/** Spanish text of the `quadlets` namespace. */
import type en from '../en/quadlets';
import type { Translation } from '../types';

export default {
  'quadlets.nav': 'Quadlet',
  'quadlets.title': 'Unidades Quadlet',
  'quadlets.summary': '{count, plural, one {# unidad} other {# unidades}}',
  'quadlets.search': 'Buscar unidades',
  'quadlets.empty': 'No hay unidades Quadlet',
  'quadlets.emptyDescription':
    'Las unidades Quadlet son archivos que Podman convierte en servicios de systemd, de modo que los contenedores se inician con el host y systemd los mantiene en ejecución. Instala una para verla aquí.',
  'quadlets.new': 'Nueva unidad',
  'quadlets.edit': 'Editar',
  'quadlets.notGenerated': 'aún no generado',
  'quadlets.gone': '{name} ya no está instalada',
  'quadlets.column.name': 'Unidad',
  'quadlets.column.kind': 'Tipo',
  'quadlets.column.service': 'Servicio',
  'quadlets.column.status': 'Estado',
  'quadlets.column.containers': 'Contenedores',
  'quadlets.status.running': 'En ejecución',
  'quadlets.status.starting': 'Iniciando',
  'quadlets.status.stopping': 'Deteniendo',
  'quadlets.status.stopped': 'Detenida',
  'quadlets.status.failed': 'Con errores',
  'quadlets.status.notLoaded': 'No cargada por systemd',
  'quadlets.status.template': 'Plantilla',
  'quadlets.action.start': 'Iniciar',
  'quadlets.action.stop': 'Detener',
  'quadlets.action.restart': 'Reiniciar',
  'quadlets.actionOne.start': 'Iniciar {name}',
  'quadlets.actionOne.stop': 'Detener {name}',
  'quadlets.actionOne.restart': 'Reiniciar {name}',
  'quadlets.done.start': '{name} iniciada',
  'quadlets.done.stop': '{name} detenida',
  'quadlets.done.restart': '{name} reiniciada',
  'quadlets.installed': '{name} instalada',
  'quadlets.saved': '{name} guardada',
  'quadlets.removed': '{name} eliminada',
  'quadlets.blocked.noCommands':
    'Homeport inicia y detiene las unidades con systemctl, que solo puede ejecutar por SSH o para un Podman de este equipo.',
  'quadlets.blocked.notLoaded':
    'systemd no ha generado el servicio de esta unidad, así que no hay nada que iniciar.',
  'quadlets.blocked.podmanTooOld':
    'Leer, instalar, editar y eliminar unidades requiere Podman 5.8 o posterior; este host ejecuta Podman {version}.',
  'quadlets.blocked.noFile':
    'Esta unidad solo se conoce por sus contenedores, así que su archivo no se puede cambiar.',
  'quadlets.note.labels':
    'Podman {version} no puede listar unidades Quadlet, así que estas provienen de los contenedores que crearon. Sus archivos, y las unidades que no crearon ningún contenedor, se muestran con Podman 5.7 y posterior.',
  'quadlets.note.readOnly':
    'Podman {version} lista las unidades, pero no puede leerlas, instalarlas, editarlas ni eliminarlas; para eso se necesita Podman 5.8 o posterior.',
  'quadlets.note.noCommands':
    'Para iniciar y detener unidades, este host tiene que alcanzarse por SSH o ser un Podman de este equipo.',
  'quadlets.detail.service': 'Servicio',
  'quadlets.detail.unit': 'Unidad de systemd',
  'quadlets.detail.scope': 'Se ejecuta como',
  'quadlets.detail.path': 'Archivo',
  'quadlets.detail.app': 'Aplicación',
  'quadlets.detail.pod': 'Pod',
  'quadlets.detail.containers':
    '{count, plural, =0 {Contenedores} one {# contenedor} other {# contenedores}}',
  'quadlets.detail.noContainers': 'No ha creado ningún contenedor, o su servicio está detenido.',
  'quadlets.scope.user': 'Una unidad de usuario (systemctl --user)',
  'quadlets.scope.system': 'Una unidad del sistema',
  'quadlets.file.title': 'Archivo de la unidad',
  'quadlets.file.unknown': 'Esta unidad solo se conoce por la etiqueta de sus contenedores.',
  'quadlets.file.tooOld':
    'Podman {version} no puede leer archivos de unidad; mostrarlos requiere Podman 5.8 o posterior.',
  'quadlets.form.newTitle': 'Nueva unidad Quadlet',
  'quadlets.form.newDescription':
    'Podman escribe el archivo donde van las unidades de su usuario y recarga systemd. Inicia la unidad cuando esté instalada.',
  'quadlets.form.editTitle': 'Editar {name}',
  'quadlets.form.editDescription':
    'Al guardar se sustituye el archivo y se recarga systemd; reinicia la unidad para que un servicio en ejecución lo use. Los archivos instalados junto a ella se mantienen.',
  'quadlets.form.name': 'Nombre de archivo',
  'quadlets.form.kind': 'Tipo',
  'quadlets.form.openFile': 'Abrir archivo…',
  'quadlets.form.openTitle': 'Abrir un archivo de unidad Quadlet',
  'quadlets.form.unitFiles': 'Archivos de unidad Quadlet',
  'quadlets.form.content': 'Archivo de la unidad',
  'quadlets.form.extra': 'Archivos junto a ella',
  'quadlets.form.extraHint':
    'Un Containerfile, YAML de kube o archivo env que use la unidad, instalado en su carpeta de aplicación.',
  'quadlets.form.extraTitle': 'Elegir archivos que instalar junto a la unidad',
  'quadlets.form.addExtra': 'Añadir archivos…',
  'quadlets.form.removeExtra': 'Quitar {name}',
  'quadlets.form.install': 'Instalar',
  'quadlets.form.save': 'Guardar',
  'quadlets.form.overwrite': 'Sobrescribir',
  'quadlets.form.nameInvalid':
    'Ponle un nombre como web.container, que termine en .container, .pod, .volume, .network, .kube, .image, .build o .artifact.',
  'quadlets.form.sectionMissing':
    'El archivo necesita una sección {section} para que Quadlet lo lea.',
  'quadlets.form.extraInvalid':
    'Cada archivo junto a la unidad necesita su propio nombre simple, sin carpetas.',
  'quadlets.remove.title': '¿Eliminar {name}?',
  'quadlets.remove.description':
    'Podman borra el archivo de la unidad y recarga systemd. Los contenedores, volúmenes e imágenes que creó se mantienen.',
  'quadlets.remove.stop': 'Detener su servicio si está en ejecución',
  'quadlets.remove.stopRunning': 'Detener antes su servicio en ejecución',
} satisfies Translation<typeof en>;
