import type { Translation } from '../../types';
import type en from '../en/site';

/** The layout, the header and the footer, in French. */
export default {
  // The layout and the facts every page repeats.
  'site.tagline': 'Chaque Docker Engine, sur un seul tableau.',
  'site.description':
    'Homeport est une application de bureau gratuite et open source pour gérer des Docker Engines où qu’ils soient — celui de cette machine, ou des moteurs distants via SSH, TLS ou TCP — sous Windows, macOS et Linux.',
  'site.ogImageAlt': 'Homeport — Chaque Docker Engine, sur un seul tableau.',
  'site.skipToContent': 'Aller au contenu',

  // The header, the language menu and the theme switch.
  'header.home': 'Accueil de Homeport',
  'header.logo': 'Homeport',
  'header.nav': 'Principale',
  'header.features': 'Fonctionnalités',
  'header.docs': 'Documentation',
  'header.changelog': 'Nouveautés',
  'header.download': 'Télécharger',
  'header.github': 'Homeport sur GitHub',
  'header.githubShort': 'GitHub',
  'header.menu': 'Menu',
  'language.label': 'Langue',
  'theme.toLight': 'Passer au thème clair',
  'theme.toDark': 'Passer au thème sombre',

  // The footer.
  'footer.product': 'Produit',
  'footer.features': 'Fonctionnalités',
  'footer.download': 'Télécharger',
  'footer.changelog': 'Nouveautés',
  'footer.releases': 'Toutes les versions',
  'footer.documentation': 'Documentation',
  'footer.introduction': 'Introduction',
  'footer.installation': 'Installation',
  'footer.hosts': 'Connecter des hôtes',
  'footer.troubleshooting': 'Dépannage',
  'footer.project': 'Projet',
  'footer.source': 'Code source',
  'footer.issue': 'Signaler un problème',
  'footer.contributing': 'Contribuer',
  'footer.security': 'Sécurité',
  'footer.legal': 'Informations légales',
  'footer.legalNotice': 'Mentions légales',
  'footer.privacy': 'Confidentialité',
  'footer.terms': 'Conditions d’utilisation',
  'footer.cookies': 'Cookies',
  'footer.licence': '© {year} Damien Reichhart. Homeport est open source, sous licence {licence}.',
  'footer.licenceName': 'GNU AGPL, version 3',
  'footer.trademark':
    'Docker est une marque de Docker, Inc. Homeport est un projet indépendant, qui n’est ni affilié à Docker, Inc. ni approuvé par cette société.',
} satisfies Translation<typeof en>;
