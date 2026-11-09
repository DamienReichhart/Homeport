---
title: Incus
description: Conecte-se ao Incus nesta máquina, via SSH ou via HTTPS com um token de confiança, e escolha o projeto que as páginas dele mostram.
---

O **Incus** executa contêineres de sistema e máquinas virtuais, as **instâncias** do [guia](/docs/guides/instances/). O Homeport se conecta a ele pela sua própria API REST, como decide a [ADR 0021](https://github.com/DamienReichhart/Homeport/blob/main/docs/adr/0021-incus-over-its-rest-api.md). Um host Incus ganha páginas próprias na barra lateral: **Instâncias**, **Imagens**, **Perfis**, **Armazenamento** e **Redes**. As páginas do Docker, as stacks e o Quadro não são oferecidos nele.

## Adicionar um host Incus

Escolha **Incus** no campo **Engine** do formulário e depois como acessá-lo:

| Conexão          | Quando                                                    | O que precisa                                                                |
| ---------------- | --------------------------------------------------------- | ---------------------------------------------------------------------------- |
| **Socket local** | O Incus roda neste computador (Linux)                     | o seu usuário no grupo `incus-admin`, ou `incus` (veja abaixo)               |
| **SSH**          | O Incus roda em um servidor no qual você pode fazer login | o usuário SSH no grupo `incus-admin`, ou `incus`                             |
| **TLS** (HTTPS)  | O Incus escuta na rede, ou o SSH não é permitido          | um token de confiança, ou um certificado de cliente em que o Incus já confia |

O **TCP** simples fica desativado: o Incus não serve a sua API sem TLS. O Homeport se conecta ao **Incus 6.0** (a versão de suporte de longo prazo) e posteriores.

### Os sockets dele

| Socket                            | Quem pode usá-lo               | O que ele alcança                                           |
| --------------------------------- | ------------------------------ | ----------------------------------------------------------- |
| `/var/lib/incus/unix.socket`      | o grupo `incus-admin`          | tudo: todos os projetos, pools de armazenamento e redes     |
| `/var/lib/incus/unix.socket.user` | o grupo `incus` (`incus-user`) | apenas o projeto do próprio usuário, criado no primeiro uso |

No Linux, **Encontrados nesta máquina** lista os dois quando respondem, o restrito como **Incus (restrito ao seu projeto)**. Um socket que você ainda não pode usar é listado com o comando que corrige isso:

```bash
sudo usermod -aG incus-admin "$USER"
```

As mudanças de grupo valem para novos logins: saia da sessão e entre de novo (ou reconecte via SSH) antes de adicionar o host. Um Incus parado diz quais unidades o iniciam:

```bash
sudo systemctl enable --now incus.socket incus.service
```

### Via HTTPS com um token de confiança

O Incus escuta na rede assim que `core.https_address` é definido. No servidor Incus:

```bash
incus config set core.https_address :8443
incus config trust add homeport
```

O segundo comando exibe um **token de confiança**: uma linha de letras e dígitos. No Homeport, escolha **Incus** e **TLS**, mantenha **Token de confiança** e cole-o. O Homeport mostra o que lê (o nome a que ele se destina, os endereços do servidor e a impressão digital do certificado, e quando ele expira) e então, em **Adicionar**:

1. cria o seu próprio certificado de cliente, com uma chave ECDSA P-384 gerada na memória;
2. tenta os endereços do token em ordem e fica com o primeiro cujo certificado tem a impressão digital que o token indica; qualquer outro é recusado, com as duas impressões digitais;
3. envia o token com o seu certificado e verifica que o Incus agora confia nele.

O host então aparece em `incus config trust list` com o nome do token. Um token funciona uma vez e expira; um usado, revogado ou expirado é recusado com o comando que cria um novo. Para revogar o Homeport depois, remova o certificado dele ali:

```bash
incus config trust list
incus config trust remove <fingerprint>
```

**Certificado existente** recebe um certificado de cliente e uma chave em que o Incus já confia (`incus config trust add-certificate`), com o próprio certificado do Incus a confiar (`/var/lib/incus/server.crt`, ou `cluster.crt` em um cluster).

### O certificado do servidor é fixado

O certificado do Incus é autoassinado, então não há CA com a qual verificá-lo. O Homeport guarda com o host o certificado em que confiou e aceita exatamente esse depois, comparado pela impressão digital SHA-256, seja qual for o endereço pelo qual é acessado. Se o Incus for reinstalado ou o certificado dele renovado, o host é recusado com a impressão digital esperada e a recebida: confira a nova no servidor com `incus info` e depois adicione o host de novo com um novo token. A chave de cliente e os dois certificados são armazenados criptografados como qualquer outro segredo; veja [segurança](/docs/reference/security/#incus-certificates).

## Projetos

O Incus guarda instâncias, imagens, perfis, volumes personalizados e redes em **projetos**. As páginas mostram um projeto de cada vez, escolhido abaixo do seletor de hosts, que lista cada um com quantas instâncias e imagens ele contém. O Homeport lembra a escolha de cada host entre uma execução e outra. Até você escolher um, o host mostra `default`, ou o seu único projeto no socket restrito. Os pools de armazenamento pertencem ao servidor inteiro; os volumes deles são mostrados para o projeto selecionado.

## Eventos

As páginas se atualizam a partir dos eventos de ciclo de vida do Incus, em todos os projetos: uma instância iniciando ou parando, um snapshot tirado, uma imagem adicionada, um perfil ou uma rede alterados. Eles também ficam na página **Eventos**. O Incus não mantém histórico deles, então o que aconteceu enquanto o Homeport estava fechado não é preenchido.

Enquanto está conectado, o Homeport envia ao Incus `GET /1.0` a cada 15 segundos. Um Incus parado deixa o host **Inacessível**, e o Homeport se reconecta sozinho quando ele volta a responder.

## O que o Homeport faz no host

Nada roda em um host Incus além do próprio Incus: os consoles e terminais passam pelos websockets do próprio Incus, e as imagens remotas são baixadas pelo Incus. O Homeport lê deste computador a lista de imagens de um servidor de imagens remoto, para os seletores de imagem.
