---
title: Perguntas frequentes
description: Perguntas comuns sobre o Homeport.
---

### O Homeport é gratuito?

Sim. O Homeport é gratuito para uso pessoal e comercial, e open source sob a [GNU Affero General Public License, version 3](https://github.com/DamienReichhart/Homeport/blob/main/LICENSE) (AGPL).

### O que a AGPL significa para mim?

Nada muda se você usa o Homeport: instale-o, execute-o em casa ou no trabalho, em quantos computadores quiser. A AGPL só exige algo de quem **distribui** o Homeport, modificado ou não, ou oferece uma versão modificada a outras pessoas por uma rede: essas pessoas devem publicar o código-fonte completo sob a mesma licença. Toda cópia do Homeport tem um link para o código-fonte da sua versão exata em **Licenças**, no fim de **Configurações**.

As versões até a 0.11 foram lançadas sob a licença MIT e continuam disponíveis sob ela.

### O Homeport substitui o Docker Desktop?

Não totalmente. O Homeport é um gerenciador: ele não executa um Docker Engine por conta própria. No Linux, instale o Docker Engine e use o Homeport como interface dele. No macOS e no Windows, você ainda precisa de algo que execute o Engine — Docker Desktop, Colima, OrbStack, Rancher Desktop ou Podman —, e o Homeport pode funcionar sobre qualquer um deles, junto com os seus servidores remotos.

### Ele funciona com o containerd?

Sim, diretamente, sem Docker: o Engine de um host pode ser o [containerd](/docs/hosts/containerd/), acessado pelo seu socket nesta máquina ou via SSH — o do k3s, o do próprio Docker Engine ou o serviço containerd de um servidor. O Homeport lista os namespaces dele, os contêineres e as suas tarefas (iniciar, parar, pausar, enviar sinal, remover), os logs deles onde o nerdctl ou o Kubernetes os gravaram, um terminal neles, as imagens (baixar, remover) e os snapshots. Ele não cria contêineres nem implanta stacks ali; o nerdctl faz isso.

### Ele funciona com o Incus?

Sim: o Engine de um host pode ser o [Incus](/docs/hosts/incus/) 6.0 ou posterior, acessado pelo seu socket nesta máquina ou via SSH, ou via HTTPS com um token de confiança de `incus config trust add`. O Homeport lista as [instâncias](/docs/guides/instances/) de um projeto, contêineres de sistema e máquinas virtuais, inicia, para, congela, exclui e cria essas instâncias, abre o console delas e um terminal nelas, tira e restaura snapshots, e gerencia imagens, perfis, pools de armazenamento e redes. Ele não implanta stacks ali e não instala nada no host.

### Ele funciona com o LXD?

Não como tal: o Homeport fala Incus, o fork comunitário do LXD, e recusa um servidor LXD, cuja API se distanciou. Migre-o com `lxd-to-incus` e depois adicione-o como um host Incus.

### Ele funciona com o Podman?

Sim, pelo socket da API compatível com Docker do Podman. O Homeport detecta os sockets habituais do Podman no Linux, todas as [máquinas Podman no macOS](/docs/hosts/local/#máquinas-podman-no-macos) em execução e o pipe da máquina Podman padrão no Windows. Qualquer outro pode ser [adicionado manualmente](/docs/hosts/local/#adicionar-um-socket-manualmente).

O Homeport também reconhece o Podman ao se conectar e então oferece o que só o Podman tem, pela API própria do Podman no mesmo socket: [pods](/docs/guides/podman-pods/), que você pode listar, iniciar, parar, criar e remover, e nos quais pode executar contêineres a partir da caixa de diálogo Executar; e [unidades Quadlet](/docs/guides/quadlet/), que você pode listar, ler, instalar, editar e remover, e iniciar ou parar via SSH.

### Ele funciona com Kubernetes, Nomad ou Docker Swarm?

Docker Swarm, sim. O Homeport encontra um Swarm pelos seus hosts manager, assim que um deles é um host, e o gerencia por completo: os nós, os serviços, as tarefas, os secrets, os configs e as stacks swarm, incluindo as stacks gerenciadas, vinculadas e sincronizadas implantadas nele. Ele também pode iniciar um Swarm em um host, e entrar em um ou sair dele. Tudo passa pela API do Engine de um manager conectado; nada é instalado nos nós. Veja [Swarms](/docs/guides/swarm/).

Kubernetes, sim. Adicione um cluster do seu kubeconfig — o deste computador ou um que você cole —, e o Homeport encontra os locais (Docker Desktop, OrbStack, Rancher Desktop, kind, k3d, minikube) para você. EKS, GKE e AKS funcionam pelo plugin exec que o kubeconfig deles indica. Você pode navegar ao vivo por Namespaces, cargas de trabalho, Pods, Services, Ingresses, ConfigMaps, Secrets, Nodes, Events e qualquer outro recurso da API, inclusive os personalizados; acompanhar logs, abrir um terminal em um Pod, encaminhar portas, escalar, reiniciar e excluir, e aplicar ou editar YAML com uma prévia. Nada é instalado no cluster: o Homeport só conversa com a API do Kubernetes que o seu kubeconfig já alcança, com os direitos das suas credenciais. Com a CLI `helm` neste computador, o Homeport também lista, instala, atualiza, reverte e desinstala [releases do Helm](/docs/clusters/helm/), e gerencia os repositórios de charts do helm. Veja [Clusters](/docs/clusters/).

Nomad, sim. Adicione um cluster pelo endereço do agente dele, com um token de ACL e o material TLS quando ele os usa. Você pode listar ao vivo os jobs e as suas alocações; executar um job a partir de HCL ou JSON depois de ver o plano do Nomad; editar e executar de novo um job, pará-lo ou eliminá-lo, e escalar os seus grupos de tarefas; acompanhar os logs de uma tarefa e abrir um shell nela; drenar nós e alternar a elegibilidade deles; e escolher um namespace. Nada é instalado no cluster e nenhuma CLI `nomad` é necessária: o Homeport conversa com a API HTTP do Nomad com os direitos do token. Veja [Clusters Nomad](/docs/clusters/nomad/).

### Eu preciso do kubectl?

Não. O Homeport lê o seu kubeconfig e conversa com a API do Kubernetes por conta própria. Os únicos programas que ele executa são os [plugins exec](/docs/clusters/exec-plugins/) que um kubeconfig indica, como a AWS CLI para EKS, o `gke-gcloud-auth-plugin` para GKE ou o `kubelogin` para AKS — os mesmos que o `kubectl` executaria. Ter o `kubectl` à mão ainda ajuda a conferir o que as suas credenciais podem fazer (`kubectl auth can-i`).

### Eu preciso instalar algo nos meus servidores?

Não — apenas um Docker Engine e um servidor SSH, que você muito provavelmente já tem. O Homeport não instala agentes.

Um Swarm também não precisa de nada além do Engine. Os nós dele, porém, devem se alcançar entre si, o que o Homeport não pode configurar para você: abra 2377/tcp (para os managers), 7946/tcp e 7946/udp, e 4789/udp entre eles. Veja [portas entre os nós](/docs/guides/swarm/#portas-entre-os-nós).

### Eu preciso do Docker instalado no meu computador para gerenciar servidores remotos?

Não. O Homeport fala a API do Engine e o SSH por conta própria. As únicas exceções são implantar stacks gerenciadas, vinculadas e sincronizadas, que usa o comando `docker compose` no seu computador, e implantá-las em um Swarm, que usa o comando `docker` (`docker stack deploy`). Nenhum dos dois precisa de um Engine no seu computador.

### O Homeport mantém as minhas stacks sincronizadas quando está fechado?

Não. As [stacks sincronizadas](/docs/guides/synced-stacks/) são verificadas em busca de novos commits, e implantadas automaticamente se você ativar isso, apenas enquanto o Homeport está aberto. O Homeport não instala nada nos seus servidores — nenhum agente, nenhum receptor de webhook —, então, quando ele está fechado, nada observa os seus repositórios. Quando você o abre de novo, ele espera um intervalo e depois verifica todos. Para implantações que devem acompanhar cada push o tempo todo, execute-as a partir da sua CI.

Para manter o Homeport rodando sem a janela, ative **Fechar para a bandeja do sistema**, e **Iniciar ao fazer login** com **Iniciar oculto na bandeja**, em [Configurações → Área de trabalho](/docs/guides/settings/#área-de-trabalho).

### O Homeport pode me avisar quando um contêiner falha?

Sim, enquanto ele está rodando — mesmo oculto na bandeja do sistema. Ele mostra uma notificação do sistema quando um contêiner para com um erro, fica sem memória ou deixa de ser íntegro, e quando um host fica inacessível ou volta. Cada tipo pode ser desativado em [Configurações → Notificações](/docs/guides/settings/#notificações). Ele fica sabendo disso pelo fluxo de eventos que já mantém com cada host, então nada é instalado nos seus servidores; quando o Homeport não está rodando, ninguém é avisado.

### O Homeport precisa do git?

Apenas para as [stacks sincronizadas](/docs/guides/synced-stacks/), que executam o `git` instalado no seu computador, com a chave de deploy ou o token próprio de cada repositório. **Configurações → Esta máquina** diz se o Homeport o encontrou. A sua própria configuração do git, os seus auxiliares de credenciais e as suas chaves SSH nunca são usados.

### Várias pessoas podem compartilhar os hosts do Homeport?

Cada pessoa executa o seu próprio Homeport, com os seus próprios hosts e chaves. Com uma chave criada pelo Homeport por pessoa e por servidor, você pode ver e revogar o acesso de cada uma em `authorized_keys`.

### Posso levar o Homeport para outro computador?

Sim, de duas formas:

- **Tudo**: **Configurações → Dados → Fazer backup…** salva todos os hosts, clusters, stacks, logins de registro, repositórios e configurações, com os seus segredos, em um único arquivo protegido por uma senha de pelo menos 12 caracteres. No novo computador, **Restaurar…**, no mesmo lugar, abre o arquivo, lista o que ele contém e substitui o que está lá. Tudo se conecta e implanta sem pedir de novo uma chave ou uma senha. Veja [Os seus dados e backups](/docs/reference/data-and-backups/#backing-up-in-homeport).
- **Alguns hosts**: na página **Hosts**, use **Exportar** com os hosts que escolher e **Importar** no outro computador, junto com os hosts que ele já tem. Veja [mudar para outro computador](/docs/reference/security/#mudar-para-outro-computador).

Copiar o `homeport.db` não funciona para os segredos: eles são criptografados com uma chave guardada no chaveiro deste computador e aparecem como **Credenciais perdidas** em outro lugar. Veja [copiar os arquivos você mesmo](/docs/reference/data-and-backups/#copying-the-files-yourself).

### Onde o Homeport guarda os dados dele?

Em um único banco de dados SQLite, `homeport.db`, na pasta de dados de aplicativos do seu usuário, com a chave que criptografa os segredos dele no chaveiro do seu sistema operacional. Nada é guardado nos seus servidores nem em lugar nenhum online. [Os seus dados e backups](/docs/reference/data-and-backups/) dá a pasta em cada sistema, o que incluir no backup e como restaurá-lo.

### Um backup inclui os meus contêineres e volumes?

Não. Um backup contém o que o Homeport guarda — como acessar os seus hosts, as definições e os ambientes das suas stacks, os logins e as configurações —, não o que roda nos seus hosts. Os seus contêineres, imagens e volumes continuam nos hosts. Para copiar os dados de um volume, [faça backup dele em um arquivo](/docs/guides/volumes-networks/#fazendo-backup-e-restaurando) a partir da página Volumes.

### O Homeport pode fazer build de imagens?

Sim, em dois lugares. A página **Imagens** faz build de uma imagem a partir de uma pasta deste computador. E uma [stack vinculada](/docs/guides/stacks/#stacks-vinculadas) — uma stack cujos arquivos compose estão em uma pasta deste computador — pode usar `build:`: o Homeport executa `docker compose` nessa pasta, então o contexto de build é enviado de lá para o Engine do host, que faz a build. **Refazer a build e implantar** refaz a build depois de uma mudança no código. Uma stack gerenciada não pode fazer build, já que o Homeport guarda só o YAML dela e não há pasta a partir da qual fazer a build: vincule a pasta no lugar, ou envie a imagem para um registro.

### Por que a minha stack aparece como externa no meu outro computador?

Uma stack iniciada com `docker compose up` em uma pasta aparece como [vinculada](/docs/guides/stacks/#como-as-stacks-vinculadas-são-encontradas) apenas em um computador em que essa pasta e os seus arquivos compose existem, no mesmo caminho, e definem os serviços que rodam. Em outro computador o caminho não existe, então o Homeport não tem arquivo compose para ela e a mostra como externa. Para gerenciá-la também de lá, clone o projeto nesse computador e vincule-o com **Nova stack → Usar uma pasta neste computador**, ou adote-a como uma stack gerenciada.

### O Homeport coleta dados sobre mim?

Não. Não há conta, telemetria, análise de uso nem relatório de falhas. A única requisição que o Homeport faz por conta própria a partir do seu computador é a verificação de atualizações, que baixa um pequeno arquivo deste site e não envia nada sobre você; você pode desativá-la em **Configurações → Atualizações**. À parte disso, a verificação de imagens mais recentes pergunta aos registros das suas imagens, pelo Engine de cada host, para qual versão as tags deles apontam; para um host local, essa requisição também sai do seu computador. Veja [segurança e os seus dados](/docs/reference/security/#com-o-que-o-homeport-se-conecta).

### O Homeport pode usar os meus registros privados?

Sim. Adicione um login para cada registro em **Configurações → Registros** — Docker Hub, GHCR ou o seu próprio. O Homeport o entrega ao Engine para baixar, enviar e fazer build em qualquer host, e baixa com ele as imagens de uma stack gerenciada, vinculada ou sincronizada antes de implantar. As senhas e os tokens são criptografados com o chaveiro do seu sistema. O Homeport não lê os logins de `docker login` deste computador.

### Como o Homeport sabe que existe uma imagem mais recente?

Ele pergunta ao registro da imagem, pelo Engine do host, para qual digest a tag aponta agora e o compara com o que o host baixou. Nada é baixado, mas o Docker Hub conta cada pergunta no seu limite de taxa de pull, como um `docker pull` que não encontra nada novo. Por isso o Homeport guarda cada resposta por 6 horas, pergunta a no máximo quatro registros de uma vez e só pergunta de novo antes disso quando você escolhe **Verificar imagens mais recentes**. Um login do Docker Hub em **Configurações → Registros** aumenta o limite. Se um registro não puder ser acessado ou estiver limitando a taxa, o Homeport não mostra nenhuma marca para as imagens dele e grava uma linha no seu log.

### Como eu atualizo?

No Windows, no macOS e com o AppImage, o Homeport avisa quando sai uma nova versão: escolha **Instalar e reiniciar**. No Linux, os pacotes `.deb` e `.rpm` são atualizados com o resto do seu sistema a partir do repositório de pacotes do Homeport. Veja [atualização](/docs/reference/updating/).

### Uma atualização vai me fazer perder os meus hosts?

Não. Cada versão do Homeport abre os dados de todas as versões anteriores, até a 0.1.0, e os atualiza sem perder nada: hosts, stacks, clusters, configurações e segredos. Voltar para uma versão mais antiga é a única coisa que não é suportada: o Homeport mais antigo para com uma mensagem em vez de alterar qualquer coisa. Veja [Os seus dados entre versões](/docs/reference/updating/#your-data-across-versions).

### O Homeport está no Homebrew, no winget, no Flathub ou no AUR?

Ainda não. No Linux, o Homeport tem os seus próprios repositórios APT e RPM, então `apt`, `dnf` e `zypper` o instalam e atualizam. Homebrew, winget, Scoop, o AUR, Flathub e Snap estão planejados; a [página de instalação](/docs/installation/#em-breve) tem o link para a issue de cada um.

### O Homeport está disponível no meu idioma?

O Homeport fala inglês, francês, espanhol, alemão, italiano, português do Brasil, russo, chinês simplificado, japonês, coreano, hindi e árabe, e segue o idioma do seu sistema operacional, a menos que você escolha um em [Configurações → Aparência](/docs/guides/settings/#aparência). O site e esta documentação também estão em todos eles; só as páginas jurídicas estão em inglês. Para adicionar um idioma, veja [Contribuir](/docs/project/contributing/#traduzir-o-homeport).

### O Homeport é acessível?

Ele pode ser usado só com o teclado, segue a configuração de movimento reduzido do seu sistema, atende ao contraste WCAG 2.2 AA, e todas as páginas são verificadas com ferramentas automáticas de acessibilidade. Uma auditoria prática com leitores de tela está em andamento. Veja [Teclado e acessibilidade](/docs/reference/accessibility/) para saber o que funciona, os limites conhecidos e como relatar uma barreira.

### Existe uma interface de linha de comando?

Não — o Homeport é um aplicativo de desktop. A sua CLI `docker` existente funciona junto com ele, e o Homeport se atualiza ao vivo quando você muda coisas pelo terminal.

### Como posso ajudar?

Relate bugs e sugira recursos nas [issues do GitHub](https://github.com/DamienReichhart/Homeport/issues), ou veja [Contribuir](/docs/project/contributing/).
