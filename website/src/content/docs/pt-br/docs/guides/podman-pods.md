---
title: Pods do Podman
description: Liste, inicie, pare e remova pods do Podman, crie um com as suas portas publicadas e execute contêineres nele.
---

Um **pod** é a forma do Podman de agrupar contêineres: eles compartilham uma rede (e o hostname e o IPC), então se alcançam em `localhost`, e iniciam e param juntos. Cada pod tem um **contêiner infra** que guarda o que eles compartilham e as portas que o pod publica. O Homeport mostra os pods nos hosts cujo Engine é o Podman, pela própria API do Podman no socket que o host já usa; nada é instalado no host.

A página **Pods** só aparece na barra lateral enquanto um host Podman está selecionado. Em qualquer outro Engine compatível com Docker ela não existe, e um link para ela diz por quê. Veja [Tipos de Engine](/docs/hosts/#tipos-de-engine) para saber como o Homeport distingue o Podman.

## A página Pods

| Coluna          | Mostra                                                                                                                         |
| --------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| **Nome**        | O nome do pod, com o seu status como um ponto.                                                                                 |
| **Status**      | **Em execução**, **Parcialmente em execução** (alguns contêineres parados), **Pausado**, **Parado**, **Criado** ou **Falhou**. |
| **Contêineres** | Quantos dos seus próprios contêineres estão rodando, sem contar o contêiner infra.                                             |
| **Redes**       | As redes em que o pod está.                                                                                                    |
| **Criado**      | Quando ele foi criado.                                                                                                         |

Cada linha tem a ação que corresponde ao seu status — **Iniciar** para um pod parado, **Parar** para um em execução — e **Remover**. Selecione um pod para abrir a página dele. A lista se atualiza sozinha conforme os contêineres do pod iniciam e param, seja pelo Homeport ou por um terminal.

Na página **Contêineres**, um contêiner que roda em um pod mostra o nome do pod ao lado do seu; selecione o nome para abrir o pod.

## A página de um pod

O cabeçalho tem **Executar contêiner** e as ações do pod: **Iniciar**, **Parar**, **Reiniciar**, **Pausar**, **Retomar** e **Remover**, conforme o status permitir. Cada uma age sobre todos os contêineres do pod de uma vez, como fazem `podman pod start` e companhia.

- **Contêineres** lista os contêineres do pod, com o contêiner infra por último. Selecione um para abrir a página dele, com os seus logs, terminal e arquivos.
- **Configurações** mostra as portas que o pod publica, os namespaces que os seus contêineres compartilham, as suas redes, o hostname e os rótulos, e o comando `podman pod create` com que ele foi criado, quando o Podman o guardou.

## Removendo um pod

**Remover** pergunta antes e diz quantos contêineres vão junto com o pod. Um pod em execução é parado primeiro: o botão diz **Parar e remover**. Os volumes do pod são mantidos.

## Criando um pod

**Novo pod** (ou `n` na página Pods, ou **Novo pod** na paleta de comandos) pede:

- um **Nome**: letras, dígitos, `_`, `.` e `-`, começando com uma letra ou um dígito;
- as **Portas** a publicar, porta do host para porta do contêiner, como na caixa de diálogo Executar. Deixe a porta do host vazia para que o Podman escolha uma livre;
- os **Rótulos**, um `key=value` por linha.

As portas pertencem ao pod: elas são publicadas no contêiner infra e chegam a qualquer contêiner do pod que escute nessa porta. A página do novo pod se abre, pronta para executar contêineres nele. Um pod não tem contêineres próprios até você executar um; o Podman o inicia com o primeiro.

## Executando um contêiner em um pod

Em um host Podman, a caixa de diálogo **Executar** tem um campo **Pod**, e o botão **Executar contêiner** de um pod abre a caixa de diálogo com esse pod escolhido. Um contêiner em um pod:

- compartilha a rede do pod, então não pode publicar portas nem entrar em outra rede: a caixa de diálogo esconde esses campos assim que um pod é escolhido;
- usa o hostname e as configurações de DNS do pod, então o Homeport recusa os seus próprios;
- mantém tudo o mais que a caixa de diálogo Executar oferece: comando, ambiente, montagens, política de reinício, limites, capabilities, dispositivos, verificação de integridade e logging.

Duplicar e Editar nunca movem um contêiner para dentro de um pod: o Podman não consegue mover um contêiner existente para um.

## O que o Homeport precisa

Os pods funcionam no Podman 4.0 e posteriores, em todo tipo de conexão — socket local, SSH, TLS ou TCP —, já que passam pelo mesmo socket que todo o resto.
