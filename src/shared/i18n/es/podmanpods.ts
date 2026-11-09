/** Spanish text of the `podmanpods` namespace. */
import type en from '../en/podmanpods';
import type { Translation } from '../types';

export default {
  'podmanpods.nav': 'Pods',
  'podmanpods.title': 'Pods',
  'podmanpods.summary': '{count, plural, one {# pod} other {# pods}}',
  'podmanpods.search': 'Buscar pods',
  'podmanpods.empty': 'No hay pods',
  'podmanpods.emptyDescription':
    'Un pod agrupa contenedores que comparten una red y se inician y detienen juntos. Crea uno y luego ejecuta contenedores en él.',
  'podmanpods.new': 'Nuevo pod',
  'podmanpods.column.name': 'Nombre',
  'podmanpods.column.status': 'Estado',
  'podmanpods.column.containers': 'Contenedores',
  'podmanpods.column.networks': 'Redes',
  'podmanpods.column.created': 'Creado',
  'podmanpods.running': '{running} de {count} en ejecución',
  'podmanpods.status.running': 'En ejecución',
  'podmanpods.status.degraded': 'En ejecución parcial',
  'podmanpods.status.paused': 'En pausa',
  'podmanpods.status.created': 'Creado',
  'podmanpods.status.stopped': 'Detenido',
  'podmanpods.status.failed': 'Con errores',
  'podmanpods.action.start': 'Iniciar',
  'podmanpods.action.stop': 'Detener',
  'podmanpods.action.restart': 'Reiniciar',
  'podmanpods.action.pause': 'Pausar',
  'podmanpods.action.unpause': 'Reanudar',
  'podmanpods.actionOne.start': 'Iniciar {name}',
  'podmanpods.actionOne.stop': 'Detener {name}',
  'podmanpods.actionOne.restart': 'Reiniciar {name}',
  'podmanpods.actionOne.pause': 'Pausar {name}',
  'podmanpods.actionOne.unpause': 'Reanudar {name}',
  'podmanpods.removeOne': 'Eliminar {name}',
  'podmanpods.done.start': '{name} iniciado',
  'podmanpods.done.stop': '{name} detenido',
  'podmanpods.done.restart': '{name} reiniciado',
  'podmanpods.done.pause': '{name} en pausa',
  'podmanpods.done.unpause': '{name} reanudado',
  'podmanpods.removed':
    '{name} eliminado, {count, plural, =0 {sin contenedores} one {con # contenedor} other {con # contenedores}}',
  'podmanpods.created': '{name} creado',
  'podmanpods.remove.title': '¿Eliminar {name}?',
  'podmanpods.remove.stopped':
    '{count, plural, =0 {Se eliminan el pod y su contenedor infra.} one {Se eliminan el pod, su contenedor infra y su # contenedor.} other {Se eliminan el pod, su contenedor infra y sus # contenedores.}} Sus volúmenes se conservan.',
  'podmanpods.remove.running':
    '{count, plural, =0 {El pod está en ejecución: se detiene y luego se elimina con su contenedor infra.} one {El pod está en ejecución: su # contenedor se detiene y luego se elimina con el pod y su contenedor infra.} other {El pod está en ejecución: sus # contenedores se detienen y luego se eliminan con el pod y su contenedor infra.}} Sus volúmenes se conservan.',
  'podmanpods.remove.stopAndRemove': 'Detener y eliminar',
  'podmanpods.detail.containers':
    '{count, plural, =0 {Contenedores} one {# contenedor} other {# contenedores}}',
  'podmanpods.detail.noContainers':
    'Aún no se ejecuta nada en este pod aparte de su contenedor infra.',
  'podmanpods.detail.infra': 'Infra',
  'podmanpods.detail.settings': 'Ajustes',
  'podmanpods.detail.ports': 'Puertos publicados',
  'podmanpods.detail.noPorts': 'Ninguno',
  'podmanpods.detail.shared': 'Namespaces compartidos',
  'podmanpods.detail.networks': 'Redes',
  'podmanpods.detail.hostname': 'Nombre de host',
  'podmanpods.detail.created': 'Creado',
  'podmanpods.detail.labels': 'Etiquetas',
  'podmanpods.detail.command': 'Creado con',
  'podmanpods.detail.runContainer': 'Ejecutar contenedor',
  'podmanpods.detail.runContainerIn': 'Ejecutar un contenedor en {name}',
  'podmanpods.newDialog.title': 'Nuevo pod',
  'podmanpods.newDialog.description':
    'Sus contenedores comparten una sola red, así que los puertos se publican en el pod, para todos ellos.',
  'podmanpods.newDialog.name': 'Nombre',
  'podmanpods.newDialog.portsHint':
    'Deja vacío el puerto del host para que Podman elija uno libre. Los contenedores que se ejecutan en el pod no pueden publicar puertos propios.',
  'podmanpods.newDialog.labels': 'Etiquetas',
  'podmanpods.newDialog.create': 'Crear pod',
  'podmanpods.form.nameInvalid':
    'Empieza con una letra o un dígito y luego usa letras, dígitos, _, . o -.',
  'podmanpods.form.portInvalid': 'Los puertos son números del 1 al 65535.',
  'podmanpods.form.labelInvalid': 'Escribe una etiqueta clave=valor por línea.',
} satisfies Translation<typeof en>;
