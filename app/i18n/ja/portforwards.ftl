### The `portforwards` namespace, in 日本語 (ja).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/ja/portforwards.ts:
### change the text there, then run it again.

portforwards-nav = ポートフォワード
portforwards-title = ポートフォワード
portforwards-summary = { $count ->
       *[other] { NUM($count) } 件のフォワード
    }
portforwards-empty-title = ポートフォワードはありません
portforwards-empty-description = Pod や Service のページからポートをフォワードすると、このコンピューターからアクセスできます。フォワードは Homeport を終了すると終わります。
portforwards-column-local = ローカルアドレス
portforwards-column-target = フォワード先
portforwards-column-namespace = Namespace
portforwards-column-cluster = クラスター
portforwards-column-connections = 接続数
portforwards-forward = フォワード…
portforwards-forwardPort = ポート { $port } をフォワード
portforwards-open = 開く
portforwards-stop = 停止
portforwards-stopFailed = Homeport はフォワードを停止できませんでした
portforwards-started = { $address } を { $target }:{ $port } にフォワードしています
portforwards-failed = ポートフォワードが失敗しました
portforwards-ended = ポートフォワードが終了しました
portforwards-dialog-title = { $target }:{ $port } をフォワード
portforwards-dialog-description = Namespace { $namespace } 内。フォワードの実行中は、ローカルポートでアクセスできます。
portforwards-dialog-localPort = ローカルポート
portforwards-dialog-anyPort = 空いている任意のポート
portforwards-dialog-loopback = 接続できるのはこのコンピューターだけです。フォワードは 127.0.0.1（と ::1）で待ち受けます。
portforwards-dialog-submit = フォワード
