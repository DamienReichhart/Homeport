import type { Translation } from '../../types';
import type en from '../en/site';

/** The layout, the header and the footer, in Italian. */
export default {
  // The layout and the facts every page repeats.
  'site.tagline': 'Ogni Docker Engine, in un’unica bacheca.',
  'site.description':
    'Homeport è un’app desktop gratuita e open source per gestire i Docker Engine ovunque si trovino — quello di questo computer o quelli remoti tramite SSH, TLS o TCP — per Windows, macOS e Linux.',
  'site.ogImageAlt': 'Homeport — Ogni Docker Engine, in un’unica bacheca.',
  'site.skipToContent': 'Vai al contenuto',

  // The header, the language menu and the theme switch.
  'header.home': 'Home di Homeport',
  'header.logo': 'Homeport',
  'header.nav': 'Principale',
  'header.features': 'Funzionalità',
  'header.docs': 'Documentazione',
  'header.changelog': 'Novità',
  'header.download': 'Download',
  'header.github': 'Homeport su GitHub',
  'header.githubShort': 'GitHub',
  'header.menu': 'Menu',
  'language.label': 'Lingua',
  'theme.toLight': 'Passa al tema chiaro',
  'theme.toDark': 'Passa al tema scuro',

  // The footer.
  'footer.product': 'Prodotto',
  'footer.features': 'Funzionalità',
  'footer.download': 'Download',
  'footer.changelog': 'Novità',
  'footer.releases': 'Tutte le release',
  'footer.documentation': 'Documentazione',
  'footer.introduction': 'Introduzione',
  'footer.installation': 'Installazione',
  'footer.hosts': 'Connettere gli host',
  'footer.troubleshooting': 'Risoluzione dei problemi',
  'footer.project': 'Progetto',
  'footer.source': 'Codice sorgente',
  'footer.issue': 'Segnala un problema',
  'footer.contributing': 'Contribuire',
  'footer.security': 'Sicurezza',
  'footer.legal': 'Informazioni legali',
  'footer.legalNotice': 'Note legali',
  'footer.privacy': 'Privacy',
  'footer.terms': 'Termini',
  'footer.cookies': 'Cookie',
  'footer.licence': '© {year} Damien Reichhart. Homeport è open source con licenza {licence}.',
  'footer.licenceName': 'GNU AGPL, versione 3',
  'footer.trademark':
    'Docker è un marchio di Docker, Inc. Homeport è un progetto indipendente, non affiliato a Docker, Inc. né approvato da essa.',
} satisfies Translation<typeof en>;
