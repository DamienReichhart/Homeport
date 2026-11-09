/** Japanese text of the `app` namespace. */
import type en from '../en/app';
import type { Translation } from '../types';

export default {
  'app.nav.main': 'メイン',
  'app.nav.docs': 'ドキュメント',
  'app.nav.github': 'GitHub',
  'app.nav.opensInBrowser': '{label}（ブラウザーで開きます）',
  'app.keychainMissing':
    'OS のキーチェーンを利用できないため、SSH キー、TLS 証明書、スタックの環境変数を保存できません。ローカル、TCP、ssh-agent のホストは引き続き使えます。',
  'app.crash.title': '問題が発生しました',
  'app.crash.description':
    'このページで Homeport に予期しない問題が発生しました。通常は再読み込みで解決します。繰り返し発生する場合は、「詳細」からレポートをコピーして issue に貼り付けてください。',
  'app.crash.reload': '再読み込み',
  'app.crash.backToOverview': '概要に戻る',
  'app.shortcuts.title': 'キーボードショートカット',
  'app.shortcuts.description':
    'ここで使えるショートカットです。N や / などのキーは、フィールドに入力している間は反応しません。',
  'app.shortcuts.general': '全般',
  'app.shortcuts.navigation': '移動',
  'app.shortcuts.hosts': 'ホスト',
  'app.shortcuts.help': 'キーボードショートカットを表示',
  'app.shortcuts.settings': '設定を開く',
  'app.shortcuts.goTo': '{page} に移動',
  'app.shortcuts.previousHost': '前のホスト',
  'app.shortcuts.nextHost': '次のホスト',
  'app.shortcuts.refresh': 'このホストの一覧を更新',
  'app.palette.title': 'コマンドパレット',
  'app.palette.description': 'ページへの移動、ホストの切り替え、コマンドの実行ができます。',
  'app.palette.open': 'コマンドパレットを開く',
  'app.palette.placeholder': 'ページ、ホスト、コマンドを入力…',
  'app.palette.empty': '一致するものはありません。',
  'app.palette.recent': '最近使用した項目',
  'app.palette.goTo': '移動',
  'app.palette.hosts': 'ホスト',
  'app.palette.actions': '操作',
  'app.palette.switchTo': '{name} に切り替え',
} satisfies Translation<typeof en>;
