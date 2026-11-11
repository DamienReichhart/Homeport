### The `config` namespace, in Français (fr).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/fr/config.ts:
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
config-searchConfigMaps = Rechercher des ConfigMaps
config-searchSecrets = Rechercher des Secrets
config-column-name = Nom
config-column-namespace = Namespace
config-column-type = Type
config-column-keys = Clés
config-column-age = Âge
config-tab-data = Données
config-tab-overview = Vue d’ensemble
config-tab-events = Événements
config-tab-source = YAML
config-detail-namespace = Namespace
config-detail-type = Type
config-detail-labels = Labels
config-detail-annotations = Annotations
config-noKeys = Aucune clé.
config-binary = (données binaires)
config-base64 = base64
config-reveal = Afficher
config-revealOne = Afficher { $key }
config-hide = Masquer
config-copy = Copier { $key }
config-reveal-failed = Homeport n’a pas pu afficher { $key }
