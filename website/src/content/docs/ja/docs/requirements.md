---
title: システム要件
description: Homeport の動作に必要なもの、そして SSH、TLS、スタックのデプロイ、Kubernetes と Nomad のクラスター、Helm を使えるようにする任意のコンポーネント。
---

## オペレーティングシステム

| システム | バージョン                                                     | アーキテクチャ        |
| -------- | -------------------------------------------------------------- | --------------------- |
| Windows  | Windows 10 と 11                                               | x64、ARM64            |
| macOS    | macOS 10.15 Catalina 以降                                      | Apple シリコン、Intel |
| Linux    | glibc 2.35 以降: Ubuntu 22.04、Debian 12、Fedora 36 とそれ以降 | x64、ARM64            |

Homeport はウィンドウの描画にシステムの Web ビューを使います。Windows では WebView2（Windows 10 と 11 に付属）、macOS では WebKit、Linux では WebKitGTK（`.deb` と `.rpm` のパッケージの依存関係としてインストールされ、AppImage には同梱）です。

## 管理する Docker Engine

Homeport にはコンテナランタイムは含まれていません。すでにどこかで動いている Docker Engine を管理します:

- **このマシン上** — Docker Engine、Docker Desktop、Colima、OrbStack、Rancher Desktop、Podman。Homeport が自動で見つけます。[ローカルのエンジン](/docs/hosts/local/)を参照してください。
- **サーバー上** — [SSH](/docs/hosts/ssh/)、または [TLS か TCP](/docs/hosts/tls-tcp/) で到達できるもの。

Docker Engine API を話すエンジンなら、Podman の Docker 互換ソケットも含め、どれでも使えます。

[containerd](/docs/hosts/containerd/) 1.7 以降も、このマシン上のソケット経由か SSH 経由で直接管理できます。k3s のもの、Docker Engine 自身のもの、サーバー上の containerd のサービスが対象です。そこでイメージをプルするには containerd 2.0 以降が必要で、ログとターミナルには SSH で到達するホストか、このコンピューター上の containerd（Linux）が必要です。

[Incus](/docs/hosts/incus/) 6.0 以降も、このマシン上のソケット経由か SSH 経由、または信頼トークンを使った HTTPS 経由で管理できます。仮想マシンには Incus サーバーでのハードウェア仮想化（`/dev/kvm`）が必要で、仮想マシン内のターミナルには、そのイメージが実行する `incus-agent` が必要です。

Podman のホストでは、Homeport は Podman 独自の API も使います。[Pod](/docs/guides/podman-pods/) には Podman 4.0 以降が必要です。[Quadlet ユニット](/docs/guides/quadlet/)は Podman 5.7 から一覧表示され、5.8 からインストール、編集、削除ができます。Quadlet ユニットの起動と停止には、SSH で到達するホストか、このコンピューター上の Podman（Linux）が必要です。

## 任意: Kubernetes クラスター

[クラスター](/docs/clusters/)を閲覧するには、そこに到達できる kubeconfig（`kubectl` が使うもの、または貼り付けたもの）と、API サーバーへのネットワークアクセスが必要です。Homeport は Kubernetes API を自分で話すため、`kubectl` は必要ありません。

- **Kubernetes のバージョン**: Kubernetes 1.24 以降を実行しているクラスター。Homeport は安定版の API（`v1`、`apps/v1`、`batch/v1`、`networking.k8s.io/v1`）だけを使い、1.33 の API を対象にビルドされています。
- **Exec プラグイン**: クラウドの kubeconfig は、認証情報を得るためにこのコンピューターで実行するプログラムを指定しています。EKS なら [AWS CLI](https://aws.amazon.com/cli/)（バージョン 2）、GKE なら Google Cloud CLI の `gke-gcloud-auth-plugin`、AKS なら `kubelogin` です。kubeconfig が指定しているものをインストールしてください。[Exec プラグイン](/docs/clusters/exec-plugins/)を参照してください。

## 任意: Nomad クラスター

[Nomad クラスター](/docs/clusters/nomad/)を管理するには、Nomad エージェントの HTTP API（既定ではポート 4646）へのネットワークアクセスと、クラスターで ACL が有効な場合はトークンが必要です。必要な [ACL ポリシー](/docs/clusters/nomad/#acl-ポリシー)を参照してください。Homeport は Nomad の API を自分で話すため、`nomad` CLI は必要なく、クラスターには何もインストールされません。

- **Nomad のバージョン**: Nomad 1.6 以降。**編集して実行** はジョブの送信済み仕様から始まります。Nomad は 1.6 以降、これを保持しています。

## 任意: OS のキーチェーン

Homeport は、SSH キー、TLS 証明書、クラスターの kubeconfig の認証情報、スタックの環境変数、レジストリのログイン情報、リポジトリのデプロイキーとトークンを、オペレーティングシステムのキーチェーンに保管した鍵で暗号化します:

- **macOS** — キーチェーン
- **Windows** — 資格情報マネージャー
- **Linux** — GNOME Keyring や KWallet などの Secret Service プロバイダー

キーチェーンがない場合、Homeport はバナーを表示しますが、ローカルソケット、暗号化なしの TCP、**ssh-agent** を使った SSH では引き続き動作します。TLS のホスト、キーを保存した SSH のホスト、クラスター、スタックの環境変数、レジストリのログイン情報、プライベートリポジトリにはキーチェーンが必要です。キーチェーンがないと、イメージは匿名でプルされ、追加できるのは公開リポジトリだけになります。[セキュリティとデータ](/docs/reference/security/)を参照してください。

## 任意: Docker Compose

[管理スタック、リンクスタック、同期スタック](/docs/guides/stacks/)をデプロイするために、Homeport は **この** マシン（ホストではありません）で `docker compose` コマンドを実行します。Compose プラグイン付きの Docker CLI（Docker Desktop に含まれています）か、スタンドアロンの `docker-compose` v2 をインストールしてください。

これがなくても、スタックは（リンクスタックも含めて）表示され、起動、停止、削除ができます。Compose が必要なのはデプロイだけです。Homeport が見つけたかどうかは **設定 → このマシン** に表示されます。

[Swarm](/docs/guides/swarm/) にスタックをデプロイする場合、Homeport は代わりにこのマシンで `docker stack deploy` を実行するため、`docker` コマンドそのものが必要です（Compose プラグインは使いません）。Docker Desktop には含まれています。Linux では、ディストリビューションの Docker CLI パッケージ（`docker-ce-cli`、`docker.io`、`docker` のいずれか）をインストールしてください。ここでエンジンを動かす必要はありません。

## 任意: git

[同期スタック](/docs/guides/synced-stacks/)は、リポジトリのクローンとフェッチに **この** マシンにインストールされた `git` を使います。Git for Windows、Apple のコマンドラインツール（`xcode-select --install`）、またはディストリビューションの `git` パッケージです。Windows では、SSH のリポジトリ用に Homeport が git にシステムの OpenSSH を使わせます。それ以外の機能はすべて git なしで動作します。Homeport が見つけたかどうかは **設定 → このマシン** に表示されます。

## 任意: Helm

クラスターで [Helm リリース](/docs/clusters/helm/)を管理するために、Homeport は **この** マシンでバージョン 3 以降の `helm` コマンドを実行します。helm はプライベートな接続を通じてクラスターに向けられるため、クラスターの認証情報がディスクに書き込まれることはなく、helm はターミナルで使うときと同じく自身のチャートリポジトリを使います。インストールには、`brew install helm`（macOS）、`winget install Helm.Helm`、Chocolatey、Scoop（Windows）、または `snap install helm --classic`、ディストリビューションのパッケージ、[helm.sh](https://helm.sh/docs/intro/install/) のスクリプト（Linux）を使います。クラスターのほかのページはすべて helm なしで動作します。Homeport が見つけたかどうかは **設定 → このマシン** に表示されます。

## 任意: システムトレイ

Homeport はシステムトレイ（macOS ではメニューバー、Windows では通知領域）に、すべてのホストとそのステータスを一覧表示するアイコンを置きます。Windows と macOS には常にシステムトレイがあります。Linux では **appindicator** ライブラリが必要です。AppImage には含まれており、`.deb` と `.rpm` のパッケージはこれを推奨パッケージにしているため、推奨パッケージを無効にしていなければ `apt`、`dnf`、`zypper` がインストールします。そうでない場合は、`libayatana-appindicator3-1`（Debian、Ubuntu、openSUSE）か `libayatana-appindicator-gtk3`（Fedora）をインストールしてください。トレイアイコンを隠すデスクトップもあり、GNOME では AppIndicator Support などの拡張機能がある場合にだけ表示されます。トレイがない場合、Homeport はウィンドウから操作し、ウィンドウを閉じると終了します。

## 機能ごとに必要なもの

| 機能                                                       | キーチェーン | Compose CLI | git  | helm |
| ---------------------------------------------------------- | :----------: | :---------: | :--: | :--: |
| Swarm へのスタックのデプロイ                               |      —       | docker CLI  |  —   |  —   |
| ローカルソケットと TCP のホスト                            |      —       |      —      |  —   |  —   |
| ssh-agent を使う SSH のホスト                              |      —       |      —      |  —   |  —   |
| Homeport のキーかインポートしたキーを使う SSH のホスト     |     はい     |      —      |  —   |  —   |
| TLS のホスト                                               |     はい     |      —      |  —   |  —   |
| コンテナ、イメージ、ボリューム、ネットワーク               |      —       |      —      |  —   |  —   |
| スタックの起動、停止、削除                                 |      —       |      —      |  —   |  —   |
| 管理スタックとリンクスタックのデプロイ                     |      —       |    はい     |  —   |  —   |
| 管理スタックや同期スタックの環境変数                       |     はい     |      —      |  —   |  —   |
| レジストリのログイン情報（プライベートイメージ、プッシュ） |     はい     |      —      |  —   |  —   |
| 公開リポジトリからの同期スタック                           |      —       |    はい     | はい |  —   |
| デプロイキーかトークンを使う同期スタック                   |     はい     |    はい     | はい |  —   |
| フォルダーからのイメージのビルド                           |      —       |      —      |  —   |  —   |
| Kubernetes クラスター                                      |     はい     |      —      |  —   |  —   |
| Nomad クラスター                                           |     はい     |      —      |  —   |  —   |
| Helm リリース                                              |     はい     |      —      |  —   | はい |
