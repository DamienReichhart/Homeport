---
title: CPU とメモリ
description: metrics-server から、Pod とそのコンテナ、Node が使っている CPU とメモリを、リクエスト、リミット、割り当て可能量と比べて確認します。
---

クラスターで [metrics-server](https://github.com/kubernetes-sigs/metrics-server) が動いていれば、Homeport は各 Pod と各 Node が使っている CPU とメモリを、一覧と、それぞれのページのライブグラフに表示します。`kubectl top` と同じように、metrics-server が提供する Metrics API（`metrics.k8s.io`）を読み取ります。

## 必要なもの

metrics-server がすでにクラスターで動いている必要があります。Homeport はクラスターに何もインストールしません。入っているかどうかは、クラスターの作り方によります。

| クラスター                     | metrics-server                                                                |
| ------------------------------ | ----------------------------------------------------------------------------- |
| GKE                            | インストール済み                                                              |
| AKS                            | インストール済み                                                              |
| k3s と k3d                     | インストール済み（k3s を `--disable metrics-server` で起動した場合を除く）    |
| Docker Desktop                 | その Kubernetes と一緒にインストール済み                                      |
| kind                           | 未インストール                                                                |
| EKS                            | 未インストール：metrics-server アドオンを追加するか、自分でインストールします |
| minikube                       | 未インストール：`minikube addons enable metrics-server`                       |
| その他のクラスター（kubeadm…） | 通常は未インストール                                                          |

自分でインストールするには、[その手順](https://github.com/kubernetes-sigs/metrics-server#installation)に従ってください。kubelet が自己署名証明書を使うローカルクラスター（kind）では、metrics-server に `--kubelet-insecure-tls` も必要です。

metrics-server がなくても何も壊れません。CPU とメモリの列は表示されず、Pod や Node の **メトリクス** タブに、metrics-server がインストールされていないことと、そのリンクが表示されます。インストール直後の metrics-server は、サンプルがそろうまで 1 分ほどかかります。その間、タブには Metrics API がまだ応答しないと表示されます。

認証情報にはメトリクスを読む権限も必要です。Namespace（またはすべての Namespace）の `pods.metrics.k8s.io` と、Node 用の `nodes.metrics.k8s.io` に対する `list` と `get` です。権限がない場合、**メトリクス** タブにどの権限が足りないかが表示され、ほかのページはすべてそのまま使えます。

## 一覧

**Pod** では、**CPU** と **メモリ** の列に、各 Pod がコンテナの合計で使っている量と、それがリミット（リミットがなければリクエスト）のどれだけにあたるかが表示されます。Pod にリミットがあるのは、すべてのコンテナにリミットがある場合だけです。リミットのないコンテナは Node 全体を使えるからです。CPU は `kubectl top` と同じくミリコア単位（`250m` は 4 分の 1 コア）、メモリは kubelet が Pod の退避に使うワーキングセットです。

**Node** では、**CPU 使用量** と **メモリ使用量** に、各 Node の使用量と、Pod に **割り当て可能** な量に対する割合が表示されます。

列の見出しをクリックすると、その列で並べ替えられます。負荷の高い Pod や Node が先頭に来ます。

## Pod や Node のページ

Pod の **メトリクス** タブには、各コンテナの CPU とメモリのグラフが、設定されていればリクエストとリミットの破線とともに表示されます。Node のタブには、その CPU とメモリのグラフが、割り当て可能量の線とともに表示されます。

metrics-server は 15 秒ごとに kubelet を読み取り、Homeport も同じ間隔で新しいサンプルを要求するので、負荷の変化は 30 秒以内にグラフに表れます。サンプルは、どのタブを開いていても、ページを開いている間だけ最大 15 分間、メモリ上にのみ保持されます。ページを閉じると読み取りが止まり、サンプルは破棄されます。Homeport はメトリクスの履歴を保存しません。履歴が必要なら、Prometheus などの監視システムを使ってください。
