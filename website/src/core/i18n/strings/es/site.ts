import type { Translation } from '../../types';
import type en from '../en/site';

/** The layout, the header and the footer, in Spanish. */
export default {
  // The layout and the facts every page repeats.
  'site.tagline': 'Una sola app para todos los Engines de contenedores.',
  'site.description':
    'Homeport es una app de escritorio gratuita y de código abierto para Docker, Podman, containerd, Incus, Kubernetes y Nomad. Se ejecuta en tu equipo y llega a tus servidores por SSH, TLS o tu kubeconfig, sin instalar nada en ellos. Para Windows, macOS y Linux.',
  'site.ogImageAlt': 'Homeport — Una sola app para todos los Engines de contenedores.',
  'site.skipToContent': 'Saltar al contenido',

  // The header, the language menu and the theme switch.
  'header.home': 'Inicio de Homeport',
  'header.logo': 'Homeport',
  'header.nav': 'Principal',
  'header.engines': 'Engines',
  'header.docs': 'Documentación',
  'header.changelog': 'Novedades',
  'header.download': 'Descargar',
  'header.github': 'Homeport en GitHub',
  'header.githubShort': 'GitHub',
  'header.menu': 'Menú',
  'language.label': 'Idioma',
  'theme.toLight': 'Cambiar al tema claro',
  'theme.toDark': 'Cambiar al tema oscuro',

  // The footer.
  'footer.product': 'Producto',
  'footer.engines': 'Engines',
  'footer.download': 'Descargar',
  'footer.changelog': 'Novedades',
  'footer.releases': 'Todas las versiones',
  'footer.documentation': 'Documentación',
  'footer.introduction': 'Introducción',
  'footer.installation': 'Instalación',
  'footer.hosts': 'Conectar hosts',
  'footer.troubleshooting': 'Solución de problemas',
  'footer.project': 'Proyecto',
  'footer.source': 'Código fuente',
  'footer.issue': 'Informar de un problema',
  'footer.contributing': 'Contribuir',
  'footer.security': 'Seguridad',
  'footer.legal': 'Legal',
  'footer.legalNotice': 'Aviso legal',
  'footer.privacy': 'Privacidad',
  'footer.terms': 'Términos',
  'footer.cookies': 'Cookies',
  'footer.licence': '© {year} Damien Reichhart. Homeport es de código abierto bajo la {licence}.',
  'footer.licenceName': 'GNU AGPL, versión 3',
  'footer.trademark':
    'Docker es una marca comercial de Docker, Inc. Homeport es un proyecto independiente, no afiliado a Docker, Inc. ni respaldado por esta empresa.',
} satisfies Translation<typeof en>;
