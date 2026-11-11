### The `repositories` namespace, in Deutsch (de).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/de/repositories.ts:
### change the text there, then run it again.

repositories-title = Repositorys
repositories-description = Git-Repositorys, aus denen synchronisierte Stacks auf jedem Host bereitgestellt werden. Homeport liest sie nur.
repositories-add = Repository hinzufügen
repositories-empty = Noch keine Repositorys. Fügen Sie eines hinzu, um einen synchronisierten Stack zu erstellen.
repositories-manageOne = Zugriff auf { $repository } verwalten
repositories-removeOne = { $repository } entfernen
repositories-remove-title = { $repository } entfernen?
repositories-remove-description = Homeport vergisst seinen Deploy-Key oder sein Token und löscht seinen Klon auf diesem Computer. Das Repository selbst bleibt unberührt. Entfernen Sie den Deploy-Key auch dort auf Ihrer Forge.
repositories-noKeychain = Deploy-Keys und Tokens werden mit einem Schlüssel verschlüsselt, der im Schlüsselbund des Betriebssystems liegt, und es ist keiner verfügbar, daher können nur öffentliche Repositorys hinzugefügt werden.
repositories-public = Ein öffentliches Repository: Homeport liest es ohne Zugangsdaten.
repositories-added = { $repository } hinzugefügt
repositories-removed = { $repository } entfernt
repositories-keyRotated = Neuer Deploy-Key erstellt. Fügen Sie ihn anstelle des alten zum Repository hinzu.
repositories-tokenReplaced = Token ersetzt
repositories-hostKeyReset = Homeport vertraut beim nächsten Mal dem Schlüssel, den der Server vorlegt.
repositories-auth-withKey = Mit einem Deploy-Key, auf { $host }
repositories-auth-withToken = Als { $username } mit einem Token, auf { $host }
repositories-auth-public = Öffentlich, auf { $host }
repositories-dialog-addTitle = Repository hinzufügen
repositories-dialog-description = Homeport klont es auf diesem Computer, um synchronisierte Stacks daraus bereitzustellen. Es committet und pusht nie.
repositories-dialog-url = Repository-URL
repositories-dialog-auth = Zugriff
repositories-dialog-authKey = Deploy-Key
repositories-dialog-authToken = Token
repositories-dialog-authNone = Öffentlich
repositories-dialog-authKeyHelp = Homeport erstellt einen Schlüssel nur für dieses Repository. Sie fügen ihn mit Lesezugriff zum Repository hinzu; Ihre eigenen SSH-Schlüssel werden nie verwendet.
repositories-dialog-authTokenHelp = Ein Zugriffstoken, das den Inhalt des Repositorys lesen darf, über HTTPS gesendet. Auf diesem Computer verschlüsselt und nie wieder angezeigt.
repositories-dialog-authNoneHelp = Für ein öffentliches Repository über HTTPS. Es wird nichts gespeichert.
repositories-dialog-username = Benutzername
repositories-dialog-token = Zugriffstoken
repositories-dialog-add = Hinzufügen
repositories-dialog-addWithKey = Hinzufügen und Deploy-Key erstellen
repositories-missing-url = Geben Sie die URL des Repositorys ein.
repositories-missing-validUrl = Geben Sie eine SSH- oder HTTPS-URL ein, etwa git@github.com:owner/repo.git oder https://gitlab.com/owner/repo.git.
repositories-missing-auth = Wählen Sie, wie Homeport dieses Repository erreicht.
repositories-missing-username = Geben Sie den Benutzernamen ein, zu dem das Token gehört.
repositories-missing-token = Geben Sie das Zugriffstoken ein.
repositories-deployKey-title = Deploy-Key
repositories-deployKey-github = GitHub: im Repository unter <strong>Settings → Deploy keys → Add deploy key</strong>.
repositories-deployKey-gitlab = GitLab: im Projekt unter <strong>Settings → Repository → Deploy keys</strong>.
repositories-deployKey-gitea = Gitea und Forgejo: im Repository unter <strong>Settings → Deploy keys → Add deploy key</strong>.
repositories-deployKey-readOnly = Lassen Sie den Schreibzugriff ausgeschaltet: Homeport liest nur. Testen Sie dann unten den Zugriff.
repositories-deployKey-fingerprint = Fingerabdruck: <mono>{ $fingerprint }</mono>
repositories-rotate-button = Neuer Schlüssel
repositories-rotate-title = Neuen Deploy-Key erstellen?
repositories-rotate-description = Der aktuelle Schlüssel funktioniert für Homeport nicht mehr. Fügen Sie den neuen zum Repository hinzu und entfernen Sie dort den alten.
repositories-rotate-confirm = Neuen Schlüssel erstellen
repositories-token-replace = Token ersetzen
repositories-token-save = Token speichern
repositories-hostKey-trusted = Vertrauenswürdiger Schlüssel von { $host }: <mono>{ $fingerprint }</mono>
repositories-hostKey-reset = Zurücksetzen
repositories-hostKey-resetTitle = Vertrauenswürdigen Schlüssel von { $host } zurücksetzen?
repositories-hostKey-resetDescription = Tun Sie dies nur, wenn { $host } seinen SSH-Schlüssel geändert hat und Sie den neuen Fingerabdruck auf dessen Website geprüft haben. Homeport vertraut dann jedem Schlüssel, den der Server als Nächstes vorlegt.
repositories-test-button = Zugriff testen
repositories-test-testing = Wird getestet…
repositories-test-success = Homeport kann es lesen: { $branches ->
        [one] { NUM($branches) } Branch
       *[other] { NUM($branches) } Branches
    } und { $tags ->
        [one] { NUM($tags) } Tag
       *[other] { NUM($tags) } Tags
    }.
