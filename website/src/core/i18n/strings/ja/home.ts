import type { Translation } from '../../types';
import type en from '../en/home';

/** The home page, in Japanese. */
export default {
  // Home: the hero and its board.
  'home.title': 'Homeport — すべての Docker Engine を、1 つのボードで。',
  'home.hero.new': '新着',
  'home.hero.release': 'Homeport {version} を公開しました',
  'home.hero.title': 'すべての Docker Engine を、1 つのボードで。',
  'home.hero.lead':
    'Homeport はあなたのマシンで動作し、SSH、TLS、TCP でホストに接続します。サーバーにエージェントは不要です。無料、オープンソース、Windows、macOS、Linux に対応。',
  'home.hero.source': 'ソースコードを読む',
  'home.board.caption':
    'ボードの例：5 台のホスト、それぞれのエンジン、Homeport の接続方法。1 台に対応が必要です。',
  'home.board.host': 'ホスト',
  'home.board.engine': 'エンジン',
  'home.board.via': '接続方法',
  'home.board.stacks': 'スタック',
  'home.board.status': '状態',
  'home.board.allRunning': 'すべて実行中',
  'home.board.needsYou': '1 個のスタックに対応が必要',
  'home.board.note': '正常なものは色が付きません。対応が必要なものだけが赤くなります。',
  'home.runtimes.label': '対応',
  'home.runtimes.title': 'いま使っているものと一緒に使えます',

  // Home: where it lives.
  'home.visits.eyebrow': '動作する場所',
  'home.visits.title':
    'Homeport はあなたのマシンにとどまり、サーバーを訪れるだけです。サーバーに住み着くことはありません。',
  'home.visits.agent.title': 'エージェントなし',
  'home.visits.agent.body':
    'サーバーにインストール、アップデート、保護するものはありません。Homeport は、すでにそこにあるエンジンと通信します。',
  'home.visits.panel.title': 'Web パネルなし',
  'home.visits.panel.body':
    'サーバーにダッシュボードをデプロイすることはなく、誰かに見つけられるのを待つログインページもありません。',
  'home.visits.account.title': 'アカウントなし',
  'home.visits.account.body':
    'サインインも、ライセンス席も、サブスクリプションも、テレメトリもありません。表示するのはあなたのコンテナだけです。',
  'home.visits.ports.title': 'サーバーで必要なもの（正直に）',
  'home.visits.ports.ssh':
    'すでに開いているポート。Homeport が作成するキーを使う場合は、{file} に 1 行が追加されます。',
  'home.visits.ports.tls':
    'エンジン自身の API ポート（通常は {port}）。公開するのはあなた自身です。',
  'home.visits.ports.tcp':
    'エンジンの API ポート（通常は {port}）で、暗号化されません。信頼できるネットワークでのみ使ってください。',
  'home.visits.ports.advice':
    'できれば SSH を使ってください。サーバーで、すでに動いているもの以外に何も追加しません。',
  'home.visits.link': '接続のしくみ',

  // Home: the features.
  'home.features.eyebrow': '機能',
  'home.features.title': 'エンジンで動いているすべてを、1 つのウィンドウで',
  'home.features.lead':
    '何が動いているか、何を出力しているか、どれだけリソースを使っているか。Docker の日々の作業を、どのサーバーにあるかを覚えておかずに行えます。',
  'home.features.hosts.title': '近くのホストも、遠くのホストも',
  'home.features.hosts.body':
    'ローカルのエンジンと Podman マシンは自動で見つかります。リモートのエンジンは、SSH（必要なら踏み台経由）、相互 TLS、TCP で追加できます。~/.ssh/config や Docker コンテキストからそのまま追加することもできます。ネットワークが一瞬途切れても、各ホストは自動で再接続します。',
  'home.features.containers.title': 'コンテナをひと目で',
  'home.features.containers.body':
    'スタックごとにまとめて表示し、CPU とメモリの使用量をリアルタイムで、異常なコンテナにはその理由を示します。制限、デバイス、ヘルスチェックを指定して実行し、コンテナを編集、複製、コミット、エクスポートできます。',
  'home.features.stacks.title': 'Compose スタック',
  'home.features.stacks.body':
    'テンプレートから始める、このコンピューターのプロジェクトフォルダーをリンクする、git リポジトリを追跡する。そのうえで、どのホストにもデプロイできます（ビルドも含む）。再デプロイや同期のたびに何が変わるかを事前に確認でき、ロールバックも、新しいイメージへのアップデートも 1 ステップで行えます。',
  'home.features.swarm.title': 'Docker Swarm',
  'home.features.swarm.body':
    'Swarm はマネージャーのホストを通じて表示されます。ノード、サービス、タスク、シークレット、コンフィグを管理し、Swarm にスタックをデプロイし、ホストから Swarm を開始、参加、離脱できます。どのノードにも何もインストールしません。',
  'home.features.podman.title': 'Podman を Podman として',
  'home.features.podman.body':
    'Homeport は接続時に Podman を認識し、Podman にしかない機能を追加します。ポートを指定して作成し、コンテナを中で実行する Podman Pod と、systemd のもとでインストール、編集、起動する Quadlet ユニットです。どちらも Podman 自身の API を使います。',
  'home.features.containerd.title': 'containerd に直接',
  'home.features.containerd.body':
    'containerd のソケットに、ローカルまたは SSH で、Docker なしで接続します。名前空間を選び、コンテナの containerd タスクを起動、停止し、シグナルを送り、ログを読み、ターミナルを開き、レジストリのログイン情報でイメージをプルし、ディスクを圧迫しているスナップショットを見つけられます。',
  'home.features.incus.title': 'Incus のインスタンス',
  'home.features.incus.body':
    'Incus をソケット、SSH、または信頼トークンを使った HTTPS で追加し、システムコンテナと仮想マシンを実行できます。リモートのイメージから作成し、起動、フリーズ、停止し、コンソールやシェルを開き、スナップショットを作成して復元し、イメージ、プロファイル、ストレージ、ネットワークを管理できます。',
  'home.features.clusters.title': 'Kubernetes と Nomad のクラスター',
  'home.features.clusters.body':
    'Kubernetes クラスターは kubeconfig から追加でき（EKS、GKE、AKS も含む）、ローカルのクラスターは 1 クリックで見つかります。ワークロード、Pod、Service、構成、あらゆる API リソースをリアルタイムで閲覧し、ログを追い、ターミナルを開き、ポートフォワードし、スケールし、プレビュー付きで YAML を適用し、Helm チャートをインストール、アップグレード、ロールバックできます。Nomad クラスターはアドレスで追加し、HCL からジョブをプラン付きで実行し、スケールや停止を行い、ログを追い、シェルを開き、ノードをドレインできます。クラスターには何もインストールしません。',
  'home.features.images.title': 'イメージ',
  'home.features.images.body':
    'Docker Hub を検索し、レジストリのログイン情報でプルとプッシュを行い、このコンピューターのフォルダーからビルドし、アーカイブを保存して読み込み、新しいイメージが出たことを確認できます。',
  'home.features.volumes.title': 'ボリュームとネットワーク',
  'home.features.volumes.body':
    'どのコンテナが何を使っているかがわかります。作成、接続、切断ができ、削除の際には何が失われるかを正確に示します。',
  'home.features.board.title': 'すべてのホストを 1 つのボードで',
  'home.features.board.body':
    'すべてのホストのすべてのコンテナを 1 つの表にまとめ、絞り込んでまとめて操作できます。ホストごとのページもあります。エンジンの詳細、1 つのダイアログで未使用を削除できるディスク使用量、エンジンが行っていることのリアルタイムのストリームを表示します。',
  'home.features.keyboard.title': 'キーボード優先',
  'home.features.keyboard.body':
    'コマンドパレットで、どこへでも移動し、ホストを切り替え、コマンドを実行し、どのホストにあるコンテナ、イメージ、ボリュームでも見つけられます。すべてのページにショートカットがあり、? で一覧を表示できます。',
  'home.features.background.title': 'バックグラウンドで見守る',
  'home.features.background.body':
    'システムトレイにすべてのホストの状態を表示し、コンテナがクラッシュしたときやホストの接続が切れたときに通知します。ログイン時に Homeport を起動し、必要になるまで隠しておくこともできます。',

  // Home: Hosts.
  'home.hosts.eyebrow': 'ホスト',
  'home.hosts.title': '手間のかからないリモートのエンジン',
  'home.hosts.body':
    'サーバーは一度追加すれば、ボードに残ります。SSH の場合、Homeport はそのホスト専用のキーを作成し、それを承認するためのコマンドを 1 つ提示します。最初の接続時にサーバーのフィンガープリントを確認し、変わったときには警告します。',
  'home.hosts.import':
    'すでに設定済みですか？{file} からサーバーを（踏み台も含めて）、Docker コンテキストを（証明書も含めて）インポートできます。新しいコンピューターに移るときは、パスフレーズで保護した 1 つのファイルで持ち運べます。',
  'home.hosts.socket.name': 'ローカルソケット',
  'home.hosts.socket.detail': 'Unix ソケットと Windows の名前付きパイプを自動で検出します。',
  'home.hosts.ssh.detail':
    'ホストごとの専用 Ed25519 キー、自分のキー、ssh-agent、Pageant に対応。踏み台経由でも使えます。',
  'home.hosts.tls.detail': 'CA、クライアント証明書、キーを使った相互 TLS。',
  'home.hosts.tcp.detail':
    '暗号化されない素の TCP。信頼できるネットワーク上のエンジンでのみ使ってください。',
  'home.hosts.link': '接続のしくみ',
  'home.hosts.screenshot':
    'SSH を選択した「ホストを追加」ページ。サーバー、ポート、ユーザー、リモートのソケットパス、このホスト用の新しいキーが表示されています。',

  // Home: containers.
  'home.containers.eyebrow': 'コンテナ',
  'home.containers.title': 'コンテナのすべてを、ワンクリックで',
  'home.containers.body':
    'コンテナを開くと、ログがすでにストリーミングされています。タブを切り替えれば、対話型のターミナル、ファイル、リアルタイムのグラフ、プロセス、完全な inspect ドキュメントを表示できます。地球の反対側のホストでも、手元のノート PC と同じように扱えます。',
  'home.containers.logs.title': 'ログ',
  'home.containers.logs.body': '追跡、検索、ストリームと時間での絞り込み、ダウンロード。',
  'home.containers.terminal.title': 'ターミナル',
  'home.containers.terminal.body': '対話型のシェルを、必要なだけ。',
  'home.containers.files.title': 'ファイル',
  'home.containers.files.body': '閲覧、アップロード、ダウンロード。フォルダーは .tar で。',
  'home.containers.stats.title': '統計',
  'home.containers.stats.body': 'CPU、メモリ、ネットワーク、ディスク I/O のグラフ。',
  'home.containers.link': 'コンテナの操作',
  'home.containers.screenshot':
    'コンテナの「ログ」タブ。検索、タイムスタンプ、ストリームの絞り込みとともに出力をストリーミングしています。',

  // Home: Stacks.
  'home.stacks.eyebrow': 'スタック',
  'home.stacks.title': 'Compose スタックを、管理、リンク、同期、または検出',
  'home.stacks.body':
    'Compose ファイルとその環境変数を Homeport に保存して、どのホストにもデプロイできます。テンプレートから始め、複数のファイルを組み合わせ、プロファイルを選べます。変数は暗号化して保存され、ディスクに書き出されることはありません。このコンピューターのプロジェクトフォルダーをリンクすることもできます。Homeport はそのファイルをその場で編集し、そこからデプロイします（ビルドも含む）。git リポジトリのブランチを、デプロイキーまたはトークンで追跡することもできます。同期のたびに、デプロイする前に新しいコミットとその変更内容を表示します。Homeport を開いている間は新しいコミットを確認し、指定すれば自動でデプロイします。再デプロイのたびに何が変わるかを先に表示し、デプロイはそれぞれ保存されるのでロールバックでき、「アップデート」で新しいイメージのプルと再デプロイを 1 ステップで行えます。手作業でデプロイされたスタックも表示されます。起動、停止、削除したり、引き継いだりできます。',
  'home.stacks.link': 'スタックの管理',
  'home.stacks.screenshot':
    '管理スタックの定義。compose.yaml エディター、その環境変数、「保存して再デプロイ」ボタンが表示されています。',

  // Home: security.
  'home.security.eyebrow': 'セキュリティ',
  'home.security.title': 'キーは封印されたまま',
  'home.security.lead': 'Homeport はサーバーを開く認証情報を扱うため、それにふさわしく扱います。',
  'home.security.rest.title': '保存時に暗号化',
  'home.security.rest.body':
    'SSH キー、TLS 証明書、スタックの環境変数、リポジトリのトークンは XChaCha20-Poly1305 で封印されます。鍵はデータの隣ではなく、OS のキーチェーンに保管されます。',
  'home.security.disk.title': '平文でディスクに置かない',
  'home.security.disk.body':
    'Homeport は SSH を自ら話すため、復号されたキーはメモリ上にとどまります。キーファイルは作らず、{dir} も変更しません。',
  'home.security.home.title': '外部への送信なし',
  'home.security.home.body':
    'アカウントも、テレメトリも、トラッキングもありません。Homeport が通信するのはあなたのエンジンだけです。すべてのリリースは公開の場でビルドされ、チェックサムとビルドの来歴が付いています。',
  'home.security.link': 'セキュリティとデータ',

  // Home: desktops and the closing call.
  'home.desktops.eyebrow': 'ダウンロード',
  'home.desktops.title': 'どのデスクトップでもネイティブに',
  'home.desktops.lead':
    'Tauri と Rust で作られた、小さく高速なアプリです。ブラウザーのタブを装ったものではありません。x64 と ARM 用のインストーラーが、Homeport を最新の状態に保ちます。',
  'home.desktops.windows.versions': '10 および 11 · x64 および ARM64',
  'home.desktops.windows.formats': 'インストーラー（.exe） · MSI',
  'home.desktops.macos.versions': 'Apple シリコンおよび Intel',
  'home.desktops.macos.formats': 'ディスクイメージ（.dmg）',
  'home.desktops.linux.versions': 'x64 および ARM64',
  'home.desktops.linux.formats': 'apt · dnf · zypper · AppImage',
  'home.cta.title': 'サーバーにインストールするものはありません。',
  'home.cta.body':
    '作業するマシンに Homeport をダウンロードして、ホストを追加してください。無料で、GNU AGPL のもとでオープンソースとして公開され、GitHub で公開の場で開発されています。',
  'home.cta.github': 'GitHub で見る',
} satisfies Translation<typeof en>;
