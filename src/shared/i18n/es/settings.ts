/** Spanish text of the `settings` namespace. */
import type en from '../en/settings';
import type { Translation } from '../types';

export default {
  'settings.nav': 'Ajustes',
  'settings.title': 'Ajustes',
  'settings.appearance.title': 'Apariencia',
  'settings.appearance.theme': 'Tema',
  'settings.theme.system': 'Igual que el sistema',
  'settings.theme.light': 'Claro',
  'settings.theme.dark': 'Oscuro',
  'settings.appearance.language': 'Idioma',
  'settings.language.system': 'Igual que el sistema ({language})',
  'settings.desktop.title': 'Escritorio',
  'settings.desktop.description': 'Cómo se ejecuta Homeport en este equipo.',
  'settings.desktop.closeToTray': 'Cerrar a la bandeja del sistema',
  'settings.desktop.closeToTrayHint':
    'Al cerrar la ventana, Homeport y sus conexiones con los hosts siguen activos en la bandeja. Sal desde el icono de la bandeja. En escritorios que ocultan los iconos de la bandeja, como GNOME sin una extensión, puede que el icono no se muestre.',
  'settings.desktop.noTray':
    'Este escritorio no tiene una bandeja del sistema que Homeport pueda usar, así que al cerrar la ventana se sale de la app.',
  'settings.desktop.startAtLogin': 'Iniciar al iniciar sesión',
  'settings.desktop.startAtLoginHint':
    'Homeport se inicia cuando inicias sesión en este equipo, así vigila tus hosts desde el principio.',
  'settings.desktop.startHidden': 'Iniciar oculto en la bandeja',
  'settings.desktop.startHiddenHint':
    'Cuando se inicia al iniciar sesión, Homeport se queda en la bandeja del sistema sin abrir su ventana.',
  'settings.desktop.startHiddenNoTray':
    'Necesita una bandeja del sistema desde la que abrir la ventana, y este escritorio no la tiene.',
  'settings.notifications.title': 'Notificaciones',
  'settings.notifications.description':
    'Homeport te avisa, incluso con su ventana cerrada a la bandeja, cuando:',
  'settings.notifications.containerDied':
    'Un contenedor se detiene con un error o se queda sin memoria',
  'settings.notifications.containerUnhealthy': 'Un contenedor pasa a no saludable',
  'settings.notifications.hostUnreachable': 'Un host conectado pasa a ser inaccesible',
  'settings.notifications.hostBack': 'Un host inaccesible vuelve a estar disponible',
  'settings.machine.title': 'Este equipo',
  'settings.machine.description': 'Lo que Homeport puede usar aquí.',
  'settings.machine.keychain': 'Llavero del sistema',
  'settings.machine.keychainAvailable':
    'Disponible: las claves SSH, los certificados, los entornos de los stacks y los inicios de sesión de los registros se guardan cifrados.',
  'settings.machine.keychainUnavailable':
    'No disponible: no se pueden guardar claves SSH, certificados TLS, entornos de stacks ni inicios de sesión de registros.',
  'settings.machine.compose': 'CLI de Compose',
  'settings.machine.composeMissing': 'Los stacks se pueden iniciar y detener, pero no desplegar.',
  'settings.machine.stackDeploy': 'CLI de Docker, para Swarms',
  'settings.machine.stackDeployMissing':
    'No se pueden desplegar stacks en un Swarm; todo lo demás de los Swarms funciona.',
  'settings.machine.git': 'git',
  'settings.machine.helm': 'CLI de Helm',
  'settings.sync.title': 'Stacks sincronizados',
  'settings.sync.interval': 'Comprobar los repositorios cada',
  'settings.sync.minutes': 'minutos',
  'settings.data.title': 'Datos',
  'settings.data.description': 'Lo que Homeport guarda en este equipo.',
  'settings.data.backup': 'Hacer copia de seguridad…',
  'settings.data.restore': 'Restaurar…',
  'settings.data.backupHint':
    'Guarda todos los hosts, stacks, inicios de sesión de registros, repositorios y ajustes, con sus secretos, en un archivo protegido con una frase de contraseña, y restáuralo en este u otro equipo.',
  'settings.backup.title': 'Hacer copia de seguridad de los datos de Homeport',
  'settings.backup.description':
    'El archivo contiene todos los hosts, clústeres, stacks, inicios de sesión de registros, repositorios y ajustes, y sus claves, certificados, tokens, contraseñas y entornos. Cualquiera que tenga el archivo y su frase de contraseña puede usarlos, así que guarda ambos en un lugar seguro. No se incluyen los eventos del Engine ni el historial de despliegues.',
  'settings.backup.passphrase': 'Frase de contraseña',
  'settings.backup.passphraseHint': 'Al menos {min} caracteres. No se puede recuperar.',
  'settings.backup.confirmation': 'Repite la frase de contraseña',
  'settings.backup.problem.short': 'Usa al menos {min} caracteres.',
  'settings.backup.problem.mismatch': 'Las dos frases de contraseña no coinciden.',
  'settings.backup.start': 'Hacer copia de seguridad…',
  'settings.backup.saveTitle': 'Guardar la copia de seguridad como',
  'settings.backup.fileType': 'Copia de seguridad de Homeport',
  'settings.backup.done':
    'Copia de seguridad de {hosts, plural, one {# host} other {# hosts}} y {stacks, plural, one {# stack} other {# stacks}} hecha',
  'settings.backup.withoutSecrets':
    'Estos se guardaron sin sus secretos, que no se pudieron abrir:',
  'settings.restore.pick': 'Restaurar desde',
  'settings.restore.title': 'Restaurar los datos de Homeport',
  'settings.restore.from': 'Desde {path}',
  'settings.restore.open': 'Abrir',
  'settings.restore.madeBy': 'Copia de seguridad hecha por Homeport {version} el {date}. Contiene:',
  'settings.restore.holds': 'Lo que contiene la copia de seguridad',
  'settings.restore.hosts': '{count, plural, one {# host} other {# hosts}}',
  'settings.restore.clusters': '{count, plural, one {# clúster} other {# clústeres}}',
  'settings.restore.stacks': '{count, plural, one {# stack} other {# stacks}}',
  'settings.restore.registries':
    '{count, plural, one {# inicio de sesión de registro} other {# inicios de sesión de registros}}',
  'settings.restore.repositories': '{count, plural, one {# repositorio} other {# repositorios}}',
  'settings.restore.settings': 'Tus ajustes',
  'settings.restore.replaces':
    'Restaurar sustituye todos los hosts, stacks, inicios de sesión de registros, repositorios y ajustes que tienes ahora, y olvida los eventos del Engine y el historial de despliegues. Antes se guarda una copia de los datos actuales en la carpeta de datos de Homeport.',
  'settings.restore.confirm': 'Sustituirlo todo',
  'settings.restore.done':
    'Se restauraron {hosts, plural, one {# host} other {# hosts}} y {stacks, plural, one {# stack} other {# stacks}}',
  'settings.data.keepEvents': 'Conservar los eventos del Engine durante',
  'settings.data.days': 'días',
  'settings.data.keepEventsHint':
    'Cada host conserva como máximo 10.000 eventos para la página Eventos. Los más antiguos se eliminan cada hora.',
  'settings.machine.gitMissing':
    'Los stacks sincronizados lo necesitan; todos los demás tipos de stack funcionan sin él.',
  'settings.machine.helmMissing':
    'No se pueden listar ni instalar releases de Helm; todo lo demás de los clústeres funciona.',
  'settings.about.tagline': 'Todos los Docker Engines, en un solo tablero.',
  'settings.about.openLogFolder': 'Abrir carpeta de logs',
  'settings.about.licenses': 'Licencias',
  'settings.licenses.title': 'Licencias',
  'settings.licenses.description':
    'Homeport es software libre bajo la GNU Affero General Public License, versión 3, creado sobre los paquetes de código abierto que aparecen a continuación.',
  'settings.licenses.source': 'El código fuente de esta versión se publica bajo la misma licencia.',
  'settings.licenses.sourceLink': 'Código fuente de v{version}',
  'settings.licenses.thirdParty': 'Software de terceros',
  'settings.licenses.missing.title': 'No incluido en esta compilación',
  'settings.licenses.missing.description':
    'Las compilaciones de release las incluyen; ejecuta <code>{command}</code> para generarlas en una compilación de desarrollo.',
  'settings.palette.group': 'Ajustes',
  'settings.palette.themeLight': 'Tema: claro',
  'settings.palette.themeDark': 'Tema: oscuro',
  'settings.palette.themeSystem': 'Tema: igual que el sistema',
} satisfies Translation<typeof en>;
