/** Japanese text of the `quadlets` namespace. */
import type en from '../en/quadlets';
import type { Translation } from '../types';

export default {
  'quadlets.nav': 'Quadlet',
  'quadlets.title': 'Quadlet ユニット',
  'quadlets.summary': '{count, plural, other {# 個のユニット}}',
  'quadlets.search': 'ユニットを検索',
  'quadlets.empty': 'Quadlet ユニットはありません',
  'quadlets.emptyDescription':
    'Quadlet ユニットは Podman が systemd サービスに変換するファイルです。これにより、コンテナはホストと一緒に起動し、systemd が実行し続けます。インストールすると、ここに表示されます。',
  'quadlets.new': '新しいユニット',
  'quadlets.edit': '編集',
  'quadlets.notGenerated': '未生成',
  'quadlets.gone': '{name} はもうインストールされていません',
  'quadlets.column.name': 'ユニット',
  'quadlets.column.kind': '種類',
  'quadlets.column.service': 'サービス',
  'quadlets.column.status': 'ステータス',
  'quadlets.column.containers': 'コンテナ',
  'quadlets.status.running': '実行中',
  'quadlets.status.starting': '起動中',
  'quadlets.status.stopping': '停止中',
  'quadlets.status.stopped': '停止',
  'quadlets.status.failed': '失敗',
  'quadlets.status.notLoaded': 'systemd に読み込まれていません',
  'quadlets.status.template': 'テンプレート',
  'quadlets.action.start': '起動',
  'quadlets.action.stop': '停止',
  'quadlets.action.restart': '再起動',
  'quadlets.actionOne.start': '{name} を起動',
  'quadlets.actionOne.stop': '{name} を停止',
  'quadlets.actionOne.restart': '{name} を再起動',
  'quadlets.done.start': '{name} を起動しました',
  'quadlets.done.stop': '{name} を停止しました',
  'quadlets.done.restart': '{name} を再起動しました',
  'quadlets.installed': '{name} をインストールしました',
  'quadlets.saved': '{name} を保存しました',
  'quadlets.removed': '{name} を削除しました',
  'quadlets.blocked.noCommands':
    'Homeport はユニットの起動と停止に systemctl を使いますが、実行できるのは SSH 経由か、このコンピューター上の Podman の場合だけです。',
  'quadlets.blocked.notLoaded':
    'systemd がこのユニットのサービスを生成していないため、起動するものがありません。',
  'quadlets.blocked.podmanTooOld':
    'ユニットの読み取り、インストール、編集、削除には Podman 5.8 以降が必要です。このホストで動作しているのは Podman {version} です。',
  'quadlets.blocked.noFile':
    'このユニットはコンテナからしか認識されていないため、ファイルを変更できません。',
  'quadlets.note.labels':
    'Podman {version} は Quadlet ユニットを一覧表示できないため、これらはユニットが作成したコンテナから取得しています。ファイルや、コンテナを作成していないユニットは、Podman 5.7 以降で表示されます。',
  'quadlets.note.readOnly':
    'Podman {version} はユニットを一覧表示できますが、読み取り、インストール、編集、削除はできません。それには Podman 5.8 以降が必要です。',
  'quadlets.note.noCommands':
    'ユニットの起動と停止には、このホストに SSH 経由で接続しているか、このコンピューター上の Podman である必要があります。',
  'quadlets.detail.service': 'サービス',
  'quadlets.detail.unit': 'systemd ユニット',
  'quadlets.detail.scope': '実行ユーザー',
  'quadlets.detail.path': 'ファイル',
  'quadlets.detail.app': 'アプリケーション',
  'quadlets.detail.pod': 'Pod',
  'quadlets.detail.containers': '{count, plural, =0 {コンテナ} other {# 個のコンテナ}}',
  'quadlets.detail.noContainers': 'コンテナを作成していないか、サービスが停止しています。',
  'quadlets.scope.user': 'ユーザーユニット（systemctl --user）',
  'quadlets.scope.system': 'システムユニット',
  'quadlets.file.title': 'ユニットファイル',
  'quadlets.file.unknown': 'このユニットは、コンテナのラベルからしか認識されていません。',
  'quadlets.file.tooOld':
    'Podman {version} はユニットファイルを読み取れません。表示するには Podman 5.8 以降が必要です。',
  'quadlets.form.newTitle': '新しい Quadlet ユニット',
  'quadlets.form.newDescription':
    'Podman はそのユーザーのユニットが置かれる場所にファイルを書き込み、systemd を再読み込みします。インストールが済んだらユニットを起動してください。',
  'quadlets.form.editTitle': '{name} を編集',
  'quadlets.form.editDescription':
    '保存するとファイルが置き換えられ、systemd が再読み込みされます。実行中のサービスに反映するには、ユニットを再起動してください。一緒にインストールされたファイルは残ります。',
  'quadlets.form.name': 'ファイル名',
  'quadlets.form.kind': '種類',
  'quadlets.form.openFile': 'ファイルを開く…',
  'quadlets.form.openTitle': 'Quadlet ユニットファイルを開く',
  'quadlets.form.unitFiles': 'Quadlet ユニットファイル',
  'quadlets.form.content': 'ユニットファイル',
  'quadlets.form.extra': '一緒に置くファイル',
  'quadlets.form.extraHint':
    'ユニットが使う Containerfile、kube YAML、env ファイルなどで、アプリケーションフォルダーにインストールされます。',
  'quadlets.form.extraTitle': 'ユニットと一緒にインストールするファイルを選択',
  'quadlets.form.addExtra': 'ファイルを追加…',
  'quadlets.form.removeExtra': '{name} を削除',
  'quadlets.form.install': 'インストール',
  'quadlets.form.save': '保存',
  'quadlets.form.nameInvalid':
    'web.container のように、.container、.pod、.volume、.network、.kube、.image、.build、.artifact のいずれかで終わる名前にしてください。',
  'quadlets.form.sectionMissing':
    'Quadlet が読み取れるように、ファイルには {section} セクションが必要です。',
  'quadlets.form.extraInvalid':
    'ユニットと一緒に置くファイルには、それぞれフォルダーを含まない単純な名前が必要です。',
  'quadlets.remove.title': '{name} を削除しますか？',
  'quadlets.remove.description':
    'Podman はユニットファイルを削除し、systemd を再読み込みします。ユニットが作成したコンテナ、ボリューム、イメージは残ります。',
  'quadlets.remove.stop': '実行中ならサービスを停止',
  'quadlets.remove.stopRunning': '先に実行中のサービスを停止',
} satisfies Translation<typeof en>;
