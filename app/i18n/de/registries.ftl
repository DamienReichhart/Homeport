### The `registries` namespace, in Deutsch (de).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/de/registries.ts:
### change the text there, then run it again.

registries-title = Registrys
registries-description = Anmeldungen für Container-Registrys, mit denen auf jedem Host private Images gepullt, gepusht und gebaut werden.
registries-add = Anmeldung hinzufügen
registries-noKeychain-title = Kein Schlüsselbund des Betriebssystems
registries-noKeychain-description = Registry-Passwörter und -Tokens werden mit einem Schlüssel verschlüsselt, der im Schlüsselbund des Betriebssystems liegt, und es ist keiner verfügbar. Images können weiterhin anonym gepullt werden.
registries-empty = Noch keine Anmeldungen: Images werden anonym gepullt.
registries-editOne = Anmeldung für { $registry } bearbeiten
registries-removeOne = Anmeldung für { $registry } entfernen
registries-remove-title = Anmeldung für { $registry } entfernen?
registries-remove-description = Images von { $registry } werden anonym gepullt, bis wieder eine Anmeldung hinzugefügt wird.
registries-saved = Anmeldung für { $registry } gespeichert
registries-removed = Anmeldung für { $registry } entfernt
registries-dialog-editTitle = Anmeldung für { $registry } bearbeiten
registries-dialog-addTitle = Registry-Anmeldung hinzufügen
registries-dialog-description = Wird auf jedem Host zum Pullen, Pushen und Bauen verwendet. Das Passwort oder Token wird auf diesem Computer verschlüsselt und nie wieder angezeigt.
registries-dialog-address = Registry
registries-dialog-username = Benutzername
registries-dialog-secret = Passwort oder Zugriffstoken
registries-dialog-keepSecret = Leer lassen, um das gespeicherte beizubehalten
registries-dialog-accepted = { $registry } hat diese Anmeldung akzeptiert.
registries-dialog-checking = Wird geprüft…
registries-dialog-check = Anmeldung prüfen
registries-missing-address = Geben Sie die Adresse der Registry ein.
registries-missing-username = Geben Sie den Benutzernamen ein.
registries-missing-secret = Geben Sie das Passwort oder Zugriffstoken ein.
registries-login-as = als { $username } auf { $registry }
registries-login-anonymously = anonym
registries-login-pulledAs = Als { $username } auf { $registry } gepullt
registries-login-pulledAnonymously = Anonym gepullt
registries-login-pushedAs = Als { $username } auf { $registry } gepusht
registries-login-pushedAnonymously = Anonym gepusht
registries-noLogin-title = Keine Anmeldung für { $registry }
registries-noLogin-description = Die meisten Registrys lehnen anonyme Pushes ab. <link>Fügen Sie zuerst unter Einstellungen → Registrys eine Anmeldung hinzu</link>.
