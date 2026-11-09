---
title: Releases do Helm
description: Liste, instale, atualize, reverta e desinstale as releases do Helm de um cluster, e gerencie os repositórios de charts, com a CLI helm deste computador.
---

**Helm**, na barra lateral de um cluster, mostra as releases do [Helm](https://helm.sh) nele: as mesmas releases que o `helm list` mostra em um terminal, tenham sido instaladas pelo Homeport ou não.

## Do que o Homeport precisa

O Homeport executa o comando `helm`, versão 3 ou posterior, **neste** computador. Instale-o com `brew install helm` no macOS, `winget install Helm.Helm` (ou Chocolatey, ou Scoop) no Windows e `snap install helm --classic`, o pacote da sua distribuição ou o script de helm.sh no Linux. **Configurações → Esta máquina** mostra o helm que o Homeport encontrou; sem um, a página Helm diz o que instalar no seu sistema, e todas as outras páginas do cluster funcionam.

O Homeport não entrega o seu kubeconfig ao helm. Para cada comando do helm, ele abre uma conexão privada com o cluster no endereço de loopback deste computador e dá ao helm um kubeconfig temporário que aponta para ela, contendo um token de uso único e nenhuma credencial do cluster. O Homeport adiciona as próprias credenciais do cluster — certificados, tokens, tokens dos plugins exec — a cada requisição no caminho, como em todas as outras páginas, e exclui o kubeconfig quando o comando termina. Assim, o helm alcança todos os clusters que o Homeport alcança, clusters na nuvem com plugins exec incluídos, e nenhuma credencial é gravada em disco.

O Helm guarda as releases como Secrets no Namespace de cada release: o Homeport não instala nada no cluster.

## Releases

A lista mostra o nome de cada release, o Namespace, o chart e a versão do chart, a versão do app, a revisão, o status e quando ela foi alterada pela última vez. Escolha um Namespace no canto superior direito, ou todos. O helm não tem atualizações em tempo real, então a lista pergunta de novo a cada 30 segundos enquanto está aberta, e **Atualizar** pergunta na hora.

Uma release **com falha** é marcada em vermelho, e uma **pendente** de uma instalação, atualização ou reversão (`pending-install`, `pending-upgrade`, `pending-rollback`) em âmbar: uma release pendente fica bloqueada até que essa operação termine.

Se as suas credenciais não podem listar Secrets em todos os Namespaces, o Homeport lista as releases do Namespace padrão do cluster em vez disso e avisa; escolha outro Namespace para ver as releases dele.

## Instalar um chart

**Instalar chart** (ou <kbd>n</kbd>) na aba Releases instala um chart no cluster selecionado:

1. **Escolha o chart.** Pesquise os charts dos [repositórios](#repositórios-de-charts) do helm e clique em **Usar** ou, em **De um registro OCI**, informe uma referência como `oci://registry-1.docker.io/bitnamicharts/nginx`, que instala sem adicionar um repositório.
2. **Escolha a versão, o nome e o Namespace.** As versões de um chart de repositório são listadas, a mais recente primeiro; a versão de um chart OCI é digitada, ou deixada vazia para a mais recente. A release recebe de início o nome do chart: no máximo 53 letras minúsculas, dígitos, hifens e pontos. O Namespace é o que a página mostra (ou o padrão do cluster) e é criado se não existir, a menos que você desmarque essa opção.
3. **Edite os valores.** O editor começa com os valores padrão do chart (`helm show values`); altere o que precisar, e **Redefinir para os padrões do chart** os restaura. Os valores são verificados como YAML antes de o helm ser executado e vão para o helm pela entrada padrão dele — nunca em um arquivo.
4. **Pré-visualizar**, opcionalmente: o helm renderiza o chart com os seus valores e a ajuda do cluster (`--dry-run=server`), e o Homeport mostra o manifesto que ele aplicaria, hooks incluídos, e as notas do chart. Nada muda no cluster.
5. **Instalar.** A saída do helm aparece aos poucos, como o `helm install` a imprime. Com **Aguardar até que os recursos dela estejam prontos**, o helm aguarda até que os Deployments, Pods e Services da release estejam prontos, por no máximo os minutos indicados (5 por padrão), e marca a release como com falha se não estiverem. **Abrir release** então abre a página dela.

Charts de uma pasta local ou de um arquivo `.tgz` não são instalados pelo Homeport; nem charts de registros OCI que exigem login, a menos que você tenha feito login com `helm registry login` em um terminal.

## A página de uma release

Clique em uma release para abri-la:

- **Histórico** — todas as revisões, a mais recente primeiro, com a data, o status, o chart, a versão do app e a descrição do helm do que aconteceu (`Install complete`, `Upgrade "web" failed: …`).
- **Valores** — os valores informados na instalação ou na atualização (**Fornecidos pelo usuário**), ou todos os valores com que o chart foi renderizado, padrões incluídos (**Calculados, com os padrões do chart**).
- **Notas** — o que o chart imprime depois de uma instalação (o `NOTES.txt` dele).
- **Manifesto** — os objetos do Kubernetes que a revisão renderizou, como YAML.
- **Recursos** — esses objetos por kind, nome e Namespace, cada um abrindo a página dele no Homeport: a página própria para Deployments, Pods, Services e afins, a página em [Recursos da API](/docs/clusters/api-resources/) para o resto.

Valores, Notas, Manifesto e Recursos mostram a revisão atual; escolha uma anterior acima deles para ver com o que ela foi instalada.

## Atualizar uma release

**Atualizar**, na página de uma release, muda a versão do chart, os valores ou ambos:

- **Chart** — o chart a partir do qual atualizar. O helm não registra de onde um chart veio, então o Homeport procura um chart com esse nome nos seus repositórios, os que têm a versão atual da release primeiro. Escolha outro com **Outro chart…**, como `repository/chart` ou uma referência `oci://` — que é o que uma release instalada a partir de um registro OCI precisa.
- **Versão do chart** — as versões desse chart, a mais recente primeiro, ou digitada para um chart OCI.
- **Valores** — o editor começa com os valores fornecidos pelo usuário na release. Eles substituem os da release: os padrões do chart preenchem o resto. **Alterações nos valores** os mostra lado a lado com os atuais, e **Redefinir para os valores atuais** os restaura.
- **Aguardar até que os recursos dela estejam prontos**, por no máximo os minutos indicados.

**Pré-visualizar** renderiza a atualização com a ajuda do cluster sem alterar nada, e mostra o novo manifesto e o que muda nele em relação ao atual. **Atualizar** executa `helm upgrade`, com a saída do helm em tempo real. Cada atualização é uma nova revisão em **Histórico**.

## Reverter

Em **Histórico**, **Reverter para a revisão N** em qualquer revisão anterior pede confirmação antes, indicando as versões do chart e do app agora e depois, e então executa `helm rollback`: o helm restaura o chart e os valores dessa revisão como uma nova revisão, então uma reversão pode ela mesma ser revertida. Marque **Aguardar até que os recursos dela estejam prontos** para que o helm aguarde por eles.

Uma release presa em **pendente** depois de uma instalação, atualização ou reversão interrompida fica bloqueada: o helm se recusa a alterá-la até que a operação termine. Revertê-la para a última revisão implantada a desbloqueia.

## Desinstalar

**Desinstalar**, na página de uma release, indica a release, o Namespace e o cluster dela e lista os objetos que serão excluídos com ela, antes de o `helm uninstall` ser executado. Os objetos que um chart marca para manter (`helm.sh/resource-policy: keep`) e os PersistentVolumeClaims dos StatefulSets ficam no cluster. **Manter o histórico dela** deixa a release listada como `uninstalled`, para que possa ser revertida; sem isso, a release some da lista. **Aguardar até que os objetos dela sejam excluídos** faz o helm aguardar por eles.

## Repositórios de charts

**Repositórios de charts**, a segunda aba da página Helm, lista os repositórios que o helm conhece neste computador: a mesma lista que o `helm repo list` em um terminal, compartilhada por todos os clusters. O Homeport não mantém uma lista própria, então um repositório adicionado em um terminal aparece aqui, e um adicionado aqui aparece no `helm repo list`.

- **Adicionar repositório** pede um nome (letras, dígitos, pontos, hifens e sublinhados) e a URL `https://` do repositório, como `https://charts.bitnami.com/bitnami`, e baixa o índice dele.
- **Atualizar todos**, ou **Atualizar** em um repositório, baixa os índices mais recentes, como o `helm repo update` faz, com a saída do helm mostrada em tempo real. Os charts publicados desde a última atualização só aparecem depois de uma.
- **Remover** esquece um repositório, depois que você confirma. As releases instaladas a partir dele continuam em execução.

**Encontrar charts** pesquisa os charts desses repositórios por nome ou descrição, como o `helm search repo` faz: digite `postgres` depois de adicionar o repositório da Bitnami para ver os charts de PostgreSQL dele, com as versões mais recentes do chart e do app.

O Homeport só adiciona repositórios públicos. Um que exige login é adicionado em um terminal, com `helm repo add --username …`, onde o helm guarda a senha dele como sempre faz; ele então aparece aqui e funciona como os outros. Um chart em um registro OCI (`oci://…`) não precisa de repositório.

## Quando não funciona

- **A CLI helm não está instalada**: instale o helm 3 como acima e depois reabra a página.
- **Helm 2**: o Homeport só controla o Helm 3 e posteriores. Instale um helm atual.
- **Não permitido**: o helm lê as releases a partir de Secrets; listá-las exige `list` em `secrets` no Namespace. Detalhes indica o que foi recusado.
- **Outra instalação, atualização ou reversão está em andamento**: a release está bloqueada por uma operação pendente. Aguarde até que ela termine, ou reverta para a última revisão implantada.
- **Já existe uma release com este nome**: os nomes de release são únicos em um Namespace. Escolha outro, ou atualize a release existente.
- **Chart ou versão não encontrado**: confira o nome e a versão; um chart de repositório precisa que o repositório dele esteja adicionado e [atualizado](#repositórios-de-charts). Para atualizar uma release cujo chart não está em nenhum dos seus repositórios, adicione esse repositório primeiro.
- **Os valores não são YAML válido**: a mensagem indica a linha; corrija-a e tente de novo.
- **Desistiu de aguardar**: com **Aguardar**, um recurso não ficou pronto a tempo. Veja os Pods e os Events da release para saber o que está travado.
- **Não é um repositório de charts, ou não pode ser alcançado**: a URL não serve nenhum `index.yaml`, ou este computador não consegue alcançá-la. Detalhes mostra o que o helm disse.
- **Não conectado**: o helm passa pela conexão do cluster no Homeport; a página do cluster diz por que ele não está conectado.
