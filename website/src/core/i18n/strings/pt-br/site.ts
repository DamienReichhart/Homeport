import type { Translation } from '../../types';
import type en from '../en/site';

/** The layout, the header and the footer, in Brazilian Portuguese. */
export default {
  // The layout and the facts every page repeats.
  'site.tagline': 'Um só app para todos os Engines de contêiner.',
  'site.description':
    'O Homeport é um app de desktop gratuito e de código aberto para Docker, Podman, containerd, Incus, Kubernetes e Nomad. Ele roda na sua máquina e alcança seus servidores por SSH, TLS ou pelo seu kubeconfig, sem nada instalado neles. Para Windows, macOS e Linux.',
  'site.ogImageAlt': 'Homeport — Um só app para todos os Engines de contêiner.',
  'site.skipToContent': 'Pular para o conteúdo',

  // The header, the language menu and the theme switch.
  'header.home': 'Página inicial do Homeport',
  'header.logo': 'Homeport',
  'header.nav': 'Principal',
  'header.engines': 'Engines',
  'header.docs': 'Documentação',
  'header.changelog': 'Histórico de versões',
  'header.download': 'Download',
  'header.github': 'Homeport no GitHub',
  'header.githubShort': 'GitHub',
  'header.menu': 'Menu',
  'language.label': 'Idioma',
  'theme.toLight': 'Mudar para o tema claro',
  'theme.toDark': 'Mudar para o tema escuro',

  // The footer.
  'footer.product': 'Produto',
  'footer.engines': 'Engines',
  'footer.download': 'Download',
  'footer.changelog': 'Histórico de versões',
  'footer.releases': 'Todas as versões',
  'footer.documentation': 'Documentação',
  'footer.introduction': 'Introdução',
  'footer.installation': 'Instalação',
  'footer.hosts': 'Conectar hosts',
  'footer.troubleshooting': 'Solução de problemas',
  'footer.project': 'Projeto',
  'footer.source': 'Código-fonte',
  'footer.issue': 'Relatar um problema',
  'footer.contributing': 'Como contribuir',
  'footer.security': 'Segurança',
  'footer.legal': 'Jurídico',
  'footer.legalNotice': 'Aviso legal',
  'footer.privacy': 'Privacidade',
  'footer.terms': 'Termos',
  'footer.cookies': 'Cookies',
  'footer.licence': '© {year} Damien Reichhart. O Homeport é de código aberto sob a {licence}.',
  'footer.licenceName': 'GNU AGPL, versão 3',
  'footer.trademark':
    'Docker é uma marca comercial da Docker, Inc. O Homeport é um projeto independente, sem afiliação com a Docker, Inc. e sem endosso dela.',
} satisfies Translation<typeof en>;
