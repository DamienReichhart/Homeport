---
title: Teclado
description: Use o Homeport sem o mouse — a paleta de comandos e a pesquisa em todos os hosts, atalhos para cada página e cada host, e a folha de ajuda que os lista.
---

O Homeport pode ser usado pelo teclado. Pressione <kbd>Mod</kbd> <kbd>K</kbd> para abrir a [paleta de comandos](#command-palette) e <kbd>?</kbd> em qualquer página para ver todos os atalhos que funcionam ali.

**Mod**, abaixo, significa <kbd>⌘</kbd> no macOS e <kbd>Ctrl</kbd> no Windows e no Linux; no macOS, <kbd>Alt</kbd> é <kbd>⌥</kbd>.

## Paleta de comandos

<kbd>Mod</kbd> <kbd>K</kbd> abre a paleta de comandos em qualquer página. Digite o que você quer, mova-se com <kbd>↑</kbd> e <kbd>↓</kbd> e pressione <kbd>Enter</kbd>; <kbd>Esc</kbd> a fecha e leva você de volta para onde estava. Ela lista:

- **Ir para** — todas as páginas da barra lateral: digite `vol` e pressione <kbd>Enter</kbd> para abrir Volumes.
- **Hosts** — todos os hosts com o seu status: digite parte do nome de um host para mudar para ele.
- **Ações** — **Executar um contêiner**, **Baixar uma imagem**, **Nova stack**, **Limpar o host selecionado**, **Adicionar host**, **Gerenciar hosts**, **Verificar atualizações** e **Mostrar atalhos de teclado**.
- **Configurações** — **Tema: Claro**, **Tema: Escuro** e **Tema: Igual ao sistema**.

As ações que precisam de um host — executar um contêiner, baixar, criar uma stack, limpar — só aparecem enquanto o host selecionado estiver conectado.

### Pesquisando em todos os hosts

Digite dois caracteres ou mais e a paleta também pesquisa os **contêineres**, as **stacks**, as **imagens**, os **volumes** e as **redes** de todos os hosts conectados — "onde o `postgres` roda?" tem resposta sem mudar de host. Ela compara nomes, IDs curtos e, para contêineres, a imagem e a stack deles; os nomes que começam com o que você digitou vêm primeiro.

- Os resultados são agrupados por tipo, cinco por tipo até **Mostrar todos**. Cada um mostra o seu host e, para contêineres e stacks, se estão rodando: _prod-server · em execução_.
- Escolher um muda para o host dele e o abre: a página de um contêiner ou de uma stack, ou a página Imagens, Volumes ou Redes pesquisada por ele.
- Os hosts que não estão conectados são ignorados, e a paleta diz quantos: _2 hosts não pesquisados (não conectado)_.

O Homeport só carrega as listas depois que você digita dois caracteres e, a partir daí, as mantém atualizadas com os eventos ao vivo de cada host. Logs, arquivos e definições compose não são pesquisados. Cada item mostra o seu atalho, se tiver um. Sem nada digitado, os itens que você usou por último vêm primeiro, em **Recentes**, até o Homeport ser encerrado.

## Atalhos

### Em qualquer lugar

| Teclas                                     | Faz                                                                                    |
| ------------------------------------------ | -------------------------------------------------------------------------------------- |
| <kbd>?</kbd>                               | Mostrar os atalhos de teclado                                                          |
| <kbd>Mod</kbd> <kbd>1</kbd> … <kbd>9</kbd> | Abrir as páginas da barra lateral em ordem: Visão geral, Contêineres, Imagens…         |
| <kbd>Mod</kbd> <kbd>,</kbd>                | Abrir Configurações                                                                    |
| <kbd>Alt</kbd> <kbd>↑</kbd> / <kbd>↓</kbd> | Selecionar o host anterior ou o próximo, na ordem em que a página Hosts os lista       |
| <kbd>Mod</kbd> <kbd>R</kbd>                | Atualizar as listas do host selecionado: contêineres, imagens, volumes, redes, stacks… |

Passe o ponteiro sobre uma página na barra lateral para ver as teclas dela.

### Em uma página

| Teclas       | Faz                                                                                                                                            |
| ------------ | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| <kbd>/</kbd> | Focar o campo de pesquisa da página. <kbd>Esc</kbd> em um campo vazio sai dele.                                                                |
| <kbd>N</kbd> | Iniciar a ação principal da página: **Executar contêiner**, **Baixar**, **Criar volume**, **Criar rede**, **Nova stack** ou **Adicionar host** |

O botão da ação principal de uma página mostra <kbd>N</kbd> quando você passa o ponteiro sobre ele.

### Enquanto você digita

As teclas únicas — <kbd>N</kbd>, <kbd>/</kbd>, <kbd>?</kbd> e as setas com <kbd>Alt</kbd> — nunca disparam enquanto você digita em um campo, no editor de uma stack ou em uma caixa de diálogo: elas digitam o caractere. Os atalhos com <kbd>Mod</kbd> funcionam ali também. No **Terminal** de um contêiner, todas as teclas vão para o contêiner e nenhum atalho dispara.
