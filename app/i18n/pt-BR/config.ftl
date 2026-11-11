### The `config` namespace, in Português (Brasil) (pt-BR).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/pt-BR/config.ts:
### change the text there, then run it again.

config-nav = Configuração
config-title = Configuração
config-configMaps = ConfigMaps
config-secrets = Secrets
config-configMap = ConfigMap
config-secret = Secret
config-configMapsSummary = { $count ->
        [one] { NUM($count) } ConfigMap
       *[other] { NUM($count) } ConfigMaps
    }
config-secretsSummary = { $count ->
        [one] { NUM($count) } Secret
       *[other] { NUM($count) } Secrets
    }
config-searchConfigMaps = Pesquisar ConfigMaps
config-searchSecrets = Pesquisar Secrets
config-column-name = Nome
config-column-namespace = Namespace
config-column-type = Tipo
config-column-keys = Chaves
config-column-age = Idade
config-tab-data = Dados
config-tab-overview = Visão geral
config-tab-events = Eventos
config-tab-source = YAML
config-detail-namespace = Namespace
config-detail-type = Tipo
config-detail-labels = Rótulos
config-detail-annotations = Anotações
config-noKeys = Nenhuma chave.
config-binary = (dados binários)
config-base64 = base64
config-reveal = Revelar
config-revealOne = Revelar { $key }
config-hide = Ocultar
config-copy = Copiar { $key }
config-reveal-failed = O Homeport não conseguiu revelar { $key }
