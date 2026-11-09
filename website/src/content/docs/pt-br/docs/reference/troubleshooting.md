---
title: Solução de problemas
description: Correções para os problemas que você tem mais chance de encontrar, pela mensagem que o Homeport mostra.
---

Se o seu problema não estiver aqui, confira o [arquivo de log](/docs/reference/security/#logs) e [abra uma issue](https://github.com/DamienReichhart/Homeport/issues) com a sua versão do Homeport, que aparece em **Configurações**.

## Instalação e inicialização

### O Windows diz "O Windows protegeu o computador"

O instalador ainda não é assinado com um certificado comercial. Escolha **Mais informações** e depois **Executar assim mesmo**. Você pode [verificar o download](/docs/reference/verifying/) antes.

### O macOS diz que o Homeport "não pode ser aberto"

O Homeport ainda não é notarizado pela Apple. Abra **Ajustes do Sistema → Privacidade e Segurança** e escolha **Abrir Mesmo Assim** ao lado do Homeport, ou execute:

```sh
xattr -dr com.apple.quarantine /Applications/Homeport.app
```

### O AppImage não inicia

Se ele mencionar FUSE, instale o `libfuse2` (`libfuse2t64` no Ubuntu 24.04 e posteriores). Confira se o arquivo é executável: `chmod +x Homeport_*.AppImage`.

### O Homeport abre no lugar errado

O Homeport reabre com o tamanho, a posição e o estado maximizado que a janela dele tinha quando você a fechou. Se a tela em que ele estava não existir mais, ele abre centralizado em uma que restou. Para esquecer o lugar salvo, encerre o Homeport e exclua `.window-state.json` da pasta de configurações dele:

| Sistema | Arquivo                                                             |
| ------- | ------------------------------------------------------------------- |
| Linux   | `~/.config/com.homeport.app/.window-state.json`                     |
| macOS   | `~/Library/Application Support/com.homeport.app/.window-state.json` |
| Windows | `%APPDATA%\com.homeport.app\.window-state.json`                     |

### Abrir o Homeport de novo não abre uma segunda janela

Só um Homeport roda de cada vez: abri-lo enquanto ele já está rodando traz a janela dele para a frente, mesmo quando ele está oculto na [bandeja do sistema](/docs/guides/overview/#system-tray). Duas cópias se conectariam duas vezes a cada host e gravariam nos mesmos dados.

### Não há ícone na bandeja

No Linux, a bandeja precisa da biblioteca appindicator, e alguns ambientes de desktop ocultam os ícones da bandeja: no GNOME, instale a extensão AppIndicator Support. Se o Homeport não encontrar nenhuma biblioteca appindicator, **Configurações → Área de trabalho** diz isso, e fechar a janela encerra o aplicativo. Veja os [requisitos](/docs/requirements/#optional-a-system-tray).

### Um banner vermelho diz que nenhum chaveiro do sistema está disponível

O Homeport não conseguiu acessar o seu chaveiro. No Linux, isso normalmente significa que nenhum Secret Service está rodando — comum em desktops mínimos e gerenciadores de janelas. Instale e inicie o GNOME Keyring (`gnome-keyring`) ou ative o Secret Service do KWallet e depois reinicie o Homeport. Até lá, use hosts de socket local, TCP ou ssh-agent.

## Atualizações e o repositório de pacotes

### "Não foi possível verificar atualizações"

O Homeport não conseguiu baixar as informações de atualização deste site: você pode estar offline, ou atrás de um proxy ou firewall que bloqueia `damienreichhart.github.io`. O Homeport continua funcionando; ele verifica de novo mais tarde, ou escolha **Configurações → Atualizações → Verificar agora**. Você sempre pode baixar a versão mais recente na [página de download](/download/).

### Uma atualização não consegue ser instalada

O Homeport só instala uma atualização cuja assinatura corresponda à sua chave de atualização, e para se o download estiver incompleto ou alterado. Tente de novo mais tarde; se continuar falhando, baixe a nova versão na [página de download](/download/) e instale-a por cima da antiga.

### O apt diz "NO_PUBKEY" ou "The repository is not signed"

A chave do Homeport está faltando em `/etc/apt/keyrings/homeport.asc`. Execute de novo a [instalação rápida](/docs/installation/#quick-install), ou os dois primeiros comandos da [configuração manual](/docs/installation/#manual-setup).

### O dnf ou o zypper pergunta se deve importar uma chave

É a chave de pacotes do Homeport, na primeira vez que o seu gerenciador de pacotes usa o repositório. Confira se a impressão digital que ele mostra corresponde à de [verificar downloads](/docs/reference/verifying/#package-signatures) e depois aceite.

### O apt diz "Hash Sum mismatch" logo depois de uma release

O repositório estava sendo atualizado enquanto o apt o lia. Execute `sudo apt update` de novo alguns minutos depois.

### O meu gerenciador de pacotes não vê a nova versão

Confira se o repositório ainda está configurado: `/etc/apt/sources.list.d/homeport.sources`, `/etc/yum.repos.d/homeport.repo` ou `/etc/zypp/repos.d/homeport.repo`. Se você o excluiu, execute a [instalação rápida](/docs/installation/#quick-install) para adicioná-lo de novo. Se você instalou a 0.2.0 ou anterior a partir de um arquivo baixado, o repositório nunca foi adicionado: veja [atualizar a partir da 0.2.0](/docs/reference/updating/#updating-from-020-or-earlier).

## Conexão

### Inacessível: Permission denied

O seu usuário não consegue abrir o socket do Docker. No Linux, adicione-o ao grupo `docker` e faça login de novo:

```sh
sudo usermod -aG docker $USER
```

### Inacessível: No such file or directory

Nada está escutando no caminho do socket. Inicie o seu Engine — Docker Desktop, Colima, OrbStack, Rancher Desktop — ou confira o caminho. Veja [Engines locais](/docs/hosts/local/#adding-a-socket-by-hand).

### "O Docker Engine não respondeu a tempo" ou "A conexão SSH expirou"

O Homeport espera 10 segundos por uma conexão. Confira o endereço e a porta, se o servidor está ligado e se nenhum firewall está no caminho. O Homeport continua tentando sozinho.

### Acesso recusado: "O servidor recusou a chave para …"

O servidor ainda não aceita a chave do Homeport. Execute no servidor o comando **Adicione a chave pública** da página do host e depois escolha **Reconectar**. Veja [Hosts SSH](/docs/hosts/ssh/#authorise-homeports-key).

### "O servidor SSH não conseguiu abrir /var/run/docker.sock"

O Homeport fez login, mas não conseguiu acessar o Docker. Confira:

- o **Caminho do socket remoto** — o Docker rootless usa `/run/user/<uid>/docker.sock`;
- se o usuário pode usar o Docker: `sudo usermod -aG docker <user>` no servidor;
- se o servidor permite o encaminhamento de sockets (`AllowStreamLocalForwarding` em `sshd_config`, ativado por padrão).

### "Nenhum ssh-agent está em execução"

Inicie o seu agente SSH e adicione a sua chave com `ssh-add`. Se você abriu o Homeport a partir de um menu do desktop, ele pode não ver o agente que o seu terminal usa; saia da sessão e entre de novo, ou inicie o Homeport a partir de um terminal em que `ssh-add -l` lista a sua chave.

No Windows, o Homeport usa o serviço OpenSSH Authentication Agent ou o Pageant do PuTTY. Abra o Pageant e carregue a sua chave, ou inicie o serviço em um PowerShell de administrador:

```powershell
Get-Service ssh-agent | Set-Service -StartupType Automatic
Start-Service ssh-agent
ssh-add
```

### Acesso recusado: "A conexão segura com … falhou"

- O certificado de CA deve ser o que assinou o certificado do servidor.
- O endereço que você digitou deve corresponder a um nome ou endereço IP no certificado do servidor.
- O servidor deve rodar com `--tlsverify` e confiar na CA do seu certificado de cliente.

### A chave do servidor mudou

O servidor SSH apresentou uma chave diferente daquela em que você confiou. Se você reinstalou o servidor ou mudou as chaves dele, escolha **Confiar na nova chave**. Se você não sabe por que ela mudou, não se conecte: pergunte a quem administra o servidor. Veja [Hosts SSH](/docs/hosts/ssh/#when-the-servers-key-changes).

### Credenciais perdidas

A chave que criptografa os seus segredos está faltando no chaveiro ou mudou — depois de uma redefinição do chaveiro, de um novo perfil de usuário ou de copiar os dados do Homeport para outro computador (use [Exportar e Importar](/docs/reference/security/#moving-to-another-computer) para isso). O Homeport mantém o host; digite de novo as credenciais dele na página do host:

- **TLS** — carregue de novo os três arquivos de certificado e escolha **Salvar**.
- **SSH com uma chave importada** — cole de novo a chave privada e a senha e escolha **Salvar**.
- **SSH com uma chave do Homeport** — escolha **Gerar nova chave** e depois autorize a nova chave no servidor.

Os ambientes das stacks gerenciadas selados com a chave antiga também precisam ser digitados de novo.

## Clusters

### Acesso recusado: "… recusou as credenciais do kubeconfig dele"

O servidor de API respondeu 401: o certificado ou o token expirou ou foi revogado. Confira se `kubectl --context <context> get namespaces` ainda funciona e depois use **Atualizar a partir do kubeconfig** na página do cluster. Para um plugin exec, o Homeport já pediu a ele um novo token uma vez.

### "… não está instalado neste computador"

O [plugin exec](/docs/clusters/exec-plugins/) do cluster (`aws`, `gke-gcloud-auth-plugin`, `kubelogin`…) não foi encontrado no `PATH` nem nos seus locais de instalação habituais. Instale-o, confira se ele roda em um terminal e depois use **Reconectar**.

### "… precisa que você faça login, o que ele não consegue fazer dentro do Homeport"

O plugin exec quer um navegador ou um código de dispositivo. Execute `kubectl --context <context> get namespaces` uma vez em um terminal, faça login ali e depois use **Reconectar**.

### "… não conseguiu obter credenciais para …"

O plugin exec rodou e falhou. **Detalhes** mostra o fim do que ele exibiu, muitas vezes um perfil expirado ou uma região errada. O Homeport tenta de novo sozinho.

### "… faz login com o auth provider gcp, que o Kubernetes não suporta mais"

Os auth providers `gcp` e `azure` foram removidos do Kubernetes. Busque as credenciais de novo com o `gke-gcloud-auth-plugin` instalado (GKE), ou execute `kubelogin convert-kubeconfig` (AKS), e depois adicione o contexto de novo.

### "Suas credenciais não permitem “…” em … no namespace …"

O RBAC do cluster não deixa o usuário do seu kubeconfig fazer isso. Escolha um namespace que você tenha permissão para ver no seletor no canto superior direito, ou peça ao administrador do cluster. A mensagem traz o comando `kubectl auth can-i` que verifica isso. As outras páginas continuam funcionando.

### Inacessível: "… recusou a conexão" ou "… não respondeu a tempo"

O servidor de API não pode ser acessado a partir deste computador: o cluster está parado (um local), atrás de uma VPN, ou o endereço do servidor no kubeconfig mudou. O Homeport continua tentando sozinho; confira `kubectl cluster-info` com o mesmo contexto.

### "O certificado de … não é assinado pela CA do kubeconfig dele"

O kind, o k3d e o minikube criam novos certificados cada vez que recriam um cluster. Use **Atualizar a partir do kubeconfig** na página do cluster.

### Nomad: "… não conhece o token ACL que o Homeport enviou"

O token foi digitado errado, expirou ou foi excluído. Crie um novo com a política de [políticas de ACL](/docs/clusters/nomad/#acl-policies) (`nomad acl token create -name=homeport -policy=homeport`) e cole o secret ID dele nas **Configurações** do cluster. O cluster espera como **Acesso recusado** até que você faça isso.

### Nomad: "… recusou: falta ao token ACL uma permissão de que isto precisa"

As políticas do token não concedem o que esta página ou ação precisa; a mensagem diz o quê (ler jobs ou logs, executar ou parar jobs, escalar, abrir um shell, alterar nós). Adicione a capacidade à política do token; [políticas de ACL](/docs/clusters/nomad/#acl-policies) lista do que cada uma precisa. As outras páginas continuam funcionando.

### Nomad: "… não tem líder: os servidores dele não conseguem chegar a um acordo"

Menos que a maioria dos servidores do cluster está rodando ou consegue se alcançar, então o Nomad não consegue responder. Confira-os com `nomad server members` em um servidor e traga de volta servidores suficientes. O Homeport continua tentando.

### Nomad: "… não tem o job …"

O job foi parado e eliminado, ou coletado pelo coletor de lixo, ou ele está em outro namespace que não o escolhido. Abra de novo a página de jobs, com **Todos os namespaces** se necessário.

### Nomad: "O grupo de tarefas … não pode executar mais de …" ou "menos de …"

O bloco `scaling` do grupo de tarefas define um `min` e um `max`, e a contagem pedida está fora deles. Mude os limites na especificação do job e execute-o de novo para ir além.

### Nomad: "O Nomad não escala … enquanto uma implantação dele estiver em andamento"

Executar uma nova versão de um job de serviço inicia uma implantação, e o Nomad se recusa a mudar a contagem dele até que todas as alocações da nova versão estejam íntegras. Espere as alocações do job se estabilizarem e depois escale de novo.

### Nomad: "A especificação do job não pode ser analisada, na linha …, coluna …"

O analisador do Nomad encontrou um problema nesse lugar; **Detalhes** tem a mensagem dele. Os blocos `variable` precisam de um valor padrão, já que o Homeport não passa arquivos de variáveis. Uma especificação JSON deve ser um job no formato da API do Nomad, como `nomad job inspect` o exibe.

### Nomad: "O Nomad não consegue posicionar todas as alocações de …"

O plano não encontrou nenhum nó elegível com os recursos que o grupo de tarefas pede, ou que atenda às suas restrições; o plano diz qual. **Executar mesmo assim** coloca em fila o que não pode ser posicionado até que um nó possa recebê-lo; ou reduza os recursos, mude as restrições ou adicione nós.

### Nomad: "… mudou desde que foi planejado, então não foi executado"

Outra pessoa executou ou parou o job entre o seu plano e **Executar**, e o Homeport não sobrescreve a mudança dela. Volte, planeje de novo e confira o novo diff antes de executar.

### Nomad: "A tarefa … da alocação … não está em execução"

Um shell precisa de uma tarefa em execução: esta ainda não iniciou, terminou ou falhou. Os logs dela continuam legíveis até que a alocação seja coletada pelo coletor de lixo.

### Nomad: "… não consegue acessar o nó em que a alocação … é executada"

Os logs e os shells passam pelos servidores até o nó cliente, e os servidores não conseguem acessá-lo: o nó está desligado, desconectado, ou o agente dele parou. Confira-o na página de nós ou com `nomad node status`.

### Helm: "A CLI helm não está instalada neste computador"

As [releases do Helm](/docs/clusters/helm/) precisam do `helm` 3 ou posterior neste computador. Instale-o (`brew install helm`, `winget install Helm.Helm` ou `snap install helm --classic`) e depois abra de novo a página do Helm; **Configurações → Esta máquina** mostra o que o Homeport encontrou. Um `helm` 2 é informado como não suportado.

### Helm: "Outra instalação, atualização ou reversão desta release ainda está em andamento"

A release está em `pending-…`: uma instalação, atualização ou reversão não terminou, muitas vezes porque foi interrompida. O helm se recusa a mudá-la até que isso termine. Espere, ou reverta a release para a sua última revisão implantada a partir do **Histórico** dela.

## Usar o Homeport

### Implantar fica desativado: "A CLI docker compose não está instalada"

O Homeport precisa do `docker compose` neste computador para implantar stacks. Instale o Docker Desktop ou a Docker CLI com o plugin Compose e depois reinicie o Homeport. **Configurações → Esta máquina** mostra se ele foi encontrado.

Se ele estiver instalado, mas não for encontrado, o Homeport pode ter um `PATH` diferente do seu terminal — comum quando iniciado a partir de um menu do desktop no macOS. Verifique se o `docker` está em um local padrão, como `/usr/local/bin`.

### A implantação falha: "usa build:"

Uma stack gerenciada não tem pasta a partir da qual fazer a build. [Vincule a pasta do projeto](/docs/guides/stacks/#creating-a-linked-stack) no lugar — uma stack vinculada faz build —, ou faça a build e envie a imagem para um registro e depois use `image:`.

### A minha stack virou externa

Uma [stack vinculada](/docs/guides/stacks/#when-the-folder-moves-or-disappears) aparece como externa, marcada com ⚠ **Os arquivos compose dela não estão mais neste computador**, quando a pasta dela foi movida, renomeada ou excluída, ou quando um arquivo compose nela foi. Nada é perdido: coloque a pasta de volta onde estava e a stack volta a ser vinculada na próxima atualização. Se a pasta mudou de lugar de vez, escolha **Esquecer** na aba **Definição** da stack e depois vincule-a de novo a partir do novo lugar com **Nova stack → Usar uma pasta neste computador**.

Uma stack iniciada a partir de uma pasta em outro computador é sempre externa aqui, já que este computador não tem esse caminho.

### "Alterado fora do Homeport"

Os arquivos compose de uma stack vinculada foram editados fora do Homeport — no seu editor, por um `git pull` — desde a última vez que o Homeport os leu. Escolha **Recarregar** para vê-los como estão. Se você estava editando no Homeport ao mesmo tempo, salvar é recusado com **… mudou fora do Homeport desde que você o abriu**, para que nenhuma das mudanças seja perdida em silêncio: recarregue e refaça a sua edição, ou escolha **Sobrescrever** para manter a sua. Veja [editar uma stack vinculada](/docs/guides/stacks/#editing-a-linked-stack).

### Baixar uma imagem privada falha

Adicione um login para o registro da imagem em **Configurações → Registros** e use **Verificar login** para confirmar que o registro o aceita. O GHCR, e as contas do Docker Hub com autenticação de dois fatores, precisam de um token de acesso em vez da senha. A caixa de diálogo de download diz qual login ela vai usar; se ela disser **anonimamente**, o registro da imagem não corresponde a nenhum login armazenado — `ghcr.io/org/app` precisa de um login para `ghcr.io`, e `org/app` de um para o Docker Hub.

### Uma build falha

A mensagem indica a etapa que falhou, como **A build falhou na etapa 3/5 (RUN make)**; **Detalhes** contém o fim da saída da build. O Homeport usa o builder clássico do Engine, então os Dockerfiles que precisam do BuildKit (`RUN --mount`, `--secret`, heredocs) falham; faça a build deles a partir de um terminal. Uma imagem `FROM` privada precisa de um login para o registro dela em **Configurações → Registros**.

### Nenhuma marca de imagem mais recente aparece

A marca só aparece quando o registro foi consultado e respondeu. Imagens com build local ou baixadas por digest nunca mostram uma. Se um registro não puder ser acessado ou limitar as requisições (o Docker Hub conta essas verificações no seu limite de pull), o Homeport grava uma linha no seu [log](/docs/reference/security/#logs) e não mostra nada; as respostas também são guardadas por 6 horas, então escolha **Verificar imagens mais recentes** para perguntar de novo agora. Um login do Docker Hub em **Configurações → Registros** aumenta o limite.

### A busca no Docker Hub mostra um erro

A busca roda no host selecionado, então precisa do acesso à internet desse host, e as tags são lidas de `hub.docker.com` por este computador. Qualquer um dos dois pode ter a taxa limitada ou ser bloqueado por um proxy. A caixa de diálogo ainda baixa qualquer referência completa que você digitar, como `postgres:17-alpine`.

### A aba Arquivos diz que o contêiner não tem as ferramentas find/stat

As imagens mínimas e distroless não incluem `find` e `stat`, que o Homeport usa para listar os arquivos. Use `docker cp` para esses contêineres.

### O terminal não abre

O contêiner deve estar em execução e ter `/bin/sh` ou `/bin/bash`. Use **Novo terminal** para executar outro programa.

### Um volume ou uma rede não é excluído

O Engine recusa enquanto um contêiner — mesmo um parado — ainda o usa. Remova antes esses contêineres; a confirmação os lista.

### "Isto requer o Podman 5.8 ou posterior"

A API do Podman lê, instala, edita e remove [unidades Quadlet](/docs/guides/quadlet/) a partir do Podman 5.8, e as lista a partir do 5.7. Atualize o Podman no host, ou use `podman quadlet` em um terminal lá. Iniciar e parar unidades funciona em qualquer Podman.

### Quadlet: Iniciar e Parar ficam desativados

O Homeport inicia e para o serviço de uma unidade com `systemctl`, que ele só consegue executar via SSH ou para um Podman neste computador (no Linux). Acesse o host via SSH em vez de TCP ou TLS. Uma unidade cujo serviço mostra **ainda não gerado** não tem nada para iniciar: o gerador do systemd ignorou o arquivo dela.

### Quadlet: "O systemd do usuário não está em execução no host"

As unidades de usuário (Podman rootless) precisam da instância systemd do usuário, que só roda enquanto o usuário está logado, a menos que o lingering esteja ativado. Como esse usuário, no host, execute `loginctl enable-linger "$USER"` e tente de novo.

### Quadlet: "O systemd não permitiu que este usuário gerencie …"

As unidades de sistema (Podman rootful) só podem ser iniciadas e paradas por um usuário com permissão para gerenciá-las, como o `root`, e o Homeport nunca usa `sudo`. Conecte-se como um usuário assim, ou use o Podman rootless, cujas unidades pertencem ao próprio usuário.

### containerd: "não tem permissão para usar o containerd em /run/containerd/containerd.sock"

O socket do containerd pertence ao root. Dê acesso a ele a um grupo com `gid` em `[grpc]` em `/etc/containerd/config.toml`, adicione o seu usuário (ou o usuário SSH) a esse grupo, reinicie o containerd e faça login de novo; [a página do containerd](/docs/hosts/containerd/#letting-your-user-use-the-socket) tem os comandos. O containerd rootless não pode ser acessado de forma alguma: o socket dele só existe dentro do namespace do RootlessKit.

### containerd: "A saída deste contêiner não é registrada"

O containerd não mantém logs; a ferramenta que iniciou a tarefa decide para onde vai a saída dela. Os contêineres criados com `ctr` normalmente não a enviam para lugar nenhum. Os contêineres do nerdctl e do Kubernetes registram logs em arquivos que o Homeport lê; veja [Logs](/docs/hosts/containerd/#logs).

### containerd: os logs ou o terminal dizem que precisam de SSH

Eles precisam dos arquivos da máquina do containerd, que o Homeport acessa via SSH, ou neste computador para um socket local no Linux. Adicione o host via SSH. Se o arquivo de log não puder ser lido, ele normalmente pertence ao root: conecte-se como um usuário com permissão para lê-lo.

### containerd: "não consegue baixar imagens para o Homeport"

O download passa pelo serviço Transfer do containerd, estável a partir do containerd 2.0. Atualize o containerd, ou baixe com `ctr images pull` ou `nerdctl pull` no host; a imagem então aparece na página Imagens.

### Incus: "não tem permissão para usar o Incus em /var/lib/incus/unix.socket"

O socket de administração do Incus é para o grupo `incus-admin`, e o `unix.socket.user` restrito, para o grupo `incus`. Adicione o seu usuário (ou o usuário SSH) com `sudo usermod -aG incus-admin "$USER"` e depois saia da sessão e entre de novo, ou reconecte via SSH. Veja [os sockets do Incus](/docs/hosts/incus/#its-sockets).

### Incus: "O Homeport não conseguiu acessar o Incus em …" ao adicionar com um token de confiança

O Incus não escuta na rede até que `core.https_address` seja definido. No servidor, execute `incus config set core.https_address :8443`, confira se um firewall deixa passar a porta 8443 e crie um novo token: o token lista os endereços em que o Incus escutava quando ele foi criado.

### Incus: "O Incus recusou o token de confiança"

Um token funciona uma vez e expira. Crie um novo no servidor com `incus config trust add homeport` e cole-o dentro do prazo de validade dele.

### Incus: "… apresentou um certificado com impressão digital …, e não a … confiável"

O certificado do servidor não é aquele em que o Homeport confiou: o Incus foi reinstalado, o certificado dele foi renovado, ou outra coisa responde nesse endereço. Confira a impressão digital no servidor com `incus info`; se for a mostrada, remova o host e adicione-o de novo com um novo token. Veja [certificados fixados](/docs/hosts/incus/#the-servers-certificate-is-pinned).

### Incus: "Este host não consegue executar máquinas virtuais"

O Incus precisa do KVM (`/dev/kvm`) e do QEMU para máquinas virtuais. Ative a virtualização no firmware do servidor (ou a virtualização aninhada para um servidor virtual), ou crie um contêiner de sistema no lugar.

### Incus: "O agente Incus da máquina virtual não está em execução"

O exec em uma máquina virtual passa pelo `incus-agent` dentro dela, que as próprias imagens do Incus iniciam na inicialização. Espere a inicialização terminar, ou use a aba **Console**, que não precisa de agente.

### "o host … não está conectado"

A ação precisava do host, mas ele se desconectou. Espere que ele se reconecte, ou escolha **Tentar agora**.
