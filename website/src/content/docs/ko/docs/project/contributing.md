---
title: 기여하기
description: 문제를 보고하고, 기능을 제안하고, Homeport에 코드를 기여하는 방법.
---

Homeport는 [github.com/DamienReichhart/Homeport](https://github.com/DamienReichhart/Homeport)에서 공개적으로 개발됩니다. 규모와 관계없이 모든 기여를 환영합니다.

## 버그 보고

다음 내용을 담아 [이슈를 여세요](https://github.com/DamienReichhart/Homeport/issues/new).

- **설정**에 있는 Homeport 버전과 플랫폼(예: `v0.2.0 · macos/aarch64`)
- 호스트 종류(로컬, SSH, TLS 또는 TCP)와 엔진 버전
- 무엇을 했고, 무엇을 기대했고, 무슨 일이 일어났는지. 정확한 메시지가 가장 도움이 됩니다.
- [로그 파일](/docs/reference/security/#로그)에서 관련 있는 내용

보안 문제는 [GitHub의 취약점 보고](https://github.com/DamienReichhart/Homeport/security/advisories/new)를 통해 비공개로 보고해 주세요.

## 기능 제안

해결책만이 아니라 해결하고 싶은 문제를 설명하는 이슈를 여세요. 엔진을 어떻게 다루는지 알면 기능에 맞는 형태를 찾는 데 도움이 됩니다.

## 코드 기여

1. [`docs/ARCHITECTURE.md`](https://github.com/DamienReichhart/Homeport/blob/main/docs/ARCHITECTURE.md)를 읽으세요. 기능은 독립적인 수직 슬라이스로 구성되고, 프런트엔드는 생성된 IPC 바인딩을 통해서만 Rust를 호출하며, Rust 로직은 Tauri 없이 단위 테스트하는 서비스에 있습니다.
2. 코드, UI 텍스트, 커밋에서 [`GLOSSARY.md`](https://github.com/DamienReichhart/Homeport/blob/main/GLOSSARY.md)에 있는 프로젝트 어휘(Host, Stack, Managed, External, Adopt)를 사용하세요. 주요 결정은 [`docs/adr`](https://github.com/DamienReichhart/Homeport/tree/main/docs/adr)에 기록되어 있습니다.
3. 색상과 어조는 [브랜드 가이드](https://github.com/DamienReichhart/Homeport/blob/main/docs/BRAND.md)를 따르세요. 평이하고, 구체적이고, 차분하게, 영국식 철자로 씁니다.
4. 다음이 통과하는지 확인하세요.

   ```sh
   bun run check
   bun run rust:lint
   bun run rust:test
   ```

5. 커밋 메시지는 [Conventional Commits](https://www.conventionalcommits.org) 형식(`feat(stacks): …`, `fix(hosts): …`)으로 작성하세요. 릴리스 노트가 커밋 메시지에서 생성됩니다.
6. `dev` 브랜치를 대상으로 풀 리퀘스트를 여세요. CI는 Linux에서 프런트엔드 검사와 라이선스 검사를, Linux, macOS, Windows에서 `cargo clippy`와 `cargo test`를 실행하므로 `#[cfg(windows)]`나 `#[cfg(target_os = "macos")]` 뒤의 코드도 병합 전에 컴파일됩니다. Docker Engine이 필요한 테스트에는 `#[ignore = "needs a local Docker Engine"]` 표시가 붙으며, 별도의 Linux 작업이 러너의 엔진으로 이 테스트를 실행합니다.

개발 환경 설정은 [소스에서 빌드](/docs/project/building/)를 참고하세요.

### 앱에 텍스트 추가

앱의 텍스트는 번역할 수 있도록 인라인이 아니라 카탈로그에 둡니다([ADR 0011](https://github.com/DamienReichhart/Homeport/blob/main/docs/adr/0011-formatjs-catalogues-and-keyed-errors.md)). 문자열을 추가하려면 다음을 따르세요.

1. [`src/shared/i18n/en/`](https://github.com/DamienReichhart/Homeport/tree/main/src/shared/i18n/en)에서 기능의 파일에 키를 추가합니다. 이미지 기능은 `images.ts`, UI 키트는 `ui.ts`이고, `common.ts`는 어디서나 같은 뜻인 단어(Cancel, Close, Retry)에만 씁니다. 키는 파일 이름으로 시작하고 텍스트의 용도를 나타냅니다: `images.remove.title`, `images.column.size`.
2. 값은 [ICU 메시지](https://formatjs.github.io/docs/core-concepts/icu-syntax/)로 작성합니다. 값에는 `{name}`을, 복수형은 직접 만들지 말고 `{count, plural, one {# image} other {# images}}`를, 강조에는 `tRich`로 렌더링되는 `<strong>…</strong>`을 씁니다. 문장 하나에 키 하나를 쓰고, 조각을 이어 붙이지 마세요. 언어마다 어순이 다르기 때문입니다.
3. 컴포넌트, 뮤테이션, 토스트 어디서든 `@/shared/i18n`의 `t('images.remove.title', { name })`로 사용합니다. 키에는 타입이 있으므로 오타가 있으면 타입 검사가 실패합니다. 리터럴 키를 사용하세요. 텍스트가 값에 따라 달라지면 `` `containers.done.${action}` ``처럼 키를 조립하지 말고 값을 키에 매핑하세요(`{ start: 'containers.done.start', stop: 'containers.done.stop' }`).

컴포넌트가 텍스트를 인라인으로 쓰면 `bun run lint`가 실패합니다. JSX 텍스트, 사람이 읽는 속성(`title`, `aria-label`, `placeholder`, `description`, `label`…), 토스트와 확인 대화 상자의 텍스트가 해당합니다. `tcp`, `nginx:latest`, `/dev/dri`, `max-size=10m`처럼 단어가 아니라 코드로 읽히는 문자열과 Docker나 Podman 같은 고유 명사는 통과합니다. 일부러 리터럴을 남기려면 바로 앞 줄에 이유를 적으세요: `// eslint-disable-next-line homeport/no-literal-ui-string -- a shell command`. `bun run check`는 코드에서 쓰지만 카탈로그에 없는 키, 아무 데서도 쓰지 않는 키, 런타임에 조립한 키가 있어도 실패합니다.

오류 메시지는 Rust에서 작성합니다. 각 크레이트는 `messages!` 블록에 키와 영어 템플릿을 선언하고, `bun run bindings`가 이를 `src/shared/i18n/en/errors.json`에 기록합니다. 이 파일은 직접 편집하지 마세요.

## Homeport 번역

앱과 웹사이트는 영어, 프랑스어, 스페인어, 독일어, 이탈리아어, 브라질 포르투갈어, 러시아어, 중국어 간체, 일본어, 한국어, 힌디어, 아랍어를 지원합니다. 각 언어는 제품 용어마다 단어 하나를 씁니다. 번역하기 전에 [`docs/i18n/`](https://github.com/DamienReichhart/Homeport/tree/main/docs/i18n)에서 찾아보고, 새 용어를 정하면 그곳에 행을 추가하세요.

### 문자열 추가 또는 변경

어떤 문자열이든, 모든 언어에 같은 `{arguments}`와 태그, 그리고 그 언어에 필요한 모든 복수형이 갖춰질 때까지 `bun run check`(따라서 CI)가 실패합니다.

- **앱 텍스트**: `src/shared/i18n/en/<namespace>.ts`에 키를 추가한 다음([앱에 텍스트 추가](#앱에-텍스트-추가) 참고), 다른 모든 언어 폴더(`fr/`, `de/`, `zh-CN/`…)의 같은 이름 파일에도 추가합니다. 할 수 있다면 번역하세요. 할 수 없다면 영어 텍스트를 그대로 복사하고 풀 리퀘스트에 그렇다고 적어 번역자가 이어서 작업할 수 있게 하세요. 키를 빠뜨려서는 안 됩니다.
- **오류 메시지나 힌트**: 크레이트의 `messages!` 블록에 선언하고 `bun run bindings`를 실행하면 `en/errors.json`에 기록됩니다. 그다음 각 언어의 `errors.json`에 키를 추가합니다. 운영 체제가 표시하는 텍스트(트레이 메뉴와 알림)도 `native.` 키와 `native.json`으로 같은 방식으로 다룹니다.
- **웹사이트 텍스트**(홈, 다운로드, 변경 내역, 헤더와 푸터): `website/src/core/i18n/strings/en/`의 알맞은 파일(`site`, `home`, `download` 또는 `changelog`)과 다른 모든 언어 폴더의 같은 파일에 키를 추가합니다. 하나라도 빠지면 `astro check`가 실패합니다. 문서 사이드바 레이블은 `website/src/app/config/sidebar/<code>.ts`에 둡니다.
- **문서 페이지**: 영어 페이지는 `website/src/content/docs/docs/`에 있습니다. 영어 페이지를 바꾸면 같은 풀 리퀘스트에서 모든 언어의 같은 페이지를 `website/src/content/docs/<code>/docs/` 아래 같은 경로에서 업데이트하세요. 페이지를 업데이트할 수 없는 언어는 번역자가 업데이트할 때까지 이전 번역을 보여 줍니다. 번역한 페이지의 제목은 영어와 같은 순서와 같은 수준으로 유지하고, 링크는 앵커를 포함해 영어에 쓰인 그대로 두세요. `bun run build` 후 `bun scripts/localize-anchors.ts <code>`가 각 앵커가 번역된 제목을 가리키게 하고, 두 번째 빌드의 링크 검사가 이를 확인합니다.

언어 하나만 확인하려면 `bun run i18n:check --locale fr`을 실행하세요.

### 언어 추가

파일을 추가하고, 웹사이트에 한 줄을 추가합니다.

1. **앱**: `src/shared/i18n/en/`을 `src/shared/i18n/<tag>/`(`pl`이나 `zh-TW` 같은 BCP 47 태그)로 복사하고, `meta.ts`(`{ name: 'Polski', dir: 'ltr' }`처럼 그 언어로 쓴 언어 이름. 오른쪽에서 왼쪽으로 쓰는 문자라면 `rtl`)를 작성한 다음 모든 파일을 번역합니다. 다른 언어처럼 각 키는 그대로 두고, 각 `export default { … }`를 `export default { … } satisfies Translation<typeof en>`으로 바꿉니다. `bun run i18n:check --locale <tag>`가 빠진 항목을 나열합니다. 다른 변경 없이도 설정에 새 언어가 표시되고, OS 언어에 따라 선택됩니다.
2. **웹사이트**: `website/src/core/site/locales.ts`에 언어를 추가한 다음, `website/src/core/i18n/strings/en/`을 복사한 `<code>/`에 문자열을(`core/i18n/index.ts`의 `TABLES`에 등록), `website/src/app/config/sidebar/<code>.ts`에 사이드바 레이블을(`sidebar.ts`에 등록. 이 파일들은 폴더를 찾아 주는 Vite 바깥에서 불러오므로, 이 두 줄이 언어에 필요한 유일한 코드입니다), `website/src/content/i18n/<tag>.json`에 번역되지 않은 페이지에 대한 Starlight 안내 문구를 추가합니다. 페이지, URL, 언어 전환기 항목, `hreflang` 링크, 사이트맵 대체 링크는 목록에 따라 자동으로 만들어집니다. 문서는 `website/src/content/docs/<code>/docs/` 아래에 둡니다. 그곳의 모든 페이지를 번역하고(없는 페이지는 안내 문구와 함께 영어로 보입니다), 빌드 후 `bun scripts/localize-anchors.ts <code>`를 실행합니다.
3. **스크린숏**: `bun run screenshots`는 새 언어를 포함한 모든 언어로 모든 스크린숏을 `website/src/assets/screenshots/<tag>/`에 캡처하므로, 각 페이지는 읽는 사람의 언어로 앱을 보여 줍니다. 영어에 있는 스크린숏이 어떤 언어에 없는 동안에는 웹사이트의 `bun test`가 실패합니다.
4. **용어**: `docs/i18n/terms-<tag>.md`를 추가합니다.

방금 추가한 언어를 테스트가 인식하지 못하면 `node_modules/.vitest-cache`를 삭제하세요. Vitest가 카탈로그 파일의 이전 목록을 가지고 있을 수 있습니다.

## 이 문서 개선

모든 문서 페이지 하단에는 **페이지 편집** 링크가 있습니다. 페이지는 [`website/src/content/docs`](https://github.com/DamienReichhart/Homeport/tree/main/website/src/content/docs)에 있는 Markdown 파일입니다.

## 릴리스 방식

새 버전 번호와 함께 `main`에 푸시하면 모든 플랫폼용 설치 프로그램을 빌드하고, 전체 테스트 모음을 실행하고, 체크섬과 빌드 출처가 포함된 GitHub 릴리스를 게시합니다. 과정은 [`docs/RELEASING.md`](https://github.com/DamienReichhart/Homeport/blob/main/docs/RELEASING.md)에 설명되어 있습니다.
