---
title: containerd
description: Conecte-se ao containerd nesta máquina ou via SSH, escolha um namespace, execute as tarefas dos seus contêineres, leia os logs deles, abra um shell e gerencie imagens e snapshots.
---

O **containerd** roda por baixo do k3s, do nerdctl, do BuildKit e de muitos servidores que não têm Docker. O Homeport se conecta a ele diretamente, pela sua própria API gRPC, como decide a [ADR 0020](https://github.com/DamienReichhart/Homeport/blob/main/docs/adr/0020-containerd-over-grpc.md). Um host containerd ganha páginas próprias na barra lateral: **Contêineres**, **Imagens**, **Snapshots** e **Namespaces**. As páginas do Docker, as stacks e o Quadro não são oferecidos nele.

## Adicionar um host containerd

Escolha **containerd** no campo **Engine** do formulário. Ele acessa o containerd apenas pelo seu socket, então a conexão é **Socket local** ou **SSH**; TCP e TLS ficam desativados, porque o containerd não serve a sua API pela rede.

| containerd                            | Socket                                                                         |
| ------------------------------------- | ------------------------------------------------------------------------------ |
| o serviço próprio do containerd       | `/run/containerd/containerd.sock`                                              |
| k3s                                   | `/run/k3s/containerd/containerd.sock`                                          |
| o containerd do próprio Docker Engine | `/run/containerd/containerd.sock`, com os seus contêineres no namespace `moby` |

No Linux, **Encontrados nesta máquina** lista o containerd deste computador e o do k3s quando eles respondem. Um socket que você ainda não pode usar é listado com a forma de corrigir isso.

### Permitir que o seu usuário use o socket

O socket do containerd pertence ao root. Para usá-lo como outro usuário, localmente ou via SSH, dê acesso a ele a um grupo em `/etc/containerd/config.toml` e adicione o usuário a esse grupo:

```bash
sudo groupadd -f containerd
sudo usermod -aG containerd "$USER"
getent group containerd | cut -d: -f3
```

Coloque o número exibido em `[grpc]` em `/etc/containerd/config.toml`, reinicie o containerd e faça login de novo:

```toml
[grpc]
  gid = 1001
```

```bash
sudo systemctl restart containerd
```

O **containerd rootless** (`containerd-rootless.sh`) mantém o seu socket dentro do namespace do RootlessKit, onde nada de fora consegue acessá-lo, nem mesmo o Homeport. O Homeport avisa quando o encontra; use o serviço de sistema do containerd no lugar.

### O que cada versão e cada conexão permitem

O Homeport se conecta ao containerd **1.7** e posteriores. O download passa pelo serviço Transfer do containerd, estável a partir da **2.0**.

| Ação                                                  | containerd        | Conexão                          |
| ----------------------------------------------------- | ----------------- | -------------------------------- |
| Namespaces, contêineres e tarefas, imagens, snapshots | 1.7 e posteriores | socket local ou SSH              |
| Baixar uma imagem                                     | 2.0 e posteriores | socket local ou SSH              |
| Logs e um terminal                                    | 1.7 e posteriores | SSH, ou um socket local no Linux |

Um containerd antigo demais para se conectar deixa o host em **Configuração incorreta**, indicando a sua versão; um antigo demais para baixar imagens avisa quando você baixa.

## Namespaces

Todo objeto do containerd vive em um **namespace**: o nerdctl usa `default`, o Kubernetes `k8s.io`, o Docker `moby`, o BuildKit `buildkit`. As páginas mostram um namespace de cada vez, escolhido abaixo do seletor de hosts, que lista cada um com quantos contêineres e imagens ele contém. O Homeport lembra a escolha de cada host entre uma execução e outra. Até você escolher um, o host mostra `default`, ou o único namespace que contém algo quando `default` está vazio, então um host k3s abre em `k8s.io`.

A página **Namespaces** cria namespaces vazios e remove os vazios. O containerd se recusa a remover um namespace que ainda contém algo, e o Homeport diz o que ele ainda contém: remova antes os contêineres, as imagens e os snapshots dele.

## Contêineres e tarefas

No containerd, um **contêiner** é apenas metadados: a imagem, a spec, o snapshot. O que roda é a sua **tarefa**. A página **Contêineres** lista os contêineres do namespace com a sua tarefa: **Em execução**, **Pausada**, **Parada** com o código de saída, ou **Sem tarefa**. Cada um diz qual ferramenta o criou — **Kubernetes** (com o seu Pod), **nerdctl**, **Docker** ou **ctr ou outro** — e os contêineres de pausa dos Pods ficam ocultos, a menos que você peça para vê-los.

| Ação             | O que o Homeport faz                                                                                                                                                                                                                                                                             |
| ---------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **Iniciar**      | Cria a tarefa sobre o snapshot do contêiner, registrando os logs onde o nerdctl registra, e a inicia. Uma tarefa parada é substituída, como `ctr task start` exige.                                                                                                                              |
| **Parar**        | Envia o sinal de parada da imagem (SIGTERM, a menos que ela indique outro) e depois SIGKILL após 10 segundos. Uma tarefa que ainda esteja viva depois disso é informada.                                                                                                                         |
| **Pausar**       | Congela a tarefa; **Retomar** a descongela.                                                                                                                                                                                                                                                      |
| **Enviar sinal** | Envia qualquer sinal, por nome (`SIGHUP`) ou número, ao processo principal da tarefa.                                                                                                                                                                                                            |
| **Remover**      | Pergunta antes; encerra uma tarefa em execução, remove a tarefa e o contêiner, e o snapshot dele com os arquivos que ele alterou, a menos que você desmarque **Remover também**: um snapshot mantido é marcado com `containerd.io/gc.root`, para que o coletor de lixo do containerd o mantenha. |

A página de um contêiner mostra o runtime, o snapshot, os rótulos e a spec OCI dele. O Homeport não cria contêineres no containerd: o containerd não tem rede própria, que o nerdctl e o CNI adicionam.

O **kubelet** gerencia os contêineres do Kubernetes: ele os inicia de novo depois de uma parada e os recria depois de uma remoção. As páginas deles dizem isso; altere o Pod no cluster dele. Remover daqui um contêiner do nerdctl deixa os registros próprios do nerdctl sobre ele (o nome e a rede), como o `ctr` faria; `nerdctl rm` é a forma mais limpa para esses.

## Logs

O containerd não mantém logs: a ferramenta que inicia uma tarefa decide para onde vai a saída dela. A aba **Logs** a lê onde essa ferramenta a gravou:

| Criado por              | Para onde vai a saída                                                                   | O que o Homeport mostra                                  |
| ----------------------- | --------------------------------------------------------------------------------------- | -------------------------------------------------------- |
| nerdctl                 | um arquivo de log JSON na pasta de estado do nerdctl, indicado pelos rótulos `nerdctl/` | esse arquivo, acompanhado                                |
| Kubernetes (`k8s.io`)   | `/var/log/pods/<pod>/<container>/N.log`, indicado no registro do CRI sobre ele          | esse arquivo, acompanhado                                |
| `ctr` e outros clientes | para onde eles a apontaram, normalmente para lugar nenhum                               | **A saída deste contêiner não é registrada**, e o motivo |

Os contêineres do nerdctl que registram logs em outro driver (journald, fluentd) indicam esse driver no lugar. O Homeport lê o arquivo com `tail` na máquina do host (veja [abaixo](#o-que-roda-no-host)); os arquivos de log normalmente pertencem ao root, então o usuário SSH precisa ter permissão para lê-los.

## Um terminal em um contêiner

A aba **Exec** abre um shell em um contêiner em execução — bash quando a imagem o tem, sh caso contrário — ou o comando que você informar, como um usuário numérico (`uid` ou `uid:gid`) e em uma pasta de sua escolha. **Abrir novamente** inicia um novo com o que você digitou.

O containerd recebe a entrada e a saída de um processo como caminhos na sua máquina. Então o Homeport executa ali um script curto que cria dois FIFOs em uma pasta temporária privada e copia o terminal por eles; o containerd inicia o processo nesses FIFOs, a partir da spec de processo do próprio contêiner. Fechar a aba encerra o processo e remove a pasta.

## Imagens

A página **Imagens** lista as imagens do namespace com as suas plataformas, os contêineres criados a partir delas e o tamanho delas para a plataforma do host, como `ctr images ls` o calcula.

**Baixar imagem** busca uma imagem pelo próprio containerd e depois a descompacta para a plataforma do host no snapshotter padrão dele, para que `nerdctl run` possa usá-la na hora. Informe uma referência completa (`ghcr.io/home/app:1.2`) ou curta (`nginx`, que vira `docker.io/library/nginx:latest`). A imagem de um registro privado é baixada com o login dele de **Configurações → Registros**, enviado ao containerd apenas quando o registro o pede, e nunca armazenado no host.

Uma imagem a partir da qual um contêiner do namespace foi criado não pode ser removida; remova antes os contêineres. O containerd libera o conteúdo de uma imagem assim que nada mais o retém.

## Snapshots

Os snapshots são onde vai o disco do containerd: as camadas das imagens descompactadas e a camada gravável de cada contêiner. A página **Snapshots** mostra os de um snapshotter (`overlayfs` primeiro, ou qualquer outro que o host tenha carregado) como uma árvore por pai, os maiores primeiro, com o uso deles como `ctr snapshot usage` o calcula e o total no topo.

Cada um diz o que o usa: um contêiner, pelo seu snapshot, ou uma imagem, pelas suas camadas. Os snapshots que nada usa e sobre os quais nada é construído — como um mantido quando o seu contêiner foi removido daqui — podem ser mostrados sozinhos com **Somente sem uso** e removidos. Os outros não podem ser removidos daqui. Um snapshot que nada referencia e que nada marca como raiz, como um que `ctr containers rm --keep-snapshot` deixa, não dura: o coletor de lixo do containerd o remove na próxima execução.

## O que roda no host

Tudo o que está acima passa pela API do containerd, exceto os logs e os terminais, que precisam dos arquivos da máquina do containerd. Para esses, o Homeport executa estes comandos, pela sessão SSH que ele já mantém, ou neste computador para um socket local no Linux, com os direitos do usuário SSH e nunca com `sudo`:

| Para        | Comando                                                                                                                           |
| ----------- | --------------------------------------------------------------------------------------------------------------------------------- |
| Logs        | `tail -n <lines> [-F] -- '<log file>'`; para um contêiner do Kubernetes sem o registro do CRI, antes um `ls` da sua pasta de logs |
| Um terminal | `mktemp -d`, `mkfifo -m 600` para dois FIFOs nessa pasta, `cat` por eles e `rm -rf` da pasta                                      |

Um socket local no macOS ou no Windows não pode executá-los, e os logs e terminais então dizem que precisam de SSH. A [página de segurança](/docs/reference/security/#o-que-o-homeport-executa-nos-seus-hosts) lista todos os comandos que o Homeport executa em um host.

## Eventos

As páginas se atualizam a partir dos próprios eventos do containerd, em todos os namespaces: uma tarefa iniciando ou terminando, um contêiner ou imagem criado ou removido, um snapshot gravado. O containerd não mantém histórico deles, então o que aconteceu enquanto o Homeport estava fechado não é preenchido.
