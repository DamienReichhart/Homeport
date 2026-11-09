---
title: Exec プラグイン
description: Homeport が kubeconfig に指定された exec 認証情報プラグインで EKS、GKE、AKS のクラスターにサインインする方法と、何がどこで実行されるか。
---

クラウドの kubeconfig には、パスワードも証明書も含まれていません。その代わり、ユーザーには **exec プラグイン** が指定されています。これはコンピューター上のプログラムで、求められると有効期間の短いトークンを出力します。Homeport は、Kubernetes の `ExecCredential` プロトコル（`client.authentication.k8s.io/v1` と `v1beta1`）に従って、`kubectl` が実行するのと同じプログラムをこのコンピューター上で実行します。

## 主なプロバイダー

| プロバイダー | kubeconfig のユーザーが実行するもの  | インストール                                                              |
| ------------ | ------------------------------------ | ------------------------------------------------------------------------- |
| EKS          | `aws eks get-token --cluster-name …` | AWS CLI バージョン 2                                                      |
| GKE          | `gke-gcloud-auth-plugin`             | Google Cloud CLI、次に `gcloud components install gke-gcloud-auth-plugin` |
| AKS          | `kubelogin get-token …`              | `az aks install-cli`、またはリリースページからの kubelogin                |

いつもどおりに kubeconfig を書き出し（`aws eks update-kubeconfig`、`gcloud container clusters get-credentials`、または `az aks get-credentials` の後に `kubelogin convert-kubeconfig`）、[そのコンテキストを追加](/docs/clusters/)します。

## 何がどこで実行されるか

- プラグインは **このコンピューター上で**、ユーザー自身として、kubeconfig が指定する引数と環境変数（たとえば `AWS_PROFILE`）で実行されます。Homeport がプラグインに対話を求めることはなく、入力も渡しません。
- デスクトップのランチャーは短い `PATH` で Homeport を起動するので、Homeport は AWS、Google Cloud、Azure の各 CLI と kubelogin の通常のインストール場所（Homebrew、SDK 自身のフォルダー、`~/.local/bin`、`~/.krew/bin`、Program Files…）も探します。kubeconfig でフルパスが指定されたプラグインは、そこから実行されます。
- プラグインが出力するトークンや証明書は、有効期限の少し前まで **メモリ上にだけ** 保持され、その後プラグインがもう一度実行されます。実行されるのはリクエストごとではなく、トークンの有効期間ごとに 1 回です。API がトークンを拒否した場合、Homeport は **アクセスが拒否されました** を表示する前に、一度だけプラグインに新しいトークンを求めます。
- プラグイン自身のキャッシュ（`~/.kube/cache`、`~/.aws/sso/cache`、gcloud の設定）はプラグインが管理するもので、Homeport はそこに何も書き込みません。

## サインインできない場合

- **「… はこのコンピューターにインストールされていません。」**: プログラムが見つかりませんでした。インストールし（メッセージにその方法が示されます。kubeconfig に `installHint` があればそれを使います）、ターミナルで動作することを確認してから、**再接続** してください。
- **「… ではサインインが必要ですが、Homeport の中ではサインインできません。」**: プラグインがブラウザーかデバイスコードを必要としています（AWS SSO、Azure のデバイスログイン、期限切れの `gcloud` セッション）。そのコンテキストでターミナルから一度 `kubectl` を実行してそこでサインインしてから、**再接続** を選んでください。プラグインはキャッシュされたサインインを見つけます。
- **「… は … の認証情報を取得できませんでした。」**: プラグインは実行されましたが、失敗しました。**詳細** には、プラグインが出力した内容の末尾が表示されます。Homeport は自動で再試行します。

ほかのメッセージについては[トラブルシューティング](/docs/reference/troubleshooting/#クラスター)を参照してください。
