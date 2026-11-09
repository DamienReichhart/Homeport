---
title: 用語集
description: Homeport で使う言葉と、その意味。
---

Homeport は物事を正確に呼び分けています。ここに挙げる用語は、アプリとこのドキュメントの全体に登場します。各項目の見出しには製品の英語の用語を示し、そのあとに日本語のアプリで使う言葉を記しています。

### Host（ホスト）

Homeport が接続する Docker Engine で、名前と到達方法（ローカルソケット、TCP、TLS、SSH）で定義されます。すべてのコンテナ、イメージ、ボリューム、ネットワーク、スタックは、ちょうど 1 つのホストに属します。[ホストと接続](/docs/hosts/)を参照してください。

### Engine kind（エンジンの種類）

ホストが指しているエンジンの種類です。Docker 互換（Docker Engine、または互換 API を通じた Podman）、containerd、Incus のいずれかです。これによってホストに表示されるページが決まり、containerd と Incus のホストには専用のページがあります。Podman は独立した種類ではなく、Homeport が接続のたびに識別します。[エンジンの種類](/docs/hosts/#エンジンの種類)を参照してください。

### Namespace (containerd's)（名前空間）

containerd のホストで、すべてのコンテナ、イメージ、スナップショットが属する空間です。nerdctl は `default`、Kubernetes は `k8s.io`、Docker は `moby` を使います。containerd のホストのページは、名前空間を 1 つずつ表示します。クラスターの Namespace は Kubernetes のものです。[containerd の名前空間](/docs/hosts/containerd/#名前空間)を参照してください。

### Snapshot（スナップショット）

containerd のホストでは、ディスク上のファイルのレイヤーです。展開されたイメージのレイヤー、またはコンテナの書き込み可能なレイヤーです。[スナップショット](/docs/hosts/containerd/#スナップショット)を参照してください。Incus のホストでは、インスタンスを復元できる保存済みの状態です。[インスタンスのスナップショット](/docs/guides/instances/#スナップショット)を参照してください。

### Instance（インスタンス）

Incus のホストが実行するものです。ホストのカーネル上で Linux ディストリビューション全体を起動するシステムコンテナ、または独自のカーネルを持つ仮想マシンです。1 つのアプリケーションを実行するコンテナとは異なります。[インスタンス](/docs/guides/instances/)を参照してください。

### Project (Incus's)（プロジェクト）

Incus のホストで、インスタンス、イメージ、プロファイル、カスタムボリューム、ネットワークが属する空間です。Incus のホストのページはプロジェクトを 1 つずつ表示し、ホストの切り替えメニューの下で選びます。[プロジェクト](/docs/hosts/incus/#プロジェクト)を参照してください。

### Profile（プロファイル）

Incus のホストで、それを指定したインスタンスが共有する構成とデバイスです。[プロファイル](/docs/guides/instances/#プロファイル)を参照してください。

### Trust token（信頼トークン）

Incus サーバーで `incus config trust add` が出力するもので、HTTPS 経由でそのサーバーを追加するために、Homeport に一度だけ貼り付けます。[信頼トークン](/docs/hosts/incus/#信頼トークンを使った-https-経由)を参照してください。

### Pod（Podman Pod）

Podman のホストで、1 つのネットワークを共有し、一緒に起動・停止するコンテナのグループです。Pod が公開するポートを保持する infra コンテナを中心に構成されます。クラスターの Pod は Kubernetes の Pod です。[Podman Pod](/docs/guides/podman-pods/) を参照してください。

### Quadlet unit（Quadlet ユニット）

Podman のホスト上のファイル（`web.container` など）で、systemd がこれをサービス（`web.service`）に変換します。これにより、Podman のコンテナが起動時に開始され、systemd がそれを動かし続けます。[Quadlet ユニット](/docs/guides/quadlet/)を参照してください。

### Cluster（クラスター）

Homeport が API を通じて接続する Kubernetes クラスターまたは Nomad クラスターです。Kubernetes のクラスターは kubeconfig のコンテキストから追加します。名前、API サーバーのアドレス、kubeconfig が与える認証情報で構成され、認証情報は暗号化して保存されます。Nomad のクラスターは、エージェントの 1 つのアドレスと、ACL トークン、TLS の情報で追加し、これらは暗号化して保存されます。Homeport はどちらにも何もインストールせず、それらの認証情報が許す範囲にだけ到達します。ホストを通じて見つかり、追加されることのない Swarm とは異なります。Kubernetes 独自のオブジェクトは Kubernetes での名前（Namespace、Pod、Deployment、Service、Node…）のまま、Nomad のオブジェクトは Nomad での名前（ジョブ、タスクグループ、タスク、アロケーション、クライアントノード、名前空間）で呼びます。[クラスター](/docs/clusters/)と [Nomad クラスター](/docs/clusters/nomad/)を参照してください。

### Allocation（アロケーション）

Nomad クラスターで、ジョブのタスクグループをクライアントノードに 1 回配置したもので、そのタスクを実行します。ログとシェルは Nomad のサーバーを経由してノードに届きます。[ログとシェル](/docs/clusters/nomad/#ログとシェル)を参照してください。

### Swarm（Swarm）

Docker Swarm のクラスターです。所属するホストを通じて見つかり、単独で追加されることはありません。Homeport は接続済みのいずれかのマネージャーのホストを通じて到達し、Swarm ID で Swarm を区別し、Swarm について何も保存しません。Docker はすべての Swarm に同じ名前を付けるため、最初のマネージャーのホストの名前で呼ばれます。[Swarm](/docs/guides/swarm/) を参照してください。

### Port forward（ポートフォワード）

このコンピューターのループバック上のポートで、Homeport が Kubernetes API を通じて Pod のポート、または準備完了の Pod の 1 つを経由して Service のポートにつなぐものです。`kubectl port-forward` と同様です。Homeport を終了すると終わります。[ポートフォワード](/docs/clusters/port-forward/)を参照してください。

### Release（リリース）

Helm のリリースです。クラスター上へのチャートの 1 回のインストールで、Namespace の中で名前が付けられ、インストール、アップグレード、ロールバックのたびに番号付きのリビジョンを持ちます。Helm がクラスター上に保持するため、ターミナルで作ったリリースも Homeport で作ったリリースも同じものです。[Helm リリース](/docs/clusters/helm/)を参照してください。

### Chart repository（チャートリポジトリ）

helm がチャートをダウンロードするサーバーで、このコンピューター上の helm 自身の一覧（`helm repo list` が表示するもの）にあります。OCI レジストリ上のチャートは、代わりに `oci://` の参照でインストールします。[チャートリポジトリ](/docs/clusters/helm/#チャートリポジトリ)を参照してください。

### Node（ノード）

Swarm の中の 1 つのエンジンで、マネージャーかワーカーです。クラスターの Node は Kubernetes の Node で、Pod を実行するマシンです。ノードがホストであるのは Homeport がそれに接続している場合だけで、ホストのページからそのノードにリンクしています。

### Service（サービス）

Swarm が実行するものです。イメージ、その設定、レプリカの数で構成されます。グローバルサービスの場合は、すべてのノードで 1 つずつタスクを実行します。

### Task（タスク）

1 つのノード上にあるサービスの 1 つの枠で、そこでコンテナとして実行されます。サービスが収束しないときは、タスクがその理由を示します。containerd のホストでは、タスクはコンテナのために実行されるもので、コンテナ自体はメタデータにすぎません。コンテナを起動するとそのタスクが作成されます。

### Stack（スタック）

ホスト上で動いている Compose プロジェクト、または Swarm 上で動いている Swarm のスタックで、プロジェクト名で識別されます。どのスタックも、管理スタック、リンクスタック、同期スタック、外部スタックのいずれかです。[スタック](/docs/guides/stacks/)と [Swarm のスタック](/docs/guides/swarm/#swarm-スタック)を参照してください。

### Managed Stack（管理スタック）

compose の定義と環境変数を Homeport が保持しているスタックで、Homeport からデプロイ、再デプロイ、編集ができます。

### Linked Stack（リンクスタック）

compose ファイルがこのコンピューター上のフォルダーにあるスタックです。Homeport はフォルダーと compose ファイルの場所を保持し、ファイルをその場で読み書きし、そのフォルダーをプロジェクトディレクトリとしてデプロイするため、`build:`、`env_file:`、相対パスが使えます。動作中のコンテナの compose ラベルから見つかるか、フォルダーから作成されます。ファイルが見つからない間は外部スタックになります。[リンクスタック](/docs/guides/stacks/#リンクスタック)を参照してください。

### Synced Stack（同期スタック）

compose の定義が git リポジトリから来るスタックです。Homeport はこのコンピューター上にそのクローンを保持し、必要に応じて、または Homeport が開いている間は一定の間隔で、ブランチやタグをフェッチし、何が変わったかを示し、compose ファイルのフォルダーからコミットをデプロイするため、`build:` と相対パスが使えます。Homeport の中で編集されることはなく、ファイルは git を通じて変更されます。Homeport が扱うのは環境変数と同期の設定だけです。[同期スタック](/docs/guides/synced-stacks/)を参照してください。

### Repository（リポジトリ）

Homeport が同期スタックのために読み込める git リポジトリです。URL と、Homeport の認証方法（Homeport が生成したデプロイキー、ユーザー名とトークン、公開リポジトリなら認証なし）で構成されます。すべてのホストで共通に 1 回だけ保持され、そこから作られたすべての同期スタックで共有されます。

### Deploy key（デプロイキー）

Homeport が 1 つのリポジトリのために生成する Ed25519 キーで、git ホスティングサービス上でそのリポジトリに読み取り専用で追加します。ご自身の SSH キーではありません。

### External Stack（外部スタック）

ホスト上で動いているのが見つかったものの、Homeport が compose の定義を持っていない（保存もされておらず、このコンピューター上のフォルダーにも、追跡しているリポジトリにもない）スタックです。起動、停止、削除はできますが、再デプロイや編集はできません。

### Adopt（引き継ぐ）

compose の定義を与えて、外部スタックを管理スタックにすることです。

### Registry login（レジストリのログイン情報）

Docker Hub や GHCR など 1 つのコンテナレジストリのために Homeport が保持するユーザー名とパスワードまたはアクセストークンで、すべてのホストでプル、プッシュ、ビルドに使われます。[設定](/docs/guides/settings/#レジストリ)を参照してください。

### Helper container（ヘルパーコンテナ）

エンジン API ではボリュームを読み込めないため、ボリュームを開く（閲覧、バックアップ、復元、複製する）ために Homeport がホスト上に作成する短命の `busybox` コンテナです。ネットワークを持たず、書き込む必要がない限りボリュームを読み取り専用でマウントし、操作が終わると削除されます。「コンテナ」の一覧や「イベント」ページに表示されることはありません。[ボリュームのファイルの閲覧](/docs/guides/volumes-networks/#ヘルパーコンテナ)を参照してください。

### Board（ボード）

接続中のすべてのホストのコンテナを 1 つの表にまとめ、それぞれにホストのラベルを付けて表示するページです。[ボード](/docs/guides/overview/#ボード)を参照してください。

### Command palette（コマンドパレット）

<kbd>Mod</kbd> <kbd>K</kbd> で開く検索ボックスです。ページへの移動、ホストの切り替え、コマンドの実行、接続中のどのホストにあるコンテナ、イメージ、ボリューム、ネットワーク、スタックの検索もできます。[キーボード](/docs/guides/keyboard/#コマンドパレット)を参照してください。

### Engine（エンジン）

マシン上でコンテナを実行する Docker Engine（`dockerd`）、または Podman などの互換エンジンです。Homeport は Docker Engine API を通じてそれと通信します。

### Socket（ソケット）

エンジンが待ち受けるアドレスです。`/var/run/docker.sock` のような Unix ソケットファイル、または Windows では `//./pipe/docker_engine` のような名前付きパイプです。

### Keychain（キーチェーン）

オペレーティングシステムが備えるパスワードとキーの安全な保管場所（macOS のキーチェーン、Windows の資格情報マネージャー、Linux の Secret Service）で、Homeport はここにシークレットを暗号化する鍵を保管します。

### Trust on first use（初回接続時の信頼）

Homeport が SSH サーバーを確認する方法です。初回接続時にサーバーのキーのフィンガープリントを確認し、その後キーが変わった場合、再び確認するまで Homeport は接続を拒否します。

## 日本語では

日本語のアプリ、ドキュメント、Web サイトはこれらの用語を使うため、画面とページの言葉は常に一致します。

| 英語           | 日本語           |
| -------------- | ---------------- |
| Host           | ホスト           |
| Engine         | エンジン         |
| Instance       | インスタンス     |
| Swarm          | Swarm            |
| Cluster        | クラスター       |
| Stack          | スタック         |
| Managed Stack  | 管理スタック     |
| Linked Stack   | リンクスタック   |
| Synced Stack   | 同期スタック     |
| External Stack | 外部スタック     |
| Adopt          | 引き継ぐ         |
| Board          | ボード           |
| Bastion        | 踏み台           |
| Deploy key     | デプロイキー     |
| Port forward   | ポートフォワード |
| Release (Helm) | リリース         |
