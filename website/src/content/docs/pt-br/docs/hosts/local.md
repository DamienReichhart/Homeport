---
title: Engines locais
description: Conecte-se ao Docker Engine, Docker Desktop, Colima, OrbStack, Rancher Desktop ou Podman nesta máquina.
---

Um host **Socket local** conversa com um Engine nesta máquina pelo seu socket Unix, ou por um named pipe no Windows. Ele não precisa de credenciais nem de chaveiro.

## Detecção automática

O Homeport procura Engines em execução nos locais habituais e lista os que respondem em **Encontrados nesta máquina**, na tela de boas-vindas e na página **Adicionar host**. Escolha **Adicionar** ao lado de um para criar um host com o nome dele.

| Runtime                  | macOS e Linux                                                                                                    | Windows                           |
| ------------------------ | ---------------------------------------------------------------------------------------------------------------- | --------------------------------- |
| Docker Engine            | `/var/run/docker.sock`                                                                                           | —                                 |
| Docker Engine (rootless) | `$XDG_RUNTIME_DIR/docker.sock`                                                                                   | —                                 |
| Docker Desktop           | `~/.docker/desktop/docker.sock`, `~/.docker/run/docker.sock`                                                     | `//./pipe/docker_engine`          |
| Colima                   | `~/.colima/default/docker.sock`                                                                                  | —                                 |
| OrbStack                 | `~/.orbstack/run/docker.sock`                                                                                    | —                                 |
| Rancher Desktop          | `~/.rd/docker.sock`                                                                                              | —                                 |
| Podman                   | `$XDG_RUNTIME_DIR/podman/podman.sock`, `/run/podman/podman.sock`; [máquinas no macOS](#máquinas-podman-no-macos) | `//./pipe/podman-machine-default` |

Cada candidato é verificado pedindo ao Engine a sua versão, então só os Engines em execução são listados. Quando vários caminhos levam ao mesmo socket — no macOS, `/var/run/docker.sock` normalmente aponta para o Docker Desktop, o OrbStack ou o Rancher Desktop —, o Engine é listado uma vez, com o primeiro nome encontrado.

### Máquinas Podman no macOS

No macOS, o Podman roda o seu Engine em uma VM (`podman machine`) cujo socket muda com cada máquina. O Homeport pede à CLI `podman` o socket de cada máquina em execução (`podman machine list` e `podman machine inspect`) e lista cada uma como **Podman (_nome da máquina_)**, por exemplo **Podman (podman-machine-default)**. Máquinas paradas não são listadas; inicie uma com `podman machine start` e abra **Adicionar host** de novo.

Se o Homeport não conseguir executar o `podman` — ele não está no PATH nem em `/opt/homebrew/bin` ou `/usr/local/bin` —, ele procura onde o Podman coloca os sockets: `$TMPDIR/podman/<machine>-api.sock` para o Podman 5 e `~/.local/share/containers/podman/machine/…/podman.sock` para o Podman 4. Quando o `podman-mac-helper` vinculou `/var/run/docker.sock` a uma máquina, essa máquina é listada uma vez, com o nome Podman dela.

## Adicionar um socket manualmente

Se o seu Engine não for detectado — um perfil do Colima diferente de `default`, uma máquina Podman no Windows com um nome diferente do padrão ou um caminho personalizado —, adicione-o você mesmo:

1. Escolha **Adicionar host…** no seletor de hosts.
2. Selecione **Socket local**.
3. Digite um **Nome** e o **Caminho do socket**: um socket Unix como `/run/user/1000/docker.sock`, ou um named pipe como `//./pipe/docker_engine` no Windows.
4. Escolha **Adicionar host**.

Para encontrar o socket que a sua Docker CLI usa, execute:

```sh
docker context inspect --format '{{.Endpoints.docker.Host}}'
```

Para uma máquina Podman:

```sh
podman machine inspect --format '{{.ConnectionInfo.PodmanSocket.Path}}'
```

## Permissões no Linux

O socket padrão `/var/run/docker.sock` pertence ao grupo `docker`. Se o seu usuário não estiver nele, o host mostra **Inacessível** com _O Homeport não tem permissão para usar o Docker Engine em /var/run/docker.sock._ Adicione-se ao grupo e faça login de novo:

```sh
sudo usermod -aG docker $USER
```

Ser membro do grupo `docker` equivale a ser root naquela máquina; o Docker rootless ou o Podman evitam isso, se isso for importante para você.
