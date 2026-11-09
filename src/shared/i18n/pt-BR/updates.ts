/** Brazilian Portuguese text of the `updates` namespace. */
import type en from '../en/updates';
import type { Translation } from '../types';

export default {
  'updates.title': 'Atualizações',
  'updates.description':
    'O Homeport pode procurar uma nova versão ao iniciar e a cada poucas horas.',
  'updates.automatic': 'Verificar atualizações automaticamente',
  'updates.checking': 'Verificando…',
  'updates.checkNow': 'Verificar agora',
  'updates.status.available': 'O Homeport {version} está disponível (você tem a {current}).',
  'updates.status.latest': 'O Homeport {current} é a versão mais recente.',

  'updates.notice.label': 'Atualização disponível',
  'updates.notice.available': 'O <strong>Homeport {version}</strong> está disponível.',
  'updates.notice.availablePackageManager':
    'O <strong>Homeport {version}</strong> está disponível. Atualize-o com o seu gerenciador de pacotes:',
  'updates.notice.dismiss': 'Dispensar',

  'updates.copyCommand': 'Copiar comando',
  'updates.whatsNew': 'Novidades',
  'updates.install': 'Instalar e reiniciar',
  'updates.download': 'Baixar',
  'updates.progress.downloading': 'Baixando…',
  'updates.progress.downloadingPercent': 'Baixando… {percent}%',
  'updates.progress.installing': 'Instalando…',

  'updates.method.inApp':
    'As atualizações são baixadas, verificadas com a chave de assinatura do Homeport e instaladas quando você escolher.',
  'updates.method.scoop': 'Instalado com o Scoop: as atualizações vêm do bucket do Homeport.',
  'updates.method.aur':
    'Instalado a partir do AUR (homeport-bin): o seu AUR helper instala as atualizações.',
  'updates.method.packageManager':
    'Instalado com {manager}: as atualizações vêm do repositório de pacotes do Homeport.',
  'updates.method.manual': 'Esta é uma build de desenvolvimento: baixe novas versões no site.',
  'updates.palette.check': 'Verificar atualizações',
} satisfies Translation<typeof en>;
