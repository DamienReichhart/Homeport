---
title: Teclado e acessibilidade
description: Como usar o Homeport com o teclado, com movimento reduzido e com um leitor de tela, o contraste que ele atende, os limites conhecidos hoje e como relatar uma barreira.
---

O Homeport pretende ser utilizável só com o teclado, com um leitor de tela e com movimento reduzido, em cada um dos seus [idiomas](/docs/guides/settings/#aparência). Esta página diz o que funciona, o que ainda não funciona e como nos avisar quando algo atrapalha você.

## Como se mover

Tudo no Homeport pode ser alcançado com <kbd>Tab</kbd> e <kbd>Shift</kbd> <kbd>Tab</kbd> e usado com <kbd>Enter</kbd> ou <kbd>Space</kbd>. Um anel visível mostra onde está o foco.

- **A barra lateral** lista todas as páginas; cada uma é um link. <kbd>Mod</kbd> <kbd>1</kbd> a <kbd>9</kbd> as abrem em ordem, e a [paleta de comandos](/docs/guides/keyboard/#paleta-de-comandos) (<kbd>Mod</kbd> <kbd>K</kbd>) alcança todas as páginas, hosts e ações digitando.
- **Listas** (contêineres, imagens, volumes, redes, stacks, hosts, Pods…): o nome de cada linha é um botão ou um link. Vá até ele com <kbd>Tab</kbd> e pressione <kbd>Enter</kbd> para abrir o item, como um clique na linha faz. As caixas de seleção e o menu da linha vêm antes e depois dele.
- **Ordenação**: o cabeçalho de uma coluna ordenável é um botão. Pressione <kbd>Enter</kbd> ou <kbd>Space</kbd> para ordenar por ela, e de novo para inverter; os leitores de tela anunciam a ordem.
- **As stacks na lista de contêineres** abrem e fecham com o botão antes do nome delas, que diz se ela está expandida.
- **As caixas de diálogo** recebem o foco quando abrem, mantêm-no dentro enquanto estão abertas e o devolvem ao que as abriu quando fecham. <kbd>Esc</kbd> fecha uma caixa de diálogo sem fazer nada.
- **Menus, seletores e abas** se movem com as teclas de seta, <kbd>Home</kbd> e <kbd>End</kbd>; <kbd>Enter</kbd> escolhe, <kbd>Esc</kbd> fecha.
- **Atalhos**: <kbd>?</kbd> em qualquer página lista os que funcionam ali; veja [Teclado](/docs/guides/keyboard/).

## Terminais, logs e o editor

O **Terminal** de um contêiner e o visualizador de logs recebem todas as teclas enquanto têm o foco, para que o programa dentro deles receba também <kbd>Tab</kbd>, <kbd>Esc</kbd> e as teclas com <kbd>Ctrl</kbd>. Saia deles com o mouse, ou movendo o foco com os comandos do próprio leitor de tela.

Os editores de compose, YAML, jobs e unidades são editores CodeMirror nomeados para os leitores de tela. Neles, <kbd>Tab</kbd> indenta; pressione <kbd>Esc</kbd> e depois <kbd>Tab</kbd> para tirar o foco do editor.

## Movimento reduzido

Quando o seu sistema operacional pede movimento reduzido (**Reduzir movimento** no macOS, **Efeitos de animação** desativado no Windows, **Reduce animation** no GNOME e no KDE), o Homeport apenas faz esmaecimentos: caixas de diálogo, menus, seletores e dicas não aumentam nem deslizam, nada pulsa, os botões não se movem quando pressionados e o cursor do terminal para de piscar. Os indicadores de carregamento continuam girando, mais devagar, porque dizem que há trabalho em andamento. A configuração é seguida à medida que muda, sem reiniciar.

## Leitores de tela

Todas as páginas e as principais caixas de diálogo são verificadas com o [axe](https://github.com/dequelabs/axe-core) a cada mudança: botões, campos e regiões têm nomes, os títulos e os pontos de referência estão em ordem, e nada usa ARIA de forma indevida. O Homeport define o idioma da página, para que um leitor de tela leia cada idioma com a voz própria dele, e o árabe é exibido da direita para a esquerda.

A janela do Homeport é um web view: WebKitGTK no Linux, WKWebView no macOS e WebView2 no Windows. Uma auditoria prática com o Orca, o VoiceOver e o NVDA em cada um está em andamento ([#184](https://github.com/DamienReichhart/Homeport/issues/184)); até que ela termine, considere a experiência com leitores de tela como testada apenas por ferramentas.

### Limites conhecidos

- **Terminais e logs**: o texto dentro do terminal de um contêiner e do visualizador de logs ainda não é lido pelos leitores de tela. Copie a saída, ou abra os logs em um terminal com `docker logs`.
- **Listas ao vivo**: as listas se atualizam no lugar à medida que o Engine informa mudanças, sem anunciar cada mudança. As notificações anunciam o que você fez.
- **Os gráficos** desenham os seus valores; o valor mais recente de cada um também está no título do gráfico, como texto.

## Contraste

O texto atende ao WCAG 2.2 AA (4,5:1) tanto no tema claro quanto no escuro, assim como as bordas dos campos, o anel de foco e as formas de status (3:1). O status nunca depende só da cor: cada um tem a sua própria forma. As proporções estão listadas no guia de marca do projeto.

## Relatar uma barreira

Se algo impedir você de usar o Homeport com o teclado, o leitor de tela, a lupa ou outra tecnologia assistiva, [abra uma issue](https://github.com/DamienReichhart/Homeport/issues/new?labels=accessibility) com o rótulo `accessibility`. Diga:

- o que você estava tentando fazer, e onde (página, caixa de diálogo, tipo de host);
- o seu sistema operacional e a versão dele, e a tecnologia assistiva e a versão dela;
- o que você ouviu ou viu, e o que esperava.

As barreiras são tratadas como bugs, não como pedidos de recurso.
