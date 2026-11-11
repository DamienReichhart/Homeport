### The `config` namespace, in Deutsch (de).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/de/config.ts:
### change the text there, then run it again.

config-nav = Konfiguration
config-title = Konfiguration
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
config-searchConfigMaps = ConfigMaps durchsuchen
config-searchSecrets = Secrets durchsuchen
config-column-name = Name
config-column-namespace = Namespace
config-column-type = Typ
config-column-keys = Schlüssel
config-column-age = Alter
config-tab-data = Daten
config-tab-overview = Übersicht
config-tab-events = Events
config-tab-source = YAML
config-detail-namespace = Namespace
config-detail-type = Typ
config-detail-labels = Labels
config-detail-annotations = Annotationen
config-noKeys = Keine Schlüssel.
config-binary = (Binärdaten)
config-base64 = base64
config-reveal = Anzeigen
config-revealOne = { $key } anzeigen
config-hide = Verbergen
config-copy = { $key } kopieren
config-reveal-failed = Homeport konnte { $key } nicht anzeigen
