---
title: Glossário
description: As palavras que o Homeport usa e o que elas significam.
---

O Homeport dá nomes precisos às coisas. Estes termos aparecem em todo o aplicativo e nesta documentação. Cada entrada traz o termo do produto em inglês, seguido da palavra que o aplicativo usa em português do Brasil.

### Host (pt-BR: _host_)

Um Docker Engine ao qual o Homeport se conecta, definido por um nome e pela forma de acessá-lo: um socket local, TCP, TLS ou SSH. Cada contêiner, imagem, volume, rede e stack pertence a exatamente um host. Veja [Hosts e conexões](/docs/hosts/).

### Engine kind (pt-BR: _tipo de Engine_)

O tipo de Engine para o qual um host aponta: compatível com Docker (Docker Engine, ou Podman pela sua API compatível), containerd ou Incus. Ele decide quais páginas um host mostra: os hosts containerd e Incus ganham páginas próprias. O Podman não é um tipo à parte: o Homeport o reconhece cada vez que se conecta. Veja [Tipos de Engine](/docs/hosts/#tipos-de-engine).

### Namespace (containerd's) (pt-BR: _namespace do containerd_)

Em um host containerd, o espaço em que vive cada contêiner, imagem e snapshot: o nerdctl usa `default`, o Kubernetes `k8s.io`, o Docker `moby`. As páginas de um host containerd mostram um de cada vez. Os Namespaces de um cluster são os do Kubernetes. Veja [namespaces do containerd](/docs/hosts/containerd/#namespaces).

### Snapshot (pt-BR: _snapshot_)

Em um host containerd, uma camada de arquivos em disco: uma camada de imagem descompactada, ou a camada gravável de um contêiner. Veja [snapshots](/docs/hosts/containerd/#snapshots). Em um host Incus, um estado salvo de uma instância, para o qual ela pode ser restaurada; veja [snapshots de instância](/docs/guides/instances/#snapshots).

### Instance (pt-BR: _instância_)

O que um host Incus executa: um contêiner de sistema, que inicializa uma distribuição Linux inteira sobre o kernel do host, ou uma máquina virtual com kernel próprio. Não é um contêiner, que executa um único aplicativo. Veja [Instâncias](/docs/guides/instances/).

### Project (Incus's) (pt-BR: _projeto_)

Em um host Incus, o espaço em que vivem as suas instâncias, imagens, perfis, volumes personalizados e redes. As páginas de um host Incus mostram um de cada vez, escolhido abaixo do seletor de hosts. Veja [projetos](/docs/hosts/incus/#projetos).

### Profile (pt-BR: _perfil_)

Em um host Incus, configurações e dispositivos compartilhados pelas instâncias que o listam. Veja [perfis](/docs/guides/instances/#perfis).

### Trust token (pt-BR: _token de confiança_)

O que `incus config trust add` exibe em um servidor Incus, colado uma vez no Homeport para adicionar esse servidor via HTTPS. Veja [tokens de confiança](/docs/hosts/incus/#via-https-com-um-token-de-confiança).

### Pod (pt-BR: _pod do Podman_)

Em um host Podman, um grupo de contêineres que compartilham uma mesma rede e iniciam e param juntos, em torno de um contêiner infra que mantém as portas que o pod publica. Os Pods de um cluster são Pods do Kubernetes. Veja [Pods do Podman](/docs/guides/podman-pods/).

### Quadlet unit (pt-BR: _unidade Quadlet_)

Um arquivo em um host Podman, como `web.container`, que o systemd transforma em um serviço, `web.service`, para que os contêineres do Podman iniciem com o sistema e o systemd os mantenha rodando. Veja [Unidades Quadlet](/docs/guides/quadlet/).

### Cluster (pt-BR: _cluster_)

Um cluster Kubernetes ou um cluster Nomad ao qual o Homeport se conecta pela API dele. Um cluster Kubernetes é adicionado a partir de um contexto de um kubeconfig: um nome, o endereço do servidor de API e as credenciais que o kubeconfig fornece, armazenadas criptografadas. Um cluster Nomad é adicionado pelo endereço de um dos seus agentes, com um token de ACL e o material TLS, armazenados criptografados. O Homeport não instala nada em nenhum dos dois e só alcança o que essas credenciais permitem. Não é um Swarm, que é encontrado pelos seus hosts e nunca adicionado. Os objetos próprios do Kubernetes mantêm os nomes do Kubernetes: Namespace, Pod, Deployment, Service, Node… Os do Nomad mantêm os do Nomad: job, grupo de tarefas, tarefa, alocação, nó cliente, namespace. Veja [Clusters](/docs/clusters/) e [Clusters Nomad](/docs/clusters/nomad/).

### Allocation (pt-BR: _alocação_)

Em um cluster Nomad, uma colocação do grupo de tarefas de um job em um nó cliente, que executa as suas tarefas. Os logs dela e um shell nela passam pelos servidores do Nomad até o nó. Veja [logs e shell](/docs/clusters/nomad/#logs-e-shell).

### Swarm (pt-BR: _Swarm_)

Um cluster Docker Swarm, encontrado pelos hosts que pertencem a ele e nunca adicionado por si só. O Homeport o acessa por qualquer host manager conectado, distingue os Swarms pelo ID de swarm e não armazena nada sobre nenhum deles. Ele recebe o nome do seu primeiro host manager, já que o Docker dá o mesmo nome a todos os Swarms. Veja [Swarms](/docs/guides/swarm/).

### Port forward (pt-BR: _encaminhamento de porta_)

Uma porta no loopback deste computador que o Homeport conecta, pela API do Kubernetes, a uma porta de um Pod, ou de um Service por meio de um dos seus Pods prontos, como `kubectl port-forward`. Ele termina quando o Homeport é encerrado. Veja [Encaminhamento de portas](/docs/clusters/port-forward/).

### Release (pt-BR: _release_)

Uma release do Helm: uma instalação de um chart em um cluster, com um nome no seu Namespace, com uma revisão numerada para cada instalação, atualização e reversão. O Helm a mantém no cluster; uma release feita em um terminal e uma feita no Homeport são a mesma coisa. Veja [Releases do Helm](/docs/clusters/helm/).

### Chart repository (pt-BR: _repositório de charts_)

Um servidor do qual o helm baixa charts, na lista própria do helm neste computador — a que `helm repo list` mostra. Os charts em um registro OCI são instalados pela sua referência `oci://`. Veja [repositórios de charts](/docs/clusters/helm/#repositórios-de-charts).

### Node (pt-BR: _nó_)

Um Engine em um Swarm: um manager ou um worker. Os Nodes de um cluster são Nodes do Kubernetes, as máquinas em que ele executa Pods. Um nó só é um host quando o Homeport se conecta a ele, e a página de um host tem um link para o nó dele.

### Service (pt-BR: _serviço_)

O que um Swarm executa: uma imagem, as suas configurações e um número de réplicas, ou uma tarefa em cada nó para um serviço global.

### Task (pt-BR: _tarefa_)

Uma vaga de um serviço em um nó, executada ali como um contêiner. Quando um serviço não converge, as tarefas dele dizem por quê. Em um host containerd, uma tarefa é o que roda para um contêiner, que é apenas metadados: iniciar um contêiner cria a sua tarefa.

### Stack (pt-BR: _stack_)

Um projeto Compose em execução em um host, ou uma stack swarm em execução em um Swarm, identificada pelo nome do projeto. Toda stack é gerenciada, vinculada, sincronizada ou externa. Veja [Stacks](/docs/guides/stacks/) e [stacks swarm](/docs/guides/swarm/#stacks-do-swarm).

### Managed Stack (pt-BR: _stack gerenciada_)

Uma stack cuja definição compose e cujo ambiente o Homeport guarda, para que ela possa ser implantada, reimplantada e editada a partir do Homeport.

### Linked Stack (pt-BR: _stack vinculada_)

Uma stack cujos arquivos compose são uma pasta deste computador. O Homeport guarda onde estão a pasta e os seus arquivos compose, lê e grava os arquivos no lugar e implanta com essa pasta como diretório do projeto, para que `build:`, `env_file:` e os caminhos relativos funcionem. É encontrada pelos rótulos compose dos contêineres em execução, ou criada a partir de uma pasta. Ela se torna externa enquanto os seus arquivos estão faltando. Veja [Stacks vinculadas](/docs/guides/stacks/#stacks-vinculadas).

### Synced Stack (pt-BR: _stack sincronizada_)

Uma stack cuja definição compose vem de um repositório git. O Homeport mantém um clone dele neste computador, busca uma branch ou tag sob demanda ou em um intervalo enquanto o Homeport está aberto, mostra o que mudou e implanta um commit a partir da pasta do arquivo compose, para que `build:` e os caminhos relativos funcionem. Ela nunca é editada no Homeport: os seus arquivos mudam pelo git; só o seu ambiente e as suas configurações de sincronização são do Homeport. Veja [Stacks sincronizadas](/docs/guides/synced-stacks/).

### Repository (pt-BR: _repositório_)

Um repositório git que o Homeport pode ler para as stacks sincronizadas: a URL dele e como o Homeport se autentica nele — uma chave de deploy que ele gerou, um nome de usuário e um token, ou nada para um público. Guardado uma vez para todos os hosts e compartilhado por todas as stacks sincronizadas criadas a partir dele.

### Deploy key (pt-BR: _chave de deploy_)

A chave Ed25519 que o Homeport gera para um repositório, que você adiciona a ele na sua forja git apenas com acesso de leitura. Nunca a sua própria chave SSH.

### External Stack (pt-BR: _stack externa_)

Uma stack encontrada em execução em um host cuja definição compose o Homeport não tem, nem armazenada, nem em uma pasta deste computador, nem em um repositório que ele acompanha. Ela pode ser iniciada, parada e removida, mas não reimplantada nem editada.

### Adopt (pt-BR: _adotar_)

Transformar uma stack externa em uma stack gerenciada, fornecendo a sua definição compose.

### Registry login (pt-BR: _login de registro_)

O nome de usuário e a senha ou o token de acesso que o Homeport guarda para um registro de contêineres, como o Docker Hub ou o GHCR, e usa em todos os hosts para baixar, enviar e fazer build. Veja [Configurações](/docs/guides/settings/#registros).

### Helper container (pt-BR: _contêiner auxiliar_)

Um contêiner `busybox` de curta duração que o Homeport cria em um host para abrir um volume — para navegar por ele, fazer backup, restaurá-lo ou cloná-lo —, já que a API do Engine não consegue ler um. Ele não tem rede, monta o volume como somente leitura, a menos que precise gravar nele, e é removido quando a ação termina. Ele nunca aparece na lista Contêineres nem na página Eventos. Veja [Navegar pelos arquivos de um volume](/docs/guides/volumes-networks/#o-contêiner-auxiliar).

### Board (pt-BR: _Quadro_)

A página que lista os contêineres de todos os hosts conectados em uma única tabela, cada um marcado com o seu host. Veja [Quadro](/docs/guides/overview/#quadro).

### Command palette (pt-BR: _paleta de comandos_)

A caixa de busca aberta com <kbd>Mod</kbd> <kbd>K</kbd>: ir para uma página, trocar de host, executar um comando ou encontrar um contêiner, imagem, volume, rede ou stack em qualquer host conectado. Veja [Teclado](/docs/guides/keyboard/#paleta-de-comandos).

### Engine (pt-BR: _Engine_)

O Docker Engine (`dockerd`), ou um compatível como o Podman, que executa contêineres em uma máquina. O Homeport conversa com ele pela API do Docker Engine.

### Socket (pt-BR: _socket_)

O endereço em que um Engine escuta: um arquivo de socket Unix como `/var/run/docker.sock`, ou um named pipe como `//./pipe/docker_engine` no Windows.

### Keychain (pt-BR: _chaveiro do sistema_)

O armazenamento seguro de senhas e chaves do seu sistema operacional — as Chaves (Keychain) do macOS, o Gerenciador de Credenciais do Windows ou um Secret Service do Linux —, onde o Homeport guarda a chave que criptografa os seus segredos.

### Trust on first use (pt-BR: _confiança no primeiro uso_)

Como o Homeport verifica os servidores SSH: você confirma a impressão digital da chave de um servidor na primeira conexão, e o Homeport se recusa a se conectar se a chave mudar depois, até que você confirme de novo.

## Em português do Brasil

O aplicativo, a documentação e o site em português do Brasil usam estes termos, para que uma tela e uma página digam sempre a mesma coisa.

| Inglês         | Português do Brasil          |
| -------------- | ---------------------------- |
| Host           | host (m.)                    |
| Engine         | Engine (m.)                  |
| Instance       | instância (f.)               |
| Swarm          | Swarm (m.)                   |
| Cluster        | cluster (m.)                 |
| Stack          | stack (f.)                   |
| Managed Stack  | stack gerenciada             |
| Linked Stack   | stack vinculada              |
| Synced Stack   | stack sincronizada           |
| External Stack | stack externa                |
| Adopt          | adotar                       |
| Board          | Quadro                       |
| Bastion        | bastion (m.)                 |
| Deploy key     | chave de deploy (f.)         |
| Port forward   | encaminhamento de porta (m.) |
| Release (Helm) | release (f.)                 |
