### The `apiresources` namespace, in English (en).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/en/apiresources.ts:
### change the text there, then run it again.

apiresources-nav = API resources
apiresources-title = API resources
apiresources-summary = { $count ->
        [one] { NUM($count) } type
       *[other] { NUM($count) } types
    } served by this cluster
apiresources-refresh = Refresh
apiresources-types = Resource types
apiresources-search = Kind or short name (pvc)
apiresources-searchObjects = Search
apiresources-noMatch = No type matches.
apiresources-coreGroup = core
apiresources-pick = Pick a type to list its objects: built-in ones, and those your CRDs add.
apiresources-typeGone = This cluster doesn’t serve that type (any more).
apiresources-cantList = Can’t be listed
apiresources-notAllowed = Your credentials can’t list it
apiresources-cantListWhy = { $kind } objects can’t be listed: its API only lets them be created or read one by one.
apiresources-empty = None here.
apiresources-column-name = Name
apiresources-column-namespace = Namespace
apiresources-column-age = Age
apiresources-tab-overview = Overview
apiresources-tab-conditions = Conditions
apiresources-tab-events = Events
apiresources-tab-source = YAML
apiresources-detail-namespace = Namespace
apiresources-detail-owners = Owned by
apiresources-detail-age = Age
apiresources-detail-labels = Labels
apiresources-detail-annotations = Annotations
apiresources-edit = Edit
apiresources-copyYaml = Copy YAML
