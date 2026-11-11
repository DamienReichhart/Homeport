### The `config` namespace, in English (en).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/en/config.ts:
### change the text there, then run it again.

config-nav = Configuration
config-title = Configuration
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
config-searchConfigMaps = Search ConfigMaps
config-searchSecrets = Search Secrets
config-column-name = Name
config-column-namespace = Namespace
config-column-type = Type
config-column-keys = Keys
config-column-age = Age
config-tab-data = Data
config-tab-overview = Overview
config-tab-events = Events
config-tab-source = YAML
config-detail-namespace = Namespace
config-detail-type = Type
config-detail-labels = Labels
config-detail-annotations = Annotations
config-noKeys = No keys.
config-binary = (binary data)
config-base64 = base64
config-reveal = Reveal
config-revealOne = Reveal { $key }
config-hide = Hide
config-copy = Copy { $key }
config-reveal-failed = Homeport couldn’t reveal { $key }
