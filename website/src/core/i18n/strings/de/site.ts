import type { Translation } from '../../types';
import type en from '../en/site';

/** The layout, the header and the footer, in German. */
export default {
  // The layout and the facts every page repeats.
  'site.tagline': 'Eine App für jede Container-Engine.',
  'site.description':
    'Homeport ist eine kostenlose Open-Source-Desktop-App für Docker, Podman, containerd, Incus, Kubernetes und Nomad. Sie läuft auf Ihrem Computer und erreicht Ihre Server über SSH, TLS oder Ihre kubeconfig, ohne dort etwas zu installieren. Für Windows, macOS und Linux.',
  'site.ogImageAlt': 'Homeport – Eine App für jede Container-Engine.',
  'site.skipToContent': 'Zum Inhalt springen',

  // The header, the language menu and the theme switch.
  'header.home': 'Homeport-Startseite',
  'header.logo': 'Homeport',
  'header.nav': 'Hauptmenü',
  'header.engines': 'Engines',
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
  'footer.engines': 'Engines',
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
