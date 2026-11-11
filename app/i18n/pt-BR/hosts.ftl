### The `hosts` namespace, in Português (Brasil) (pt-BR).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/pt-BR/hosts.ts:
### change the text there, then run it again.

hosts-title = Hosts
hosts-description = Os Docker Engines aos quais o Homeport se conecta.
hosts-add = Adicionar host
hosts-empty = Nenhum host ainda.
hosts-column-name = Nome
hosts-column-type = Tipo
hosts-column-address = Endereço
hosts-column-status = Status
hosts-statusTitle = { $label }: { $detail }
hosts-saved = Host salvo
hosts-notFound = Este host não existe mais.
hosts-addPage-description = Conecte o Homeport a um Docker Engine.
hosts-switcher-label = Host
hosts-switcher-placeholder = Nenhum host
hosts-switcher-add = Adicionar host…
hosts-switcher-manage = Gerenciar hosts
hosts-switcher-hosts = Hosts
hosts-switcher-clusters = Clusters
hosts-switcher-addCluster = Adicionar cluster…
hosts-switcher-manageClusters = Gerenciar clusters
hosts-switcher-whyUnusable = Por que { $name } não pode ser usado
hosts-detail-enabled = Ativado
hosts-detail-reconnect = Reconectar
hosts-connection-title = Conexão
hosts-remove-title = Remover host
hosts-remove-description = O Homeport esquece este host, as credenciais dele e as definições das stacks gerenciadas dele. Nada do que está em execução no host é afetado.
hosts-remove-confirmTitle = Remover { $name }?
hosts-remove-confirmDescription = Exclui a conexão armazenada, a chave SSH ou os certificados, e as definições compose das stacks gerenciadas dele. Contêineres, imagens e volumes no host permanecem como estão.
hosts-sshSetup-title = Autorizar o Homeport no servidor
hosts-sshSetup-description = Execute estes comandos em <mono>{ $server }</mono> como o usuário SSH e depois reconecte.
hosts-sshSetup-descriptionBastions = O Homeport faz login em cada um destes com a mesma chave. Adicione-a em todos, como o usuário SSH de cada um, e depois reconecte.
hosts-sshSetup-servers = Adicione-a em:
hosts-sshSetup-addKey = 1. Adicione a chave pública
hosts-sshSetup-dockerGroup = 2. Permita que o usuário acesse o socket do Engine (se necessário)
hosts-sshSetup-fingerprint = Impressão digital da chave <mono>{ $fingerprint }</mono>
hosts-regenerate-button = Gerar nova chave
hosts-regenerate-title = Gerar uma nova chave SSH?
hosts-regenerate-description = A chave atual deixa de funcionar. Você vai precisar adicionar a nova chave pública ao servidor.
hosts-regenerate-confirm = Gerar nova
hosts-trust-untrusted-title = Este é o servidor certo?
hosts-trust-untrusted-description = Primeira conexão. O servidor se identifica com a impressão digital de chave <mono>{ $fingerprint }</mono>. Compare-a com <mono>{ $command }</mono> no servidor.
hosts-trust-untrusted-button = Confiar nesta chave
hosts-trust-untrusted-bastionTitle = Este é o bastion certo?
hosts-trust-untrusted-bastionDescription = Primeira conexão com o bastion <mono>{ $bastion }</mono>. Ele se identifica com a impressão digital de chave <mono>{ $fingerprint }</mono>. Compare-a com <mono>{ $command }</mono> no bastion.
hosts-trust-mismatch-bastionTitle = A chave do bastion { $bastion } mudou
hosts-trust-mismatch-title = A chave do servidor mudou
hosts-trust-mismatch-description = Era esperada <mono>{ $expected }</mono>, mas o servidor apresentou <mono>{ $fingerprint }</mono>. Isso acontece depois de reinstalar o servidor, mas também pode significar que alguém está interceptando a conexão.
hosts-trust-mismatch-button = Confiar na nova chave
hosts-trust-mismatch-confirmTitle = Confiar na nova chave do servidor?
hosts-trust-mismatch-confirmDescription = Faça isso somente se você souber por que a chave mudou. Nova impressão digital: { $fingerprint }.
hosts-trust-mismatch-confirm = Confiar na nova chave
hosts-kind-ssh = SSH
hosts-kind-tls = TLS
hosts-kind-tcp = TCP
hosts-kind-local = Socket local
hosts-form-keepHint = Deixe em branco para manter o valor armazenado.
hosts-form-name = Nome
hosts-form-engine = Engine
hosts-form-engineHint = Compatível com Docker abrange Docker e Podman; o Homeport os distingue ao se conectar. O containerd é acessado pelo socket dele, aqui ou via SSH; o Incus também via HTTPS (TLS).
hosts-form-socketOnly = { $engine } só responde no socket dele: conecte localmente ou via SSH.
hosts-form-noPlainTcp = { $engine } não responde via TCP simples: use TLS, SSH ou o socket dele.
hosts-form-socketPath = Caminho do socket
hosts-form-socketPathHint = Um socket Unix ou um named pipe como { $pipe } no Windows.
hosts-form-address = Endereço
hosts-form-addressHint = host:porta, por exemplo { $example }.
hosts-form-tcpWarning-title = Sem criptografia e sem autenticação
hosts-form-tcpWarning-description = Qualquer pessoa que consiga acessar esta porta controla o Engine. Prefira SSH ou TLS fora de uma rede confiável.
hosts-form-loadFromFile = Carregar de arquivo…
hosts-incus-trust = Confiança
hosts-incus-trust-token = Token de confiança
hosts-incus-trust-certificate = Certificado existente
hosts-incus-token = Token de confiança
hosts-incus-tokenHint = Execute isto no servidor Incus e cole o que ele imprimir. O Homeport cria o próprio certificado, confere o do servidor com o token e se registra.
hosts-incus-tokenReads = Para { $name }, em { $addresses }
hosts-incus-tokenFingerprint = Certificado do servidor { $fingerprint }
hosts-incus-tokenExpires = Expira { $time }
hosts-incus-serverCert = Certificado do servidor a confiar
hosts-incus-serverCert-choose = Escolha o certificado do servidor
hosts-incus-serverCertHint = O próprio certificado do Incus (/var/lib/incus/server.crt). O Homeport aceita exatamente este, seja qual for o endereço.
hosts-incus-clientCert = Certificado de cliente
hosts-incus-clientCertHint = Já confiável para o Incus: incus config trust add-certificate.
hosts-incus-clientKey = Chave de cliente
hosts-keychain-title = Nenhum chaveiro do sistema
hosts-keychain-description = Chaves e certificados são criptografados com uma chave guardada no chaveiro do sistema, e nenhum está disponível. Use o ssh-agent, um socket local ou TCP em vez disso.
hosts-ssh-server = Servidor
hosts-ssh-port = Porta
hosts-ssh-user = Usuário
hosts-ssh-userHint = Precisa de acesso ao socket do Docker, normalmente por estar no grupo docker.
hosts-ssh-socketPath = Caminho do socket remoto
hosts-ssh-socketPathHint = O Docker rootless usa { $path }.
hosts-ssh-auth = Autenticação
hosts-ssh-auth-generated = Nova chave para este host
hosts-ssh-auth-imported = Chave existente
hosts-ssh-generated-editing = Este host mantém a chave dele. Gere uma nova na página do host.
hosts-ssh-generated-new = O Homeport cria uma chave Ed25519 para este host e mostra o comando que a autoriza no servidor.
hosts-ssh-privateKey = Chave privada
hosts-ssh-privateKey-choose = Escolha a chave privada
hosts-ssh-privateKeyHint = Armazenada criptografada; ela nunca sai desta máquina.
hosts-ssh-passphrase = Senha da chave
hosts-bastions-title = Bastions
hosts-bastions-description = Servidores nos quais o Homeport faz login primeiro, em ordem, quando este só aceita SSH a partir deles (ProxyJump).
hosts-bastions-add = Adicionar bastion
hosts-bastions-one = Bastion { $n }
hosts-bastions-server = Servidor do bastion { $n }
hosts-bastions-port = Porta do bastion { $n }
hosts-bastions-user = Usuário do bastion { $n }
hosts-bastions-moveUp = Mover o bastion { $n } para cima
hosts-bastions-moveDown = Mover o bastion { $n } para baixo
hosts-bastions-remove = Remover o bastion { $n }
hosts-bastions-auth-sameAsHost = Igual ao do host
hosts-bastions-auth-imported = Chave própria
hosts-bastions-auth-agent = ssh-agent
hosts-import-title = Importar
hosts-import-description = Adicione os servidores que a sua configuração SSH descreve ou os Engines dos seus contextos Docker.
hosts-import-sshConfig-button = Importar de ~/.ssh/config
hosts-import-sshConfig-title = Importar de ~/.ssh/config
hosts-import-sshConfig-description = Todos os servidores que a sua configuração SSH nomeia, com o usuário, a porta, a chave e os bastions ProxyJump de cada um. As chaves são lidas uma vez e armazenadas criptografadas; o arquivo não é alterado.
hosts-import-sshConfig-empty = A sua configuração SSH não nomeia nenhum servidor sem curingas.
hosts-import-dockerContexts-button = Importar contextos Docker
hosts-import-dockerContexts-title = Importar contextos Docker
hosts-import-dockerContexts-description = Os contextos da CLI do Docker, com os certificados TLS deles. Os certificados são lidos uma vez e armazenados criptografados; o ~/.docker não é alterado.
hosts-import-dockerContexts-empty = A CLI do Docker não tem nenhum contexto além de default.
hosts-import-column-name = Nome do host
hosts-import-column-address = Endereço e socket
hosts-import-column-auth = Autentica com
hosts-import-column-notes = Observações
hosts-import-pick = Importar { $name }
hosts-import-name = Nome do host para { $name }
hosts-import-passphrase = Senha da chave de { $name }
hosts-import-socket = Caminho do socket remoto de { $name }
hosts-import-via = através de { $bastions }
hosts-import-nameTaken = Já existe um host com este nome; renomeie-o para importá-lo.
hosts-import-auth-key = a chave dele
hosts-import-auth-agent = ssh-agent
hosts-import-auth-certificates = certificados TLS
hosts-import-auth-none = nada
hosts-import-problem-nameEmpty = Dê um nome a ele.
hosts-import-problem-nameTwice = Dois hosts ficariam com este nome.
hosts-import-problem-nameTaken = Escolha outro nome.
hosts-import-problem-passphrase = Digite a senha da chave.
hosts-import-problem-socket = O caminho do socket precisa ser absoluto.
hosts-import-submit = { $count ->
        [0] Importar
        [one] Importar { NUM($count) } host
       *[other] Importar { NUM($count) } hosts
    }
hosts-import-failed = { $name } não foi importado:
hosts-import-done = { $count ->
        [one] { NUM($count) } host importado
       *[other] { NUM($count) } hosts importados
    }
hosts-bundle-fileType = Exportação do Homeport
hosts-bundle-passphrase = Senha
hosts-bundle-passphraseHint = Pelo menos { $min } caracteres. O Homeport não a guarda: sem ela, o arquivo não pode ser aberto.
hosts-bundle-confirmation = Senha novamente
hosts-bundle-problem-short = Use pelo menos { $min } caracteres.
hosts-bundle-problem-mismatch = As duas senhas são diferentes.
hosts-bundle-export-button = Exportar
hosts-bundle-export-title = Exportar hosts
hosts-bundle-export-description = Grava os hosts que você escolher, com as chaves, os certificados e as chaves de servidor confiáveis deles, em um único arquivo criptografado com uma senha. Importe-o em outro computador para usá-los lá.
hosts-bundle-export-hosts = Hosts a exportar
hosts-bundle-export-saveTitle = Salvar a exportação
hosts-bundle-export-submit = { $count ->
        [0] Exportar
        [one] Exportar { NUM($count) } host…
       *[other] Exportar { NUM($count) } hosts…
    }
hosts-bundle-export-done = { $count ->
        [one] { NUM($count) } host exportado
       *[other] { NUM($count) } hosts exportados
    }
hosts-bundle-export-withoutSecrets = Estes foram exportados sem as credenciais, que não podem ser abertas neste computador:
hosts-bundle-import-button = Importar
hosts-bundle-import-title = Importar hosts de uma exportação
hosts-bundle-import-description = Abra um arquivo exportado do Homeport com a senha dele e depois escolha os hosts a adicionar. As credenciais deles são armazenadas criptografadas neste computador.
hosts-bundle-import-choose = Escolher arquivo…
hosts-bundle-import-chooseTitle = Escolha uma exportação do Homeport
hosts-bundle-import-open = Abrir
hosts-bundle-import-renamed = Já existe um host chamado { $name }, então este é renomeado.
hosts-bundle-import-secretsMissing = Exportado sem as credenciais; digite-as na página dele depois de importar.
hosts-tls-ca = Certificado de CA
hosts-tls-ca-choose = Escolha o certificado de CA
hosts-tls-cert = Certificado de cliente
hosts-tls-cert-choose = Escolha o certificado de cliente
hosts-tls-key = Chave de cliente
hosts-tls-key-choose = Escolha a chave de cliente
hosts-palette-add = Adicionar host
hosts-palette-manage = Gerenciar hosts
