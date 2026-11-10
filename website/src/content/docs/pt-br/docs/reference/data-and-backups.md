---
title: Os seus dados e backups
description: Onde o Homeport guarda os dados dele em cada sistema, como fazer backup e restaurá-los, e o que digitar de novo quando os segredos são perdidos.
---

O Homeport guarda o que sabe no seu computador: como acessar os seus hosts e clusters, as definições e os ambientes das suas stacks, os seus logins de registro, repositórios e configurações. Esta página diz onde isso fica, o que copiar e como recuperar. Para saber como tudo isso é protegido, veja [A segurança e os seus dados](/docs/reference/security/).

O que roda nos seus hosts e clusters não faz parte dos dados do Homeport: contêineres, imagens, volumes e objetos do Kubernetes ou do Nomad ficam onde estão, e nenhum backup do Homeport os contém. Para copiar o conteúdo de um volume, [faça backup dele em um arquivo](/docs/guides/volumes-networks/#fazendo-backup-e-restaurando) a partir da página Volumes.

## O que fica onde

### A pasta de dados

| Sistema | Pasta                                             |
| ------- | ------------------------------------------------- |
| Linux   | `~/.local/share/com.homeport.app/`                |
| macOS   | `~/Library/Application Support/com.homeport.app/` |
| Windows | `%APPDATA%\com.homeport.app\`                     |

No Linux, a pasta fica em `$XDG_DATA_HOME` quando essa variável está definida. Se o Homeport não conseguir abrir o banco de dados dele, a janela mostra o caminho completo do arquivo, com **Abrir pasta**.

| Na pasta                             | O que é                                                                                                                                                                                        |
| ------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `homeport.db`                        | O banco de dados: todos os hosts, clusters, stacks, logins de registro, repositórios e configurações, o histórico de implantações e os eventos do Engine. Os segredos nele são criptografados. |
| `homeport.db-wal`, `homeport.db-shm` | O write-ahead log do SQLite e o índice dele, presentes enquanto o Homeport está em execução. As mudanças mais recentes podem estar só no arquivo `-wal` até o Homeport ser fechado.            |
| `homeport.db.before-restore-<date>`  | Uma cópia do banco de dados feita logo antes de cada [restauração](#restaurando-um-backup). Exclua as que você não quiser mais.                                                                |
| `repositories/` e `synced-stacks/`   | Os clones e as cópias de trabalho dos repositórios das [stacks sincronizadas](/docs/guides/synced-stacks/). O Homeport os cria de novo a partir dos próprios repositórios.                     |

[O que o Homeport armazena](/docs/reference/security/#o-que-o-homeport-armazena) lista o que o banco de dados contém e quais partes são criptografadas.

### A entrada no chaveiro

A chave que criptografa os segredos não fica na pasta. Ela é uma entrada no chaveiro do seu sistema operacional, com o serviço `com.homeport.app` e a conta `secrets-key`: no **Acesso às Chaves** no macOS, no **Gerenciador de Credenciais → Credenciais do Windows** no Windows, e em **Senhas e chaves** (Seahorse) ou no KWallet no Linux. Ela pertence à sua conta de usuário neste computador.

O banco de dados e essa entrada andam juntos. Um banco de dados sem a sua chave ainda abre, com todos os nomes, endereços e arquivos compose, mas nenhum dos segredos dele pode ser descriptografado: hosts e clusters mostram **Credenciais perdidas**. É por isso que uma cópia da pasta não basta para mudar para outro computador, e é por isso que o [backup no aplicativo](#fazendo-backup-no-homeport) é o recomendado.

### Os logs e a posição da janela

| Sistema | Logs                                    | Tamanho e posição da janela                                         |
| ------- | --------------------------------------- | ------------------------------------------------------------------- |
| Linux   | `~/.local/share/com.homeport.app/logs/` | `~/.config/com.homeport.app/.window-state.json`                     |
| macOS   | `~/Library/Logs/com.homeport.app/`      | `~/Library/Application Support/com.homeport.app/.window-state.json` |
| Windows | `%LOCALAPPDATA%\com.homeport.app\logs\` | `%APPDATA%\com.homeport.app\.window-state.json`                     |

Não vale a pena fazer backup de nenhum dos dois. **Configurações → Sobre → Abrir pasta de logs** abre os logs.

### O que continua sendo seu

- **As pastas das stacks vinculadas.** O Homeport guarda apenas onde ficam a pasta e os arquivos compose de uma [stack vinculada](/docs/guides/stacks/#stacks-vinculadas). A pasta, os arquivos compose e o `.env` dela são seus: faça backup deles como faz com os seus outros projetos, normalmente com git.
- **O seu kubeconfig, `~/.ssh` e `~/.docker`.** O Homeport os lê quando você importa a partir deles e nunca os altera.
- **Os repositórios de charts e os logins do helm.** Eles são do próprio helm, compartilhados com o seu terminal.

## Fazendo backup no Homeport

**Configurações → Dados → Fazer backup…** grava tudo em um único arquivo `.homeport`. Este é o backup preferível, porque ele leva os próprios segredos, protegidos por uma senha em vez do chaveiro deste computador: ele é restaurado em qualquer computador.

1. Abra **Configurações** e, em **Dados**, escolha **Fazer backup…**.
2. Digite duas vezes uma senha de pelo menos 12 caracteres. O Homeport nunca a armazena, e nada consegue abrir o arquivo sem ela.
3. Escolha onde salvar o arquivo.

Depois, o Homeport diz o que salvou e indica tudo o que teve de salvar sem o segredo, porque esse segredo não pôde ser descriptografado neste computador.

| No backup                                                                                                 | Fora do backup                                                                |
| --------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------- |
| Os hosts, com as chaves, senhas, certificados, bastions e as chaves de servidor que você confiou          | Os eventos do Engine                                                          |
| Os clusters, com os certificados, tokens e configurações de plugins exec                                  | O histórico de implantações                                                   |
| Os arquivos compose e os ambientes das stacks gerenciadas                                                 | As pastas e os arquivos das stacks vinculadas                                 |
| Os caminhos das pastas e dos arquivos das stacks vinculadas                                               | Os clones das stacks sincronizadas (criados de novo na próxima sincronização) |
| O repositório, a branch ou a tag, as configurações de sincronização e o ambiente das stacks sincronizadas | Os logs, e o tamanho e a posição da janela                                    |
| As stacks implantadas em um Swarm                                                                         | Tudo o que está nos seus hosts e clusters: contêineres, imagens, volumes      |
| Os logins de registro, com as senhas e os tokens deles                                                    | Os repositórios de charts do helm                                             |
| Os repositórios, com as chaves de deploy e os tokens deles                                                | Os encaminhamentos de porta, que terminam quando o Homeport é fechado         |
| As suas configurações                                                                                     |                                                                               |

O arquivo inteiro é criptografado com a senha, então sem ela o arquivo não revela nada, nem mesmo o nome de um host. Ele contém todos os segredos que o Homeport tem: guarde-o, e a senha dele, com o mesmo cuidado que as próprias chaves. [Backups](/docs/reference/security/#backups) descreve a criptografia.

Faça backup antes de experimentar uma prerelease, antes de reinstalar o seu sistema e sempre que tiver adicionado hosts que você não gostaria de configurar de novo.

### Restaurando um backup

1. Neste ou em outro computador, abra **Configurações** e, em **Dados**, escolha **Restaurar…**.
2. Escolha o arquivo `.homeport` e digite a senha dele. O Homeport lista o que o arquivo contém, e qual Homeport o criou e quando, antes de alterar qualquer coisa.
3. Escolha **Substituir tudo**.

Restaurar substitui todos os dados do Homeport pelos do backup; não mescla. O Homeport primeiro copia o banco de dados atual para `homeport.db.before-restore-<date>` na pasta de dados e depois substitui tudo em uma única etapa, então uma falha deixa os seus dados como estavam. Cada segredo é criptografado de novo com a chave do chaveiro deste computador.

Depois:

- Hosts e clusters se reconectam na hora, sem pedir uma chave, uma senha ou uma chave de servidor para confiar.
- Uma stack vinculada cuja pasta não está neste computador, no mesmo caminho, aparece como externa até estar.
- As stacks sincronizadas clonam o repositório de novo na próxima sincronização.
- A página Eventos e o histórico de implantações começam vazios.

Um backup feito por um Homeport mais antigo é restaurado em um mais novo. Um feito por um Homeport mais novo é recusado: atualize o Homeport primeiro.

Para voltar ao que você tinha antes de uma restauração, saia do Homeport, renomeie `homeport.db` para outra coisa, renomeie o `homeport.db.before-restore-<date>` que você quer para `homeport.db`, exclua qualquer `homeport.db-wal` e `homeport.db-shm` que tenha ficado ao lado dele e abra o Homeport de novo.

### Levando só alguns hosts

**Exportar** e **Importar** na página Hosts levam os hosts que você escolher, com os segredos deles, em um arquivo do mesmo tipo, e os adicionam aos hosts que o outro computador já tem. Veja [mudar para outro computador](/docs/reference/security/#mudar-para-outro-computador).

## Copiando os arquivos você mesmo

Uma cópia do banco de dados serve para um backup do sistema deste computador, como o Time Machine, o Histórico de Arquivos ou um script seu. Ela mantém o que o backup no aplicativo deixa de fora, os eventos do Engine e o histórico de implantações, mas **não a chave do chaveiro**: ela é restaurada com os segredos apenas para o mesmo usuário no mesmo computador, ou em um cujo chaveiro foi restaurado junto com ela. Em qualquer outro lugar, todos os nomes, endereços e arquivos compose voltam, e os segredos precisam ser [digitados de novo](#se-os-segredos-forem-perdidos).

### Com o Homeport fechado

Saia do Homeport, pelo **Sair** da bandeja se ele estiver [fechado para a bandeja do sistema](/docs/guides/settings/#área-de-trabalho), e depois copie a pasta de dados inteira. Depois que o Homeport sai, o `homeport.db` contém tudo e os arquivos `-wal` e `-shm` deixam de existir.

### Com o Homeport em execução

Não copie só o `homeport.db` enquanto o Homeport está em execução: as mudanças mais recentes ainda podem estar no `homeport.db-wal`, e uma cópia feita no meio de uma gravação pode ficar inconsistente. Em vez disso, peça ao SQLite uma cópia consistente, com a ferramenta de linha de comando `sqlite3` (incluída no macOS; no Linux e no Windows, instale-a a partir do pacote `sqlite3` da sua distribuição ou de [sqlite.org](https://sqlite.org/download.html)):

```sh
# Linux
sqlite3 ~/.local/share/com.homeport.app/homeport.db ".backup '$HOME/homeport-backup.db'"

# macOS
sqlite3 "$HOME/Library/Application Support/com.homeport.app/homeport.db" ".backup '$HOME/homeport-backup.db'"
```

```powershell
# Windows
sqlite3 "$env:APPDATA\com.homeport.app\homeport.db" ".backup '$HOME\homeport-backup.db'"
```

O resultado é um único arquivo completo, sem nenhum `-wal` ao lado. Para verificá-lo:

```sh
sqlite3 ~/homeport-backup.db "PRAGMA integrity_check;"
```

Ele imprime `ok`.

### Restaurando uma cópia

1. Saia do Homeport.
2. Na pasta de dados, tire o `homeport.db` do caminho e exclua `homeport.db-wal` e `homeport.db-shm` se estiverem lá: eles pertencem ao banco de dados que você está substituindo.
3. Coloque a sua cópia lá, com o nome `homeport.db`. Se você copiou a pasta inteira, coloque a pasta de volta.
4. Abra o Homeport.

O Homeport atualiza uma cópia feita por uma versão mais antiga quando a abre. Uma cópia usada por último por um Homeport mais novo é recusada, intacta, com [uma mensagem que indica a versão a instalar](/docs/reference/troubleshooting/#este-banco-de-dados-foi-usado-pela-última-vez-pelo-homeport--ou-por-uma-versão-mais-recente).

## Se os segredos forem perdidos

Depois de uma redefinição do chaveiro, de uma nova conta de usuário ou de um banco de dados copiado aberto em outro computador, o Homeport mantém tudo o que ainda consegue ler e marca o que não consegue. Nada é excluído, e o que não é segredo (nomes, endereços, arquivos compose, caminhos de pastas) está intacto. Digite de novo:

| O quê                                   | Como                                                                                                                                                                  |
| --------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Hosts TLS**                           | Na página do host, carregue de novo o certificado da CA, o certificado do cliente e a chave do cliente, e escolha **Salvar**.                                         |
| **Hosts SSH com uma chave importada**   | Cole de novo a chave privada e a senha dela, e escolha **Salvar**.                                                                                                    |
| **Hosts SSH com uma chave do Homeport** | Escolha **Gerar nova chave** e depois [autorize a nova chave](/docs/hosts/ssh/#autorize-a-chave-do-homeport) no servidor. Remova lá a linha `homeport@` antiga.       |
| **Hosts Incus via HTTPS**               | Remova o host e [adicione-o de novo com um novo token de confiança](/docs/hosts/incus/#via-https-com-um-token-de-confiança).                                          |
| **Clusters Kubernetes**                 | **Atualizar a partir do kubeconfig** na página do cluster.                                                                                                            |
| **Clusters Nomad**                      | Cole o token ACL, e os certificados se ele os usar, nas **Configurações** do cluster.                                                                                 |
| **Ambientes das stacks**                | Digite de novo o ambiente de uma stack gerenciada ou sincronizada na página dela. As stacks vinculadas guardam o delas no `.env` da pasta, que o Homeport nunca teve. |
| **Logins de registro**                  | Em **Configurações → Registros**, edite cada login e digite a senha ou o token dele.                                                                                  |
| **Repositórios**                        | Em **Configurações → Repositórios**, crie uma **Nova chave** e adicione-a ao repositório na sua forja, ou use **Substituir o token**.                                 |

Os hosts de socket local, TCP e ssh-agent não guardam nenhum segredo e continuam funcionando. Os ambientes registrados no histórico de implantações não podem ser digitados de novo; os arquivos compose do histórico continuam legíveis.

Veja [Credenciais perdidas](/docs/reference/troubleshooting/#credenciais-perdidas) na solução de problemas.

## Removendo os seus dados

[Remover os seus dados](/docs/reference/updating/#remover-os-seus-dados) lista as pastas e a entrada do chaveiro a excluir, e as chaves a remover dos seus servidores.
