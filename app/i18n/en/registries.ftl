### The `registries` namespace, in English (en).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/en/registries.ts:
### change the text there, then run it again.

registries-title = Registries
registries-description = Logins for container registries, used to pull private images, push and build on every Host.
registries-add = Add login
registries-noKeychain-title = No OS keychain
registries-noKeychain-description = Registry passwords and tokens are encrypted with a key kept in the OS keychain, and none is available. Images can still be pulled anonymously.
registries-empty = No logins yet: images are pulled anonymously.
registries-editOne = Edit the login for { $registry }
registries-removeOne = Remove the login for { $registry }
registries-remove-title = Remove the login for { $registry }?
registries-remove-description = Images from { $registry } will be pulled anonymously until a login is added again.
registries-saved = Saved the login for { $registry }
registries-removed = Removed the login for { $registry }
registries-dialog-editTitle = Edit the login for { $registry }
registries-dialog-addTitle = Add a registry login
registries-dialog-description = Used to pull, push and build on every Host. The password or token is encrypted on this computer and never shown again.
registries-dialog-address = Registry
registries-dialog-username = User name
registries-dialog-secret = Password or access token
registries-dialog-keepSecret = Leave empty to keep the stored one
registries-dialog-accepted = { $registry } accepted this login.
registries-dialog-checking = Checking…
registries-dialog-check = Check login
registries-missing-address = Enter the registry’s address.
registries-missing-username = Enter the user name.
registries-missing-secret = Enter the password or access token.
registries-login-as = as { $username } on { $registry }
registries-login-anonymously = anonymously
registries-login-pulledAs = Pulled as { $username } on { $registry }
registries-login-pulledAnonymously = Pulled anonymously
registries-login-pushedAs = Pushed as { $username } on { $registry }
registries-login-pushedAnonymously = Pushed anonymously
registries-noLogin-title = No login for { $registry }
registries-noLogin-description = Most registries refuse anonymous pushes. <link>Add a login in Settings → Registries</link> first.
