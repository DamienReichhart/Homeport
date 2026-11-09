import type { Translation } from '../../types';
import type en from '../en/changelog';

/** The changelog page, in Brazilian Portuguese. */
export default {
  'changelog.title': 'Histórico de versões — Homeport',
  'changelog.description':
    'O que há de novo em cada versão do Homeport: recursos, correções e outras mudanças.',
  'changelog.eyebrow': 'Histórico de versões',
  'changelog.heading': 'Novidades do Homeport',
  'changelog.lead':
    'Todas as versões, da mais nova para a mais antiga. As notas são geradas a partir dos commits do projeto; os arquivos estão na {link}.',
  'changelog.downloadPage': 'página de download',
  'changelog.notesInEnglish':
    'As notas de versão abaixo estão em inglês, como foram publicadas no GitHub.',
  'changelog.empty': 'O histórico de versões está no {link}.',
  'changelog.github': 'GitHub',
  'changelog.latest': 'Mais recente',
  'changelog.prerelease': 'Pré-lançamento',
  'changelog.onGitHub': 'Versão no GitHub',
  'changelog.download': 'Download',
} satisfies Translation<typeof en>;
