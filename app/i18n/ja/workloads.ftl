### The `workloads` namespace, in 日本語 (ja).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/ja/workloads.ts:
### change the text there, then run it again.

workloads-nav = ワークロード
workloads-title = ワークロード
workloads-summary = { $count ->
       *[other] { NUM($count) } 個のワークロード
    }
workloads-search = ワークロードを検索
workloads-empty = ここには何もありません。
workloads-kind-deployment = Deployment
workloads-kind-statefulSet = StatefulSet
workloads-kind-daemonSet = DaemonSet
workloads-kind-job = Job
workloads-kind-cronJob = CronJob
workloads-one-deployment = Deployment
workloads-one-statefulSet = StatefulSet
workloads-one-daemonSet = DaemonSet
workloads-one-job = Job
workloads-one-cronJob = CronJob
workloads-column-name = 名前
workloads-column-namespace = Namespace
workloads-column-health = 正常性
workloads-column-ready = 準備完了
workloads-column-completions = 完了数
workloads-column-active = アクティブ
workloads-column-upToDate = 最新
workloads-column-schedule = スケジュール
workloads-column-lastSchedule = 前回の実行
workloads-column-images = イメージ
workloads-column-age = 経過時間
workloads-tab-overview = 概要
workloads-tab-pods = Pod
workloads-tab-jobs = Job
workloads-tab-logs = ログ
workloads-tab-conditions = 状態
workloads-tab-events = イベント
workloads-tab-source = YAML
workloads-detail-namespace = Namespace
workloads-detail-ready = 準備完了
workloads-detail-upToDate = 最新
workloads-detail-schedule = スケジュール
workloads-detail-lastSchedule = 前回の実行
workloads-detail-strategy = 戦略
workloads-detail-selector = Pod の選択条件
workloads-detail-owner = 制御元
workloads-detail-age = 経過時間
workloads-detail-labels = ラベル
workloads-detail-annotations = アノテーション
workloads-detail-containers = コンテナ
workloads-detail-noJobs = 起動した Job で、まだ保持されているものはありません。
workloads-container-name = コンテナ
workloads-container-image = イメージ
workloads-container-ports = ポート
workloads-action-menu = { $name } の操作
workloads-action-scale = スケール
workloads-action-restart = 再起動
workloads-action-trigger = 今すぐ実行
workloads-action-suspend = 一時停止
workloads-action-resume = 再開
workloads-scale-title = { $name } をスケール
workloads-scale-description = { $cluster } の Namespace { $namespace } 内。現在 { $current ->
       *[other] { NUM($current) } 個の Pod
    }を実行しています。
workloads-scale-replicas = Pod
workloads-scale-autoscaled = オートスケーラー { $autoscaler } が Pod を管理しているため、次回のチェックで数が元に戻されます。
workloads-scale-toZero = Pod が 0 個の場合、再びスケールアップするまでサービスを提供しません。
workloads-scale-submit = スケール
workloads-scale-done = { $name } を { $replicas ->
       *[other] { NUM($replicas) } 個の Pod
    }にスケールしました
workloads-scale-failed = Homeport は { $name } をスケールできませんでした
workloads-restart-title = { $name } を再起動しますか？
workloads-restart-description = { $cluster } の Namespace { $namespace } 内の Pod が、ロールアウト戦略に従って 1 つずつ置き換えられます。kubectl rollout restart と同じです。
workloads-restart-confirm = 再起動
workloads-restart-done = { $name } を再起動しています
workloads-restart-failed = Homeport は { $name } を再起動できませんでした
workloads-suspend-title = { $name } を一時停止しますか？
workloads-suspend-description = 再開するまで、{ $cluster } の Namespace { $namespace } で新しい Job は起動されません。すでに実行中の Job は続行されます。
workloads-suspend-confirm = 一時停止
workloads-suspend-done = { $name } を一時停止しました
workloads-suspend-failed = Homeport は { $name } を一時停止できませんでした
workloads-resume-done = { $name } を再開しました
workloads-resume-failed = Homeport は { $name } を再開できませんでした
workloads-trigger-done = Job { $job } を起動しました
workloads-trigger-failed = Homeport は { $name } から Job を起動できませんでした
