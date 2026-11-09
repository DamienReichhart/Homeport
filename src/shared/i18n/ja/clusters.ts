/** Japanese text of the `clusters` namespace. */
import type en from '../en/clusters';
import type { Translation } from '../types';

export default {
  'clusters.title': 'クラスター',
  'clusters.summary': '{count, plural, other {# 個のクラスター}}',
  'clusters.add': 'クラスターを追加',
  'clusters.notFound': 'このクラスターはもう存在しません。',
  'clusters.empty.title': 'まだクラスターはありません',
  'clusters.empty.description':
    'kubeconfig から到達できる Kubernetes クラスターを追加してください。Homeport はクラスターに何もインストールしません。',
  'clusters.column.name': '名前',
  'clusters.column.status': 'ステータス',
  'clusters.column.kind': '種類',
  'clusters.column.server': 'API',
  'clusters.column.auth': 'サインイン方法',
  'clusters.palette.add': 'クラスターを追加',
  'clusters.palette.manage': 'クラスターを管理',

  'clusters.auth.none': '認証情報なし',
  'clusters.auth.clientCertificate': 'クライアント証明書',
  'clusters.auth.token': 'トークン',
  'clusters.auth.basic': 'ユーザー名とパスワード',
  'clusters.auth.exec': 'Exec プラグイン（{command}）',
  'clusters.auth.oidc': 'OpenID Connect',
  'clusters.auth.aclToken': 'ACL トークン',

  'clusters.addPage.description':
    '選んだ Kubernetes コンテキスト、または指定した Nomad エージェントが、それぞれ 1 つのクラスターになります。認証情報は暗号化して保存され、ディスクに平文で保存されることはありません。',
  'clusters.addPage.fromMachine': 'このマシンの kubeconfig から',
  'clusters.addPage.paste': 'kubeconfig を貼り付け',
  'clusters.addPage.nomad': 'Nomad',
  'clusters.paste.description':
    'kubeconfig はメモリ上で読み込まれ、ファイルとして保存されることはありません。参照するファイルは完全なパスで指定する必要があります。',
  'clusters.paste.label': 'kubeconfig',
  'clusters.paste.read': 'コンテキストを読み込む',

  'clusters.import.readFrom': '{paths} から読み込みました',
  'clusters.import.noContexts': 'この kubeconfig にはコンテキストがありません',
  'clusters.import.pick': '{context} をインポート',
  'clusters.import.name': '{context} から作成するクラスターの名前',
  'clusters.import.current': '現在',
  'clusters.import.added': '{name} としてすでに追加されています。',
  'clusters.import.refresh': '更新する',
  'clusters.import.submit': '{count, plural, =0 {インポート} other {# 個のクラスターをインポート}}',
  'clusters.import.done': '{count, plural, other {# 個のクラスターを追加しました}}',
  'clusters.import.problem.empty': 'すべてのクラスターに名前を付けてください。',
  'clusters.import.problem.duplicate': '2 つのクラスターが「{name}」という名前になります。',
  'clusters.import.problem.taken': '「{name}」という名前のクラスターはすでに存在します。',

  'clusters.detail.enabled': '有効',
  'clusters.detail.reconnect': '再接続',
  'clusters.detail.name': '名前',
  'clusters.detail.rename': '名前を変更',
  'clusters.detail.renamed': 'クラスターの名前を変更しました',
  'clusters.connection.title': '接続',
  'clusters.connection.server': 'API サーバー',
  'clusters.connection.address': 'アドレス',
  'clusters.connection.region': 'リージョン',
  'clusters.connection.customCa': '独自の CA 証明書',
  'clusters.connection.clientCertificate': 'クライアント証明書',
  'clusters.connection.checkedAs': '証明書を {name} として検証',
  'clusters.connection.version': 'バージョン',
  'clusters.connection.platform': 'プラットフォーム',
  'clusters.connection.namespace': 'デフォルトの Namespace',
  'clusters.connection.auth': 'サインイン方法',
  'clusters.connection.tlsServerName': '証明書の名前',
  'clusters.connection.tls': 'TLS',
  'clusters.connection.insecure': 'サーバーの証明書は検証されません（insecure-skip-tls-verify）',
  'clusters.connection.proxy': 'プロキシ',
  'clusters.connection.source': 'インポート元',
  'clusters.source.machine': 'このマシンの kubeconfig のコンテキスト {context}',
  'clusters.source.pasted': '貼り付けた kubeconfig のコンテキスト {context}',

  'clusters.nomad.description':
    'Homeport は、このコンピューターから Nomad エージェント（サーバーまたはクライアント）の HTTP API と通信します。クラスターには何もインストールされず、ここに nomad CLI も必要ありません。',
  'clusters.nomad.name': '名前',
  'clusters.nomad.address': 'アドレス',
  'clusters.nomad.addressHelp':
    'エージェントの HTTP API で、nomad CLI に NOMAD_ADDR で渡すものと同じです。指定がなければポートは 4646 です。',
  'clusters.nomad.region': 'リージョン',
  'clusters.nomad.regionHelp': 'エージェント自身のリージョンを使う場合は空欄のままにします。',
  'clusters.nomad.token': 'ACL トークン',
  'clusters.nomad.tokenHelp':
    'クラスターで ACL が有効な場合の、トークンのシークレット ID です。Homeport のドキュメントに、そのためのポリシーがあります。',
  'clusters.nomad.tls': 'TLS 設定',
  'clusters.nomad.tlsHelp':
    '独自の CA で HTTPS を提供するエージェントや、クライアント証明書を求めるエージェントの場合のみ必要です。PEM ファイルの内容を貼り付けてください。',
  'clusters.nomad.ca': 'CA 証明書',
  'clusters.nomad.caHelp':
    'エージェントの証明書に署名した CA で、NOMAD_CACERT で指定するものです。このコンピューターの CA は、指定しなくても信頼されます。',
  'clusters.nomad.clientCertificate': 'クライアント証明書',
  'clusters.nomad.clientKey': 'クライアントキー',
  'clusters.nomad.tlsServerName': 'TLS サーバー名',
  'clusters.nomad.tlsServerNameHelp':
    'アドレスの名前と異なる場合に、エージェントの証明書の検証に使う名前で、NOMAD_TLS_SERVER_NAME で設定するものです。',
  'clusters.nomad.keep': '保存済み。使い続ける場合は空欄のままにします',
  'clusters.nomad.willBeRemoved': '保存すると削除されます',
  'clusters.nomad.remove': '削除',
  'clusters.nomad.keepIt': '残す',
  'clusters.nomad.test': '接続をテスト',
  'clusters.nomad.testing': 'テスト中…',
  'clusters.nomad.testedVersion': 'Nomad {version}（リージョン {region}）に接続しました。',
  'clusters.nomad.testedNoVersion':
    'リージョン {region} に接続しました。トークンでエージェントを読み取れないため、バージョンはわかりません。',
  'clusters.nomad.add': 'クラスターを追加',
  'clusters.nomad.added': '{name} を追加しました',
  'clusters.nomad.settings': '設定',
  'clusters.nomad.settingsDescription':
    '保存すると再接続します。トークンと TLS キーは二度と表示されません。使い続ける場合は空欄のままにしてください。',
  'clusters.nomad.save': '変更を保存',
  'clusters.nomad.saved': 'クラスターを保存しました',

  'clusters.refresh.title': 'kubeconfig から更新',
  'clusters.refresh.machine':
    'このマシンの kubeconfig からコンテキスト {context} を再び読み込みます。クラスターが再作成されたときや、認証情報が更新されたときに使います。',
  'clusters.refresh.pasted':
    'kubeconfig を再び貼り付けて、コンテキスト {context} を読み込み直します。クラスターが再作成されたときや、認証情報が更新されたときに使います。',
  'clusters.refresh.submit': '更新',
  'clusters.refresh.done': 'kubeconfig からクラスターを更新しました',

  'clusters.remove.title': 'クラスターを削除',
  'clusters.remove.description':
    'Homeport はクラスターと、保存されている認証情報を破棄します。クラスターや kubeconfig には何の変更も加えません。',
  'clusters.remove.confirmTitle': '{name} を削除しますか？',
  'clusters.remove.confirmDescription': 'kubeconfig から再び追加できます。',
} satisfies Translation<typeof en>;
