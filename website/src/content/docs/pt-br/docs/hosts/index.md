---
title: Hosts e conexões
description: Como o Homeport se conecta a Docker Engines, ao containerd e ao Incus, o que significa cada status de host e como funciona a reconexão.
---

Um **host** é um Docker Engine, containerd ou Incus ao qual o Homeport se conecta. Ele tem um nome e uma forma de ser acessado. Cada contêiner, instância, imagem, volume, rede e stack que você vê pertence a exatamente um host.

## Tipos de conexão

| Tipo             | Acessa um Engine…                                      | Credenciais                                                                            | Guia                                                           |
| ---------------- | ------------------------------------------------------ | -------------------------------------------------------------------------------------- | -------------------------------------------------------------- |
| **Socket local** | nesta máquina, por um socket ou pipe                   | nenhuma                                                                                | [Engines locais](/docs/hosts/local/)                           |
| **SSH**          | em um servidor no qual você pode fazer login           | uma chave do Homeport, a sua própria chave ou o ssh-agent                              | [SSH](/docs/hosts/ssh/)                                        |
| **TLS**          | exposto com `dockerd --tlsverify`, ou o HTTPS do Incus | certificado de CA, certificado e chave do cliente; para o Incus, um token de confiança | [TLS e TCP](/docs/hosts/tls-tcp/), [Incus](/docs/hosts/incus/) |
| **TCP**          | exposto sem criptografia, em uma rede confiável        | nenhuma                                                                                | [TLS e TCP](/docs/hosts/tls-tcp/)                              |

Para um servidor remoto, prefira **SSH**: ele não precisa de nada exposto além da porta SSH e é o mais simples de configurar com segurança.

## Tipos de Engine

Além de como acessá-lo, cada host diz para que tipo de Engine ele aponta, no campo **Engine** do formulário. Isso decide quais páginas a barra lateral mostra para aquele host.

| Engine                    | O que abrange                                                                    | Páginas                                                                                                |
| ------------------------- | -------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------ |
| **Compatível com Docker** | Docker Engine, Docker Desktop, Colima, OrbStack, Rancher Desktop, Podman         | Contêineres, Imagens, Volumes, Redes, Stacks, Eventos; e, no Podman, também as páginas próprias dele   |
| **containerd**            | o serviço próprio do containerd, o do k3s, o containerd do próprio Docker Engine | Contêineres, Imagens, Snapshots e Namespaces do containerd; veja [containerd](/docs/hosts/containerd/) |
| **Incus**                 | Incus 6.0 e posteriores: contêineres de sistema e máquinas virtuais              | Instâncias, Imagens, Perfis, Armazenamento e Redes; veja [Incus](/docs/hosts/incus/)                   |

O formulário escolhe o Engine a partir do caminho do socket quando ele indica o socket do containerd ou do Incus. O containerd só é acessado pelo seu socket, neste computador ou via SSH, então TCP e TLS ficam desativados para ele; o Incus, pelo seu socket ou via HTTPS (TLS), então só o TCP fica desativado. Os hosts adicionados antes do Homeport 0.18 são compatíveis com Docker, sem nenhuma outra mudança.

O **Podman** é um Engine compatível com Docker que o Homeport reconhece cada vez que se conecta, porque o mesmo socket pode trocar de produto: o status do host passa a mostrar **Podman** e a sua versão em vez dos do Engine, e a barra lateral adiciona as páginas que só o Podman tem. Uma página que o host selecionado não pode mostrar não fica na barra lateral, e abri-la a partir de um link diz por quê.

Um host que pertence a um Docker Swarm mostra **Manager de** ou **Worker de** o seu Swarm, e adicionar um manager como host basta para o Swarm aparecer na barra lateral. A página de um host também tem um cartão **Swarm** para iniciar um Swarm, entrar em um ou sair dele. Veja [Swarms](/docs/guides/swarm/).

## Adicionar, editar e remover hosts

- **Adicione** um host com **Adicionar host…** no seletor de hosts, no topo da barra lateral, ou com **Adicionar host** na Visão geral.
- **Gerenciar hosts**, no seletor de hosts, lista todos os hosts com o tipo, o endereço e o status. Selecione um para abrir a página dele.
- Na página de um host, você pode mudar os detalhes da conexão e escolher **Salvar**; o Homeport se reconecta com as novas configurações. O tipo de conexão de um host não pode ser mudado — remova-o e adicione-o de novo.
- O interruptor **Ativado** desconecta um host sem esquecê-lo. Um host desativado não tenta de novo até que você o ative outra vez.
- **Remover host** esquece o host, as suas credenciais armazenadas, as definições compose das suas stacks gerenciadas, as suas stacks vinculadas e o histórico de implantação das suas stacks. Nada que roda no host é tocado: contêineres, imagens e volumes ficam como estão, assim como as pastas das stacks vinculadas. As stacks de um Swarm que o host gerencia pertencem ao Swarm e são mantidas.

Os nomes dos hosts devem ser únicos, sem diferenciar maiúsculas de minúsculas.

## Importar hosts

**Adicionar host** oferece duas formas de adicionar muitos hosts de uma vez, a partir do que este computador já descreve. Cada uma abre uma prévia: escolha as entradas a importar, renomeie-as e escolha **Importar**.

- **Importar de ~/.ssh/config** adiciona os servidores que a sua configuração SSH indica, com o usuário, a porta, a chave e os bastions `ProxyJump`. Veja [Importar de ~/.ssh/config](/docs/hosts/ssh/#importar-de-sshconfig).
- **Importar contextos Docker** adiciona os Engines dos contextos da Docker CLI (`docker context ls`), com os seus certificados TLS.

As chaves e os certificados são lidos uma vez, quando você importa, e armazenados criptografados como qualquer outro; nem `~/.ssh` nem `~/.docker` são alterados.

### Contextos Docker

O Homeport lê todos os contextos em `~/.docker/contexts` (ou em `$DOCKER_CONFIG`, quando definido) e mapeia o endpoint Docker de cada um:

| Endpoint                                               | Vira                                                            |
| ------------------------------------------------------ | --------------------------------------------------------------- |
| `unix:///…`, `npipe:////./pipe/…`                      | um host **Socket local**                                        |
| `tcp://host:port` sem arquivos TLS                     | um host **TCP**                                                 |
| `tcp://host:port` com `ca.pem`, `cert.pem` e `key.pem` | um host **TLS**, com os seus certificados                       |
| `ssh://user@host:port`                                 | um host **SSH** que usa o ssh-agent, com `/var/run/docker.sock` |

O contexto `default` embutido não é oferecido: a [detecção local](/docs/hosts/local/) já encontra o Engine desta máquina. Um contexto é mostrado, mas não pode ser escolhido quando:

- ele define `SkipTLSVerify`, já que o Homeport sempre verifica o certificado do Engine;
- a pasta TLS dele contém só alguns dos três arquivos;
- o endpoint dele é outra coisa, como um do Kubernetes;
- um host já acessa o mesmo Engine. O nome dele diz qual.

Um contexto cujo nome já é o de um host é oferecido como `name (2)`.

## Mudar para outro computador

**Exportar**, na página Hosts, salva os hosts que você escolher em um arquivo protegido por uma senha; **Importar**, na página Hosts do outro computador, abre esse arquivo e adiciona os hosts que você escolher, prontos para conectar. Um host cujo nome já está em uso lá é proposto como `name (2)`, e você pode renomeá-lo ou pulá-lo. Veja [mudar para outro computador](/docs/reference/security/#mudar-para-outro-computador) para saber como o arquivo é protegido. Para levar tudo de uma vez, incluindo clusters, stacks, logins de registro, repositórios e configurações, use um [backup](/docs/reference/data-and-backups/#fazendo-backup-no-homeport).

## Selecionar um host

Toda página de recursos — Contêineres, Imagens, Volumes, Redes, Stacks e Eventos — mostra o host selecionado no **seletor de hosts**. As páginas de um host containerd também mostram um namespace de cada vez, escolhido [abaixo do seletor](/docs/hosts/containerd/#namespaces), e as de um host Incus, um [projeto](/docs/hosts/incus/#projetos). Troque de host ali, ou selecione o cartão de um host na [Visão geral](/docs/guides/overview/). O Homeport lembra a sua escolha entre uma execução e outra.

## Status

Cada host mostra um status ao lado do nome. Quando o host selecionado não está conectado, as páginas de recursos mostram o status dele com **Tentar agora** e **Abrir configurações do host**.

| Status                              | Significado                                                                              | O que acontece em seguida                                                          |
| ----------------------------------- | ---------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------- |
| **Conectado**                       | O Engine respondeu. A versão dele é mostrada, ou a do Podman, do containerd ou do Incus. | O Homeport continua observando a conexão.                                          |
| **Conectando**                      | O Homeport está acessando o Engine.                                                      | —                                                                                  |
| **Desativado**                      | O interruptor **Ativado** do host está desligado.                                        | Nada até que você o ligue.                                                         |
| **Inacessível**                     | A rede, o socket ou o Engine não respondeu.                                              | Tenta de novo sozinho, esperando mais a cada vez.                                  |
| **Acesso recusado**                 | O servidor SSH recusou a chave, o ssh-agent falhou ou o handshake TLS falhou.            | Corrija a causa e depois use **Reconectar**.                                       |
| **Chave do servidor não confiável** | Primeira conexão SSH: a chave do servidor ou de um bastion precisa da sua aprovação.     | [Confie na chave](/docs/hosts/ssh/#confie-na-chave-do-servidor) na página do host. |
| **A chave do servidor mudou**       | O servidor SSH apresentou uma chave diferente daquela em que você confiou.               | [Investigue primeiro](/docs/hosts/ssh/#quando-a-chave-do-servidor-muda).           |
| **Credenciais perdidas**            | As credenciais armazenadas não podem ser descriptografadas.                              | [Digite-as de novo](/docs/reference/troubleshooting/#credenciais-perdidas).        |
| **Chaveiro indisponível**           | O host precisa de segredos armazenados, mas não há chaveiro do sistema.                  | Veja os [requisitos](/docs/requirements/#opcional-um-chaveiro-do-sistema).         |
| **Configuração incorreta**          | Algo nas configurações do host está faltando ou é inválido.                              | A mensagem diz o quê; corrija e use **Salvar**.                                    |

## Reconexão

O Homeport se conecta a todos os hosts ativados quando inicia, e cada host tem a sua própria conexão, então um servidor lento nunca atrasa outro.

Enquanto está conectado, o Homeport escuta o fluxo de eventos do Engine — é assim que as páginas se atualizam sozinhas — e verifica o Engine a cada 15 segundos. Se a conexão cair, o host fica **Inacessível** e o Homeport tenta de novo depois de 1 segundo, depois 2, 4, 8 e assim por diante, até uma vez por minuto. Uma conexão bem-sucedida zera a espera.

Só o status **Inacessível** tenta de novo automaticamente. Todo outro problema precisa de algo da sua parte, e então **Reconectar** (na página do host) ou **Tentar agora** tenta de novo na hora.

## Eventos e atualizações ao vivo

O Homeport não faz polling. Quando o Engine informa uma mudança — um contêiner iniciando, uma imagem sendo baixada —, as páginas afetadas se atualizam em uma fração de segundo. A página [Eventos](/docs/guides/overview/#eventos) mostra o fluxo bruto.
