import type { Translation } from '../../types';
import type en from '../en/site';

/** The layout, the header and the footer, in German. */
export default {
  // The layout and the facts every page repeats.
  'site.tagline': 'Jede Docker Engine, auf einem Board.',
  'site.description':
    'Homeport ist eine kostenlose Open-Source-Desktop-App zum Verwalten von Docker Engines überall – auf diesem Computer oder entfernt über SSH, TLS oder TCP – für Windows, macOS und Linux.',
  'site.ogImageAlt': 'Homeport – Jede Docker Engine, auf einem Board.',
  'site.skipToContent': 'Zum Inhalt springen',

  // The header, the language menu and the theme switch.
  'header.home': 'Homeport-Startseite',
  'header.logo': 'Homeport',
  'header.nav': 'Hauptmenü',
  'header.features': 'Funktionen',
  'header.docs': 'Doku',
  'header.changelog': 'Änderungen',
  'header.download': 'Download',
  'header.github': 'Homeport auf GitHub',
  'header.githubShort': 'GitHub',
  'header.menu': 'Menü',
  'language.label': 'Sprache',
  'theme.toLight': 'Zum hellen Design wechseln',
  'theme.toDark': 'Zum dunklen Design wechseln',

  // The footer.
  'footer.product': 'Produkt',
  'footer.features': 'Funktionen',
  'footer.download': 'Download',
  'footer.changelog': 'Änderungen',
  'footer.releases': 'Alle Releases',
  'footer.documentation': 'Dokumentation',
  'footer.introduction': 'Einführung',
  'footer.installation': 'Installation',
  'footer.hosts': 'Hosts verbinden',
  'footer.troubleshooting': 'Fehlerbehebung',
  'footer.project': 'Projekt',
  'footer.source': 'Quellcode',
  'footer.issue': 'Problem melden',
  'footer.contributing': 'Mitwirken',
  'footer.security': 'Sicherheit',
  'footer.legal': 'Rechtliches',
  'footer.legalNotice': 'Impressum',
  'footer.privacy': 'Datenschutz',
  'footer.terms': 'Nutzungsbedingungen',
  'footer.cookies': 'Cookies',
  'footer.licence': '© {year} Damien Reichhart. Homeport ist Open Source unter der {licence}.',
  'footer.licenceName': 'GNU AGPL, Version 3',
  'footer.trademark':
    'Docker ist eine Marke von Docker, Inc. Homeport ist ein unabhängiges Projekt, das nicht mit Docker, Inc. verbunden ist und nicht von Docker, Inc. unterstützt wird.',
} satisfies Translation<typeof en>;
