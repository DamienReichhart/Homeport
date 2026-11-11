### The `containerd` namespace, in 日本語 (ja).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/ja/containerd.ts:
### change the text there, then run it again.

containerd-state-none = タスクなし
containerd-state-created = 作成済み
containerd-state-running = 実行中
containerd-state-paused = 一時停止
containerd-state-pausing = 一時停止中
containerd-state-stopped = 停止
containerd-state-unknown = 不明
containerd-state-exited = 終了（{ $code }）
containerd-createdBy-kubernetes = Kubernetes
containerd-createdBy-nerdctl = nerdctl
containerd-createdBy-docker = Docker
containerd-createdBy-other = ctr またはその他
containerd-containers-nav = コンテナ
containerd-containers-title = コンテナ
containerd-containers-summary = 名前空間 { $namespace } に { $count ->
       *[other] { NUM($count) } 個のコンテナ
    }、うち { $running } 個が実行中。
containerd-containers-search = 名前、ID、イメージで絞り込み
containerd-containers-showSandboxes = Pod の pause コンテナを表示
containerd-containers-sandbox = pause
containerd-containers-empty = { $namespace } にコンテナはありません
containerd-containers-emptyDescription = nerdctl、ctr、Kubernetes で作成されたコンテナがここに表示されます。ほかのものを見るには、ホストの切り替えメニューの下で別の名前空間を選んでください。
containerd-containers-column-name = 名前
containerd-containers-column-image = イメージ
containerd-containers-column-state = タスク
containerd-containers-column-createdBy = 作成元
containerd-containers-column-created = 作成日時
containerd-containers-action-start = 起動
containerd-containers-action-stop = 停止
containerd-containers-action-pause = 一時停止
containerd-containers-action-resume = 再開
containerd-containers-action-kill = シグナルを送信
containerd-containers-actionOne-start = { $name } を起動
containerd-containers-actionOne-stop = { $name } を停止
containerd-containers-actionOne-pause = { $name } を一時停止
containerd-containers-actionOne-resume = { $name } を再開
containerd-containers-removeOne = { $name } を削除
containerd-containers-done-start = { $name } を起動しました
containerd-containers-done-stop = { $name } を停止しました
containerd-containers-done-pause = { $name } を一時停止しました
containerd-containers-done-resume = { $name } を再開しました
containerd-containers-done-kill = { $name } に { $signal } を送信しました
containerd-containers-removed = { $name } を削除しました
containerd-containers-kubelet = このコンテナは Kubernetes が管理しています。停止すると kubelet が再び起動し、削除すると再び作成します。代わりにクラスターでその Pod を変更してください。
containerd-containers-remove-title = { $name } を削除しますか？
containerd-containers-remove-running = タスクが実行中です。Homeport はタスクを強制終了してから、タスクとコンテナを削除します。
containerd-containers-remove-stopped = Homeport はタスクとコンテナを削除します。
containerd-containers-remove-snapshot = スナップショット（{ $key }）も、変更されたファイルごと削除する
containerd-containers-remove-killAndRemove = 強制終了して削除
containerd-containers-kill-title = { $name } にシグナルを送信
containerd-containers-kill-description = ctr task kill -s と同じように、タスクのメインプロセスに送信します。SIGHUP のような名前、または番号で指定します。
containerd-containers-kill-signal = シグナル
containerd-containers-kill-send = 送信
containerd-containers-backToList = コンテナに戻る
containerd-containers-tab-overview = 概要
containerd-containers-tab-logs = ログ
containerd-containers-tab-exec = Exec
containerd-containers-tab-spec = Spec
containerd-logs-from = 読み込み元: { $path }
containerd-logs-none-title = このコンテナの出力は記録されていません
containerd-logs-none-description = containerd はログを保持しません。出力の行き先は、タスクを起動したツールが決めます。ctr やほとんどのクライアントで作成したコンテナは出力をどこにも送りません。nerdctl と Kubernetes はファイルに書き込み、Homeport はそれを読み込みます。
containerd-logs-driver-title = nerdctl はこのコンテナの出力を { $driver } に送っています
containerd-logs-driver-description = Homeport が読み込めるのは nerdctl のデフォルトの json-file ログだけです。{ $driver } が保管している場所で読むか、json-file ドライバーでコンテナを再び実行してください。
containerd-exec-startFirst = ターミナルを開くには、コンテナを起動してください。
containerd-exec-command = コマンド
containerd-exec-commandPlaceholder = コマンドを指定しなければシェル
containerd-exec-user = ユーザー ID
containerd-exec-workingDir = フォルダー
containerd-exec-open = 再び開く
containerd-exec-exited = プロセスが終了しました。
containerd-exec-exitedWithCode = プロセスがコード { $code } で終了しました。
containerd-containers-field-id = ID
containerd-containers-field-namespace = 名前空間
containerd-containers-field-image = イメージ
containerd-containers-field-runtime = ランタイム
containerd-containers-field-snapshot = スナップショット
containerd-containers-field-pid = PID
containerd-containers-field-exit = 前回の終了
containerd-containers-field-pod = Kubernetes Pod
containerd-containers-field-created = 作成日時
containerd-containers-field-updated = 更新日時
containerd-containers-field-extensions = 拡張
containerd-containers-labels = ラベル
containerd-containers-noSpec = containerd はこのコンテナのランタイム spec を保持していません。
containerd-images-nav = イメージ
containerd-images-title = イメージ
containerd-images-summary = 名前空間 { $namespace } に { $count ->
       *[other] { NUM($count) } 個のイメージ
    }。
containerd-images-pull = イメージをプル
containerd-images-pulled = { $reference } を { $namespace } にプルしました
containerd-images-removed = { $name } を削除しました
containerd-images-search = 名前で絞り込み
containerd-images-empty = { $namespace } にイメージはありません
containerd-images-emptyDescription = イメージをプルしてください。containerd はイメージを取得し、コンテナを実行できるように、このホストのプラットフォーム向けに展開します。
containerd-images-column-name = 名前
containerd-images-column-platforms = プラットフォーム
containerd-images-column-size = サイズ
containerd-images-column-containers = コンテナ
containerd-images-column-created = 作成日時
containerd-images-sizeUnknown = このホストのプラットフォーム向けの内容が、すべてそろってはいません。
containerd-images-removeOne = { $name } を削除
containerd-images-inUse = この名前空間のコンテナがこのイメージから作成されています。先にそれらを削除してください。
containerd-images-remove-title = { $name } を削除しますか？
containerd-images-remove-description = containerd は名前を削除し、ほかに保持しているものがなくなった時点で、その内容とスナップショットを解放します。
containerd-snapshots-nav = スナップショット
containerd-snapshots-title = スナップショット
containerd-snapshots-summary = { $namespace } の { $count ->
       *[other] { NUM($count) } 個のスナップショット
    }が { $total } を使用しています。{ $unused ->
        [0] 未使用のものはありません
       *[other] { NUM($unused) } 個が未使用です
    }（{ $unusedSize }）。
containerd-snapshots-snapshotter = スナップショッター
containerd-snapshots-unusedOnly = 未使用のみ
containerd-snapshots-unused = 未使用
containerd-snapshots-usedByContainer = コンテナ { $id }
containerd-snapshots-usedByImage = イメージ { $name }
containerd-snapshots-empty = { $snapshotter } にスナップショットはありません
containerd-snapshots-emptyDescription = スナップショットは、展開されたイメージのレイヤーと、コンテナの書き込み可能なレイヤーです。
containerd-snapshots-column-key = キー
containerd-snapshots-column-kind = 種類
containerd-snapshots-column-size = サイズ
containerd-snapshots-column-usedBy = 使用元
containerd-snapshots-kind-committed = レイヤー
containerd-snapshots-kind-active = 書き込み可能
containerd-snapshots-kind-view = ビュー
containerd-snapshots-kind-unknown = 不明
containerd-snapshots-removeOne = { $key } を削除
containerd-snapshots-removed = スナップショット { $key } を削除しました
containerd-snapshots-inUse = 使用中か、これを基にしたものがあります。先にコンテナまたはイメージを削除してください。
containerd-snapshots-remove-title = スナップショット { $key } を削除しますか？
containerd-snapshots-remove-description = 使用しているものはありません。そのファイル（{ $size }）がホストから削除されます。
containerd-namespaces-nav = 名前空間
containerd-namespaces-title = 名前空間
containerd-namespaces-summary = { $count ->
       *[other] { NUM($count) } 個の名前空間
    }。containerd のオブジェクトはすべていずれかの名前空間に属します。nerdctl は default、Kubernetes は k8s.io、Docker は moby を使います。
containerd-namespaces-new = 新しい名前空間
containerd-namespaces-created = 名前空間 { $name } を作成しました
containerd-namespaces-removed = 名前空間 { $name } を削除しました
containerd-namespaces-shown = 表示中
containerd-namespaces-show = 表示
containerd-namespaces-removeOne = { $name } を削除
containerd-namespaces-notEmpty = 削除できるのは空の名前空間だけです。先にそのコンテナとイメージを削除してください。
containerd-namespaces-remove-title = 名前空間 { $name } を削除しますか？
containerd-namespaces-remove-description = 件数に表れないスナップショットや内容がまだ残っている場合、containerd は削除を拒否します。
containerd-namespaces-column-name = 名前
containerd-namespaces-column-containers = コンテナ
containerd-namespaces-column-images = イメージ
containerd-namespaces-column-labels = ラベル
containerd-namespaces-empty = 名前空間はありません
containerd-namespaces-emptyDescription = containerd は、初めて何かが作成されるときに default を作成します。
containerd-namespaces-newDialog-title = 新しい名前空間
containerd-namespaces-newDialog-description = コンテナとイメージをほかと分けておくための、空の名前空間です。
containerd-namespaces-newDialog-name = 名前
containerd-namespaces-newDialog-invalid = 英字と数字を使い、区切りには単独のドット、ハイフン、アンダースコアを使ってください。
containerd-namespaces-newDialog-create = 作成
