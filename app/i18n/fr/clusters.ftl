### The `clusters` namespace, in Français (fr).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/fr/clusters.ts:
### change the text there, then run it again.

clusters-title = Clusters
clusters-summary = { $count ->
        [one] { NUM($count) } cluster
       *[other] { NUM($count) } clusters
    }
clusters-add = Ajouter un cluster
clusters-notFound = Ce cluster n’existe plus.
clusters-empty-title = Aucun cluster pour l’instant
clusters-empty-description = Ajoutez les clusters Kubernetes que votre kubeconfig atteint. Homeport n’y installe rien.
clusters-column-name = Nom
clusters-column-status = État
clusters-column-kind = Type
clusters-column-server = API
clusters-column-auth = Se connecte avec
clusters-palette-add = Ajouter un cluster
clusters-palette-manage = Gérer les clusters
clusters-auth-none = Aucun identifiant
clusters-auth-clientCertificate = Certificat client
clusters-auth-token = Jeton
clusters-auth-basic = Nom d’utilisateur et mot de passe
clusters-auth-exec = Plugin exec ({ $command })
clusters-auth-oidc = OpenID Connect
clusters-auth-aclToken = Jeton ACL
clusters-addPage-description = Chaque contexte Kubernetes que vous choisissez, ou chaque agent Nomad que vous indiquez, devient un cluster. Ses identifiants sont enregistrés chiffrés, jamais en clair sur le disque.
clusters-addPage-fromMachine = Depuis le kubeconfig de cette machine
clusters-addPage-paste = Coller un kubeconfig
clusters-addPage-nomad = Nomad
clusters-paste-description = Le kubeconfig est lu en mémoire et jamais enregistré dans un fichier. Les fichiers auxquels il fait référence doivent être indiqués par des chemins complets.
clusters-paste-label = Kubeconfig
clusters-paste-read = Lire les contextes
clusters-import-readFrom = Lu depuis { $paths }
clusters-import-noContexts = Ce kubeconfig n’a aucun contexte
clusters-import-pick = Importer { $context }
clusters-import-name = Nom du cluster créé à partir de { $context }
clusters-import-current = actuel
clusters-import-added = Déjà ajouté sous le nom { $name }.
clusters-import-refresh = L’actualiser
clusters-import-submit = { $count ->
        [0] Importer
        [one] Importer { NUM($count) } cluster
       *[other] Importer { NUM($count) } clusters
    }
clusters-import-done = { $count ->
        [one] { NUM($count) } cluster ajouté
       *[other] { NUM($count) } clusters ajoutés
    }
clusters-import-problem-empty = Donnez un nom à chaque cluster.
clusters-import-problem-duplicate = Deux clusters seraient nommés « { $name } ».
clusters-import-problem-taken = Un cluster nommé « { $name } » existe déjà.
clusters-detail-enabled = Activé
clusters-detail-reconnect = Reconnecter
clusters-detail-name = Nom
clusters-detail-rename = Renommer
clusters-detail-renamed = Cluster renommé
clusters-connection-title = Connexion
clusters-connection-server = Serveur d’API
clusters-connection-address = Adresse
clusters-connection-region = Région
clusters-connection-customCa = Son propre certificat d’AC
clusters-connection-clientCertificate = Certificat client
clusters-connection-checkedAs = Certificat vérifié au nom de { $name }
clusters-connection-version = Version
clusters-connection-platform = Plateforme
clusters-connection-namespace = Namespace par défaut
clusters-connection-auth = Se connecte avec
clusters-connection-tlsServerName = Nom du certificat
clusters-connection-tls = TLS
clusters-connection-insecure = Le certificat du serveur n’est pas vérifié (insecure-skip-tls-verify)
clusters-connection-proxy = Proxy
clusters-connection-source = Importé depuis
clusters-source-machine = Contexte { $context } du kubeconfig de cette machine
clusters-source-pasted = Contexte { $context } d’un kubeconfig collé
clusters-nomad-description = Homeport communique avec l’API HTTP d’un agent Nomad, serveur ou client, depuis cet ordinateur. Rien n’est installé sur le cluster, et le CLI nomad n’est pas nécessaire ici.
clusters-nomad-name = Nom
clusters-nomad-address = Adresse
clusters-nomad-addressHelp = L’API HTTP de l’agent, telle que NOMAD_ADDR la donne au CLI nomad. Port 4646 si aucun n’est indiqué.
clusters-nomad-region = Région
clusters-nomad-regionHelp = Laissez vide pour utiliser la région de l’agent.
clusters-nomad-token = Jeton ACL
clusters-nomad-tokenHelp = L’ID secret du jeton, quand les ACL sont activées sur le cluster. La documentation de Homeport fournit une politique pour ce jeton.
clusters-nomad-tls = Paramètres TLS
clusters-nomad-tlsHelp = Uniquement pour un agent qui sert en HTTPS avec sa propre AC, ou qui demande un certificat client. Collez le contenu des fichiers PEM.
clusters-nomad-ca = Certificat d’AC
clusters-nomad-caHelp = L’AC qui a signé le certificat de l’agent, telle que NOMAD_CACERT la désigne. Les AC de cet ordinateur sont approuvées sans celle-ci.
clusters-nomad-clientCertificate = Certificat client
clusters-nomad-clientKey = Clé client
clusters-nomad-tlsServerName = Nom du serveur TLS
clusters-nomad-tlsServerNameHelp = Le nom auquel le certificat de l’agent est comparé, quand ce n’est pas celui de l’adresse, tel que NOMAD_TLS_SERVER_NAME le définit.
clusters-nomad-keep = Enregistré ; laissez vide pour le conserver
clusters-nomad-willBeRemoved = Sera supprimé à l’enregistrement
clusters-nomad-remove = Supprimer
clusters-nomad-keepIt = Le conserver
clusters-nomad-test = Tester la connexion
clusters-nomad-testing = Test en cours…
clusters-nomad-testedVersion = Connecté à Nomad { $version }, région { $region }.
clusters-nomad-testedNoVersion = Connecté à la région { $region }. Le jeton ne peut pas lire l’agent : sa version n’est donc pas connue.
clusters-nomad-add = Ajouter le cluster
clusters-nomad-added = { $name } ajouté
clusters-nomad-settings = Paramètres
clusters-nomad-settingsDescription = L’enregistrement reconnecte. Le jeton et la clé TLS ne sont plus jamais affichés : laissez-les vides pour les conserver.
clusters-nomad-save = Enregistrer les modifications
clusters-nomad-saved = Cluster enregistré
clusters-refresh-title = Actualiser depuis le kubeconfig
clusters-refresh-machine = Relit le contexte { $context } depuis le kubeconfig de cette machine, au cas où le cluster aurait été recréé ou ses identifiants renouvelés.
clusters-refresh-pasted = Collez à nouveau le kubeconfig pour relire le contexte { $context }, au cas où le cluster aurait été recréé ou ses identifiants renouvelés.
clusters-refresh-submit = Actualiser
clusters-refresh-done = Cluster actualisé depuis son kubeconfig
clusters-remove-title = Supprimer le cluster
clusters-remove-description = Homeport oublie le cluster et ses identifiants enregistrés. Rien ne change sur le cluster ni dans votre kubeconfig.
clusters-remove-confirmTitle = Supprimer { $name } ?
clusters-remove-confirmDescription = Vous pourrez l’ajouter à nouveau depuis son kubeconfig.
