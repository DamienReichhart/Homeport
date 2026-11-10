---
title: Configurações
description: Tema e idioma, a bandeja do sistema, iniciar ao fazer login, notificações, atualizações, logins de registro, repositórios, a frequência de verificação das stacks sincronizadas, o que esta máquina suporta, versão e licenças.
---

**Configurações** é a última página da barra lateral, acima dos links **Documentação** e **GitHub**, que abrem no seu navegador.

## Aparência

**Tema** é **Igual ao sistema** por padrão, seguindo o modo claro ou escuro do seu sistema operacional conforme ele muda. Escolha **Claro** ou **Escuro** para fixá-lo.

**Idioma** é **Igual ao sistema** por padrão: o Homeport aparece no idioma do seu sistema operacional quando o tem, e em inglês caso contrário. A opção cita o idioma que encontrou, como **Igual ao sistema (Français)**. Escolha um idioma para usar esse; a janela inteira muda na hora, sem reiniciar, e o menu da bandeja e as notificações também.

O Homeport fala inglês, francês (Français), espanhol (Español), alemão (Deutsch), italiano (Italiano), português do Brasil (Português), russo (Русский), chinês simplificado (简体中文), japonês (日本語), coreano (한국어), hindi (हिन्दी) e árabe (العربية), que ele mostra da direita para a esquerda. As variantes regionais seguem o seu idioma: o francês do Canadá mostra o francês, o português europeu mostra o português do Brasil. O chinês tradicional ainda não está disponível e mostra o inglês.

As datas, os números e os tamanhos também seguem o idioma, no estilo regional do seu sistema quando ele tem um para esse idioma: _1.5 GB_ e _4 Nov 2026_ em inglês britânico, _1,5 Go_ e _4 nov. 2026_ em francês. Os nomes, IDs, comandos, logs e o que os Engines e os clusters dizem ficam como estão, e os **Detalhes** de um erro mantêm o texto original do sistema ou da biblioteca que falhou, para que um relatório de bug fique igual em todos os idiomas.

## Área de trabalho

Como o Homeport é executado neste computador.

- **Fechar para a bandeja do sistema** — desativado por padrão, então fechar a janela encerra o Homeport. Ative para esconder a janela na [bandeja do sistema](/docs/guides/overview/#bandeja-do-sistema): o Homeport continua rodando com as conexões com os hosts abertas. Traga-o de volta pelo ícone da bandeja ou abrindo o Homeport de novo; encerre-o com **Sair** na bandeja. A opção fica esmaecida, com uma explicação, em um ambiente de trabalho sem bandeja.
- **Iniciar ao fazer login** — desativado por padrão. Ative e o Homeport inicia quando você faz login, para acompanhar os seus hosts desde o início. A opção mostra o que o seu sistema tem: se você mesmo remover o Homeport dos seus itens de login, ela aparece desativada. No Windows ele aparece em **Gerenciador de Tarefas → Aplicativos de inicialização**, no macOS em **Ajustes do Sistema → Geral → Itens de Início**, no Linux em `~/.config/autostart/`. A entrada de um AppImage acompanha o arquivo quando você o move, a partir da próxima vez que você o abrir.
- **Iniciar oculto na bandeja** — desativado por padrão, e só disponível enquanto **Iniciar ao fazer login** está ativado. Quando o Homeport inicia ao fazer login, ele fica na [bandeja do sistema](/docs/guides/overview/#bandeja-do-sistema) sem abrir a janela. Abrir o Homeport você mesmo sempre mostra a janela e, sem bandeja, ela sempre abre.

## Notificações

O Homeport mostra uma notificação do seu sistema operacional quando algo acontece em um host, esteja a janela aberta, minimizada ou [fechada para a bandeja](#área-de-trabalho). Cada tipo tem a sua própria opção, todas ativadas por padrão, e uma opção tem efeito na hora:

| Opção                                                 | Exemplo                                                                         |
| ----------------------------------------------------- | ------------------------------------------------------------------------------- |
| **Um contêiner para com um erro ou fica sem memória** | _web-1 parou em prod-server (código de saída 137)_, _db ficou sem memória em …_ |
| **Um contêiner deixa de estar íntegro**               | _api não está íntegro em prod-server_                                           |
| **Um host conectado fica inacessível**                | _prod-server está inacessível: …_                                               |
| **Um host inacessível volta**                         | _prod-server está conectado novamente._                                         |

Parar um contêiner — com **Parar** no Homeport, `docker stop`, `docker kill` ou `docker compose down` — não é uma falha, então não mostra nada; nem um contêiner que sai com o código 0. Um host que não consegue se conectar quando o Homeport inicia também não é uma queda: só um que estava conectado. Um contêiner preso reiniciando, ou um host que cai sem parar, notifica no máximo uma vez a cada 5 minutos.

As notificações vêm das conexões que o Homeport já mantém, então nada é instalado nos seus hosts, e elas param quando o Homeport é encerrado. O seu sistema operacional decide como elas aparecem e se aparecem: verifique as configurações de notificação dele para o Homeport se nenhuma aparecer.

## Atualizações

- **Verificar atualizações automaticamente** — ativado por padrão: o Homeport procura uma nova versão quando inicia e a cada 6 horas, e mostra uma barra no topo da janela quando há uma.
- **Verificar agora** — verifica na hora, mesmo com as verificações automáticas desativadas, e diz se você tem a versão mais recente.

Quando há uma atualização disponível, o cartão mostra o que você pode fazer, o que depende de como você instalou o Homeport:

- **Windows, macOS e o AppImage** — **Instalar e reiniciar** baixa a atualização, verifica a assinatura dela, a instala e reabre o Homeport.
- **Pacotes `.deb` e `.rpm`** — o seu gerenciador de pacotes instala as atualizações, então o cartão mostra o comando a executar, com um botão **Copiar comando**.

**Novidades** abre o [changelog](/changelog/). Veja [atualizando](/docs/reference/updating/) para saber mais.

## Registros

Logins para registros de contêineres — Docker Hub, GitHub Container Registry (GHCR) ou o seu próprio —, guardados uma vez no Homeport e usados em todos os hosts: quando você baixa uma imagem, executa um contêiner a partir de uma que ainda não está lá, envia, faz build, ou implanta uma stack gerenciada, vinculada ou sincronizada. Há um login por endereço de registro.

- **Adicionar login** pede o **Registro** (`docker.io`, `ghcr.io`, `registry.example.com:5000`), o **Nome de usuário** e a **Senha ou token de acesso**. O GHCR, e as contas do Docker Hub com autenticação de dois fatores, precisam de um token de acesso em vez da senha.
- **Verificar login** pergunta ao registro, a partir deste computador, se ele aceita o login, como faz `docker login`, sem salvar nada. Um login recusado é informado com uma dica; um registro que o Homeport não consegue alcançar informa isso.
- **Editar** muda o endereço ou o nome de usuário; deixe a senha vazia para manter a armazenada. **Remover** esquece o login: as imagens desse registro voltam a ser baixadas anonimamente.

O Docker Hub pode ser informado como `docker.io`, `index.docker.io` ou `https://index.docker.io/v1/`; são um só registro. Uma imagem sem registro no nome, como `nginx` ou `bitnami/redis`, usa o login do Docker Hub.

As senhas e os tokens são criptografados com a chave do chaveiro do sistema e nunca mais são mostrados, registrados em log nem colocados nos detalhes de erros. Sem chaveiro, o cartão explica que os logins não podem ser armazenados, e as imagens são baixadas anonimamente. Veja [segurança](/docs/reference/security/).

O Homeport não lê o `~/.docker/config.json` nem os credential helpers dele.

## Repositórios

Os repositórios git a partir dos quais as [stacks sincronizadas](/docs/guides/synced-stacks/) são implantadas, guardados uma vez e utilizáveis em todos os hosts.

- **Adicionar repositório** pede a URL do repositório e como o Homeport o alcança: uma **Chave de deploy** que ele cria para esse repositório (URLs SSH), um **Token** com o seu nome de usuário (URLs HTTPS), ou **Público** para um que qualquer pessoa pode ler.
- O botão de engrenagem mostra a chave de deploy de um repositório, com onde adicioná-la no GitHub, GitLab, Gitea e Forgejo, e **Nova chave** para substituí-la; ou **Substituir o token**. **Testar acesso** lista as branches e as tags dele a partir deste computador. Depois que o Homeport confia na chave de um servidor SSH, a impressão digital dela aparece ali, com **Redefinir** para quando a forja realmente a mudou.
- **Remover** esquece o repositório e exclui o clone dele neste computador; isso é recusado enquanto uma stack sincronizada o usa.

As chaves e os tokens são criptografados com a chave do chaveiro do sistema e nunca mais são mostrados, registrados em log nem colocados nos detalhes de erros. A sua própria configuração do git, os credential helpers e as chaves SSH nunca são usados. Veja [repositórios](/docs/guides/synced-stacks/#repositórios).

## Stacks sincronizadas

**Verificar repositórios a cada** define quantos minutos o Homeport espera entre as verificações de novos commits nos repositórios das stacks sincronizadas: 5 por padrão, no mínimo 1. O Homeport só verifica enquanto está aberto; nada é sincronizado enquanto ele está fechado. Veja [polling e implantação automática](/docs/guides/synced-stacks/#polling-e-implantação-automática).

## Dados

O que o Homeport guarda neste computador.

### Fazendo backup e restaurando

**Fazer backup…** salva todos os hosts, clusters, stacks, logins de registro, repositórios e configurações, com as chaves, certificados, tokens, senhas e ambientes deles, em um único arquivo `.homeport`. Digite duas vezes uma senha de pelo menos 12 caracteres: o arquivo inteiro é criptografado com ela, e ela não pode ser recuperada. Os eventos do Engine e o histórico de implantações não são incluídos, nem nada que esteja nos seus hosts ou clusters — contêineres, imagens, volumes e recursos do Kubernetes ficam onde estão.

**Restaurar…** abre um arquivo desses com a senha e lista o que ele contém: os hosts e os clusters pelo nome, e quantas stacks, logins de registro e repositórios. **Substituir tudo** então substitui todos os dados do Homeport pelos do backup, neste ou em outro computador:

- Tudo se conecta e implanta na hora: hosts SSH e TLS, registros e repositórios sem pedir de novo uma chave ou senha, nem para confiar na chave de um servidor.
- Uma [stack vinculada](/docs/guides/stacks/#stacks-vinculadas) cuja pasta não está neste computador aparece como externa até a pasta estar lá, no mesmo caminho.
- As [stacks sincronizadas](/docs/guides/synced-stacks/) clonam o repositório de novo na próxima sincronização.
- Os dados atuais são copiados antes para `homeport.db.before-restore-<date>` na pasta de dados do Homeport. Restaurar um arquivo errado, um de um Homeport mais recente, ou um que falha no meio do caminho não muda nada.

Para adicionar alguns hosts aos que você já tem, use **Exportar** e **Importar** na página Hosts. [Os seus dados e backups](/docs/reference/data-and-backups/) diz onde ficam os arquivos do Homeport, o que um backup inclui e como copiar o banco de dados você mesmo.

### Mantendo os eventos do Engine

**Manter eventos do Engine por** define por quantos dias os [eventos do Engine](/docs/guides/overview/#eventos) de cada host são mantidos: 30 por padrão, de 1 a 365. Cada host mantém no máximo 10.000 eventos, seja qual for esse valor. Os mais antigos são removidos quando o Homeport inicia e a cada hora, e na hora quando você diminui o número.

## Esta máquina

O que o Homeport pode usar neste computador:

- **Chaveiro do sistema** — **Disponível** quando chaves SSH, certificados, ambientes de stacks e logins de registro podem ser armazenados criptografados; **Indisponível** caso contrário. Veja os [requisitos](/docs/requirements/#opcional-um-chaveiro-do-sistema).
- **CLI do Compose** — a versão do Compose encontrada, como `docker compose v2.29.1`, ou **Não encontrado** se as stacks podem ser iniciadas e paradas, mas não implantadas. Veja [Stacks](/docs/guides/stacks/#o-que-a-implantação-precisa).
- **git** — o git encontrado, como `/usr/bin/git 2.43.0`, ou o que instalar se as stacks sincronizadas não puderem ser usadas. Veja os [requisitos](/docs/requirements/#opcional-git).
- **CLI do Helm** — o helm encontrado, como `/usr/local/bin/helm v3.16.2`, ou o que instalar neste sistema se as [releases do Helm](/docs/clusters/helm/) não puderem ser listadas nem instaladas. O Helm 2 é informado como sem suporte. Veja os [requisitos](/docs/requirements/#opcional-helm).

O Homeport verifica as CLIs sempre que esta página abre, então uma instalada desde a abertura do aplicativo aparece; depois de instalar um chaveiro, encerre e reabra o Homeport.

## Sobre

Mostra a versão do Homeport e a plataforma para a qual a build dele foi feita, por exemplo `v1.0.0 · macos/aarch64`.

**Abrir pasta de logs** mostra onde o Homeport grava o [log](/docs/reference/security/#logs) dele.

**Exportar diagnóstico…** salva um zip para anexar quando você [relatar um problema](https://github.com/DamienReichhart/Homeport/issues/new/choose): um resumo das versões e dos tipos de hosts e clusters que você usa, e os logs com os segredos removidos. A caixa de diálogo mostra o que há nele primeiro, e nada é enviado para lugar nenhum. Veja [Exportando o diagnóstico](/docs/reference/troubleshooting/#exporting-diagnostics-for-a-bug-report).

**Licenças** mostra a licença do Homeport (a GNU AGPL, versão 3), um link para o código-fonte da versão que você executa e os avisos de todos os pacotes de código aberto com que ele é construído.
