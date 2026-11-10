---
title: 소스에서 빌드
description: 소스 코드로 Homeport를 빌드하고 실행합니다.
---

Homeport는 [Tauri 2](https://tauri.app) 앱입니다. Rust 코어 위에 React와 TypeScript 인터페이스가 있으며, 코어는 [bollard](https://github.com/fussybeaver/bollard)로 Docker Engine과 통신하고 데이터를 SQLite에 저장합니다.

## 사전 요구 사항

- [Bun](https://bun.sh) 1.4 이상
- [Rust](https://www.rust-lang.org/tools/install) stable(구성 요소는 저장소의 `rust-toolchain.toml`이 지정합니다)
- 플랫폼별 [Tauri 시스템 의존성](https://tauri.app/start/prerequisites/). Debian과 Ubuntu에서는 다음과 같습니다.

  ```sh
  sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev libsoup-3.0-dev librsvg2-dev \
    libayatana-appindicator3-dev libssl-dev
  ```

- 릴리스 빌드에는 서드파티 라이선스 고지를 생성하는 [cargo-about](https://github.com/EmbarkStudios/cargo-about)이 필요합니다.

  ```sh
  cargo install cargo-about --locked --features cli
  ```

## 실행

```sh
git clone https://github.com/DamienReichhart/Homeport.git
cd Homeport
bun install
bun run tauri:dev
```

`tauri:dev`는 인터페이스에 핫 리로드를 적용하여 앱을 시작합니다. Rust 코드를 바꾸면 다시 빌드하고 다시 시작합니다.

## 설치 프로그램 빌드

```sh
bun run tauri:build
```

현재 플랫폼용 설치 프로그램이 `target/release/bundle/`에 만들어집니다. 빌드는 먼저 라이선스 고지를 생성하며, 의존성의 라이선스가 `about.toml`에서 검토되지 않았으면 중단됩니다.

로컬 빌드는 앱 내 업데이터용으로 서명되지 않으며 `.rpm`도 서명되지 않은 채로 남습니다. 서명 키는 [릴리스 워크플로](https://github.com/DamienReichhart/Homeport/blob/main/docs/RELEASING.md)에만 있습니다. 직접 빌드한 사본도 업데이트를 확인하지만, **설치 후 다시 시작**은 릴리스 워크플로가 서명한 버전만 설치합니다.

## 유용한 명령

| 명령                  | 하는 일                                                                                                                                                                                                  |
| --------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `bun run check`       | 타입 검사, 린트, 포맷 검사, 프런트엔드 테스트                                                                                                                                                            |
| `bun run rust:lint`   | 경고를 오류로 처리하는 `cargo clippy`                                                                                                                                                                    |
| `bun run rust:test`   | Rust 단위 테스트                                                                                                                                                                                         |
| `bun run bindings`    | IPC용 TypeScript 바인딩 다시 생성                                                                                                                                                                        |
| `bun run licenses`    | 서드파티 라이선스 고지 생성                                                                                                                                                                              |
| `bun run screenshots` | 이 웹사이트에 쓰이는 스크린숏을 모든 언어로 다시 캡처(언어 하나만 다시 캡처하려면 `--locale fr`). 크기는 2880×1800으로 픽셀 밀도의 두 배. 무손실로 압축되도록 먼저 oxipng를 설치(`cargo install oxipng`) |

통합 테스트는 로컬 Docker Engine 및 Compose CLI와 통신합니다. 이 테스트에는 `#[ignore = "needs a local Docker Engine"]`(또는 `"… and the compose CLI"`, 일부는 `"… access to Docker Hub"`) 표시가 붙어 있으며, `homeport-test-*`라는 이름의 리소스만 만들고 끝나면 삭제합니다. 이 컴퓨터에 엔진과 compose 플러그인을 두고 실행하세요.

```sh
cargo test --workspace --exclude homeport -- --ignored
```

CI도 `dev`로의 모든 푸시, 모든 풀 리퀘스트, 모든 릴리스에서 `.github/workflows/ci.yml`의 `engine-tests` 작업으로 이 테스트를 실행합니다. 이어서 `scripts/ci/engine-test-leftovers.sh`를 실행하는데, 테스트가 `homeport-test-*` 컨테이너, 이미지, 볼륨 또는 네트워크를 남겼으면 실패합니다. 테스트 후 로컬에서 실행하면 같은 검사를 할 수 있습니다.

### 엔드투엔드 테스트

`e2e/`는 실제 앱, 즉 Rust 백엔드, 웹뷰, 그 사이의 호출을 `tauri-driver`를 거쳐 [WebdriverIO](https://webdriver.io)로 로컬 Docker Engine에서 구동합니다. 시나리오는 다음과 같습니다. 앱이 시작되어 로컬 엔진에 연결합니다. 컨테이너의 로그가 스트리밍되고 앱에서 컨테이너를 중지합니다. 이미지를 풀합니다. 암호화된 환경 변수가 있는 관리형 스택을 배포합니다. 다시 시작해도 호스트, 스택, 그 비밀 정보가 유지됩니다. 기존 키로 SSH 호스트를 추가하고, 서버 키를 신뢰하고, 연결합니다. 요소는 CSS 클래스가 아니라 역할, 레이블, 텍스트로 찾습니다.

이 테스트는 Linux에서만 실행됩니다. macOS에는 웹뷰용 WebDriver가 없기 때문입니다. [사전 요구 사항](#prerequisites)과 Docker Engine 외에 `tauri-driver`(`cargo install tauri-driver --locked`), WebKit의 드라이버, 가상 디스플레이, 키링이 필요합니다. Debian과 Ubuntu에서는 `webkit2gtk-driver`, `xvfb`, `dbus-x11`, `gnome-keyring`입니다.

```sh
bun tauri build --debug --no-bundle
cd e2e
bun install
./run.sh
```

이 빌드는 릴리스처럼 프런트엔드를 포함하므로, `commands::builder()`에 없는 명령, `src-tauri/capabilities/`에 없는 권한, 콘텐츠 보안 정책(Content Security Policy)이 막는 스타일은 여기서 실패합니다. `run.sh`는 실행할 때마다 새 데이터 폴더와 전용 키링, 디스플레이를 쓰므로 사용자의 Homeport는 건드리지 않습니다. WebdriverIO 옵션은 그 뒤에 넘깁니다. 예: `./run.sh --spec specs/ssh.e2e.ts`. SSH 시나리오는 `e2e/support/sshd/`로 작은 SSH 서버 이미지를 빌드합니다. 미리 빌드한 이미지를 쓰려면 `HOMEPORT_E2E_SSHD_IMAGE`를 설정하세요. 테스트가 시작하는 컨테이너의 이름은 `homeport-e2e-*`이며 끝나면 삭제됩니다.

실패하면 `e2e/artifacts/`에 스크린샷과 앱의 로그가 남습니다. CI는 `.github/workflows/ci.yml`의 `e2e` 작업에서 `dev`로의 모든 푸시와 모든 풀 리퀘스트마다 테스트를 실행하며, 실패하면 그 폴더를 보관합니다.

## 이 웹사이트

사이트는 `website/`에 있으며 [Astro](https://astro.build)와 [Starlight](https://starlight.astro.build)로 빌드합니다.

```sh
cd website
bun install
bun run dev      # http://localhost:4321/Homeport/
bun run build    # checks types and links, then builds into website/dist/
```

사이트를 변경하는 `main` 푸시마다, 그리고 다운로드 링크를 최신으로 유지하기 위해 릴리스마다 **Pages** 워크플로가 GitHub Pages에 배포합니다.
