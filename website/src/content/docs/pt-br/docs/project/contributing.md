---
title: Contribuir
description: Como relatar problemas, sugerir recursos e contribuir com código para o Homeport.
---

O Homeport é desenvolvido abertamente em [github.com/DamienReichhart/Homeport](https://github.com/DamienReichhart/Homeport). Contribuições de todos os tamanhos são bem-vindas.

## Relatar um bug

[Abra uma issue](https://github.com/DamienReichhart/Homeport/issues/new/choose) com:

- o que você fez, o que esperava e o que aconteceu — a mensagem exata é o que mais ajuda;
- o [pacote de diagnóstico](/docs/reference/troubleshooting/#exportando-o-diagnóstico-para-um-relatório-de-bug) de **Configurações → Sobre → Exportar diagnóstico…**: um único zip com a versão e a plataforma do Homeport, os tipos de hosts, Engines e clusters que você usa, e os logs dele com os segredos removidos. Você lê o que há nele antes de salvá-lo, e nada é enviado para lugar nenhum.

Se o Homeport não iniciar, informe a versão dele e o seu sistema operacional, e qualquer coisa relevante do [arquivo de log](/docs/reference/security/#logs).

Relate os problemas de segurança de forma privada, pelo [relato de vulnerabilidades do GitHub](https://github.com/DamienReichhart/Homeport/security/advisories/new).

## Sugerir um recurso

Abra uma issue descrevendo o problema que você quer resolver, não só a solução. Saber como você trabalha com os seus Engines ajuda a encontrar a forma certa para ele.

## Contribuir com código

1. Leia [`docs/ARCHITECTURE.md`](https://github.com/DamienReichhart/Homeport/blob/main/docs/ARCHITECTURE.md). Os recursos são fatias verticais autocontidas, o frontend só chama o Rust pelos bindings de IPC gerados, e a lógica em Rust fica em serviços com testes unitários sem o Tauri.
2. Use o vocabulário do projeto de [`GLOSSARY.md`](https://github.com/DamienReichhart/Homeport/blob/main/GLOSSARY.md) — Host, Stack, Managed, External, Adopt — no código, nos textos da interface e nos commits. As principais decisões ficam registradas em [`docs/adr`](https://github.com/DamienReichhart/Homeport/tree/main/docs/adr).
3. Siga o [guia de marca](https://github.com/DamienReichhart/Homeport/blob/main/docs/BRAND.md) para as cores e o tom: simples, específico, calmo, em inglês britânico.
4. Verifique se estes passam:

   ```sh
   bun run check
   bun run rust:lint
   bun run rust:test
   ```

5. Mantenha a promessa dos dados: cada versão do Homeport abre os dados de todas as versões anteriores, até a 0.1.0, e os atualiza sem perder nada: hosts, stacks, clusters, configurações e segredos. Uma mudança de esquema é uma nova migração, nunca uma edição de uma já lançada, e a CI a verifica com um banco de dados de exemplo de cada release. Veja [Data compatibility](https://github.com/DamienReichhart/Homeport/blob/main/docs/RELEASING.md#data-compatibility).
6. Escreva as mensagens de commit como [Conventional Commits](https://www.conventionalcommits.org) — `feat(stacks): …`, `fix(hosts): …`. As notas de release são geradas a partir delas.
7. Abra um pull request para a branch `dev`. A CI executa as verificações do frontend e a verificação de licenças no Linux, e `cargo clippy` e `cargo test` no Linux, macOS e Windows, então o código atrás de `#[cfg(windows)]` ou `#[cfg(target_os = "macos")]` é compilado antes de ser mesclado. Um teste que precisa de um Docker Engine é marcado com `#[ignore = "needs a local Docker Engine"]`; um job Linux separado os executa com o Engine do runner.

Veja [fazer build a partir do código-fonte](/docs/project/building/) para preparar o ambiente.

### Adicionar texto ao aplicativo

O texto do aplicativo fica em um catálogo, nunca no meio do código, para que possa ser traduzido ([ADR 0011](https://github.com/DamienReichhart/Homeport/blob/main/docs/adr/0011-formatjs-catalogues-and-keyed-errors.md)). Para adicionar uma string:

1. Adicione uma chave ao arquivo do recurso em [`src/shared/i18n/en/`](https://github.com/DamienReichhart/Homeport/tree/main/src/shared/i18n/en) — `images.ts` para o recurso de imagens, `ui.ts` para o kit de interface, `common.ts` apenas para palavras que significam o mesmo em todo lugar (Cancel, Close, Retry). As chaves começam com o nome do arquivo e dizem para que serve o texto: `images.remove.title`, `images.column.size`.
2. Escreva o valor como uma [mensagem ICU](https://formatjs.github.io/docs/core-concepts/icu-syntax/): `{name}` para um valor, `{count, plural, one {# image} other {# images}}` em vez de montar plurais à mão, `<strong>…</strong>` para ênfase renderizada com `tRich`. Uma chave por frase; não cole fragmentos, porque a ordem das palavras muda entre os idiomas.
3. Use-a com `t('images.remove.title', { name })` de `@/shared/i18n` — em componentes, mutations e notificações da mesma forma. As chaves são tipadas, então um erro de digitação falha na verificação de tipos. Use chaves literais: quando o texto depende de um valor, mapeie o valor para chaves (`{ start: 'containers.done.start', stop: 'containers.done.stop' }`) em vez de montar `` `containers.done.${action}` ``.

`bun run lint` falha quando um componente escreve texto diretamente: texto JSX, props que as pessoas leem (`title`, `aria-label`, `placeholder`, `description`, `label`…) e o texto de notificações e confirmações. As strings que se leem como código, e não como palavras — `tcp`, `nginx:latest`, `/dev/dri`, `max-size=10m` — e os nomes próprios como Docker ou Podman passam. Para manter um literal de propósito, diga o motivo na linha anterior: `// eslint-disable-next-line homeport/no-literal-ui-string -- a shell command`. `bun run check` também falha com uma chave usada no código, mas ausente do catálogo, uma chave que nada usa e uma chave montada em tempo de execução.

As mensagens de erro são escritas em Rust: cada crate declara as suas chaves e os modelos em inglês no seu bloco `messages!`, e `bun run bindings` os grava em `src/shared/i18n/en/errors.json`. Não edite esse arquivo à mão.

## Traduzir o Homeport

O aplicativo e o site falam inglês, francês, espanhol, alemão, italiano, português do Brasil, russo, chinês simplificado, japonês, coreano, hindi e árabe. Cada idioma usa uma palavra por termo do produto: procure-as em [`docs/i18n/`](https://github.com/DamienReichhart/Homeport/tree/main/docs/i18n) antes de traduzir e adicione uma linha ali quando escolher uma nova.

### Adicionar ou alterar uma string

Seja qual for a string, `bun run check` (e, portanto, a CI) falha até que todos os idiomas a tenham, com os mesmos `{arguments}` e tags, e com todas as formas de plural de que esse idioma precisa.

- **Texto do aplicativo**: adicione a chave a `src/shared/i18n/en/<namespace>.ts` (veja [Adicionar texto ao aplicativo](#adicionar-texto-ao-aplicativo)) e depois ao arquivo de mesmo nome em todas as outras pastas de idioma (`fr/`, `de/`, `zh-CN/`…). Traduza-a se puder. Se não puder, copie o texto em inglês ali e diga isso no pull request, para que um tradutor possa dar continuidade; nunca deixe a chave de fora.
- **Uma mensagem de erro ou dica**: declare-a no bloco `messages!` da crate e execute `bun run bindings`, que grava `en/errors.json`; depois adicione a chave ao `errors.json` de cada idioma. O texto que o sistema operacional mostra — o menu da bandeja e as notificações — funciona da mesma forma, com uma chave `native.` e `native.json`.
- **Texto do site** (página inicial, download, changelog, cabeçalho e rodapé): adicione a chave ao arquivo certo de `website/src/core/i18n/strings/en/` (`site`, `home`, `download` ou `changelog`) e ao mesmo arquivo da pasta de cada outro idioma; `astro check` falha quando um deles não a tem. Um rótulo da barra lateral da documentação vai em `website/src/app/config/sidebar/<code>.ts`.
- **Uma página da documentação**: a página em inglês fica em `website/src/content/docs/docs/`. Quando você a alterar, atualize a mesma página em todos os idiomas, no mesmo caminho em `website/src/content/docs/<code>/docs/`, no mesmo pull request; um idioma cuja página você não puder atualizar mostra a tradução anterior até que um tradutor o faça. Mantenha os títulos das páginas traduzidas na mesma ordem e nos mesmos níveis do inglês, e os links exatamente como o inglês os escreve, âncoras incluídas: depois de `bun run build`, `bun scripts/localize-anchors.ts <code>` aponta cada âncora para o título traduzido, e a verificação de links de uma segunda build confirma isso.

Verifique um idioma com `bun run i18n:check --locale fr`.

### Adicionar um idioma

Arquivos, mais uma linha para o site:

1. **Aplicativo**: copie `src/shared/i18n/en/` para `src/shared/i18n/<tag>/` (uma tag BCP 47 como `pl` ou `zh-TW`), escreva o `meta.ts` dele (`{ name: 'Polski', dir: 'ltr' }`, o nome que o idioma dá a si mesmo; `rtl` para uma escrita da direita para a esquerda) e traduza todos os arquivos, mantendo cada chave e transformando cada `export default { … }` em `export default { … } satisfies Translation<typeof en>`, como os outros idiomas fazem. `bun run i18n:check --locale <tag>` lista o que está faltando. As Configurações listam o novo idioma, e o idioma do sistema operacional o escolhe, sem nenhuma outra mudança.
2. **Site**: adicione o idioma a `website/src/core/site/locales.ts` e depois as strings dele em uma cópia de `website/src/core/i18n/strings/en/` chamada `<code>/` (registrada em `TABLES` em `core/i18n/index.ts`), os rótulos da barra lateral em `website/src/app/config/sidebar/<code>.ts` (registrados em `sidebar.ts`; essas duas linhas são o único código de que um idioma precisa, porque esses arquivos são carregados fora do Vite, que é quem encontra as pastas) e o aviso do Starlight para páginas não traduzidas em `website/src/content/i18n/<tag>.json`. As páginas, as URLs, as entradas do seletor de idioma, os links `hreflang` e as alternativas do sitemap decorrem da lista. A documentação dele vai em `website/src/content/docs/<code>/docs/`: traduza ali todas as páginas (uma que faltar mostra o inglês com um aviso) e depois execute `bun scripts/localize-anchors.ts <code>` após uma build.
3. **Capturas de tela**: `bun run screenshots` captura todas as imagens em todos os idiomas, incluindo o novo, em `website/src/assets/screenshots/<tag>/`, para que cada página mostre o aplicativo no idioma de quem a lê. O `bun test` do site falha enquanto faltar a um idioma uma captura que o inglês tem.
4. **Termos**: adicione `docs/i18n/terms-<tag>.md`.

Se um teste não enxergar um idioma que você acabou de adicionar, exclua `node_modules/.vitest-cache`: o Vitest pode manter uma lista antiga de arquivos de catálogo.

## Melhorar esta documentação

Toda página da documentação tem um link **Editar página** no fim. As páginas são arquivos Markdown em [`website/src/content/docs`](https://github.com/DamienReichhart/Homeport/tree/main/website/src/content/docs).

## Como as releases acontecem

Todo push para `main` com um novo número de versão gera instaladores para todas as plataformas, executa toda a suíte de testes e publica uma release no GitHub com checksums e proveniência de build. O processo é descrito em [`docs/RELEASING.md`](https://github.com/DamienReichhart/Homeport/blob/main/docs/RELEASING.md).
