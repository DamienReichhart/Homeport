import type { Translation } from '../../types';
import type en from '../en/site';

/** The layout, the header and the footer, in Italian. */
export default {
  // The layout and the facts every page repeats.
  'site.tagline': 'Un’unica app per ogni Engine di container.',
  'site.description':
    'Homeport è un’app desktop gratuita e open source per Docker, Podman, containerd, Incus, Kubernetes e Nomad. Funziona sul tuo computer e raggiunge i tuoi server tramite SSH, TLS o il tuo kubeconfig, senza installarvi nulla. Per Windows, macOS e Linux.',
  'site.ogImageAlt': 'Homeport — Un’unica app per ogni Engine di container.',
  'site.skipToContent': 'Vai al contenuto',

  // The header, the language menu and the theme switch.
  'header.home': 'Home di Homeport',
  'header.logo': 'Homeport',
  'header.nav': 'Principale',
  'header.engines': 'Engine',
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
  'footer.engines': 'Engine',
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
