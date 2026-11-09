/** Spanish text of the `updates` namespace. */
import type en from '../en/updates';
import type { Translation } from '../types';

export default {
  'updates.title': 'Actualizaciones',
  'updates.description': 'Homeport puede buscar una versión nueva al iniciarse y cada pocas horas.',
  'updates.automatic': 'Buscar actualizaciones automáticamente',
  'updates.checking': 'Comprobando…',
  'updates.checkNow': 'Comprobar ahora',
  'updates.status.available': 'Homeport {version} está disponible (tienes {current}).',
  'updates.status.latest': 'Homeport {current} es la versión más reciente.',
  'updates.notice.label': 'Actualización disponible',
  'updates.notice.available': '<strong>Homeport {version}</strong> está disponible.',
  'updates.notice.availablePackageManager':
    '<strong>Homeport {version}</strong> está disponible. Actualízalo con tu gestor de paquetes:',
  'updates.notice.dismiss': 'Descartar',
  'updates.copyCommand': 'Copiar comando',
  'updates.whatsNew': 'Novedades',
  'updates.install': 'Instalar y reiniciar',
  'updates.download': 'Descargar',
  'updates.progress.downloading': 'Descargando…',
  'updates.progress.downloadingPercent': 'Descargando… {percent} %',
  'updates.progress.installing': 'Instalando…',
  'updates.method.inApp':
    'Las actualizaciones se descargan, se verifican con la clave de firma de Homeport y se instalan cuando tú decidas.',
  'updates.method.scoop': 'Instalado con Scoop: las actualizaciones vienen del bucket de Homeport.',
  'updates.method.aur':
    'Instalado desde el AUR (homeport-bin): tu asistente de AUR instala las actualizaciones.',
  'updates.method.packageManager':
    'Instalado con {manager}: las actualizaciones vienen del repositorio de paquetes de Homeport.',
  'updates.method.manual':
    'Esta es una compilación de desarrollo: descarga las versiones nuevas desde el sitio web.',
  'updates.palette.check': 'Buscar actualizaciones',
} satisfies Translation<typeof en>;
