---
title: Requisitos do sistema
description: O que o Homeport precisa para rodar e as peças opcionais que habilitam SSH, TLS, implantações de stacks, clusters Kubernetes e Nomad, e o Helm.
---

## Sistemas operacionais

| Sistema | Versões                                                                     | Arquiteturas         |
| ------- | --------------------------------------------------------------------------- | -------------------- |
| Windows | Windows 10 e 11                                                             | x64, ARM64           |
| macOS   | macOS 10.15 Catalina ou posterior                                           | Apple silicon, Intel |
| Linux   | glibc 2.35 ou posterior: Ubuntu 22.04, Debian 12, Fedora 36 e mais recentes | x64, ARM64           |

O Homeport usa o web view do sistema para desenhar a sua janela: WebView2 no Windows (incluído no Windows 10 e 11), WebKit no macOS e WebKitGTK no Linux (instalado como dependência dos pacotes `.deb` e `.rpm` e embutido no AppImage).

## Um Docker Engine para gerenciar

O Homeport não inclui um runtime de contêineres. Ele gerencia Docker Engines que já rodam em algum lugar:

- **Nesta máquina** — Docker Engine, Docker Desktop, Colima, OrbStack, Rancher Desktop ou Podman. O Homeport os encontra para você; veja [Engines locais](/docs/hosts/local/).
- **Em um servidor** — acessível via [SSH](/docs/hosts/ssh/) ou via [TLS ou TCP](/docs/hosts/tls-tcp/).

Qualquer Engine que fale a API do Docker Engine funciona, incluindo o socket compatível com Docker do Podman.

Ele também gerencia o [containerd](/docs/hosts/containerd/) 1.7 e posteriores diretamente, pelo seu socket nesta máquina ou via SSH: o do k3s, o do próprio Docker Engine ou o serviço containerd de um servidor. Baixar imagens nele exige o containerd 2.0 ou posterior, e os logs e terminais exigem que o host seja acessado via SSH, ou um containerd neste computador (Linux).

Ele também gerencia o [Incus](/docs/hosts/incus/) 6.0 e posteriores, pelo seu socket nesta máquina ou via SSH, ou via HTTPS com um token de confiança. As máquinas virtuais exigem virtualização de hardware (`/dev/kvm`) no servidor Incus, e um terminal em uma delas exige o `incus-agent` que a sua imagem executa.

Em um host Podman, o Homeport também usa a API própria do Podman: os [pods](/docs/guides/podman-pods/) exigem o Podman 4.0 ou posterior, e as [unidades Quadlet](/docs/guides/quadlet/) são listadas a partir do Podman 5.7 e instaladas, editadas e removidas a partir do 5.8. Iniciar e parar uma unidade Quadlet exige que o host seja acessado via SSH, ou um Podman neste computador (Linux).

## Opcional: clusters Kubernetes

Para navegar por [clusters](/docs/clusters/), o Homeport precisa de um kubeconfig que os alcance (o que o `kubectl` usa, ou um que você cole) e de acesso de rede aos servidores de API deles. Ele fala a API do Kubernetes por conta própria, então o `kubectl` não é necessário.

- **Versões do Kubernetes**: clusters rodando o Kubernetes 1.24 ou posterior. O Homeport usa apenas APIs estáveis (`v1`, `apps/v1`, `batch/v1`, `networking.k8s.io/v1`) e é construído com base na API 1.33.
- **Plugins exec**: um kubeconfig de nuvem indica um programa a ser executado neste computador para obter as credenciais — a [AWS CLI](https://aws.amazon.com/cli/) (versão 2) para EKS, o `gke-gcloud-auth-plugin` da Google Cloud CLI para GKE, o `kubelogin` para AKS. Instale o que o seu kubeconfig indica; veja [plugins exec](/docs/clusters/exec-plugins/).

## Opcional: clusters Nomad

Para gerenciar [clusters Nomad](/docs/clusters/nomad/), o Homeport precisa de acesso de rede à API HTTP de um agente Nomad (porta 4646 por padrão) e, quando o cluster tem ACLs habilitadas, de um token; veja [a política de ACL](/docs/clusters/nomad/#acl-policies) de que ele precisa. Ele fala a API do Nomad por conta própria, então a CLI `nomad` não é necessária, e nada é instalado no cluster.

- **Versões do Nomad**: Nomad 1.6 ou posterior. **Editar e executar** parte da especificação enviada de um job, que o Nomad guarda a partir da 1.6.

## Opcional: um chaveiro do sistema

O Homeport criptografa chaves SSH, certificados TLS, as credenciais do kubeconfig dos clusters, os ambientes das stacks, os logins de registro e as chaves de deploy e tokens dos repositórios com uma chave que ele guarda no chaveiro do seu sistema operacional:

- **macOS** — Chaves (Keychain)
- **Windows** — Gerenciador de Credenciais
- **Linux** — um provedor de Secret Service, como o GNOME Keyring ou o KWallet

Sem um, o Homeport mostra um banner e continua funcionando com sockets locais, TCP simples e SSH via **ssh-agent**. Hosts TLS, hosts SSH com chaves armazenadas, clusters, ambientes de stacks, logins de registro e repositórios privados precisam do chaveiro; sem ele, as imagens são baixadas anonimamente e só repositórios públicos podem ser adicionados. Veja [segurança e os seus dados](/docs/reference/security/).

## Opcional: Docker Compose

Para implantar [stacks gerenciadas, vinculadas e sincronizadas](/docs/guides/stacks/), o Homeport executa o comando `docker compose` **nesta** máquina — não no host. Instale a Docker CLI com o plugin Compose (o Docker Desktop o inclui) ou o `docker-compose` v2 independente.

Sem ele, as stacks continuam aparecendo — stacks vinculadas incluídas — e podem ser iniciadas, paradas e removidas; só a implantação precisa do Compose. **Configurações → Esta máquina** mostra se o Homeport o encontrou.

Para implantar uma stack em um [Swarm](/docs/guides/swarm/), o Homeport executa `docker stack deploy` nesta máquina, então ele precisa do próprio comando `docker` (o plugin Compose não é usado). O Docker Desktop o tem; no Linux, instale o pacote da Docker CLI da sua distribuição (`docker-ce-cli`, `docker.io` ou `docker`). Nenhum Engine precisa rodar aqui.

## Opcional: git

As [stacks sincronizadas](/docs/guides/synced-stacks/) executam o `git` instalado **nesta** máquina para clonar e buscar os seus repositórios: Git for Windows, as ferramentas de linha de comando da Apple (`xcode-select --install`) ou o pacote `git` da sua distribuição. No Windows, o Homeport o aponta para o OpenSSH do sistema para repositórios SSH. Todos os outros recursos funcionam sem git; **Configurações → Esta máquina** mostra se o Homeport o encontrou.

## Opcional: Helm

Para gerenciar [releases do Helm](/docs/clusters/helm/) em um cluster, o Homeport executa o comando `helm`, versão 3 ou posterior, **nesta** máquina. Ele aponta o helm para o cluster por uma conexão privada, então nenhuma credencial do cluster é gravada em disco, e o helm usa os seus próprios repositórios de charts, como em um terminal. Instale-o com `brew install helm` (macOS), `winget install Helm.Helm`, Chocolatey ou Scoop (Windows), ou `snap install helm --classic`, o pacote da sua distribuição ou o script de [helm.sh](https://helm.sh/docs/intro/install/) (Linux). Todas as outras páginas de cluster funcionam sem ele; **Configurações → Esta máquina** mostra se o Homeport o encontrou.

## Opcional: uma bandeja do sistema

O Homeport coloca um ícone na bandeja do sistema (a barra de menus no macOS, a área de notificação no Windows) que lista todos os hosts com o seu status. Windows e macOS sempre têm uma. No Linux, ela precisa da biblioteca **appindicator**: o AppImage a inclui, e os pacotes `.deb` e `.rpm` a recomendam, então o `apt`, o `dnf` e o `zypper` a instalam, a menos que você tenha desativado os pacotes recomendados. Caso contrário, instale `libayatana-appindicator3-1` (Debian, Ubuntu, openSUSE) ou `libayatana-appindicator-gtk3` (Fedora). Alguns ambientes de desktop ocultam os ícones da bandeja — o GNOME só os mostra com uma extensão como AppIndicator Support. Sem bandeja, o Homeport funciona pela sua janela, e fechar a janela o encerra.

## Do que cada recurso precisa

| Recurso                                            | Chaveiro | CLI do Compose | git | helm |
| -------------------------------------------------- | :------: | :------------: | :-: | :--: |
| Implantar stacks em um Swarm                       |    —     |   docker CLI   |  —  |  —   |
| Hosts de socket local e TCP                        |    —     |       —        |  —  |  —   |
| Hosts SSH com ssh-agent                            |    —     |       —        |  —  |  —   |
| Hosts SSH com uma chave do Homeport ou importada   |   Sim    |       —        |  —  |  —   |
| Hosts TLS                                          |   Sim    |       —        |  —  |  —   |
| Contêineres, imagens, volumes, redes               |    —     |       —        |  —  |  —   |
| Iniciar, parar e remover stacks                    |    —     |       —        |  —  |  —   |
| Implantar stacks gerenciadas e vinculadas          |    —     |      Sim       |  —  |  —   |
| O ambiente de uma stack gerenciada ou sincronizada |   Sim    |       —        |  —  |  —   |
| Logins de registro (imagens privadas, envios)      |   Sim    |       —        |  —  |  —   |
| Stacks sincronizadas de um repositório público     |    —     |      Sim       | Sim |  —   |
| Stacks sincronizadas com chave de deploy ou token  |   Sim    |      Sim       | Sim |  —   |
| Fazer build de uma imagem a partir de uma pasta    |    —     |       —        |  —  |  —   |
| Clusters Kubernetes                                |   Sim    |       —        |  —  |  —   |
| Clusters Nomad                                     |   Sim    |       —        |  —  |  —   |
| Releases do Helm                                   |   Sim    |       —        |  —  | Sim  |
