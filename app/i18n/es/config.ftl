### The `config` namespace, in Español (es).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/es/config.ts:
### change the text there, then run it again.

config-nav = Configuración
config-title = Configuración
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
config-searchConfigMaps = Buscar ConfigMaps
config-searchSecrets = Buscar Secrets
config-column-name = Nombre
config-column-namespace = Namespace
config-column-type = Tipo
config-column-keys = Claves
config-column-age = Antigüedad
config-tab-data = Datos
config-tab-overview = Resumen
config-tab-events = Eventos
config-tab-source = YAML
config-detail-namespace = Namespace
config-detail-type = Tipo
config-detail-labels = Etiquetas
config-detail-annotations = Anotaciones
config-noKeys = No hay claves.
config-binary = (datos binarios)
config-base64 = base64
config-reveal = Mostrar
config-revealOne = Mostrar { $key }
config-hide = Ocultar
config-copy = Copiar { $key }
config-reveal-failed = Homeport no pudo mostrar { $key }
