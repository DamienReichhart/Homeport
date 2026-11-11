### The `repositories` namespace, in 日本語 (ja).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/ja/repositories.ts:
### change the text there, then run it again.

repositories-title = リポジトリ
repositories-description = すべてのホストで、同期スタックのデプロイ元になる Git リポジトリです。Homeport は読み取りだけを行います。
repositories-add = リポジトリを追加
repositories-empty = まだリポジトリはありません。同期スタックを作成するには、リポジトリを追加してください。
repositories-manageOne = { $repository } へのアクセスを管理
repositories-removeOne = { $repository } を削除
repositories-remove-title = { $repository } を削除しますか？
repositories-remove-description = Homeport はデプロイキーまたはトークンを破棄し、このコンピューター上のクローンを削除します。リポジトリ自体には手を加えません。フォージ側でもデプロイキーを削除してください。
repositories-noKeychain = デプロイキーとトークンは OS のキーチェーンに保管された鍵で暗号化されますが、利用できるキーチェーンがないため、公開リポジトリしか追加できません。
repositories-public = 公開リポジトリです。Homeport は認証情報なしで読み取ります。
repositories-added = { $repository } を追加しました
repositories-removed = { $repository } を削除しました
repositories-keyRotated = 新しいデプロイキーを作成しました。古いキーの代わりにリポジトリに追加してください。
repositories-tokenReplaced = トークンを置き換えました
repositories-hostKeyReset = Homeport は、次回サーバーが提示するキーを信頼します。
repositories-auth-withKey = デプロイキーで接続（{ $host }）
repositories-auth-withToken = { $username } としてトークンで接続（{ $host }）
repositories-auth-public = 公開（{ $host }）
repositories-dialog-addTitle = リポジトリを追加
repositories-dialog-description = Homeport は、同期スタックをデプロイするためにこのコンピューターにクローンします。コミットやプッシュは一切行いません。
repositories-dialog-url = リポジトリの URL
repositories-dialog-auth = アクセス
repositories-dialog-authKey = デプロイキー
repositories-dialog-authToken = トークン
repositories-dialog-authNone = 公開
repositories-dialog-authKeyHelp = Homeport はこのリポジトリ専用のキーを作成します。読み取りアクセスでリポジトリに追加してください。ご自身の SSH キーが使われることはありません。
repositories-dialog-authTokenHelp = リポジトリの内容を読み取れるアクセストークンで、HTTPS で送信されます。このコンピューター上で暗号化され、二度と表示されません。
repositories-dialog-authNoneHelp = HTTPS 経由の公開リポジトリ用です。何も保存されません。
repositories-dialog-username = ユーザー名
repositories-dialog-token = アクセストークン
repositories-dialog-add = 追加
repositories-dialog-addWithKey = 追加してデプロイキーを作成
repositories-missing-url = リポジトリの URL を入力してください。
repositories-missing-validUrl = git@github.com:owner/repo.git や https://gitlab.com/owner/repo.git のような SSH または HTTPS の URL を入力してください。
repositories-missing-auth = Homeport がこのリポジトリにアクセスする方法を選んでください。
repositories-missing-username = トークンの所有者のユーザー名を入力してください。
repositories-missing-token = アクセストークンを入力してください。
repositories-deployKey-title = デプロイキー
repositories-deployKey-github = GitHub: リポジトリの <strong>Settings → Deploy keys → Add deploy key</strong>。
repositories-deployKey-gitlab = GitLab: プロジェクトの <strong>Settings → Repository → Deploy keys</strong>。
repositories-deployKey-gitea = Gitea と Forgejo: リポジトリの <strong>Settings → Deploy keys → Add deploy key</strong>。
repositories-deployKey-readOnly = 書き込みアクセスはオフのままにしてください。Homeport は読み取りしか行いません。その後、下でアクセスをテストしてください。
repositories-deployKey-fingerprint = フィンガープリント: <mono>{ $fingerprint }</mono>
repositories-rotate-button = 新しいキー
repositories-rotate-title = 新しいデプロイキーを作成しますか？
repositories-rotate-description = 現在のキーは Homeport で使えなくなります。新しいキーをリポジトリに追加し、古いキーをリポジトリから削除してください。
repositories-rotate-confirm = 新しいキーを作成
repositories-token-replace = トークンを置き換え
repositories-token-save = トークンを保存
repositories-hostKey-trusted = { $host } の信頼済みキー: <mono>{ $fingerprint }</mono>
repositories-hostKey-reset = リセット
repositories-hostKey-resetTitle = { $host } の信頼済みキーをリセットしますか？
repositories-hostKey-resetDescription = { $host } が SSH キーを変更し、新しいフィンガープリントをその Web サイトで確認した場合にのみ行ってください。Homeport は、次に提示されるキーをそのまま信頼します。
repositories-test-button = アクセスをテスト
repositories-test-testing = テスト中…
repositories-test-success = Homeport は読み取れます: { $branches ->
       *[other] { NUM($branches) } 個のブランチ
    }、{ $tags ->
       *[other] { NUM($tags) } 個のタグ
    }。
