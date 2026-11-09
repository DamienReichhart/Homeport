---
title: 参与贡献
description: 如何报告问题、建议功能以及为 Homeport 贡献代码。
---

Homeport 在 [github.com/DamienReichhart/Homeport](https://github.com/DamienReichhart/Homeport) 上公开开发。欢迎任何规模的贡献。

## 报告 bug

[创建 issue](https://github.com/DamienReichhart/Homeport/issues/new)，并附上：

- 你的 Homeport 版本和平台，可在**设置**中找到（例如 `v0.2.0 · macos/aarch64`）；
- 主机的类型（本地、SSH、TLS 或 TCP）及其引擎版本；
- 你做了什么、预期结果是什么、实际发生了什么，确切的消息最有帮助；
- [日志文件](/docs/reference/security/#日志)中的任何相关内容。

请通过 [GitHub 的漏洞报告](https://github.com/DamienReichhart/Homeport/security/advisories/new)私下报告安全问题。

## 建议功能

创建 issue，描述你希望解决的问题，而不仅仅是解决方案。了解你如何使用你的引擎，有助于为功能找到合适的形式。

## 贡献代码

1. 阅读 [`docs/ARCHITECTURE.md`](https://github.com/DamienReichhart/Homeport/blob/main/docs/ARCHITECTURE.md)。功能是自包含的垂直切片，前端只通过生成的 IPC 绑定调用 Rust，Rust 逻辑位于无需 Tauri 即可进行单元测试的服务中。
2. 在代码、界面文本和提交中使用 [`GLOSSARY.md`](https://github.com/DamienReichhart/Homeport/blob/main/GLOSSARY.md) 中的项目词汇：Host、Stack、Managed、External、Adopt。主要决策记录在 [`docs/adr`](https://github.com/DamienReichhart/Homeport/tree/main/docs/adr) 中。
3. 颜色和语气遵循[品牌指南](https://github.com/DamienReichhart/Homeport/blob/main/docs/BRAND.md)：朴实、具体、平和，使用英式拼写。
4. 确保以下命令通过：

   ```sh
   bun run check
   bun run rust:lint
   bun run rust:test
   ```

5. 提交信息遵循 [Conventional Commits](https://www.conventionalcommits.org)，例如 `feat(stacks): …`、`fix(hosts): …`。发布说明由它们生成。
6. 针对 `dev` 分支创建 pull request。CI 会在 Linux 上运行前端检查和许可证检查，并在 Linux、macOS 和 Windows 上运行 `cargo clippy` 和 `cargo test`，因此 `#[cfg(windows)]` 或 `#[cfg(target_os = "macos")]` 下的代码在合并前都会被编译。需要 Docker Engine 的测试被标记为 `#[ignore = "needs a local Docker Engine"]`；一个单独的 Linux job 会在 runner 的引擎上运行这些测试。

参见[从源代码构建](/docs/project/building/)来完成环境设置。

### 向应用添加文本

应用的文本保存在目录（catalogue）中，从不内联，以便翻译（[ADR 0011](https://github.com/DamienReichhart/Homeport/blob/main/docs/adr/0011-formatjs-catalogues-and-keyed-errors.md)）。添加字符串的步骤：

1. 在 [`src/shared/i18n/en/`](https://github.com/DamienReichhart/Homeport/tree/main/src/shared/i18n/en) 中该功能的文件里添加一个键：镜像功能用 `images.ts`，UI 组件库用 `ui.ts`，`common.ts` 只用于在任何地方含义都相同的词（Cancel、Close、Retry）。键以文件名开头，并说明文本的用途：`images.remove.title`、`images.column.size`。
2. 将值写成 [ICU 消息](https://formatjs.github.io/docs/core-concepts/icu-syntax/)：用 `{name}` 表示值，用 `{count, plural, one {# image} other {# images}}` 代替手动构造复数，用 `<strong>…</strong>` 表示强调（通过 `tRich` 渲染）。每个句子一个键；不要把片段拼接在一起，因为不同语言的词序不同。
3. 通过 `@/shared/i18n` 中的 `t('images.remove.title', { name })` 使用它，在组件、mutation 和 toast 中都一样。键是有类型的，因此拼写错误会导致类型检查失败。请使用字面量键：当文本取决于某个值时，将该值映射到键（`{ start: 'containers.done.start', stop: 'containers.done.stop' }`），而不是构造 `` `containers.done.${action}` ``。

当组件内联写入文本时，`bun run lint` 会失败：JSX 文本、供人阅读的属性（`title`、`aria-label`、`placeholder`、`description`、`label`…），以及 toast 和确认框的文本。读起来像代码而不是文字的字符串（`tcp`、`nginx:latest`、`/dev/dri`、`max-size=10m`）以及 Docker 或 Podman 等专有名词可以通过。如果有意保留字面量，请在上一行说明原因：`// eslint-disable-next-line homeport/no-literal-ui-string -- a shell command`。当代码中使用了目录中缺少的键、存在没有被使用的键，或在运行时构造键时，`bun run check` 也会失败。

错误消息在 Rust 中编写：每个 crate 在其 `messages!` 块中声明键和英文模板，`bun run bindings` 会将它们写入 `src/shared/i18n/en/errors.json`。不要手动编辑该文件。

## 翻译 Homeport

应用和网站支持英语、法语、西班牙语、德语、意大利语、巴西葡萄牙语、俄语、简体中文、日语、韩语、印地语和阿拉伯语。每种语言对每个产品术语只使用一个词：翻译前请在 [`docs/i18n/`](https://github.com/DamienReichhart/Homeport/tree/main/docs/i18n) 中查找，选定新词时请在那里添加一行。

### 添加或修改字符串

无论是什么字符串，在每种语言都具备它（包含相同的 `{arguments}` 和标签，以及该语言所需的每种复数形式）之前，`bun run check`（因而 CI）都会失败。

- **应用文本**：将键添加到 `src/shared/i18n/en/<namespace>.ts`（参见[向应用添加文本](#向应用添加文本)），然后添加到其他每个语言文件夹（`fr/`、`de/`、`zh-CN/`…）中的同名文件。能翻译就翻译。如果不能，请把英文文本复制过去，并在 pull request 中说明，以便译者跟进；绝不要遗漏该键。
- **错误消息或提示**：在 crate 的 `messages!` 块中声明它并运行 `bun run bindings`，它会写入 `en/errors.json`；然后将该键添加到每种语言的 `errors.json`。操作系统显示的文本（托盘菜单和通知）也以同样方式处理，使用 `native.` 键和 `native.json`。
- **网站文本**（首页、下载、更新日志、页眉和页脚）：将键添加到 `website/src/core/i18n/strings/en/` 中相应的文件（`site`、`home`、`download` 或 `changelog`），以及其他每种语言文件夹中的同名文件；缺少时 `astro check` 会失败。文档侧边栏标签放在 `website/src/app/config/sidebar/<code>.ts` 中。
- **文档页面**：英文页面位于 `website/src/content/docs/docs/`。修改英文页面时，请在同一个 pull request 中更新 `website/src/content/docs/fr/docs/` 下相同路径的法文页面。其他语言在有人翻译之前会显示英文页面，并附带一条说明。

使用 `bun run i18n:check --locale fr` 检查一种语言。

### 添加语言

需要添加一些文件，网站还需要一行代码：

1. **应用**：将 `src/shared/i18n/en/` 复制到 `src/shared/i18n/<tag>/`（BCP 47 标签，例如 `pl` 或 `zh-TW`），编写其 `meta.ts`（`{ name: 'Polski', dir: 'ltr' }`，即该语言对自身的称呼；从右到左书写的文字使用 `rtl`），并翻译每个文件，保留每个键，并像其他语言一样将每个 `export default { … }` 改为 `export default { … } satisfies Translation<typeof en>`。`bun run i18n:check --locale <tag>` 会列出缺少的内容。“设置”中会列出新语言，操作系统的语言会自动选中它，无需其他更改。
2. **网站**：将该语言添加到 `website/src/core/site/locales.ts`，然后在 `website/src/core/i18n/strings/en/` 的副本中（命名为 `<code>/`，并在 `core/i18n/index.ts` 的 `TABLES` 中注册）添加其字符串，在 `website/src/app/config/sidebar/<code>.ts` 中添加其侧边栏标签（在 `sidebar.ts` 中注册；这两行是添加一种语言唯一需要的代码，因为这些文件在 Vite 之外加载，而文件夹是由 Vite 发现的），并在 `website/src/content/i18n/<tag>.json` 中添加 Starlight 对未翻译页面的说明。其页面、URL、语言切换器条目、`hreflang` 链接和站点地图备用链接都会根据该列表自动生成。翻译后的文档放在 `website/src/content/docs/<code>/docs/` 下。
3. **截图**：`bun run screenshots --locale <tag>` 会以新语言截取每张截图，保存到 `website/src/assets/screenshots/<tag>/`；没有截图的页面会显示英文截图。
4. **术语**：添加 `docs/i18n/terms-<tag>.md`。

如果某个测试看不到你刚添加的语言，请删除 `node_modules/.vitest-cache`：Vitest 可能会保留旧的目录文件列表。

## 改进本文档

每个文档页面底部都有一个**编辑此页**链接。这些页面是 [`website/src/content/docs`](https://github.com/DamienReichhart/Homeport/tree/main/website/src/content/docs) 中的 Markdown 文件。

## 发布流程

每次推送到 `main` 且版本号更新时，都会为每个平台构建安装程序，运行完整的测试套件，并发布一个包含校验和与构建来源证明的 GitHub release。该流程在 [`docs/RELEASING.md`](https://github.com/DamienReichhart/Homeport/blob/main/docs/RELEASING.md) 中有说明。
