### The `apiresources` namespace, in Español (es).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/es/apiresources.ts:
### change the text there, then run it again.

apiresources-nav = Recursos de la API
apiresources-title = Recursos de la API
apiresources-summary = { $count ->
        [one] { NUM($count) } tipo servido
       *[other] { NUM($count) } tipos servidos
    } por este clúster
apiresources-refresh = Actualizar
apiresources-types = Tipos de recurso
apiresources-search = Kind o nombre corto (pvc)
apiresources-searchObjects = Buscar
apiresources-noMatch = Ningún tipo coincide.
apiresources-coreGroup = core
apiresources-pick = Elige un tipo para listar sus objetos: los integrados y los que añaden tus CRD.
apiresources-typeGone = Este clúster no sirve ese tipo (o ya no lo sirve).
apiresources-cantList = No se puede listar
apiresources-notAllowed = Tus credenciales no pueden listarlo
apiresources-cantListWhy = Los objetos { $kind } no se pueden listar: su API solo permite crearlos o leerlos de uno en uno.
apiresources-empty = No hay ninguno aquí.
apiresources-column-name = Nombre
apiresources-column-namespace = Namespace
apiresources-column-age = Antigüedad
apiresources-tab-overview = Resumen
apiresources-tab-conditions = Condiciones
apiresources-tab-events = Eventos
apiresources-tab-source = YAML
apiresources-detail-namespace = Namespace
apiresources-detail-owners = Propiedad de
apiresources-detail-age = Antigüedad
apiresources-detail-labels = Etiquetas
apiresources-detail-annotations = Anotaciones
apiresources-edit = Editar
apiresources-copyYaml = Copiar YAML
