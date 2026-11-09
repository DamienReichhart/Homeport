/** Spanish text of the `registries` namespace. */
import type en from '../en/registries';
import type { Translation } from '../types';

export default {
  'registries.title': 'Registros',
  'registries.description':
    'Inicios de sesión de registros de contenedores, usados para descargar imágenes privadas, subir y compilar en todos los hosts.',
  'registries.add': 'Añadir inicio de sesión',
  'registries.noKeychain.title': 'No hay llavero del sistema',
  'registries.noKeychain.description':
    'Las contraseñas y tokens de los registros se cifran con una clave guardada en el llavero del sistema, y no hay ninguno disponible. Las imágenes se pueden seguir descargando de forma anónima.',
  'registries.empty': 'Aún no hay inicios de sesión: las imágenes se descargan de forma anónima.',
  'registries.editOne': 'Editar el inicio de sesión de {registry}',
  'registries.removeOne': 'Quitar el inicio de sesión de {registry}',
  'registries.remove.title': '¿Quitar el inicio de sesión de {registry}?',
  'registries.remove.description':
    'Las imágenes de {registry} se descargarán de forma anónima hasta que se vuelva a añadir un inicio de sesión.',
  'registries.saved': 'Se guardó el inicio de sesión de {registry}',
  'registries.removed': 'Se quitó el inicio de sesión de {registry}',
  'registries.dialog.editTitle': 'Editar el inicio de sesión de {registry}',
  'registries.dialog.addTitle': 'Añadir un inicio de sesión del registro',
  'registries.dialog.description':
    'Se usa para descargar, subir y compilar en todos los hosts. La contraseña o el token se cifran en este equipo y no se vuelven a mostrar.',
  'registries.dialog.address': 'Registro',
  'registries.dialog.username': 'Nombre de usuario',
  'registries.dialog.secret': 'Contraseña o token de acceso',
  'registries.dialog.keepSecret': 'Déjalo vacío para conservar el guardado',
  'registries.dialog.accepted': '{registry} aceptó este inicio de sesión.',
  'registries.dialog.checking': 'Comprobando…',
  'registries.dialog.check': 'Comprobar inicio de sesión',
  'registries.missing.address': 'Introduce la dirección del registro.',
  'registries.missing.username': 'Introduce el nombre de usuario.',
  'registries.missing.secret': 'Introduce la contraseña o el token de acceso.',
  'registries.login.as': 'como {username} en {registry}',
  'registries.login.anonymously': 'de forma anónima',
  'registries.login.pulledAs': 'Descargada como {username} en {registry}',
  'registries.login.pulledAnonymously': 'Descargada de forma anónima',
  'registries.login.pushedAs': 'Subida como {username} en {registry}',
  'registries.login.pushedAnonymously': 'Subida de forma anónima',
  'registries.noLogin.title': 'No hay inicio de sesión para {registry}',
  'registries.noLogin.description':
    'La mayoría de los registros rechazan las subidas anónimas. Primero, <link>añade un inicio de sesión en Ajustes → Registros</link>.',
} satisfies Translation<typeof en>;
