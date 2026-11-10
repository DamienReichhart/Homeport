---
title: コントリビューション
description: 問題の報告、機能の提案、Homeport へのコードのコントリビューションの方法。
---

Homeport は [github.com/DamienReichhart/Homeport](https://github.com/DamienReichhart/Homeport) でオープンに開発されています。規模を問わず、コントリビューションを歓迎します。

## バグの報告

次の内容を添えて [issue を作成](https://github.com/DamienReichhart/Homeport/issues/new/choose)してください:

- 何をしたか、何を期待したか、何が起きたか（正確なメッセージが最も役に立ちます）
- **設定 → Homeport について → 診断情報をエクスポート…** で作成する[診断情報のバンドル](/docs/reference/troubleshooting/#exporting-diagnostics-for-a-bug-report)。Homeport のバージョンとプラットフォーム、使用しているホスト、エンジン、クラスターの種類、シークレットを取り除いたログを 1 つの zip にまとめたものです。保存する前に中身を確認でき、どこにも何も送信されません。

Homeport が起動しない場合は、代わりにそのバージョンとオペレーティングシステム、そして[ログファイル](/docs/reference/security/#ログ)の中の関連する部分を添えてください。

セキュリティの問題は、[GitHub の脆弱性報告](https://github.com/DamienReichhart/Homeport/security/advisories/new)を通じて非公開で報告してください。

## 機能の提案

解決策だけでなく、解決したい問題を説明する issue を作成してください。エンジンをどのように扱っているかがわかると、機能の適切な形を見つける助けになります。

## コードのコントリビューション

1. [`docs/ARCHITECTURE.md`](https://github.com/DamienReichhart/Homeport/blob/main/docs/ARCHITECTURE.md) を読んでください。機能は自己完結した縦割りのスライスで構成され、フロントエンドは生成された IPC バインディングを通じてのみ Rust を呼び出し、Rust のロジックは Tauri なしで単体テストされるサービスに置かれます。
2. コード、UI のテキスト、コミットでは、[`GLOSSARY.md`](https://github.com/DamienReichhart/Homeport/blob/main/GLOSSARY.md) にあるプロジェクトの用語（Host、Stack、Managed、External、Adopt）を使ってください。主な決定事項は [`docs/adr`](https://github.com/DamienReichhart/Homeport/tree/main/docs/adr) に記録されています。
3. 色と文体は[ブランドガイド](https://github.com/DamienReichhart/Homeport/blob/main/docs/BRAND.md)に従ってください。平易に、具体的に、落ち着いて、イギリス式の綴りで書きます。
4. 次が通ることを確認してください:

   ```sh
   bun run check
   bun run rust:lint
   bun run rust:test
   ```

5. データの約束を守ってください。Homeport のどのバージョンも、0.1.0 までさかのぼるすべての以前のバージョンのデータを開き、何も失わずにアップグレードします。ホスト、スタック、クラスター、設定、シークレットのすべてです。スキーマの変更は新しいマイグレーションとして行い、リリース済みのマイグレーションを編集してはいけません。CI は、すべてのリリースのサンプルデータベースに対してこれを確認します。[データの互換性](https://github.com/DamienReichhart/Homeport/blob/main/docs/RELEASING.md#data-compatibility)を参照してください。
6. コミットメッセージは [Conventional Commits](https://www.conventionalcommits.org) の形式（`feat(stacks): …`、`fix(hosts): …`）で書いてください。リリースノートはそこから生成されます。
7. `dev` ブランチに対してプルリクエストを作成してください。CI は Linux でフロントエンドのチェックとライセンスのチェックを、Linux、macOS、Windows で `cargo clippy` と `cargo test` を実行するため、`#[cfg(windows)]` や `#[cfg(target_os = "macos")]` の下のコードもマージ前にコンパイルされます。Docker Engine が必要なテストには `#[ignore = "needs a local Docker Engine"]` を付けます。別の Linux のジョブが、ランナーのエンジンに対してそれらを実行します。

セットアップについては[ソースからのビルド](/docs/project/building/)を参照してください。

### アプリにテキストを追加する

アプリのテキストは、翻訳できるように、インラインではなくカタログに置かれます（[ADR 0011](https://github.com/DamienReichhart/Homeport/blob/main/docs/adr/0011-formatjs-catalogues-and-keyed-errors.md)）。文字列を追加するには:

1. [`src/shared/i18n/en/`](https://github.com/DamienReichhart/Homeport/tree/main/src/shared/i18n/en) にある機能のファイルにキーを追加します。images 機能なら `images.ts`、UI キットなら `ui.ts`、どこでも同じ意味の言葉（Cancel、Close、Retry）だけなら `common.ts` です。キーはファイル名で始まり、テキストの用途を表します: `images.remove.title`、`images.column.size`。
2. 値は [ICU メッセージ](https://formatjs.github.io/docs/core-concepts/icu-syntax/)として書きます。値には `{name}`、複数形は手で組み立てずに `{count, plural, one {# image} other {# images}}`、強調には `tRich` で描画される `<strong>…</strong>` を使います。1 文につき 1 キーにし、断片をつなぎ合わせないでください。語順は言語によって変わるためです。
3. `@/shared/i18n` の `t('images.remove.title', { name })` で使います。コンポーネント、ミューテーション、トーストのどれでも同じです。キーには型が付いているため、打ち間違えると型チェックで失敗します。リテラルのキーを使ってください。テキストが値によって変わる場合は、`` `containers.done.${action}` `` を組み立てるのではなく、値をキーに対応付けます（`{ start: 'containers.done.start', stop: 'containers.done.stop' }`）。

コンポーネントがテキストをインラインで書くと、`bun run lint` が失敗します。対象は、JSX のテキスト、人が読む props（`title`、`aria-label`、`placeholder`、`description`、`label`…）、トーストと確認のテキストです。言葉ではなくコードとして読める文字列（`tcp`、`nginx:latest`、`/dev/dri`、`max-size=10m`）と、Docker や Podman などの固有名詞は通ります。意図的にリテラルを残すには、その前の行で理由を書きます: `// eslint-disable-next-line homeport/no-literal-ui-string -- a shell command`。`bun run check` は、コードで使われているのにカタログにないキー、何にも使われていないキー、実行時に組み立てられるキーでも失敗します。

エラーメッセージは Rust で書かれます。各クレートが `messages!` ブロックでキーと英語のテンプレートを宣言し、`bun run bindings` がそれらを `src/shared/i18n/en/errors.json` に書き出します。このファイルは手で編集しないでください。

## Homeport の翻訳

アプリと Web サイトは、英語、フランス語、スペイン語、ドイツ語、イタリア語、ブラジルポルトガル語、ロシア語、簡体字中国語、日本語、韓国語、ヒンディー語、アラビア語に対応しています。各言語は製品の用語ごとに 1 つの言葉を使います。翻訳する前に [`docs/i18n/`](https://github.com/DamienReichhart/Homeport/tree/main/docs/i18n) で調べ、新しい言葉を選んだときはそこに行を追加してください。

### 文字列の追加や変更

どの文字列でも、すべての言語に、同じ `{arguments}` とタグ、そしてその言語が必要とするすべての複数形がそろうまで、`bun run check`（したがって CI）は失敗します。

- **アプリのテキスト**: キーを `src/shared/i18n/en/<namespace>.ts` に追加し（[アプリにテキストを追加する](#アプリにテキストを追加する)を参照）、続いてほかのすべての言語フォルダー（`fr/`、`de/`、`zh-CN/`…）の同名のファイルにも追加します。できれば翻訳してください。できない場合は英語のテキストをそこにコピーし、翻訳者が後で対応できるよう、プルリクエストでその旨を書いてください。キーを省くことは決してしないでください。
- **エラーメッセージやヒント**: クレートの `messages!` ブロックで宣言して `bun run bindings` を実行すると、`en/errors.json` が書き出されます。その後、各言語の `errors.json` にキーを追加します。オペレーティングシステムが表示するテキスト（トレイのメニューと通知）も、`native.` キーと `native.json` で同じように扱います。
- **Web サイトのテキスト**（ホーム、ダウンロード、変更履歴、ヘッダーとフッター）: `website/src/core/i18n/strings/en/` の適切なファイル（`site`、`home`、`download`、`changelog`）と、ほかのすべての言語のフォルダーの同じファイルにキーを追加します。どれかに欠けていると `astro check` が失敗します。ドキュメントのサイドバーのラベルは `website/src/app/config/sidebar/<code>.ts` に置きます。
- **ドキュメントのページ**: 英語のページは `website/src/content/docs/docs/` にあります。変更したときは、同じプルリクエストで、`website/src/content/docs/<code>/docs/` の同じパスにある、すべての言語の同じページも更新してください。ページを更新できない言語では、翻訳者が更新するまで以前の翻訳が表示されます。翻訳したページの見出しは英語と同じ順序・同じレベルに保ち、リンクはアンカーも含めて英語の書き方のままにしてください。`bun run build` の後、`bun scripts/localize-anchors.ts <code>` が各アンカーを翻訳後の見出しに向け、2 回目のビルドのリンクチェックでそれが確認されます。

1 つの言語を確認するには `bun run i18n:check --locale fr` を使います。

### 言語の追加

必要なのはファイルと、Web サイト用の 1 行です:

1. **アプリ**: `src/shared/i18n/en/` を `src/shared/i18n/<tag>/`（`pl` や `zh-TW` などの BCP 47 タグ）にコピーし、その `meta.ts`（`{ name: 'Polski', dir: 'ltr' }`。その言語自身による言語名。右から左に書く文字では `rtl`）を書き、すべてのファイルを翻訳します。各キーはそのまま残し、ほかの言語と同様に、各 `export default { … }` を `export default { … } satisfies Translation<typeof en>` にします。`bun run i18n:check --locale <tag>` で不足しているものが一覧表示されます。ほかに何も変更しなくても、設定に新しい言語が表示され、OS の言語に応じて選ばれます。
2. **Web サイト**: `website/src/core/site/locales.ts` に言語を追加し、続いてその文字列を `website/src/core/i18n/strings/en/` のコピーである `<code>/` に（`core/i18n/index.ts` の `TABLES` に登録）、サイドバーのラベルを `website/src/app/config/sidebar/<code>.ts` に（`sidebar.ts` に登録。これらのファイルは、フォルダーを見つける役割の Vite の外で読み込まれるため、言語に必要なコードはこの 2 行だけです）、翻訳されていないページ用の Starlight の注記を `website/src/content/i18n/<tag>.json` に追加します。ページ、URL、言語切り替えの項目、`hreflang` のリンク、サイトマップの代替ページは、この一覧から自動で生成されます。ドキュメントは `website/src/content/docs/<code>/docs/` の下に置きます。そこにあるすべてのページを翻訳し（ないページは注記付きで英語が表示されます）、ビルドの後に `bun scripts/localize-anchors.ts <code>` を実行します。
3. **スクリーンショット**: `bun run screenshots` で、すべてのスクリーンショットを新しい言語も含むすべての言語で `website/src/assets/screenshots/<tag>/` に撮影し、各ページで読者の言語のアプリを表示します。英語にあるスクリーンショットがどれかの言語に欠けている間は、Web サイトの `bun test` が失敗します。
4. **用語**: `docs/i18n/terms-<tag>.md` を追加します。

追加したばかりの言語がテストで認識されない場合は、`node_modules/.vitest-cache` を削除してください。Vitest がカタログファイルの古い一覧を保持していることがあります。

## このドキュメントの改善

ドキュメントの各ページの下部には **ページを編集** のリンクがあります。ページは [`website/src/content/docs`](https://github.com/DamienReichhart/Homeport/tree/main/website/src/content/docs) にある Markdown ファイルです。

## リリースの流れ

新しいバージョン番号を含む `main` へのプッシュのたびに、すべてのプラットフォーム向けのインストーラーがビルドされ、テストスイート全体が実行され、チェックサムとビルド来歴付きの GitHub リリースが公開されます。手順は [`docs/RELEASING.md`](https://github.com/DamienReichhart/Homeport/blob/main/docs/RELEASING.md) で説明しています。
