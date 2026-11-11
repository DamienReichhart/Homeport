### The `repositories` namespace, in Português (Brasil) (pt-BR).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/pt-BR/repositories.ts:
### change the text there, then run it again.

repositories-title = Repositórios
repositories-description = Repositórios git a partir dos quais as stacks sincronizadas são implantadas, em todos os hosts. O Homeport apenas os lê.
repositories-add = Adicionar repositório
repositories-empty = Nenhum repositório ainda. Adicione um para criar uma stack sincronizada.
repositories-manageOne = Gerenciar o acesso a { $repository }
repositories-removeOne = Remover { $repository }
repositories-remove-title = Remover { $repository }?
repositories-remove-description = O Homeport esquece a chave de deploy ou o token dele e exclui o clone neste computador. O repositório em si não é alterado. Remova também a chave de deploy dele na sua forja.
repositories-noKeychain = Chaves de deploy e tokens são criptografados com uma chave guardada no chaveiro do sistema, e nenhum está disponível, então só repositórios públicos podem ser adicionados.
repositories-public = Um repositório público: o Homeport o lê sem credenciais.
repositories-added = { $repository } adicionado
repositories-removed = { $repository } removido
repositories-keyRotated = Uma nova chave de deploy foi criada. Adicione-a ao repositório no lugar da antiga.
repositories-tokenReplaced = Token substituído
repositories-hostKeyReset = O Homeport vai confiar na chave que o servidor apresentar da próxima vez.
repositories-auth-withKey = Com uma chave de deploy, em { $host }
repositories-auth-withToken = Como { $username } com um token, em { $host }
repositories-auth-public = Público, em { $host }
repositories-dialog-addTitle = Adicionar um repositório
repositories-dialog-description = O Homeport o clona neste computador para implantar stacks sincronizadas a partir dele. Ele nunca faz commit nem push.
repositories-dialog-url = URL do repositório
repositories-dialog-auth = Acesso
repositories-dialog-authKey = Chave de deploy
repositories-dialog-authToken = Token
repositories-dialog-authNone = Público
repositories-dialog-authKeyHelp = O Homeport cria uma chave só para este repositório. Você a adiciona ao repositório com acesso de leitura; suas próprias chaves SSH nunca são usadas.
repositories-dialog-authTokenHelp = Um token de acesso que pode ler o conteúdo do repositório, enviado via HTTPS. Criptografado neste computador e nunca mais mostrado.
repositories-dialog-authNoneHelp = Para um repositório público via HTTPS. Nada é armazenado.
repositories-dialog-username = Nome de usuário
repositories-dialog-token = Token de acesso
repositories-dialog-add = Adicionar
repositories-dialog-addWithKey = Adicionar e criar uma chave de deploy
repositories-missing-url = Digite a URL do repositório.
repositories-missing-validUrl = Digite uma URL SSH ou HTTPS, como git@github.com:owner/repo.git ou https://gitlab.com/owner/repo.git.
repositories-missing-auth = Escolha como o Homeport acessa este repositório.
repositories-missing-username = Digite o nome de usuário ao qual o token pertence.
repositories-missing-token = Digite o token de acesso.
repositories-deployKey-title = Chave de deploy
repositories-deployKey-github = GitHub: <strong>Settings → Deploy keys → Add deploy key</strong> do repositório.
repositories-deployKey-gitlab = GitLab: <strong>Settings → Repository → Deploy keys</strong> do projeto.
repositories-deployKey-gitea = Gitea e Forgejo: <strong>Settings → Deploy keys → Add deploy key</strong> do repositório.
repositories-deployKey-readOnly = Deixe o acesso de escrita desativado: o Homeport apenas lê. Depois teste o acesso abaixo.
repositories-deployKey-fingerprint = Impressão digital: <mono>{ $fingerprint }</mono>
repositories-rotate-button = Nova chave
repositories-rotate-title = Criar uma nova chave de deploy?
repositories-rotate-description = A chave atual deixa de funcionar para o Homeport. Adicione a nova ao repositório e remova a antiga de lá.
repositories-rotate-confirm = Criar uma nova chave
repositories-token-replace = Substituir o token
repositories-token-save = Salvar token
repositories-hostKey-trusted = Chave confiável de { $host }: <mono>{ $fingerprint }</mono>
repositories-hostKey-reset = Redefinir
repositories-hostKey-resetTitle = Redefinir a chave confiável de { $host }?
repositories-hostKey-resetDescription = Faça isso somente se { $host } mudou a chave SSH e você conferiu a nova impressão digital no site dele. O Homeport vai confiar em qualquer chave que ele apresentar em seguida.
repositories-test-button = Testar acesso
repositories-test-testing = Testando…
repositories-test-success = O Homeport consegue lê-lo: { $branches ->
        [one] { NUM($branches) } branch
       *[other] { NUM($branches) } branches
    } e { $tags ->
        [one] { NUM($tags) } tag
       *[other] { NUM($tags) } tags
    }.
