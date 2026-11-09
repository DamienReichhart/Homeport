---
title: Сборка из исходного кода
description: Сборка и запуск Homeport из исходного кода.
---

Homeport — приложение на [Tauri 2](https://tauri.app): интерфейс на React и TypeScript поверх ядра на Rust, которое общается с движками Docker через [bollard](https://github.com/fussybeaver/bollard) и хранит свои данные в SQLite.

## Что нужно заранее

- [Bun](https://bun.sh) 1.4 или новее
- [Rust](https://www.rust-lang.org/tools/install), стабильная версия (компоненты выбирает файл `rust-toolchain.toml` репозитория)
- [Системные зависимости Tauri](https://tauri.app/start/prerequisites/) для вашей платформы. В Debian и Ubuntu:

  ```sh
  sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev libsoup-3.0-dev librsvg2-dev \
    libayatana-appindicator3-dev libssl-dev
  ```

- Для релизных сборок — [cargo-about](https://github.com/EmbarkStudios/cargo-about), который формирует лицензионные уведомления сторонних компонентов:

  ```sh
  cargo install cargo-about --locked --features cli
  ```

## Запуск

```sh
git clone https://github.com/DamienReichhart/Homeport.git
cd Homeport
bun install
bun run tauri:dev
```

`tauri:dev` запускает приложение с горячей перезагрузкой интерфейса; изменения в Rust вызывают пересборку и перезапуск.

## Сборка установщиков

```sh
bun run tauri:build
```

Установщики для вашей платформы записываются в `target/release/bundle/`. Сборка сначала формирует лицензионные уведомления и останавливается, если лицензия какой-либо зависимости не проверена в `about.toml`.

Локальные сборки не подписаны для встроенного средства обновления, а `.rpm` остаётся неподписанным; ключи подписи есть только у [процесса выпуска](https://github.com/DamienReichhart/Homeport/blob/main/docs/RELEASING.md). Собранная вами копия всё равно проверяет обновления, но её кнопка **Установить и перезапустить** устанавливает только версии, подписанные процессом выпуска.

## Полезные команды

| Команда               | Что делает                                                                                                 |
| --------------------- | ---------------------------------------------------------------------------------------------------------- |
| `bun run check`       | Проверка типов, линтер, проверка форматирования и тесты фронтенда                                          |
| `bun run rust:lint`   | `cargo clippy` с предупреждениями как ошибками                                                             |
| `bun run rust:test`   | Модульные тесты Rust                                                                                       |
| `bun run bindings`    | Пересоздаёт привязки TypeScript для IPC                                                                    |
| `bun run licenses`    | Формирует лицензионные уведомления сторонних компонентов                                                   |
| `bun run screenshots` | Заново снимает скриншоты, используемые на этом сайте, на всех языках (`--locale fr`, чтобы переснять один) |

Интеграционные тесты обращаются к локальному Docker Engine и CLI Compose. Они помечены `#[ignore = "needs a local Docker Engine"]` (или `"… and the compose CLI"`, а некоторые — `"… access to Docker Hub"`), создают только ресурсы с именами `homeport-test-*` и затем удаляют их. Запускайте их при наличии движка и плагина compose на этом компьютере:

```sh
cargo test --workspace --exclude homeport -- --ignored
```

CI тоже запускает их — в задании `engine-tests` файла `.github/workflows/ci.yml` — при каждой отправке в `dev`, каждом pull request и каждом релизе. Затем задание запускает `scripts/ci/engine-test-leftovers.sh`, который завершается ошибкой, если какой-либо тест оставил после себя контейнер, образ, том или сеть `homeport-test-*`; запустите его локально после тестов, чтобы проверить то же самое.

## Этот сайт

Сайт находится в `website/` и собирается с помощью [Astro](https://astro.build) и [Starlight](https://starlight.astro.build):

```sh
cd website
bun install
bun run dev      # http://localhost:4321/Homeport/
bun run build    # checks types and links, then builds into website/dist/
```

Он публикуется на GitHub Pages процессом **Pages** при каждой отправке в `main`, затрагивающей его, и после каждого релиза, чтобы ссылки на загрузку оставались актуальными.
