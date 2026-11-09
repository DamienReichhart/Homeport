import type { Translation } from '../../types';
import type en from '../en/changelog';

/** The changelog page, in Japanese. */
export default {
  'changelog.title': '変更履歴 — Homeport',
  'changelog.description': 'Homeport の各リリースの新機能、修正、その他の変更点です。',
  'changelog.eyebrow': '変更履歴',
  'changelog.heading': 'Homeport の新機能',
  'changelog.lead':
    'すべてのリリースを新しい順に掲載しています。リリースノートはプロジェクトのコミットから生成されています。ファイルは{link}にあります。',
  'changelog.downloadPage': 'ダウンロードページ',
  'changelog.notesInEnglish': '以下のリリースノートは、GitHub で公開されているとおり英語です。',
  'changelog.empty': 'リリース履歴は {link} にあります。',
  'changelog.github': 'GitHub',
  'changelog.latest': '最新',
  'changelog.prerelease': 'プレリリース',
  'changelog.onGitHub': 'GitHub のリリース',
  'changelog.download': 'ダウンロード',
} satisfies Translation<typeof en>;
