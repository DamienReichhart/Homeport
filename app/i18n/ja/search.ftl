### The `search` namespace, in 日本語 (ja).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/ja/search.ts:
### change the text there, then run it again.

search-kind-container = コンテナ
search-kind-stack = スタック
search-kind-image = イメージ
search-kind-volume = ボリューム
search-kind-network = ネットワーク
search-onHostState = { $host } · { $state ->
        [running] 実行中
       *[other] 停止
    }
search-showAll = { $count } 件をすべて表示
search-skipped = { $count ->
       *[other] { NUM($count) } 台のホスト
    }は検索されていません（未接続）
