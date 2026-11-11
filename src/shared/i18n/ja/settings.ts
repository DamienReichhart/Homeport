/** Japanese text of the `settings` namespace. */
import type en from '../en/settings';
import type { Translation } from '../types';

export default {
  'settings.nav': '設定',
  'settings.title': '設定',
  'settings.appearance.title': '外観',
  'settings.appearance.theme': 'テーマ',
  'settings.theme.system': 'システムに合わせる',
  'settings.theme.light': 'ライト',
  'settings.theme.dark': 'ダーク',
  'settings.appearance.language': '言語',
  'settings.language.system': 'システムに合わせる（{language}）',
  'settings.desktop.title': 'デスクトップ',
  'settings.desktop.description': 'このコンピューターでの Homeport の動作です。',
  'settings.desktop.closeToTray': '閉じるとシステムトレイに常駐',
  'settings.desktop.closeToTrayHint':
    'ウィンドウを閉じても、Homeport とホストへの接続はトレイで動作し続けます。終了はトレイアイコンから行います。拡張機能のない GNOME など、トレイアイコンを隠すデスクトップでは、アイコンが表示されないことがあります。',
  'settings.desktop.noTray':
    'このデスクトップには Homeport が使えるシステムトレイがないため、ウィンドウを閉じると終了します。',
  'settings.desktop.startAtLogin': 'ログイン時に起動',
  'settings.desktop.startAtLoginHint':
    'このコンピューターにログインすると Homeport が起動し、最初からホストを監視します。',
  'settings.desktop.startHidden': 'トレイに隠した状態で起動',
  'settings.desktop.startHiddenHint':
    'ログイン時に起動したとき、Homeport はウィンドウを開かずにシステムトレイに常駐します。',
  'settings.desktop.startHiddenNoTray':
    'ウィンドウを開くためのシステムトレイが必要ですが、このデスクトップにはありません。',
  'settings.notifications.title': '通知',
  'settings.notifications.description':
    'Homeport は、ウィンドウを閉じてトレイに常駐している間も、次の場合に通知します:',
  'settings.notifications.containerDied':
    'コンテナがエラーで停止したとき、またはメモリ不足になったとき',
  'settings.notifications.containerUnhealthy': 'コンテナが異常な状態になったとき',
  'settings.notifications.hostUnreachable': '接続中のホストに到達できなくなったとき',
  'settings.notifications.hostBack': '到達できなかったホストが復帰したとき',
  'settings.machine.title': 'このマシン',
  'settings.machine.description': 'ここで Homeport が使えるものです。',
  'settings.machine.keychain': 'OS のキーチェーン',
  'settings.machine.keychainAvailable':
    '利用可能 — SSH キー、証明書、スタックの環境変数、レジストリのログイン情報は暗号化して保存されます。',
  'settings.machine.keychainUnavailable':
    '利用不可 — SSH キー、TLS 証明書、スタックの環境変数、レジストリのログイン情報を保存できません。',
  'settings.machine.keychainEntryMissing':
    'エントリーがありません — Homeport のシークレットを封印したキーのエントリーがキーチェーンにないため、シークレットを開くことも保存することもできません。',
  'settings.machine.startOverDescription':
    'Homeport が以前とは別のキーチェーンを使っている場合は、元のキーチェーンに戻して Homeport を再起動してください。エントリーが完全に失われた場合は、新しいキーでやり直してください。',
  'settings.machine.startOver': '新しいキーでやり直す',
  'settings.machine.startOverTitle': '新しいキーでやり直しますか？',
  'settings.machine.startOverConfirmDescription':
    '失われたキーで封印されたものは、新しいキーでは開けません。各ホスト、クラスター、レジストリー、リポジトリーの認証情報と各スタックの環境を入力し直すか、バックアップを復元してください。',
  'settings.machine.compose': 'Compose CLI',
  'settings.machine.composeMissing': 'スタックの起動と停止はできますが、デプロイはできません。',
  'settings.machine.stackDeploy': 'Docker CLI（Swarm 用）',
  'settings.machine.stackDeployMissing':
    'Swarm にスタックをデプロイできません。それ以外の Swarm の機能はすべて使えます。',
  'settings.machine.git': 'git',
  'settings.machine.helm': 'Helm CLI',
  'settings.sync.title': '同期スタック',
  'settings.sync.interval': 'リポジトリの確認間隔',
  'settings.sync.minutes': '分',
  'settings.data.title': 'データ',
  'settings.data.description': 'Homeport がこのコンピューターに保持しているものです。',
  'settings.data.backup': 'バックアップ…',
  'settings.data.restore': '復元…',
  'settings.data.backupHint':
    'すべてのホスト、スタック、レジストリのログイン情報、リポジトリ、設定を、シークレットも含めてパスフレーズで保護された 1 つのファイルに保存し、このコンピューターや別のコンピューターで復元できます。',
  'settings.backup.title': 'Homeport のデータをバックアップ',
  'settings.backup.description':
    'このファイルには、すべてのホスト、クラスター、スタック、レジストリのログイン情報、リポジトリ、設定と、それらのキー、証明書、トークン、パスワード、環境変数が含まれます。ファイルとパスフレーズを持っている人は誰でもそれらを使えるため、両方を安全に保管してください。エンジンのイベントとデプロイ履歴は含まれません。',
  'settings.backup.passphrase': 'パスフレーズ',
  'settings.backup.passphraseHint': '{min} 文字以上。復元することはできません。',
  'settings.backup.confirmation': 'パスフレーズ（確認）',
  'settings.backup.problem.short': '{min} 文字以上にしてください。',
  'settings.backup.problem.mismatch': '2 つのパスフレーズが一致しません。',
  'settings.backup.start': 'バックアップ…',
  'settings.backup.saveTitle': 'バックアップの保存先',
  'settings.backup.fileType': 'Homeport バックアップ',
  'settings.backup.done':
    '{hosts, plural, other {# 台のホスト}}と {stacks, plural, other {# 個のスタック}}をバックアップしました',
  'settings.backup.withoutSecrets':
    '次のものは、シークレットを開けなかったため、シークレットなしでバックアップされました:',
  'settings.restore.pick': '復元元',
  'settings.restore.title': 'Homeport のデータを復元',
  'settings.restore.from': '復元元: {path}',
  'settings.restore.open': '開く',
  'settings.restore.madeBy':
    '{date} に Homeport {version} でバックアップされました。含まれているもの:',
  'settings.restore.holds': 'バックアップの内容',
  'settings.restore.hosts': '{count, plural, other {# 台のホスト}}',
  'settings.restore.clusters': '{count, plural, other {# 個のクラスター}}',
  'settings.restore.stacks': '{count, plural, other {# 個のスタック}}',
  'settings.restore.registries': '{count, plural, other {# 件のレジストリのログイン情報}}',
  'settings.restore.repositories': '{count, plural, other {# 個のリポジトリ}}',
  'settings.restore.settings': '設定',
  'settings.restore.withoutSecrets':
    '次のものは、その時点でシークレットを開けなかったため、シークレットなしでバックアップされています。復元するとシークレットなしで戻り、ここにあるシークレットは置き換えられます。このうちレジストリーのログインはまったく戻りません:',
  'settings.restore.withoutSecretsList': 'シークレットなしでバックアップされたもの',
  'settings.restore.replaces':
    '復元すると、現在のすべてのホスト、スタック、レジストリのログイン情報、リポジトリ、設定が置き換えられ、エンジンのイベントとデプロイ履歴は破棄されます。その前に、現在のデータのコピーが Homeport のデータフォルダーに保存されます。',
  'settings.restore.confirm': 'すべて置き換え',
  'settings.restore.done':
    '{hosts, plural, other {# 台のホスト}}と {stacks, plural, other {# 個のスタック}}を復元しました',
  'settings.data.keepEvents': 'エンジンのイベントの保持期間',
  'settings.data.days': '日',
  'settings.data.keepEventsHint':
    '各ホストは「イベント」ページ用に最大 10,000 件のイベントを保持します。古いものは 1 時間ごとに削除されます。',
  'settings.machine.gitMissing':
    '同期スタックに必要です。それ以外の種類のスタックは、なくても動作します。',
  'settings.machine.helmMissing':
    'Helm リリースの一覧表示とインストールができません。それ以外のクラスターの機能はすべて使えます。',
  'settings.about.tagline': 'すべての Docker Engine を、1 つのボードで。',
  'settings.about.openLogFolder': 'ログフォルダーを開く',
  'settings.about.licenses': 'ライセンス',
  'settings.about.exportDiagnostics': '診断情報をエクスポート…',
  'settings.diagnostics.title': '診断情報をエクスポート',
  'settings.diagnostics.description':
    'バグ報告に添付する zip ファイルです。バージョンと、使用しているホストやクラスターの種類の概要、およびシークレットを取り除いた Homeport のログが含まれます。どこにも送信されません。ファイルを共有するかどうかはあなたが決めます。',
  'settings.diagnostics.includeNames': '名前とアドレスを含める',
  'settings.diagnostics.includeNamesHelp':
    'オフの場合、ホストとクラスターは概要とログの中で host-1、cluster-1… と表示されます。',
  'settings.diagnostics.files': 'zip 内のファイル',
  'settings.diagnostics.save': '保存…',
  'settings.diagnostics.saveTitle': '診断情報を保存',
  'settings.diagnostics.fileType': 'Zip アーカイブ',
  'settings.diagnostics.done': '診断情報を {path} に保存しました。',
  'settings.licenses.title': 'ライセンス',
  'settings.licenses.description':
    'Homeport は GNU Affero General Public License バージョン 3 に基づくフリーソフトウェアで、以下のオープンソースパッケージを使って作られています。',
  'settings.licenses.source': 'このバージョンのソースコードは、同じライセンスで公開されています。',
  'settings.licenses.sourceLink': 'v{version} のソース',
  'settings.licenses.thirdParty': 'サードパーティソフトウェア',
  'settings.licenses.missing.title': 'このビルドには含まれていません',
  'settings.licenses.missing.description':
    'リリースビルドには同梱されています。開発ビルドで生成するには <code>{command}</code> を実行してください。',
  'settings.palette.group': '設定',
  'settings.palette.themeLight': 'テーマ: ライト',
  'settings.palette.themeDark': 'テーマ: ダーク',
  'settings.palette.themeSystem': 'テーマ: システムに合わせる',
} satisfies Translation<typeof en>;
