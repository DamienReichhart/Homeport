import type { Translation } from '../../types';
import type en from '../en/site';

/** The layout, the header and the footer, in Japanese. */
export default {
  // The layout and the facts every page repeats.
  'site.tagline': 'あらゆるコンテナエンジンを、1 つのアプリで。',
  'site.description':
    'Homeport は、Docker、Podman、containerd、Incus、Kubernetes、Nomad のための無料でオープンソースのデスクトップアプリです。あなたのマシンで動作し、SSH、TLS、または kubeconfig でサーバーに接続します。サーバーには何もインストールしません。Windows、macOS、Linux に対応。',
  'site.ogImageAlt': 'Homeport — あらゆるコンテナエンジンを、1 つのアプリで。',
  'site.skipToContent': '本文へスキップ',

  // The header, the language menu and the theme switch.
  'header.home': 'Homeport ホーム',
  'header.logo': 'Homeport',
  'header.nav': 'メイン',
  'header.engines': 'エンジン',
  'header.docs': 'ドキュメント',
  'header.changelog': '変更履歴',
  'header.download': 'ダウンロード',
  'header.github': 'GitHub の Homeport',
  'header.githubShort': 'GitHub',
  'header.menu': 'メニュー',
  'language.label': '言語',
  'theme.toLight': 'ライトテーマに切り替え',
  'theme.toDark': 'ダークテーマに切り替え',

  // The footer.
  'footer.product': '製品',
  'footer.engines': 'エンジン',
  'footer.download': 'ダウンロード',
  'footer.changelog': '変更履歴',
  'footer.releases': 'すべてのリリース',
  'footer.documentation': 'ドキュメント',
  'footer.introduction': 'はじめに',
  'footer.installation': 'インストール',
  'footer.hosts': 'ホストの接続',
  'footer.troubleshooting': 'トラブルシューティング',
  'footer.project': 'プロジェクト',
  'footer.source': 'ソースコード',
  'footer.issue': '問題を報告',
  'footer.contributing': 'コントリビューション',
  'footer.security': 'セキュリティ',
  'footer.legal': '法的情報',
  'footer.legalNotice': '法的通知',
  'footer.privacy': 'プライバシー',
  'footer.terms': '利用規約',
  'footer.cookies': 'Cookie',
  'footer.licence':
    '© {year} Damien Reichhart. Homeport は {licence} のもとで公開されているオープンソースです。',
  'footer.licenceName': 'GNU AGPL バージョン 3',
  'footer.trademark':
    'Docker は Docker, Inc. の商標です。Homeport は独立したプロジェクトであり、Docker, Inc. とは提携しておらず、その承認も受けていません。',
} satisfies Translation<typeof en>;
