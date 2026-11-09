---
title: 从源代码构建
description: 从源代码构建并运行 Homeport。
---

Homeport 是一个 [Tauri 2](https://tauri.app) 应用：在 Rust 核心之上构建的 React 和 TypeScript 界面，Rust 核心使用 [bollard](https://github.com/fussybeaver/bollard) 与 Docker Engine 通信，并将数据保存在 SQLite 中。

## 前提条件

- [Bun](https://bun.sh) 1.4 或更高版本
- [Rust](https://www.rust-lang.org/tools/install) 稳定版（仓库中的 `rust-toolchain.toml` 会选择所需组件）
- 适用于你的平台的 [Tauri 系统依赖](https://tauri.app/start/prerequisites/)。在 Debian 和 Ubuntu 上：

  ```sh
  sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev libsoup-3.0-dev librsvg2-dev \
    libayatana-appindicator3-dev libssl-dev
  ```

- 对于发布构建，还需要 [cargo-about](https://github.com/EmbarkStudios/cargo-about)，它用于生成第三方许可证声明：

  ```sh
  cargo install cargo-about --locked --features cli
  ```

## 运行

```sh
git clone https://github.com/DamienReichhart/Homeport.git
cd Homeport
bun install
bun run tauri:dev
```

`tauri:dev` 会启动应用，界面支持热重载；Rust 代码的更改会触发重新构建并重启应用。

## 构建安装程序

```sh
bun run tauri:build
```

适用于你的平台的安装程序会写入 `target/release/bundle/`。构建会先生成许可证声明，如果某个依赖的许可证尚未在 `about.toml` 中审查，构建就会停止。

本地构建没有为应用内更新程序签名，`.rpm` 也不会被签名；只有[发布工作流](https://github.com/DamienReichhart/Homeport/blob/main/docs/RELEASING.md)拥有签名密钥。你自己构建的副本仍会检查更新，但其**安装并重启**只会安装由发布工作流签名的版本。

## 常用命令

| 命令                  | 作用                                                     |
| --------------------- | -------------------------------------------------------- |
| `bun run check`       | 类型检查、lint、格式检查和前端测试                       |
| `bun run rust:lint`   | 将警告视为错误的 `cargo clippy`                          |
| `bun run rust:test`   | Rust 单元测试                                            |
| `bun run bindings`    | 重新生成 IPC 的 TypeScript 绑定                          |
| `bun run licenses`    | 生成第三方许可证声明                                     |
| `bun run screenshots` | 重新截取本网站使用的截图（`--locale fr` 只截取一种语言） |

集成测试会与本地 Docker Engine 和 Compose CLI 通信。它们被标记为 `#[ignore = "needs a local Docker Engine"]`（或 `"… and the compose CLI"`，少数为 `"… access to Docker Hub"`），只创建名为 `homeport-test-*` 的资源，并在之后删除它们。在本机上有引擎和 compose 插件时运行它们：

```sh
cargo test --workspace --exclude homeport -- --ignored
```

CI 也会运行它们，位于 `.github/workflows/ci.yml` 的 `engine-tests` job 中，每次推送到 `dev`、每个 pull request 和每次发布时都会运行。该 job 随后会运行 `scripts/ci/engine-test-leftovers.sh`，如果某个测试留下了 `homeport-test-*` 容器、镜像、卷或网络，它就会失败；在本地运行测试后也可以运行它做同样的检查。

## 本网站

网站位于 `website/` 中，使用 [Astro](https://astro.build) 和 [Starlight](https://starlight.astro.build) 构建：

```sh
cd website
bun install
bun run dev      # http://localhost:4321/Homeport/
bun run build    # checks types and links, then builds into website/dist/
```

每次推送到 `main` 且涉及网站时，以及每次发布之后，**Pages** 工作流都会将其部署到 GitHub Pages，以保证下载链接始终最新。
