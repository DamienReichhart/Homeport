/** Spanish text of the `repositories` namespace. */
import type en from '../en/repositories';
import type { Translation } from '../types';

export default {
  'repositories.title': 'Repositorios',
  'repositories.description':
    'Repositorios git desde los que se despliegan los stacks sincronizados, en todos los hosts. Homeport solo los lee.',
  'repositories.add': 'Añadir repositorio',
  'repositories.empty': 'Aún no hay repositorios. Añade uno para crear un stack sincronizado.',
  'repositories.manageOne': 'Gestionar el acceso a {repository}',
  'repositories.removeOne': 'Quitar {repository}',
  'repositories.remove.title': '¿Quitar {repository}?',
  'repositories.remove.description':
    'Homeport olvida su clave de despliegue o su token y borra su clon en este equipo. El repositorio en sí no se toca. Quita también la clave de despliegue del repositorio en tu forja.',
  'repositories.noKeychain':
    'Las claves de despliegue y los tokens se cifran con una clave guardada en el llavero del sistema, y no hay ninguno disponible, así que solo se pueden añadir repositorios públicos.',
  'repositories.public': 'Un repositorio público: Homeport lo lee sin credenciales.',
  'repositories.added': 'Se añadió {repository}',
  'repositories.removed': 'Se quitó {repository}',
  'repositories.keyRotated':
    'Se creó una clave de despliegue nueva. Añádela al repositorio en lugar de la anterior.',
  'repositories.tokenReplaced': 'Se sustituyó el token',
  'repositories.hostKeyReset':
    'Homeport confiará en la clave que presente el servidor la próxima vez.',
  'repositories.auth.withKey': 'Con una clave de despliegue, en {host}',
  'repositories.auth.withToken': 'Como {username} con un token, en {host}',
  'repositories.auth.public': 'Público, en {host}',
  'repositories.dialog.addTitle': 'Añadir un repositorio',
  'repositories.dialog.description':
    'Homeport lo clona en este equipo para desplegar stacks sincronizados desde él. Nunca hace commit ni push.',
  'repositories.dialog.url': 'URL del repositorio',
  'repositories.dialog.auth': 'Acceso',
  'repositories.dialog.authKey': 'Clave de despliegue',
  'repositories.dialog.authToken': 'Token',
  'repositories.dialog.authNone': 'Público',
  'repositories.dialog.authKeyHelp':
    'Homeport crea una clave solo para este repositorio. Tú la añades al repositorio con acceso de lectura; tus propias claves SSH nunca se usan.',
  'repositories.dialog.authTokenHelp':
    'Un token de acceso que pueda leer el contenido del repositorio, enviado por HTTPS. Se cifra en este equipo y no se vuelve a mostrar.',
  'repositories.dialog.authNoneHelp': 'Para un repositorio público por HTTPS. No se guarda nada.',
  'repositories.dialog.username': 'Nombre de usuario',
  'repositories.dialog.token': 'Token de acceso',
  'repositories.dialog.add': 'Añadir',
  'repositories.dialog.addWithKey': 'Añadir y crear una clave de despliegue',
  'repositories.missing.url': 'Introduce la URL del repositorio.',
  'repositories.missing.validUrl':
    'Introduce una URL SSH o HTTPS, como git@github.com:owner/repo.git o https://gitlab.com/owner/repo.git.',
  'repositories.missing.auth': 'Elige cómo accede Homeport a este repositorio.',
  'repositories.missing.username': 'Introduce el nombre de usuario al que pertenece el token.',
  'repositories.missing.token': 'Introduce el token de acceso.',
  'repositories.deployKey.title': 'Clave de despliegue',
  'repositories.deployKey.github':
    'GitHub: en el repositorio, <strong>Settings → Deploy keys → Add deploy key</strong>.',
  'repositories.deployKey.gitlab':
    'GitLab: en el proyecto, <strong>Settings → Repository → Deploy keys</strong>.',
  'repositories.deployKey.gitea':
    'Gitea y Forgejo: en el repositorio, <strong>Settings → Deploy keys → Add deploy key</strong>.',
  'repositories.deployKey.readOnly':
    'Deja desactivado el acceso de escritura: Homeport solo lee. Luego prueba el acceso abajo.',
  'repositories.deployKey.fingerprint': 'Huella: <mono>{fingerprint}</mono>',
  'repositories.rotate.button': 'Nueva clave',
  'repositories.rotate.title': '¿Crear una clave de despliegue nueva?',
  'repositories.rotate.description':
    'La clave actual deja de funcionar para Homeport. Añade la nueva al repositorio y quita allí la anterior.',
  'repositories.rotate.confirm': 'Crear una clave nueva',
  'repositories.token.replace': 'Sustituir el token',
  'repositories.token.save': 'Guardar token',
  'repositories.hostKey.trusted': 'Clave de confianza de {host}: <mono>{fingerprint}</mono>',
  'repositories.hostKey.reset': 'Restablecer',
  'repositories.hostKey.resetTitle': '¿Restablecer la clave de confianza de {host}?',
  'repositories.hostKey.resetDescription':
    'Hazlo solo si {host} cambió su clave SSH y comprobaste la nueva huella en su sitio web. Homeport confiará en cualquier clave que presente a continuación.',
  'repositories.test.button': 'Probar acceso',
  'repositories.test.testing': 'Probando…',
  'repositories.test.success':
    'Homeport puede leerlo: {branches, plural, one {# rama} other {# ramas}} y {tags, plural, one {# etiqueta} other {# etiquetas}}.',
} satisfies Translation<typeof en>;
