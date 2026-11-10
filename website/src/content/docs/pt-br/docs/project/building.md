---
title: Fazer build a partir do código-fonte
description: Faça a build e execute o Homeport a partir do código-fonte.
---

O Homeport é um aplicativo [Tauri 2](https://tauri.app): uma interface em React e TypeScript sobre um núcleo em Rust, que conversa com os Docker Engines com o [bollard](https://github.com/fussybeaver/bollard) e armazena os seus dados em SQLite.

## Pré-requisitos

- [Bun](https://bun.sh) 1.4 ou posterior
- [Rust](https://www.rust-lang.org/tools/install), stable (o `rust-toolchain.toml` do repositório escolhe os componentes)
- As [dependências de sistema do Tauri](https://tauri.app/start/prerequisites/) para a sua plataforma. No Debian e no Ubuntu:

  ```sh
  sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev libsoup-3.0-dev librsvg2-dev \
    libayatana-appindicator3-dev libssl-dev
  ```

- Para builds de release, o [cargo-about](https://github.com/EmbarkStudios/cargo-about), que gera os avisos de licença de terceiros:

  ```sh
  cargo install cargo-about --locked --features cli
  ```

## Executar

```sh
git clone https://github.com/DamienReichhart/Homeport.git
cd Homeport
bun install
bun run tauri:dev
```

`tauri:dev` inicia o aplicativo com hot reload para a interface; as mudanças em Rust refazem a build e o reiniciam.

## Gerar os instaladores

```sh
bun run tauri:build
```

Os instaladores para a sua plataforma são gravados em `target/release/bundle/`. A build gera primeiro os avisos de licença e para se a licença de uma dependência não tiver sido revisada em `about.toml`.

As builds locais não são assinadas para o atualizador do aplicativo e deixam o `.rpm` sem assinatura; só o [workflow de release](https://github.com/DamienReichhart/Homeport/blob/main/docs/RELEASING.md) tem as chaves de assinatura. Uma cópia cuja build você mesmo fez ainda verifica atualizações, mas o **Instalar e reiniciar** dela só instala versões assinadas pelo workflow de release.

## Comandos úteis

| Comando               | O que faz                                                                                                                                                                                                                                      |
| --------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `bun run check`       | Verificação de tipos, lint, verificação de formatação e testes do frontend                                                                                                                                                                     |
| `bun run rust:lint`   | `cargo clippy` com avisos tratados como erros                                                                                                                                                                                                  |
| `bun run rust:test`   | Testes unitários em Rust                                                                                                                                                                                                                       |
| `bun run bindings`    | Gera de novo os bindings TypeScript para o IPC                                                                                                                                                                                                 |
| `bun run licenses`    | Gera os avisos de licença de terceiros                                                                                                                                                                                                         |
| `bun run screenshots` | Captura de novo as capturas de tela usadas neste site, em todos os idiomas (`--locale fr` para refazer um), em 2880×1800, o dobro da densidade de pixels. Instale antes o oxipng (`cargo install oxipng`) para que sejam comprimidas sem perda |

Os testes de integração conversam com o Docker Engine local e com a CLI do Compose. Eles são marcados com `#[ignore = "needs a local Docker Engine"]` (ou `"… and the compose CLI"`, e alguns com `"… access to Docker Hub"`), criam apenas recursos chamados `homeport-test-*` e os removem depois. Execute-os com um Engine e o plugin compose nesta máquina:

```sh
cargo test --workspace --exclude homeport -- --ignored
```

A CI também os executa, no job `engine-tests` de `.github/workflows/ci.yml`, a cada push para `dev`, a cada pull request e a cada release. O job então executa `scripts/ci/engine-test-leftovers.sh`, que falha se um teste deixou para trás um contêiner, imagem, volume ou rede `homeport-test-*`; execute-o localmente depois dos testes para fazer a mesma verificação.

### Testes de ponta a ponta

`e2e/` controla o aplicativo de verdade — o backend Rust, o webview e as chamadas entre eles — com o [WebdriverIO](https://webdriver.io) por meio do `tauri-driver`, contra o Docker Engine local. Os cenários: o aplicativo inicia e se conecta ao Engine local; os logs de um contêiner são transmitidos e ele é parado pelo aplicativo; uma imagem é baixada; uma stack gerenciada é implantada com um ambiente criptografado; uma reinicialização mantém os hosts, a stack e o segredo dela; e um host SSH é adicionado com uma chave existente, com a chave do servidor confiada, e conectado. Eles encontram os elementos por papel, rótulo e texto, nunca por classe CSS.

Eles rodam apenas no Linux: o macOS não tem WebDriver para o seu webview. Além dos [pré-requisitos](#prerequisites) e de um Docker Engine, eles precisam do `tauri-driver` (`cargo install tauri-driver --locked`), do driver do WebKit, de uma tela virtual e de um chaveiro: no Debian e no Ubuntu, `webkit2gtk-driver`, `xvfb`, `dbus-x11` e `gnome-keyring`.

```sh
bun tauri build --debug --no-bundle
cd e2e
bun install
./run.sh
```

A build incorpora o frontend, como uma release faz, então um comando ausente de `commands::builder()`, uma permissão ausente de `src-tauri/capabilities/` ou um estilo que a Content Security Policy bloqueia falha aqui. O `run.sh` dá a cada execução uma pasta de dados nova, o seu próprio chaveiro e a sua própria tela, para nunca mexer no seu Homeport. Passe as opções do WebdriverIO depois dele, por exemplo `./run.sh --spec specs/ssh.e2e.ts`. O cenário SSH faz a build de uma pequena imagem de servidor SSH a partir de `e2e/support/sshd/`; defina `HOMEPORT_E2E_SSHD_IMAGE` para usar uma com build feita antes. Os contêineres que os testes iniciam se chamam `homeport-e2e-*` e são removidos depois.

Uma falha deixa uma captura de tela e o log do aplicativo em `e2e/artifacts/`. A CI executa os testes no job `e2e` de `.github/workflows/ci.yml`, a cada push para `dev` e a cada pull request, e mantém essa pasta quando um falha.

## Este site

O site fica em `website/` e é construído com [Astro](https://astro.build) e [Starlight](https://starlight.astro.build):

```sh
cd website
bun install
bun run dev      # http://localhost:4321/Homeport/
bun run build    # checks types and links, then builds into website/dist/
```

Ele é implantado no GitHub Pages pelo workflow **Pages** a cada push para `main` que o altera, e depois de cada release, para que os links de download continuem atualizados.
