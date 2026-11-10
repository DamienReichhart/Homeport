---
title: Segurança e os seus dados
description: O que o Homeport armazena, onde, como os segredos são criptografados e o que ele nunca faz.
---

O Homeport guarda credenciais que abrem os seus servidores. Esta página explica exatamente o que ele mantém, e como isso é protegido. [Os seus dados e backups](/docs/reference/data-and-backups/) diz o que copiar para fazer backup e como restaurar.

## O que o Homeport armazena

Tudo fica em um único banco de dados SQLite, `homeport.db`, na pasta de dados de aplicativos do seu usuário:

| Sistema | Pasta                                             |
| ------- | ------------------------------------------------- |
| Linux   | `~/.local/share/com.homeport.app/`                |
| macOS   | `~/Library/Application Support/com.homeport.app/` |
| Windows | `%APPDATA%\com.homeport.app\`                     |

Ele contém:

| Dados                                                           | Armazenamento     |
| --------------------------------------------------------------- | ----------------- |
| Nomes dos hosts, endereços, usuários SSH e caminhos de socket   | Texto simples     |
| Chaves públicas SSH e as chaves de servidor em que você confiou | Texto simples     |
| Chaves privadas SSH e as suas senhas                            | **Criptografado** |
| Certificados de CA TLS, certificados e chaves de cliente        | **Criptografado** |
| Chaves de cliente e certificados fixados dos hosts Incus        | **Criptografado** |
| Arquivos compose das stacks gerenciadas                         | Texto simples     |
| Ambientes das stacks gerenciadas                                | **Criptografado** |
| Pastas e caminhos dos arquivos compose das stacks vinculadas    | Texto simples     |
| Histórico de implantação: arquivos compose e perfis             | Texto simples     |
| Histórico de implantação: ambientes                             | **Criptografado** |
| Endereços e nomes de usuário dos registros                      | Texto simples     |
| Senhas e tokens de acesso dos registros                         | **Criptografado** |
| URLs dos repositórios, metades públicas das chaves de deploy    | Texto simples     |
| Chaves de servidor dos repositórios em que você confiou         | Texto simples     |
| Chaves de deploy e tokens de acesso dos repositórios            | **Criptografado** |
| Nomes dos clusters, URLs dos servidores de API e namespaces     | Texto simples     |
| Como cada cluster faz login, o seu contexto do kubeconfig       | Texto simples     |
| CA, certificados e chaves de cliente dos clusters               | **Criptografado** |
| Tokens, senhas e tokens OIDC dos clusters                       | **Criptografado** |
| Comandos, argumentos e ambiente dos plugins exec                | **Criptografado** |
| O seu tema, a última seleção e o namespace por cluster          | Texto simples     |
| Os eventos do Engine de cada host, para a página Eventos        | Texto simples     |

O Homeport não copia mais nada dos seus Engines: contêineres, imagens, volumes e logs são lidos ao vivo e nunca salvos, exceto quando você mesmo baixa logs ou arquivos, ou faz backup de um volume. Os eventos do Engine que ele mantém — o que aconteceu com qual contêiner, imagem, volume ou rede, com os seus rótulos — são guardados por 30 dias por padrão ([Configurações → Dados](/docs/guides/settings/#dados)), no máximo 10.000 por host, e removidos com o host deles ou com **Limpar histórico**.

### Pastas das stacks vinculadas

Para uma [stack vinculada](/docs/guides/stacks/#stacks-vinculadas), o Homeport armazena apenas onde estão a pasta e os arquivos compose dela, e uma impressão digital dos arquivos para saber quando eles mudam — nunca uma cópia deles. Ele lê os arquivos compose quando você abre a stack e só grava neles quando você escolhe **Salvar** na aba **Definição**, substituindo cada arquivo editado de uma só vez e mantendo as permissões dele; ele não sobrescreve um arquivo que mudou desde que você o abriu, a menos que você diga para fazer isso. O Homeport nunca lê nem grava o `.env` da pasta: o Compose o lê quando roda. Esquecer uma stack vinculada nunca mexe na pasta.

### Repositórios

Para as [stacks sincronizadas](/docs/guides/synced-stacks/), o Homeport guarda a URL de cada repositório e, ou uma chave de deploy que ele gerou para esse repositório (com a metade privada criptografada), ou o seu nome de usuário e um token de acesso (criptografado). O conteúdo de um repositório não é segredo: o clone dele fica em uma pasta `repositories` ao lado de `homeport.db`.

### Swarms

O Homeport não armazena nada sobre um [Swarm](/docs/guides/swarm/): ele o encontra pelos seus hosts manager cada vez que se conecta. Os valores dos secrets do Swarm vão para o Swarm e não são guardados pelo Homeport: um valor que você digita ou carrega de um arquivo é enviado ao Engine uma vez, nunca gravado em `homeport.db`, em um arquivo ou no log, e descartado da memória quando a caixa de diálogo fecha. O Engine nunca devolve o valor de um secret, então o Homeport também não pode mostrá-lo. Os tokens de entrada são buscados em um manager quando você precisa deles e nunca armazenados nem registrados no log.

### Clusters

As credenciais de um [cluster](/docs/clusters/) vêm do seu kubeconfig e são armazenadas apenas criptografadas, como as chaves SSH; os arquivos a que o kubeconfig se refere são lidos uma vez e incorporados, um kubeconfig colado nunca é gravado em um arquivo e o seu kubeconfig nunca é alterado. Um [plugin exec](/docs/clusters/exec-plugins/) roda neste computador, como você, e o token ou certificado que ele exibe é mantido apenas na memória, até expirar. O Homeport não armazena mais nada sobre o que roda em um cluster: Pods, cargas de trabalho e o resto são lidos ao vivo. Os [encaminhamentos de porta](/docs/clusters/port-forward/) escutam apenas no loopback deste computador (`127.0.0.1` e `::1`), então nada mais na sua rede pode usá-los, e eles terminam quando o Homeport é encerrado. As mudanças feitas com [Aplicar YAML ou Editar](/docs/clusters/yaml/) são registradas no cluster com o field manager `homeport`.

O [Helm](/docs/clusters/helm/) executa a sua CLI `helm` contra um cluster sem o seu kubeconfig e sem gravar nenhuma credencial do cluster em disco: para cada comando, o Homeport abre uma conexão privada no loopback deste computador (`127.0.0.1`, uma porta aleatória) que só aceita requisições com um token aleatório, e dá ao helm um kubeconfig temporário, legível só por você, contendo esse token e nada mais. O Homeport adiciona as credenciais do próprio cluster a cada requisição no caminho, e exclui o kubeconfig e fecha a conexão quando o comando termina. Os valores do chart vão para o helm pela entrada padrão, nunca em um arquivo. Os repositórios de charts, os logins de registro e os caches próprios do helm continuam sendo do helm, como em um terminal; o Homeport só adiciona repositórios públicos e não armazena nenhuma senha de repositório.

O token de ACL, o certificado de CA, o certificado de cliente e a chave de um [cluster Nomad](/docs/clusters/nomad/) são armazenados apenas criptografados, como as chaves SSH; o endereço, a região e o nome de servidor TLS dele ficam em texto claro. O token viaja no cabeçalho `X-Nomad-Token` de cada requisição, nunca em uma URL, inclusive no WebSocket de um shell. O Homeport não guarda cópia da especificação de um job: o Nomad a armazena com o job.

**Os valores dos Secrets do Kubernetes** só são buscados quando você escolhe **Revelar** para uma chave, uma de cada vez: as listas, a página de um Secret e o YAML dele trazem as chaves e os tamanhos, nunca os valores. Um valor revelado fica na memória dessa página — não em um cache, não em `homeport.db`, não no log — e é ocultado de novo quando você sai da página.

## Como os segredos são criptografados

- Cada segredo é selado individualmente com **XChaCha20-Poly1305**, uma cifra autenticada, então a adulteração é detectada, além de a leitura ser impedida.
- A chave de 256 bits é aleatória, criada na primeira execução, e guardada no **chaveiro do sistema** — as Chaves (Keychain) do macOS, o Gerenciador de Credenciais do Windows ou o Secret Service no Linux — sob o serviço `com.homeport.app`. Ela nunca é armazenada junto com os dados.
- Não há senha mestra: o chaveiro já vincula a chave à sua conta de usuário.

Alguém que tenha apenas uma cópia de `homeport.db` pode ler os nomes e os endereços dos hosts e dos clusters, mas não as suas chaves, certificados, tokens, ambientes de stacks ou senhas de registro.

### Sem um chaveiro

O Homeport mostra um banner e não armazena segredos. Os hosts de socket local, TCP e ssh-agent continuam funcionando. Veja os [requisitos](/docs/requirements/#opcional-um-chaveiro-do-sistema).

### Se a chave do chaveiro for perdida

Redefinir o chaveiro, ou mover `homeport.db` para outro computador ou usuário, deixa segredos que não podem ser descriptografados. O Homeport não exclui nada: os hosts e clusters afetados mostram **Credenciais perdidas** até que você digite as credenciais deles de novo; [Se os segredos forem perdidos](/docs/reference/data-and-backups/#if-the-secrets-are-lost) lista o que digitar de novo, e onde. Para mover os seus dados, [faça um backup](#backups) ou [exporte os seus hosts](#moving-to-another-computer).

### Mudar para outro computador

**Exportar**, na página Hosts, grava os hosts que você escolher em um único arquivo, com as chaves, os certificados e as chaves de servidor confiáveis deles, criptografados com uma senha que você escolhe:

- O arquivo inteiro é criptografado (XChaCha20-Poly1305, com a chave derivada da senha pelo Argon2id), então, sem a senha, ele não revela nada — nem mesmo os nomes ou os endereços dos hosts. Qualquer mudança no arquivo o torna ilegível.
- A senha precisa ter pelo menos 12 caracteres. O Homeport nunca a armazena nem a registra no log; se você a perder, o arquivo não pode ser aberto.
- **Importar**, no outro computador, pede a senha e depois armazena os segredos de cada host criptografados com a chave do chaveiro daquele computador, como qualquer outro.
- Um host cujas credenciais já estão perdidas neste computador é exportado sem elas, e a exportação diz qual.

Trate o arquivo como as chaves que ele contém: exclua-o depois de importado. O formato é descrito na [ADR 0013](https://github.com/DamienReichhart/Homeport/blob/main/docs/adr/0013-passphrase-protected-export-bundles.md).

### Backups

**Configurações → Dados → Fazer backup…** grava tudo o que está acima — hosts, clusters, stacks, logins de registro, repositórios e configurações, com os seus segredos abertos — em um único arquivo no mesmo formato criptografado, com uma senha que você digita duas vezes. Os eventos do Engine e o histórico de implantação não são incluídos. Um segredo que não pode ser aberto neste computador fica de fora, e o backup diz qual.

**Restaurar…** pede a senha e lista o que o arquivo contém antes de mudar qualquer coisa. Restaurar então:

1. copia o banco de dados atual para `homeport.db.before-restore-<date and time>`, ao lado de `homeport.db`;
2. substitui todos os hosts, clusters, stacks, logins, repositórios e configurações de uma só vez, de modo que uma falha deixa os seus dados como estavam;
3. criptografa de novo todos os segredos com a chave do chaveiro deste computador.

Um backup feito por um Homeport mais recente é recusado; um feito por um Homeport mais antigo é restaurado. O backup contém todos os segredos que o Homeport tem: guarde-o, e a senha dele, com o mesmo cuidado que as próprias chaves. [Os seus dados e backups](/docs/reference/data-and-backups/) tem os passos, o que um backup inclui e deixa de fora, e como copiar o banco de dados você mesmo.

## As chaves nunca tocam o disco em texto claro

O Homeport tem o seu próprio cliente SSH, então ele nunca grava uma chave privada em um arquivo para o `ssh` ler, e nunca altera `~/.ssh`. As chaves descriptografadas ficam na memória.

Quando o `docker compose` implanta uma stack em um host SSH ou TLS, o Homeport abre um socket privado de curta duração neste computador — legível apenas pelo seu usuário, ou um named pipe que recusa clientes remotos no Windows — e o encaminha pela sua própria conexão, para aquele único comando.

As chaves e os tokens dos repositórios também nunca chegam ao disco. Para cada comando `git`, o Homeport serve um agente SSH privado que contém apenas a chave de deploy daquele repositório — em um socket que só o seu usuário pode abrir, ou um named pipe que recusa clientes remotos no Windows — ou passa o token em um cabeçalho HTTPS pelo ambiente do comando. A sua própria configuração do git, os seus auxiliares de credenciais e as suas chaves SSH ficam de fora, então nada é guardado em cache no seu chaveiro ou em `~/.git-credentials`.

Os logins de registro também nunca chegam ao `docker compose`: o Homeport baixa ele mesmo as imagens de uma stack, pela API do Engine, antes de executar `docker compose up`, então o ambiente do comando e os arquivos que ele recebe não contêm nenhuma senha nem token.

## Confiar nos servidores

Na primeira vez que o Homeport se conecta a um servidor SSH, ele pede que você confirme a impressão digital da chave do servidor, e se recusa a se conectar se essa chave mudar depois, até que você confirme de novo. Os hosts TLS verificam o certificado do servidor com a sua CA e o nome do servidor; os hosts Incus via HTTPS aceitam apenas o certificado com o qual foram adicionados. Veja [Hosts SSH](/docs/hosts/ssh/#primeira-conexão).

### Certificados do Incus

Adicionar um [host Incus via HTTPS](/docs/hosts/incus/#via-https-com-um-token-de-confiança) cria na memória um certificado de cliente e a sua chave privada, uma chave ECDSA P-384, e os armazena criptografados com os outros segredos do host, nunca em texto claro no disco. O próprio certificado do Incus é armazenado da mesma forma e fixado: o Homeport aceita exatamente esse certificado do servidor, comparado pela impressão digital SHA-256, e recusa um diferente, mostrando as duas impressões digitais. O token de confiança é usado uma vez e não é guardado. Remova o certificado do Homeport no servidor com `incus config trust remove` para revogá-lo.

## Com o que o Homeport se conecta

- **Os hosts que você adiciona.** Quando você baixa, envia ou faz build de uma imagem, é o Engine que contata o registro, não o Homeport. O Homeport entrega a ele o login armazenado para aquele registro junto com a requisição, pela conexão do host; o Engine não o guarda.
- **Os registros das suas imagens, pelo Engine de cada host**, para saber se uma [imagem mais recente](/docs/guides/containers/#imagens-mais-recentes) foi publicada: quando a página Contêineres, Imagens ou Stacks carrega (cada resposta é guardada por 6 horas) e quando você escolhe **Verificar imagens mais recentes**. O Engine pergunta ao registro o digest para o qual uma tag aponta, com o login armazenado para os registros privados, e não baixa nada. Para um host local, o Engine roda no seu computador, então a requisição sai dele.
- **O Docker Hub, quando você navega pelas tags** na caixa de diálogo de download: o Homeport lista as tags de um repositório a partir de `hub.docker.com`, deste computador, com o seu login do Docker Hub quando há um armazenado. A busca de repositórios, por sua vez, passa pelo Engine do host.
- **Um registro, quando você verifica o login dele** em **Configurações → Registros**: o Homeport consulta o endpoint `/v2/` do registro, e o serviço de tokens dele, deste computador, enviando o nome de usuário e a senha ou o token via HTTPS (HTTP simples apenas para um registro neste computador, como `localhost:5000`).
- **Os servidores dos seus repositórios**, para as [stacks sincronizadas](/docs/guides/synced-stacks/): quando você testa o acesso, cria ou sincroniza uma stack sincronizada, e em um intervalo enquanto o Homeport está aberto, se a verificação periódica estiver ativada. O Homeport executa o `git` deste computador com a chave de deploy ou o token daquele repositório.
- **Os clusters que você adiciona**, pelo servidor de API do Kubernetes que o kubeconfig deles indica (e o proxy dele, quando há um), com as credenciais que ele fornece, verificando o certificado do servidor com a CA do kubeconfig. Os plugins exec contatam a sua própria nuvem (AWS, Google Cloud, Azure) a partir deste computador, como fazem para o `kubectl`.
- **Os clusters Nomad que você adiciona**, pelo endereço de agente que você informa, e nada mais: os logs e shells de um job passam por esse agente, que os encaminha ao nó cliente, então o Homeport nunca se conecta aos próprios nós. O certificado do agente é verificado com a CA que você informa, ou com as CAs deste computador.
- **Os servidores de imagens do Incus**, para os seletores de imagem em um [host Incus](/docs/guides/instances/#criando-uma-instância): o Homeport baixa deste computador a lista de imagens de um servidor (`streams/v1/images.json`, de `images.linuxcontainers.org`, a menos que você escolha outro) e a guarda por dez minutos. O Incus baixa as próprias imagens.
- **Repositórios de charts e registros OCI, pelo helm**, quando você adiciona ou atualiza um repositório, busca charts, ou instala ou atualiza um chart: o `helm` baixa índices e charts deste computador, como faz em um terminal.
- **Este site, para verificar atualizações**: na inicialização e a cada 6 horas, o Homeport baixa `updates/latest.json`, um pequeno arquivo que lista a versão mais recente. A requisição não leva nada sobre você, os seus hosts ou os seus contêineres, mas, como qualquer requisição web, ela chega ao GitHub Pages, que hospeda este site, a partir do seu endereço IP. Desative-a em **Configurações → Atualizações**. As atualizações em si são baixadas das GitHub Releases, e o Homeport só instala uma se ela estiver assinada com a chave de atualização do Homeport. Veja [atualização](/docs/reference/updating/).

## O que o Homeport executa nos seus hosts

O Homeport conversa com o Engine de um host pela API dele e só executa comandos no servidor de um host para o que essa API não consegue fazer:

| Quando                                                                                        | Onde                                                             | Comando                                                                                                                                    |
| --------------------------------------------------------------------------------------------- | ---------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| O servidor de um host SSH se recusa a abrir o socket do Engine dele                           | o servidor SSH                                                   | uma verificação em shell POSIX de se o socket existe e pode ser lido, e de se um Engine está instalado                                     |
| **Iniciar**, **Parar** ou **Reiniciar** uma [unidade Quadlet](/docs/guides/quadlet/)          | o servidor SSH, ou este computador para um socket local no Linux | `systemctl [--user] start\|stop\|restart '<unit>.service'`                                                                                 |
| Os [logs de um contêiner containerd](/docs/hosts/containerd/#logs)                            | o servidor SSH, ou este computador para um socket local no Linux | `tail -n <lines> [-F] -- '<log file>'`, depois de um `ls` da pasta de logs de um contêiner do Kubernetes quando o CRI não indica o arquivo |
| Um [terminal em um contêiner containerd](/docs/hosts/containerd/#um-terminal-em-um-contêiner) | o servidor SSH, ou este computador para um socket local no Linux | `mktemp -d`, `mkfifo -m 600` para dois FIFOs nessa pasta, `cat` por eles e depois `rm -rf` da pasta                                        |

Eles rodam com os direitos do usuário SSH (ou os seus, neste computador) e nunca com `sudo`. Os valores são verificados e colocados entre aspas antes de entrarem em um comando. Instalar, editar e remover unidades Quadlet passa pela API do Podman, que grava os arquivos ela mesma. Nada roda em um host Incus: o console e os terminais dele passam pela própria API do Incus. Os únicos arquivos que o Homeport cria em um host são os dois FIFOs de um terminal containerd, em uma pasta temporária privada removida quando o terminal fecha.

## O que o Homeport nunca faz

- **Nenhuma conta.** Não há nada em que se cadastrar.
- **Nenhuma telemetria, análise de uso ou relatório de falhas.** Quando você relata um bug, o [pacote de diagnóstico](/docs/reference/troubleshooting/#exporting-diagnostics-for-a-bug-report) é um arquivo que você mesmo exporta, lê e anexa.
- **Nada instalado nos seus clusters.** O Homeport só usa a API do Kubernetes, com os direitos das suas credenciais, e não muda nada sem que você peça: tudo o que remove ou substitui algo pergunta antes, indicando o cluster.
- **Nada instalado nos seus hosts.** Os únicos comandos que o Homeport executa em um host estão [listados acima](#o-que-o-homeport-executa-nos-seus-hosts). O único contêiner que o Homeport inicia por conta própria é um [contêiner auxiliar](/docs/guides/volumes-networks/#o-contêiner-auxiliar) de curta duração quando você abre, faz backup, restaura ou clona um volume; ele não tem rede e é removido quando termina. A imagem dele só é baixada depois que você concorda.
- **Nenhuma instalação silenciosa.** Uma atualização só é instalada quando você escolhe **Instalar e reiniciar**; os pacotes Linux só são atualizados pelo seu gerenciador de pacotes.

A [política de privacidade](/privacy/) explica o que isso significa juridicamente: quem é responsável pelos seus dados, com que base e como exercer os seus direitos.

## Logs

O Homeport grava um pequeno arquivo de log — algumas dezenas de kilobytes, substituído quando fica cheio — para diagnosticar problemas. Ele contém eventos de conexão e erros, não as suas credenciais nem a saída dos contêineres.

| Sistema | Pasta                                   |
| ------- | --------------------------------------- |
| Linux   | `~/.local/share/com.homeport.app/logs/` |
| macOS   | `~/Library/Logs/com.homeport.app/`      |
| Windows | `%LOCALAPPDATA%\com.homeport.app\logs\` |

O Homeport não tem telemetria nem relatório de falhas: um log nunca sai do seu computador, a menos que você o envie. Para um relatório de bug, **Configurações → Sobre → Exportar diagnóstico…** salva um zip com um resumo das versões e estes logs, depois de substituir tudo o que tem cara de segredo (chaves, senhas, tokens, credenciais em URLs) e, a menos que você peça o contrário, os nomes e endereços dos seus hosts e clusters. Veja [Exportando o diagnóstico](/docs/reference/troubleshooting/#exporting-diagnostics-for-a-bug-report).

## Confiar em um download

As releases são geradas pelo GitHub Actions a partir do código-fonte público. Cada arquivo vem com um checksum SHA-256 e uma atestação de proveniência de build assinada que o vincula ao commit e ao workflow exatos que o geraram. Veja [verificar downloads](/docs/reference/verifying/).

## Relatar uma vulnerabilidade

Relate os problemas de segurança de forma privada pelo [relato privado de vulnerabilidades do GitHub](https://github.com/DamienReichhart/Homeport/security/advisories/new), e não em uma issue pública.
