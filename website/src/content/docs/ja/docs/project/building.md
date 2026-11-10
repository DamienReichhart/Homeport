---
title: ソースからのビルド
description: ソースコードから Homeport をビルドして実行します。
---

Homeport は [Tauri 2](https://tauri.app) のアプリです。Rust のコアの上に React と TypeScript のインターフェースがあり、コアは [bollard](https://github.com/fussybeaver/bollard) で Docker Engine と通信し、データを SQLite に保存します。

## 前提条件

- [Bun](https://bun.sh) 1.4 以降
- [Rust](https://www.rust-lang.org/tools/install) の stable 版（コンポーネントはリポジトリの `rust-toolchain.toml` が選びます）
- お使いのプラットフォーム向けの [Tauri のシステム依存関係](https://tauri.app/start/prerequisites/)。Debian と Ubuntu では:

  ```sh
  sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev libsoup-3.0-dev librsvg2-dev \
    libayatana-appindicator3-dev libssl-dev
  ```

- リリースビルドには、サードパーティのライセンス表記を生成する [cargo-about](https://github.com/EmbarkStudios/cargo-about):

  ```sh
  cargo install cargo-about --locked --features cli
  ```

## 実行する

```sh
git clone https://github.com/DamienReichhart/Homeport.git
cd Homeport
bun install
bun run tauri:dev
```

`tauri:dev` は、インターフェースのホットリロードを有効にしてアプリを起動します。Rust を変更すると、再ビルドして再起動します。

## インストーラーをビルドする

```sh
bun run tauri:build
```

お使いのプラットフォーム向けのインストーラーが `target/release/bundle/` に書き出されます。ビルドはまずライセンス表記を生成し、依存関係のライセンスが `about.toml` でレビューされていない場合は中止します。

ローカルのビルドは、アプリ内アップデーター向けに署名されず、`.rpm` も署名されないままです。署名鍵を持っているのは[リリースワークフロー](https://github.com/DamienReichhart/Homeport/blob/main/docs/RELEASING.md)だけです。自分でビルドしたコピーもアップデートを確認しますが、その **インストールして再起動** がインストールするのは、リリースワークフローによって署名されたバージョンだけです。

## 便利なコマンド

| コマンド              | 内容                                                                                                                                                                                                                                  |
| --------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `bun run check`       | 型チェック、lint、フォーマットチェック、フロントエンドのテスト                                                                                                                                                                        |
| `bun run rust:lint`   | 警告をエラーとして扱う `cargo clippy`                                                                                                                                                                                                 |
| `bun run rust:test`   | Rust の単体テスト                                                                                                                                                                                                                     |
| `bun run bindings`    | IPC 用の TypeScript バインディングを再生成                                                                                                                                                                                            |
| `bun run licenses`    | サードパーティのライセンス表記を生成                                                                                                                                                                                                  |
| `bun run screenshots` | この Web サイトで使うスクリーンショットをすべての言語で撮り直す（1 言語だけ撮り直すなら `--locale fr`）。サイズは 2880×1800 で、ピクセル密度は 2 倍。可逆圧縮されるよう、先に oxipng をインストールしておく（`cargo install oxipng`） |

統合テストは、ローカルの Docker Engine と Compose CLI と通信します。これらのテストには `#[ignore = "needs a local Docker Engine"]`（または `"… and the compose CLI"`、一部は `"… access to Docker Hub"`）が付いており、`homeport-test-*` という名前のリソースだけを作成し、終了後に削除します。このマシンにエンジンと compose プラグインがある状態で実行してください:

```sh
cargo test --workspace --exclude homeport -- --ignored
```

CI でも、`.github/workflows/ci.yml` の `engine-tests` ジョブで、`dev` へのすべてのプッシュ、すべてのプルリクエスト、すべてのリリースのたびに実行されます。その後、このジョブは `scripts/ci/engine-test-leftovers.sh` を実行し、テストが `homeport-test-*` のコンテナ、イメージ、ボリューム、ネットワークを残していれば失敗します。同じことを確認するには、テストの後にローカルで実行してください。

## この Web サイト

サイトは `website/` にあり、[Astro](https://astro.build) と [Starlight](https://starlight.astro.build) でビルドされています:

```sh
cd website
bun install
bun run dev      # http://localhost:4321/Homeport/
bun run build    # checks types and links, then builds into website/dist/
```

サイトに関係する `main` へのプッシュのたびに、そしてダウンロードのリンクを最新に保つためにリリースのたびに、**Pages** ワークフローによって GitHub Pages にデプロイされます。
