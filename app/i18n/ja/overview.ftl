### The `overview` namespace, in 日本語 (ja).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/ja/overview.ts:
### change the text there, then run it again.

overview-nav = 概要
overview-title = 概要
overview-addHost = ホストを追加
overview-addCluster = クラスターを追加
overview-cluster-nodes = { $total } 個中 { $ready } 個の Node が準備完了
overview-cluster-nodesNotAllowed = Node: 許可されていません
overview-cluster-pods = { $total } 個中 { $running } 個の Pod が実行中
overview-cluster-podsNotAllowed = Pod: 許可されていません
overview-cluster-notAllowed = 許可されていません。
overview-cluster-versionTitle = クラスター
overview-cluster-version = バージョン
overview-cluster-platform = プラットフォーム
overview-cluster-server = API サーバー
overview-nomad-region = リージョン
overview-nomad-address = アドレス
overview-cluster-namespace = Namespace
overview-cluster-allNamespaces = すべての Namespace
overview-cluster-nodesTitle = Node
overview-cluster-nodesReady = 準備完了
overview-cluster-allNodes = すべての Node
overview-cluster-podsTitle = Pod
overview-cluster-running = 実行中 { $count }
overview-cluster-pending = 保留中 { $count }
overview-cluster-crashing = 失敗中 { $count }
overview-cluster-done = 完了・その他 { $count }
overview-cluster-allPods = すべての Pod
overview-cluster-unreadyTitle = 準備ができていないワークロード
overview-cluster-allReady = すべての Deployment、StatefulSet、DaemonSet の Pod が準備完了です。
overview-cluster-warningsTitle = 最近の警告
overview-cluster-noWarnings = 警告イベントはありません。
overview-cluster-allEvents = すべてのイベント
overview-host-running = <strong>{ $running }</strong> / { $total } 実行中
overview-host-images = { $count ->
       *[other] { NUM($count) } 個のイメージ
    }
overview-host-containerdContainers = { $namespaces ->
       *[other] { NUM($namespaces) } 個の名前空間
    }に { $count ->
       *[other] { NUM($count) } 個のコンテナ
    }
overview-host-incusInstances = { $projects ->
       *[other] { NUM($projects) } 個のプロジェクト
    }に { $count ->
       *[other] { NUM($count) } 個のインスタンス
    }
overview-incus-title = Incus
overview-incus-version = バージョン
overview-incus-os = システム
overview-incus-arch = アーキテクチャ
overview-incus-projects = プロジェクト
overview-incus-shown = 表示中
overview-incus-holds = インスタンス { $instances ->
       *[other] { NUM($instances) } 個
    }、イメージ { $images ->
       *[other] { NUM($images) } 個
    }
overview-incus-instances = インスタンス
overview-incus-images = イメージ
overview-incus-storage = ストレージ
overview-containerd-title = containerd
overview-containerd-version = バージョン
overview-containerd-revision = リビジョン
overview-containerd-platform = プラットフォーム
overview-containerd-namespaces = 名前空間
overview-containerd-shown = 表示中
overview-containerd-holds = コンテナ { $containers ->
       *[other] { NUM($containers) } 個
    }、イメージ { $images ->
       *[other] { NUM($images) } 個
    }
overview-containerd-containers = コンテナ
overview-containerd-images = イメージ
overview-containerd-snapshots = スナップショット
overview-host-cpu = { $cpus ->
       *[other] { NUM($cpus) } 個の CPU
    }中 { $percent }
overview-host-cpuTitle = コンテナの CPU 使用率（CPU 1 個につき 100%）
overview-host-memoryTitle = コンテナのメモリ使用量
overview-engine-title = エンジン
overview-engine-name = 名前
overview-engine-version = バージョン
overview-engine-os = OS
overview-engine-kernel = カーネル
overview-engine-architecture = アーキテクチャ
overview-engine-resources = リソース
overview-engine-resourcesValue = { $cpus ->
       *[other] { NUM($cpus) } 個の CPU
    } · { $memory }
overview-engine-storageDriver = ストレージドライバー
overview-engine-rootDir = ルートディレクトリ
overview-disk-title = ディスク使用量
overview-disk-cleanUp = 未使用を削除…
overview-disk-column-type = 種類
overview-disk-column-total = 合計
overview-disk-column-active = 使用中
overview-disk-column-size = サイズ
overview-disk-column-reclaimable = 解放可能
overview-disk-images = イメージ
overview-disk-containers = コンテナ
overview-disk-volumes = ボリューム
overview-disk-buildCache = ビルドキャッシュ
overview-disk-partial = この Docker Engine はバージョン 29 より古く、報告される情報が少なくなります。イメージのサイズは推定値で、ボリュームとビルドキャッシュのサイズは取得できません。
overview-events-title = 最近のイベント
overview-events-all = すべてのイベント
overview-events-empty = Homeport が接続してから何も起きていません。
overview-prune-title = 未使用を削除
overview-prune-description = このホストから削除するものを選んでください。この操作は元に戻せません。
overview-prune-confirm = 未使用を削除
overview-prune-confirmWithVolumes = ボリュームのデータも含めて削除
overview-prune-containers = 停止中のコンテナ
overview-prune-containers-detail = 実行されていないコンテナをすべて削除します。
overview-prune-images = dangling イメージ
overview-prune-images-detail = ビルドやプルの後に残った、タグのないレイヤーです。
overview-prune-allImages = 未使用のすべてのイメージ
overview-prune-allImages-detail = どのコンテナも使っていないタグ付きイメージも削除します。
overview-prune-networks = 未使用のネットワーク
overview-prune-networks-detail = コンテナが接続されていないカスタムネットワークです。
overview-prune-buildCache = ビルドキャッシュ
overview-prune-buildCache-detail = キャッシュされたビルドレイヤーです。
overview-prune-volumes = 未使用のボリューム
overview-prune-volumes-detail = どのコンテナも使っていないボリュームのデータを、名前付きボリュームも含めてすべて削除します。
overview-pruned = コンテナ { $containers ->
       *[other] { NUM($containers) } 個
    }、イメージ { $images ->
       *[other] { NUM($images) } 個
    }、ネットワーク { $networks ->
       *[other] { NUM($networks) } 個
    }、ボリューム { $volumes ->
       *[other] { NUM($volumes) } 個
    }を削除し、{ $space } を解放しました。
overview-palette-prune = 選択したホストの未使用データを削除
overview-host-onBoard = ボードでこのホストのコンテナを表示
