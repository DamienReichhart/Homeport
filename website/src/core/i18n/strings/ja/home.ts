/** The home page, in Japanese. */
export default {
  // Home: the hero and its map.
  'home.title': 'Homeport — あらゆるコンテナエンジンを、1 つのアプリで',
  'home.hero.title': 'あらゆるコンテナエンジンを、1 つのアプリで。',
  'home.hero.lead':
    'Homeport はあなたのマシンで動作し、サーバーには何もインストールしません。無料のオープンソースです。',
  'home.hero.docs': 'ドキュメントを読む',
  'home.map.label':
    '路線図として描いたサーバー群の例。このマシンは 5 本の路線の終点です。ローカルソケットは laptop へ、SSH は prod-eu-1、nas、edge-1 へ、TLS は staging と vps-fra-1 へ、TCP は lan-box へ、クラスター API は k8s-prod、jobs、lab へ通じています。vps-fra-1 上の 1 個のスタックに対応が必要です。',
  'home.map.thisMachine': 'このマシン',
  'home.map.needsYou': '1 個のスタックに対応が必要',
  'home.map.keyNeedsYou': '要対応',
  'home.map.example': 'サーバー群の例',

  // Home: the questions people open it for.
  'home.questions.title': '1 つの問いに、1 つの場所。',
  'home.questions.lead':
    'Homeport は、何かを確かめるために開き、確かめたら作業に戻るためのアプリです。どの答えも、ボードから一歩のところにあります。',
  'home.questions.up.title': '動いているか？',
  'home.questions.up.body':
    'ボードはすべてのホスト、Swarm、クラスターを一度に表示し、問題のないものより先に、要対応のものを並べます。',
  'home.questions.up.link': 'ボードと概要',
  'home.questions.up.screenshot':
    'ボード。3 台のホストそれぞれのタイル、要対応のコンテナ 1 つ、その下にすべてのホストのコンテナをまとめた 1 つの表。',
  'home.questions.died.title': 'なぜ止まったのか？',
  'home.questions.died.body':
    'コンテナを開くと、ログがすでにストリーミングされています。ターミナル、ファイル、リアルタイムのグラフ、inspect ドキュメントへは、タブを切り替えるだけです。',
  'home.questions.died.link': 'コンテナの操作',
  'home.questions.died.screenshot':
    'コンテナの「ログ」タブ。検索、タイムスタンプ、ストリームの絞り込みとともに出力をストリーミングしています。',
  'home.questions.changed.title': '何が変わったのか？',
  'home.questions.changed.body':
    '各ホストは、そのエンジンが行ったことを記録し続けます。再起動したコンテナ、プルされたイメージ、マウントされたボリュームなどです。',
  'home.questions.changed.link': 'イベント',
  'home.questions.changed.screenshot':
    'ホストの「イベント」ページ。ヘルスチェック、再起動、プル、マウントが新しい順に、それぞれが属するスタックとともに並んでいます。',
  'home.questions.act.title': 'そして変更する。何が変わるかを先に確かめて。',
  'home.questions.act.body':
    '再デプロイでは、実行する前に何が変わるかを表示します。デプロイはそれぞれ保存されるのでロールバックでき、「アップデート」で新しいイメージを 1 ステップでプルできます。',
  'home.questions.act.link': 'スタックの管理',
  'home.questions.act.screenshot':
    '管理スタックの定義。compose.yaml エディター、その環境変数、「デプロイ」ボタンが表示されています。',

  // Home: the lines, and what each needs on the server.
  'home.lines.title': 'サーバーには何も住み着きません。',
  'home.lines.lead':
    'Homeport はあなたのマシンにとどまり、サーバーを訪れるだけです。インストールするエージェントも、公開する Web パネルも、作成するアカウントもなく、テレメトリもありません。',
  'home.lines.line': '路線',
  'home.lines.reaches': '接続先',
  'home.lines.needs': 'サーバーで必要なもの',
  'home.lines.socket.name': 'ローカルソケット',
  'home.lines.socket.reaches':
    'このコンピューター上の Docker Desktop、OrbStack、Colima、Rancher Desktop、Podman マシン。自動で見つかります。',
  'home.lines.socket.needs': '何も必要ありません。エンジンはすでにこのマシンにあります。',
  'home.lines.ssh.reaches':
    'サーバー上の Docker Engine、Podman、containerd、Incus。必要なら踏み台を経由します。',
  'home.lines.ssh.needs':
    'すでに開いているポート。Homeport が作成するキーを使う場合は、{file} に 1 行が追加されます。',
  'home.lines.tls.reaches':
    'Docker 互換エンジンの API。あなたの CA とクライアント証明書を使います。',
  'home.lines.tls.needs':
    'エンジン自身の API ポート（通常は {port}）。公開するのはあなた自身です。',
  'home.lines.tcp.reaches': 'Docker 互換エンジンの API。暗号化されません。',
  'home.lines.tcp.needs':
    'エンジンの API ポート（通常は {port}）。信頼できるネットワークでのみ使ってください。',
  'home.lines.cluster.name': 'クラスター API',
  'home.lines.cluster.reaches':
    'kubeconfig からの Kubernetes、エージェントのアドレスで接続する Nomad、信頼トークンを使った HTTPS での Incus。',
  'home.lines.cluster.needs':
    'クラスターがすでに提供している API。ノードには何もインストールしません。',
  'home.lines.advice':
    'できれば SSH を使ってください。サーバーで、すでに動いているもの以外に何も追加しません。',
  'home.lines.import':
    'サーバーの設定はもう書いてありますか？{file} から踏み台も含めてインポートするか、Docker コンテキストからインポートできます。',
  'home.lines.link': '接続のしくみ',

  // Home: the Engines, as the map's index of stations.
  'home.engines.title': 'どのエンジンも、同じ言葉で。',
  'home.engines.lead':
    'どのエンジンで動いていても、ホストはホスト、スタックはスタックです。エンジンに独自の機能があれば、Homeport はそれも表示します。',
  'home.engines.docker.body':
    'コンテナ、イメージ、ボリューム、ネットワーク、Compose スタック。スタックは Homeport に保存するか、フォルダーにリンクするか、git リポジトリから同期します。',
  'home.engines.swarm.body':
    'ノード、サービス、タスク、シークレット、コンフィグを、マネージャーのホストを通じて表示します。Swarm にスタックをデプロイできます。',
  'home.engines.podman.body':
    'Podman Pod と、systemd のもとでインストールして起動する Quadlet ユニット。どちらも Podman 自身の API を使います。',
  'home.engines.containerd.body':
    '名前空間、containerd タスク、ログ、ターミナル、そしてディスクを圧迫しているスナップショット。Docker は不要です。',
  'home.engines.incus.body':
    'システムコンテナと仮想マシン。そのコンソール、スナップショット、イメージ、プロファイル、ストレージ、ネットワーク。',
  'home.engines.kubernetes.body':
    'ワークロード、Pod、Service、あらゆる API リソースをリアルタイムで。ログ、ターミナル、ポートフォワード、プレビュー付きの YAML、Helm リリース。',
  'home.engines.nomad.body':
    'HCL からプラン付きで実行するジョブ、そのアロケーションとログ、タスク内のシェル、ノードのドレイン。',

  // Home: security.
  'home.security.title': 'キーは封印されたまま。',
  'home.security.lead': 'Homeport はサーバーを開く認証情報を預かるため、それにふさわしく扱います。',
  'home.security.rest.title': '保存時に暗号化',
  'home.security.rest.body':
    'SSH キー、TLS 証明書、スタックの環境変数、リポジトリのトークンは XChaCha20-Poly1305 で封印されます。鍵はデータの隣ではなく、OS のキーチェーンに保管されます。',
  'home.security.disk.title': '平文でディスクに置かない',
  'home.security.disk.body':
    'Homeport は SSH を自ら話すため、復号されたキーはメモリ上にとどまります。キーファイルは作らず、{dir} も変更しません。',
  'home.security.home.title': '外部への送信なし',
  'home.security.home.body':
    'アカウントも、テレメトリも、トラッキングもありません。すべてのリリースは公開の場でビルドされ、チェックサムとビルドの来歴が付いています。',
  'home.security.link': 'セキュリティとデータ',

  // Home: the close, where the visitor starts.
  'home.close.title': 'このマシンから始めましょう。',
  'home.close.body':
    '作業するコンピューターに Homeport をダウンロードして、ホストを追加してください。無料で、GNU AGPL のもとでオープンソースとして公開され、GitHub で公開の場で開発されています。',
  'home.close.github': 'GitHub で見る',
  'home.desktops.windows.versions': '10 および 11、x64 および ARM64',
  'home.desktops.windows.formats': 'インストーラー（.exe）または MSI',
  'home.desktops.macos.versions': 'Apple シリコンおよび Intel',
  'home.desktops.macos.formats': 'ディスクイメージ（.dmg）',
  'home.desktops.linux.versions': 'x64 および ARM64',
  'home.desktops.linux.formats': 'apt、dnf、zypper または AppImage',
};
