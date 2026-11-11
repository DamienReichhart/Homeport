### The `network` namespace, in 日本語 (ja).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/ja/network.ts:
### change the text there, then run it again.

network-nav = ネットワーク
network-title = ネットワーク
network-services = Service
network-ingresses = Ingress
network-service = Service
network-ingress = Ingress
network-servicesSummary = { $count ->
       *[other] { NUM($count) } 個の Service
    }
network-ingressesSummary = { $count ->
       *[other] { NUM($count) } 個の Ingress
    }
network-searchServices = Service を検索
network-searchIngresses = Ingress を検索
network-column-name = 名前
network-column-namespace = Namespace
network-column-type = タイプ
network-column-clusterIp = クラスター IP
network-column-external = 外部
network-column-ports = ポート
network-column-class = クラス
network-column-hosts = ホスト
network-column-addresses = アドレス
network-column-age = 経過時間
network-tab-overview = 概要
network-tab-pods = Pod
network-tab-rules = ルール
network-tab-events = イベント
network-tab-source = YAML
network-detail-namespace = Namespace
network-detail-type = タイプ
network-detail-clusterIp = クラスター IP
network-detail-external = 外部アドレス
network-detail-selector = 送信先の Pod の条件
network-detail-noSelector = Pod を選択しません。エンドポイントは手動で管理されています。
network-detail-affinity = セッションアフィニティ
network-detail-class = クラス
network-detail-addresses = アドレス
network-detail-tls = TLS
network-detail-age = 経過時間
network-detail-labels = ラベル
network-detail-annotations = アノテーション
network-port-name = 名前
network-port-port = ポート
network-port-target = ターゲットポート
network-port-node = ノードポート
network-rule-host = ホスト
network-rule-path = パス
network-rule-type = 一致方法
network-rule-backend = バックエンド
