/** Japanese text of the `images` namespace. */
import type en from '../en/images';
import type { Translation } from '../types';

export default {
  'images.nav': 'イメージ',
  'images.title': 'イメージ',
  'images.summary': '{count, plural, other {# 個のイメージ}} · {size}',
  'images.cleanUp': '未使用を削除',
  'images.cleanUp.dangling': 'dangling イメージ',
  'images.cleanUp.all': '未使用のすべてのイメージ',
  'images.saveSelected': '{count, plural, one {イメージを保存…} other {# 個のイメージを保存…}}',
  'images.load': '読み込み…',
  'images.importTar': 'tar からインポート…',
  'images.build': 'ビルド',
  'images.pull': 'イメージをプル',
  'images.search': 'イメージを検索',
  'images.empty.title': 'イメージはありません',
  'images.empty.description': 'まずはレジストリからイメージをプルしてください。',

  'images.column.name': '名前',
  'images.column.id': 'ID',
  'images.column.created': '作成日時',
  'images.column.size': 'サイズ',
  'images.column.status': 'ステータス',
  'images.moreTags': 'ほか {count} 個のタグ',
  'images.inUse': '使用中 · {count}',
  'images.unused': '未使用',
  'images.runOne': '{name} を実行',
  'images.action.inspect': '検査',
  'images.action.tag': 'タグ付け',
  'images.action.push': 'プッシュ',
  'images.action.save': '保存…',

  'images.remove.title': '{name} を削除しますか？',
  'images.remove.everyName':
    'イメージのすべての名前（{names}）と、ほかで使われていないレイヤーが削除されます。',
  'images.remove.description':
    'イメージと、ほかのイメージが使っていないレイヤーが削除されます。後で再びプルできます。',
  'images.prune.allTitle': '未使用のイメージをすべて削除しますか？',
  'images.prune.danglingTitle': 'dangling イメージを削除しますか？',
  'images.prune.allDescription':
    'どのコンテナも使っていないイメージは、タグ付きのものも含めてすべて削除されます。',
  'images.prune.danglingDescription':
    'ビルドやプルの後に残った、タグのないイメージが削除されます。',

  'images.removed': '{name} を削除しました',
  'images.pruned': '{count, plural, other {# 個のイメージ}}を削除し、{space} を解放しました',
  'images.pulled': '{reference} をプルしました',
  'images.pushed': '{reference} をプッシュしました',
  'images.built': '{name} をビルドしました',
  'images.builtUnnamed': 'イメージをビルドしました',
  'images.saved': '{path} を保存しました',
  'images.loadedOne': '{name} を読み込みました',
  'images.loaded': '{count, plural, other {# 個のイメージ}}を読み込みました',
  'images.imported': '{name} をインポートしました',

  'images.files.save': 'イメージの保存先',
  'images.files.load': 'イメージの読み込み元',
  'images.files.import': 'ファイルシステムのインポート元',
  'images.files.imageArchives': 'イメージアーカイブ',
  'images.files.imageArchive': 'イメージアーカイブ',
  'images.files.filesystemArchives': 'ファイルシステムアーカイブ',

  'images.buildDialog.title': 'イメージをビルド',
  'images.buildDialog.description':
    'このコンピューター上のフォルダーからビルドします。Homeport は .dockerignore で除外されたものを除いてフォルダーをホストに送り、ホストのエンジンがビルドします。',
  'images.buildDialog.built': '✓ ビルド完了',
  'images.buildDialog.folder': 'フォルダー',
  'images.buildDialog.chooseFolder': 'ビルドするフォルダーを選択',
  'images.buildDialog.choose': '選択…',
  'images.buildDialog.dockerfile': 'Dockerfile',
  'images.buildDialog.target': 'ターゲットステージ',
  'images.buildDialog.targetPlaceholder': '最後のステージ',
  'images.buildDialog.names': '名前',
  'images.buildDialog.buildArgs': 'ビルド引数',
  'images.buildDialog.buildArgsPlaceholder': 'VERSION=1.0、1 行に 1 つ',
  'images.buildDialog.noCache': 'ビルドキャッシュを使わない',
  'images.buildDialog.pull': '新しいベースイメージをプル',
  'images.buildForm.noFolder': 'ビルドするフォルダーを選んでください。',
  'images.buildForm.argNeedsName': '「{line}」には「=」の前に名前が必要です。',

  'images.hub.unavailable':
    '現在 Docker Hub を検索できません: {error} nginx:1.27 のような完全な名前であれば、引き続きプルできます。',
  'images.hub.noMatch': 'Docker Hub に「{term}」に一致するものはありません。',
  'images.hub.results': 'Docker Hub の検索結果',
  'images.hub.result': '{name}、スター {stars}',
  'images.hub.resultOfficial': '{name}、公式イメージ、スター {stars}',
  'images.hub.official': '公式',
  'images.hub.backToSearch': '検索',
  'images.hub.tagCount': '{count, plural, other {# 個のタグ}}',
  'images.hub.filterTags': 'タグを絞り込み',
  'images.hub.tags': 'タグ',
  'images.hub.noTagMatch': '「{filter}」に一致するタグはありません。',
  'images.hub.moreTags': 'その他のタグ',

  'images.importDialog.title': 'ファイルシステムをイメージとしてインポート',
  'images.importDialog.description':
    '{path} は、独自の履歴や設定を持たない 1 レイヤーのイメージになります。実行できるようにするには、下でコマンドを指定してください。',
  'images.importDialog.name': 'イメージ名',
  'images.importDialog.changes': '変更',
  'images.importDialog.changesHint':
    'Dockerfile の命令を 1 行に 1 つ: CMD、ENTRYPOINT、ENV、EXPOSE、LABEL、ONBUILD、USER、VOLUME、WORKDIR。',
  'images.importDialog.import': 'インポート',

  'images.inspect.layers': 'レイヤー',
  'images.inspect.raw': '生の JSON',
  'images.inspect.created': '作成日時',
  'images.inspect.instruction': '命令',
  'images.inspect.size': 'サイズ',

  'images.pullDialog.title': 'イメージをプル',
  'images.pullDialog.description':
    'Docker Hub を名前で検索するか、nginx:1.27 や ghcr.io/org/app:v2 など、ホストから到達できる任意の参照を入力してください。',
  'images.pullDialog.placeholder': 'postgres、または nginx:1.27',
  'images.pullDialog.reference': 'イメージの参照',
  'images.pullDialog.pull': 'プル',
  'images.done': '完了しました。',

  'images.pushDialog.title': '{name} をプッシュ',
  'images.pushDialog.description':
    'ghcr.io/org/app:v2 のように名前の先頭にあるレジストリへ、org/app:v2 のような名前なら Docker Hub へプッシュします。',
  'images.pushDialog.target': 'プッシュする名前',
  'images.pushDialog.push': 'プッシュ',
  'images.pushDialog.tagsFirst': 'タグ付けと同じように、まずイメージにこの名前が付けられます。',
  'images.pushDialog.uploading': 'アップロード中 {current} / {total}',
  'images.push.pushed': '{pushed, plural, other {# 個のレイヤー}}をプッシュしました',
  'images.push.pushedAndExisting':
    '{pushed, plural, other {# 個のレイヤー}}をプッシュしました。{existing} 個はすでにレジストリにあります',

  'images.tagDialog.title': '{name} にタグ付け',
  'images.tagDialog.newName': '新しい名前',
  'images.tagDialog.add': 'タグを追加',

  'images.transfer.savingOne': '{name} を保存中',
  'images.transfer.saving': '{count, plural, other {# 個のイメージ}}を保存中',
  'images.transfer.to': '保存先: {path}',
  'images.transfer.from': '読み込み元: {path}',
  'images.transfer.written': '書き込み済み',
  'images.transfer.sent': '送信済み',
  'images.transfer.loading': 'イメージを読み込み中',
  'images.transfer.importing': '{reference} をインポート中',
  'images.transfer.doneWith': '完了: {images}',
  'images.palette.pull': 'イメージをプル',
} satisfies Translation<typeof en>;
