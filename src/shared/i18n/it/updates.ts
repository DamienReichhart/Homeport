/** Italian text of the `updates` namespace. */
import type en from '../en/updates';
import type { Translation } from '../types';

export default {
  'updates.title': 'Aggiornamenti',
  'updates.description': 'Homeport può cercare una nuova versione all’avvio e ogni poche ore.',
  'updates.automatic': 'Cerca aggiornamenti automaticamente',
  'updates.checking': 'Controllo in corso…',
  'updates.checkNow': 'Controlla ora',
  'updates.status.available': 'È disponibile Homeport {version} (hai la {current}).',
  'updates.status.latest': 'Homeport {current} è la versione più recente.',

  'updates.notice.label': 'Aggiornamento disponibile',
  'updates.notice.available': 'È disponibile <strong>Homeport {version}</strong>.',
  'updates.notice.availablePackageManager':
    'È disponibile <strong>Homeport {version}</strong>. Aggiornalo con il tuo gestore di pacchetti:',
  'updates.notice.dismiss': 'Ignora',

  'updates.copyCommand': 'Copia il comando',
  'updates.whatsNew': 'Novità',
  'updates.install': 'Installa e riavvia',
  'updates.download': 'Scarica',
  'updates.progress.downloading': 'Download in corso…',
  'updates.progress.downloadingPercent': 'Download in corso… {percent}%',
  'updates.progress.installing': 'Installazione in corso…',

  'updates.method.inApp':
    'Gli aggiornamenti vengono scaricati, verificati con la chiave di firma di Homeport e installati quando decidi tu.',
  'updates.method.scoop':
    'Installato con Scoop: gli aggiornamenti arrivano dal bucket di Homeport.',
  'updates.method.aur':
    'Installato da AUR (homeport-bin): gli aggiornamenti li installa il tuo helper AUR.',
  'updates.method.packageManager':
    'Installato con {manager}: gli aggiornamenti arrivano dal repository dei pacchetti di Homeport.',
  'updates.method.manual':
    'Questa è una build di sviluppo: scarica le nuove versioni dal sito web.',
  'updates.palette.check': 'Cerca aggiornamenti',
} satisfies Translation<typeof en>;
