### The `config` namespace, in Italiano (it).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/it/config.ts:
### change the text there, then run it again.

config-nav = Configurazione
config-title = Configurazione
config-configMaps = ConfigMap
config-secrets = Secret
config-configMap = ConfigMap
config-secret = Secret
config-configMapsSummary = { $count ->
        [one] { NUM($count) } ConfigMap
       *[other] { NUM($count) } ConfigMap
    }
config-secretsSummary = { $count ->
        [one] { NUM($count) } Secret
       *[other] { NUM($count) } Secret
    }
config-searchConfigMaps = Cerca ConfigMap
config-searchSecrets = Cerca Secret
config-column-name = Nome
config-column-namespace = Namespace
config-column-type = Tipo
config-column-keys = Chiavi
config-column-age = Età
config-tab-data = Dati
config-tab-overview = Panoramica
config-tab-events = Eventi
config-tab-source = YAML
config-detail-namespace = Namespace
config-detail-type = Tipo
config-detail-labels = Etichette
config-detail-annotations = Annotazioni
config-noKeys = Nessuna chiave.
config-binary = (dati binari)
config-base64 = base64
config-reveal = Mostra
config-revealOne = Mostra { $key }
config-hide = Nascondi
config-copy = Copia { $key }
config-reveal-failed = Homeport non è riuscito a mostrare { $key }
