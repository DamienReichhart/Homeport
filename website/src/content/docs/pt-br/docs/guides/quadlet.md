---
title: Unidades Quadlet
description: Liste as unidades Quadlet de um host Podman com o seu status no systemd, leia os arquivos delas, instale, edite e remova-as, e inicie ou pare os seus serviços.
---

O **Quadlet** é a forma como o Podman executa serviços de longa duração sob o systemd. Uma unidade Quadlet é um arquivo — `web.container`, `shop.pod`, `cache.volume`, `lan.network`, `app.kube`, `base.image`, `api.build` — que o gerador do systemd transforma em um serviço, `web.service`, que inicia o contêiner na inicialização e o reinicia quando ele para. O Podman rootful guarda as suas unidades em `/etc/containers/systemd/`; o Podman rootless guarda as de cada usuário em `~/.config/containers/systemd/`.

A página **Quadlet** aparece na barra lateral enquanto um host Podman está selecionado. Ela passa pela própria API do Podman no socket que o host já usa, e pelo `systemctl` para iniciar e parar, como decide o [ADR 0019](https://github.com/DamienReichhart/Homeport/blob/main/docs/adr/0019-quadlet-units-through-podmans-api-and-systemctl.md).

## O que cada Podman e cada conexão permitem

A API do Podman ganhou suporte ao Quadlet ao longo de duas versões, e ela nunca inicia nem para o serviço de uma unidade. Então o que o Homeport consegue fazer depende do Podman do host e de como o Homeport o alcança:

| Ação                                        | Podman               | Conexão                          |
| ------------------------------------------- | -------------------- | -------------------------------- |
| Listar as unidades com o seu status         | 5.7 e posteriores    | qualquer                         |
| Listar as unidades a partir dos contêineres | 4.0 a 5.6            | qualquer                         |
| Ler o arquivo de uma unidade                | 5.8 e posteriores    | qualquer                         |
| Instalar, editar e remover unidades         | 5.8 e posteriores    | qualquer                         |
| Iniciar, parar e reiniciar um serviço       | qualquer com Quadlet | SSH, ou um socket local no Linux |

No Podman anterior ao 5.7, as unidades são encontradas pelo rótulo `PODMAN_SYSTEMD_UNIT` que o Podman coloca nos contêineres que elas criam, então uma unidade só aparece depois de ter criado um contêiner, com o nome do seu serviço. A página diz isso acima da lista.

Iniciar e parar precisam do `systemctl`, que o Homeport executa onde o Podman roda: pela sessão SSH de um host SSH, ou neste computador para um socket local no Linux. Um host TCP ou TLS, ou um socket local no macOS ou no Windows (onde o Podman roda em uma máquina virtual), pode fazer todo o resto; os botões ficam desativados, com o motivo.

## A página Quadlet

| Coluna          | Mostra                                                                                                                                     |
| --------------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| **Unidade**     | O nome do arquivo da unidade e a aplicação com que ela foi instalada.                                                                      |
| **Tipo**        | `container`, `pod`, `volume`, `network`, `kube`, `image`, `build` ou `artifact`.                                                           |
| **Serviço**     | O serviço systemd que ela gera, ou **ainda não gerado** até o systemd carregá-la.                                                          |
| **Status**      | **Em execução**, **Iniciando**, **Parando**, **Parado**, **Falhou**, **Não carregado pelo systemd** ou **Modelo**, conforme diz o systemd. |
| **Contêineres** | Quantos contêineres ela criou.                                                                                                             |

Cada linha tem **Iniciar** para uma unidade parada ou **Parar** para uma em execução. Selecione uma unidade para abrir a página dela. A lista se atualiza sozinha conforme os contêineres dela iniciam e param.

Uma unidade **Não carregado pelo systemd** está instalada, mas o gerador do systemd a ignorou: em geral, um erro no arquivo. No host, `systemctl --user daemon-reload` (ou `systemctl daemon-reload` para o Podman rootful) e depois `journalctl` dizem por quê.

Na página **Contêineres**, um contêiner criado por uma unidade mostra o serviço da unidade ao lado do seu nome; selecione-o para abrir a unidade.

## A página de uma unidade

- O cabeçalho tem **Iniciar**, **Parar** e **Reiniciar**, **Editar** e **Remover**.
- **Serviço** mostra a unidade systemd, se ela é uma unidade de sistema ou de usuário (`systemctl --user`), onde o arquivo dela está no host, e a sua aplicação e o seu pod, quando os tem.
- **Arquivo da unidade** mostra o arquivo como o Podman o lê, com destaque de sintaxe.
- **Contêineres** lista os contêineres que ela criou, e cada um abre a página dele.

Parar o contêiner de uma unidade pela página Contêineres é brigar com o systemd, que o inicia de novo por causa de `Restart=`. Pare a unidade.

## Instalando uma unidade

**Nova unidade** (ou `n` na página Quadlet, ou **Nova unidade** na paleta de comandos) abre um editor com um ponto de partida para o tipo que você escolher, ou um arquivo que você abre deste computador com **Abrir arquivo…**. Dê a ela um nome com a extensão, como `web.container`; o arquivo precisa da seção do seu tipo, `[Container]` para um `.container`.

**Adicionar arquivos…** coloca outros arquivos ao lado da unidade — um `Containerfile` para um `.build`, o YAML de um `.kube`, um arquivo de ambiente — em uma pasta de aplicação com o nome da unidade, como faz `podman quadlet install`.

O Podman grava os arquivos onde ficam as unidades do seu usuário e recarrega o systemd, então o serviço passa a existir na hora. Instalar não o inicia: escolha **Iniciar** quando ela estiver lá. Uma unidade cujo nome já está em uso é recusada; edite a existente.

## Editando e removendo

**Editar** abre o arquivo da unidade. **Salvar** o grava no lugar e recarrega o systemd; os arquivos instalados ao lado dele permanecem. Um serviço em execução mantém as configurações antigas até você o **Reiniciar**.

**Remover** exclui o arquivo da unidade e recarrega o systemd. **Parar o serviço em execução dela antes** vem ativado por padrão; sem isso, o Podman se recusa a remover uma unidade em execução. Os contêineres, volumes e imagens que a unidade criou são mantidos.

## O que o Homeport executa no host

Para **Iniciar**, **Parar** e **Reiniciar**, e nada mais, o Homeport executa um comando no servidor do host por SSH (ou neste computador para um socket local), com os direitos do usuário SSH:

```bash
systemctl --user start 'web.service'
```

`--user` está ali para as unidades de um Podman rootless e é omitido para as do Podman rootful; `start` vira `stop` ou `restart` para essas ações. O Homeport verifica o nome do serviço e o coloca entre aspas, e nunca usa `sudo`. Então:

- As **unidades de sistema** (Podman rootful) precisam de um usuário SSH com permissão para gerenciá-las, como `root`.
- As **unidades de usuário** (Podman rootless) precisam da instância systemd do usuário. Para que ela rode sem uma sessão de login, e para que as unidades iniciem com o sistema, ative o lingering uma vez, como esse usuário, no host:

```bash
loginctl enable-linger "$USER"
```

A [página de segurança](/docs/reference/security/#o-que-o-homeport-executa-nos-seus-hosts) lista todos os comandos que o Homeport executa em um host.
