import type { Translation } from '../../types';
import type en from '../en/site';

/** The layout, the header and the footer, in Brazilian Portuguese. Still in English: translate each value of ../en/site.ts. */
export default {
  // The layout and the facts every page repeats.
  'site.tagline': 'Every Docker Engine, on one board.',
  'site.description':
    'Homeport is a free, open-source desktop app for managing Docker Engines anywhere — this machine’s, or remote ones over SSH, TLS or TCP — for Windows, macOS and Linux.',
  'site.ogImageAlt': 'Homeport — Every Docker Engine, on one board.',
  'site.skipToContent': 'Skip to content',

  // The header, the language menu and the theme switch.
  'header.home': 'Homeport home',
  'header.logo': 'Homeport',
  'header.nav': 'Main',
  'header.features': 'Features',
  'header.docs': 'Docs',
  'header.changelog': 'Changelog',
  'header.download': 'Download',
  'header.github': 'Homeport on GitHub',
  'header.githubShort': 'GitHub',
  'header.menu': 'Menu',
  'language.label': 'Language',
  'theme.toLight': 'Switch to light theme',
  'theme.toDark': 'Switch to dark theme',

  // The footer.
  'footer.product': 'Product',
  'footer.features': 'Features',
  'footer.download': 'Download',
  'footer.changelog': 'Changelog',
  'footer.releases': 'All releases',
  'footer.documentation': 'Documentation',
  'footer.introduction': 'Introduction',
  'footer.installation': 'Installation',
  'footer.hosts': 'Connecting Hosts',
  'footer.troubleshooting': 'Troubleshooting',
  'footer.project': 'Project',
  'footer.source': 'Source code',
  'footer.issue': 'Report an issue',
  'footer.contributing': 'Contributing',
  'footer.security': 'Security',
  'footer.legal': 'Legal',
  'footer.legalNotice': 'Legal notice',
  'footer.privacy': 'Privacy',
  'footer.terms': 'Terms',
  'footer.cookies': 'Cookies',
  'footer.licence': '© {year} Damien Reichhart. Homeport is open source under the {licence}.',
  'footer.licenceName': 'GNU AGPL, version 3',
  'footer.trademark':
    'Docker is a trademark of Docker, Inc. Homeport is an independent project, not affiliated with or endorsed by Docker, Inc.',
} satisfies Translation<typeof en>;
