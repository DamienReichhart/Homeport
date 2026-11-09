/** Italian text of the `app` namespace. */
import type en from '../en/app';
import type { Translation } from '../types';

export default {
  'app.nav.main': 'Principale',
  'app.nav.docs': 'Documentazione',
  'app.nav.github': 'GitHub',
  'app.nav.opensInBrowser': '{label} (si apre nel browser)',
  'app.keychainMissing':
    'Non è disponibile alcun portachiavi del sistema operativo, quindi non è possibile memorizzare chiavi SSH, certificati TLS e ambienti degli stack. Gli host locali, TCP e ssh-agent funzionano comunque.',
  'app.startup.title': 'Homeport non riesce ad aprire il suo database',
  'app.startup.description':
    'Non è stato avviato nient’altro. Il database si trova in questo file:',
  'app.startup.openFolder': 'Apri la cartella',
  'app.crash.title': 'Si è verificato un problema',
  'app.crash.description':
    'Homeport ha riscontrato un problema imprevisto in questa pagina. Di solito ricaricarla risolve; se succede ancora, copia il rapporto da Dettagli in una segnalazione.',
  'app.crash.reload': 'Ricarica',
  'app.crash.backToOverview': 'Torna alla panoramica',
  'app.shortcuts.title': 'Scorciatoie da tastiera',
  'app.shortcuts.description':
    'Le scorciatoie disponibili qui. Tasti come N e / non hanno effetto mentre scrivi in un campo.',
  'app.shortcuts.general': 'Generali',
  'app.shortcuts.navigation': 'Vai a',
  'app.shortcuts.hosts': 'Host',
  'app.shortcuts.help': 'Mostra le scorciatoie da tastiera',
  'app.shortcuts.settings': 'Apri Impostazioni',
  'app.shortcuts.goTo': 'Vai a {page}',
  'app.shortcuts.previousHost': 'Host precedente',
  'app.shortcuts.nextHost': 'Host successivo',
  'app.shortcuts.refresh': 'Aggiorna gli elenchi di questo host',
  'app.palette.title': 'Riquadro comandi',
  'app.palette.description': 'Vai a una pagina, cambia host o esegui un comando.',
  'app.palette.open': 'Apri il riquadro comandi',
  'app.palette.placeholder': 'Digita una pagina, un host o un comando…',
  'app.palette.empty': 'Nessun risultato.',
  'app.palette.recent': 'Recenti',
  'app.palette.goTo': 'Vai a',
  'app.palette.hosts': 'Host',
  'app.palette.actions': 'Azioni',
  'app.palette.switchTo': 'Passa a {name}',
} satisfies Translation<typeof en>;
