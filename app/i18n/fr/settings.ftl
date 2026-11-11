### The `settings` namespace, in Français (fr).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/fr/settings.ts:
### change the text there, then run it again.

settings-nav = Paramètres
settings-title = Paramètres
settings-appearance-title = Apparence
settings-appearance-theme = Thème
settings-theme-system = Suivre le système
settings-theme-light = Clair
settings-theme-dark = Sombre
settings-appearance-language = Langue
settings-language-system = Suivre le système ({ $language })
settings-desktop-title = Bureau
settings-desktop-description = Comment Homeport fonctionne sur cet ordinateur.
settings-desktop-closeToTray = Fermer dans la zone de notification
settings-desktop-closeToTrayHint = Fermer la fenêtre laisse Homeport et ses connexions aux hôtes actifs dans la zone de notification. Quittez depuis l’icône de la zone de notification. Sur les bureaux qui masquent ces icônes, comme GNOME sans extension, l’icône peut ne pas apparaître.
settings-desktop-noTray = Ce bureau n’a pas de zone de notification utilisable par Homeport : fermer la fenêtre le quitte donc.
settings-desktop-startAtLogin = Démarrer à l’ouverture de session
settings-desktop-startAtLoginHint = Homeport démarre quand vous ouvrez une session sur cet ordinateur, et surveille ainsi vos hôtes dès le début.
settings-desktop-startHidden = Démarrer masqué dans la zone de notification
settings-desktop-startHiddenHint = Quand il démarre à l’ouverture de session, Homeport reste dans la zone de notification sans ouvrir sa fenêtre.
settings-desktop-startHiddenNoTray = Nécessite une zone de notification depuis laquelle ouvrir la fenêtre, ce que ce bureau n’a pas.
settings-notifications-title = Notifications
settings-notifications-description = Homeport vous prévient, même avec sa fenêtre fermée dans la zone de notification, quand :
settings-notifications-containerDied = Un conteneur s’arrête sur une erreur ou manque de mémoire
settings-notifications-containerUnhealthy = Un conteneur passe en mauvaise santé
settings-notifications-hostUnreachable = Un hôte connecté devient injoignable
settings-notifications-hostBack = Un hôte injoignable est de retour
settings-machine-title = Cette machine
settings-machine-description = Ce que Homeport peut utiliser ici.
settings-machine-keychain = Trousseau du système
settings-machine-keychainAvailable = Disponible — les clés SSH, les certificats, les environnements des stacks et les identifiants de registre sont enregistrés chiffrés.
settings-machine-keychainUnavailable = Indisponible — les clés SSH, les certificats TLS, les environnements des stacks et les identifiants de registre ne peuvent pas être enregistrés.
settings-machine-keychainEntryMissing = Entrée manquante — le trousseau n’a pas d’entrée pour la clé qui a scellé les secrets de Homeport : ils ne peuvent donc être ni ouverts ni enregistrés.
settings-machine-startOverDescription = Si Homeport utilise un autre trousseau qu’avant, revenez à celui-ci et redémarrez Homeport. Si l’entrée a disparu pour de bon, repartez avec une nouvelle clé.
settings-machine-startOver = Repartir avec une nouvelle clé
settings-machine-startOverTitle = Repartir avec une nouvelle clé ?
settings-machine-startOverConfirmDescription = Une nouvelle clé ne peut pas ouvrir ce qui a été scellé avec celle qui manque : vous saisirez à nouveau les identifiants de chaque hôte, cluster, registre et dépôt, et l’environnement de chaque stack, ou vous restaurerez une sauvegarde.
settings-machine-compose = CLI Compose
settings-machine-composeMissing = Les stacks peuvent être démarrées et arrêtées, mais pas déployées.
settings-machine-stackDeploy = CLI Docker, pour les Swarms
settings-machine-stackDeployMissing = Les stacks ne peuvent pas être déployées sur un Swarm ; tout le reste des Swarms fonctionne.
settings-machine-git = git
settings-machine-helm = CLI Helm
settings-sync-title = Stacks synchronisées
settings-sync-interval = Vérifier les dépôts toutes les
settings-sync-minutes = minutes
settings-data-title = Données
settings-data-description = Ce que Homeport conserve sur cet ordinateur.
settings-data-backup = Sauvegarder…
settings-data-restore = Restaurer…
settings-data-backupHint = Enregistrez chaque hôte, stack, identifiant de registre, dépôt et paramètre, avec leurs secrets, dans un seul fichier protégé par une phrase secrète, et restaurez-le sur cet ordinateur ou un autre.
settings-backup-title = Sauvegarder les données de Homeport
settings-backup-description = Le fichier contient chaque hôte, cluster, stack, identifiant de registre, dépôt et paramètre, ainsi que leurs clés, certificats, jetons, mots de passe et environnements. Quiconque possède ce fichier et sa phrase secrète peut les utiliser : conservez-les donc en lieu sûr. Les événements des moteurs et l’historique des déploiements ne sont pas inclus.
settings-backup-passphrase = Phrase secrète
settings-backup-passphraseHint = Au moins { $min } caractères. Elle ne peut pas être récupérée.
settings-backup-confirmation = Phrase secrète, à nouveau
settings-backup-problem-short = Utilisez au moins { $min } caractères.
settings-backup-problem-mismatch = Les deux phrases secrètes diffèrent.
settings-backup-start = Sauvegarder…
settings-backup-saveTitle = Enregistrer la sauvegarde sous
settings-backup-fileType = Sauvegarde Homeport
settings-backup-done = { $hosts ->
        [one] { NUM($hosts) } hôte
       *[other] { NUM($hosts) } hôtes
    } et { $stacks ->
        [one] { NUM($stacks) } stack
       *[other] { NUM($stacks) } stacks
    } sauvegardés
settings-backup-withoutSecrets = Ces éléments ont été sauvegardés sans leurs secrets, qui n’ont pas pu être ouverts :
settings-restore-pick = Restaurer depuis
settings-restore-title = Restaurer les données de Homeport
settings-restore-from = Depuis { $path }
settings-restore-open = Ouvrir
settings-restore-madeBy = Sauvegardé par Homeport { $version } le { $date }. Contenu :
settings-restore-holds = Ce que contient la sauvegarde
settings-restore-hosts = { $count ->
        [one] { NUM($count) } hôte
       *[other] { NUM($count) } hôtes
    }
settings-restore-clusters = { $count ->
        [one] { NUM($count) } cluster
       *[other] { NUM($count) } clusters
    }
settings-restore-stacks = { $count ->
        [one] { NUM($count) } stack
       *[other] { NUM($count) } stacks
    }
settings-restore-registries = { $count ->
        [one] { NUM($count) } identifiant de registre
       *[other] { NUM($count) } identifiants de registre
    }
settings-restore-repositories = { $count ->
        [one] { NUM($count) } dépôt
       *[other] { NUM($count) } dépôts
    }
settings-restore-settings = Vos paramètres
settings-restore-withoutSecrets = Ces éléments ont été sauvegardés sans leurs secrets, qui n’ont pas pu être ouverts à ce moment-là. Ils reviennent sans eux, à la place des secrets qu’ils ont ici, et les identifiants de registre parmi eux ne reviennent pas du tout :
settings-restore-withoutSecretsList = Sauvegardés sans leurs secrets
settings-restore-replaces = La restauration remplace chaque hôte, stack, identifiant de registre, dépôt et paramètre que vous avez actuellement, et oublie les événements des moteurs et l’historique des déploiements. Une copie des données actuelles est d’abord conservée dans le dossier de données de Homeport.
settings-restore-confirm = Tout remplacer
settings-restore-done = { $hosts ->
        [one] { NUM($hosts) } hôte
       *[other] { NUM($hosts) } hôtes
    } et { $stacks ->
        [one] { NUM($stacks) } stack
       *[other] { NUM($stacks) } stacks
    } restaurés
settings-data-keepEvents = Conserver les événements des moteurs pendant
settings-data-days = jours
settings-data-keepEventsHint = Chaque hôte conserve au plus 10 000 événements pour la page Événements. Les plus anciens sont supprimés toutes les heures.
settings-machine-gitMissing = Les stacks synchronisées en ont besoin ; tous les autres types de stack fonctionnent sans.
settings-machine-helmMissing = Les releases Helm ne peuvent être ni listées ni installées ; tout le reste des clusters fonctionne.
settings-about-tagline = Chaque Docker Engine, sur un seul tableau.
settings-about-openLogFolder = Ouvrir le dossier des journaux
settings-about-licenses = Licences
settings-about-exportDiagnostics = Exporter le diagnostic…
settings-diagnostics-title = Exporter le diagnostic
settings-diagnostics-description = Un fichier zip à joindre à un rapport de bug : un résumé des versions et des types d’hôtes et de clusters que vous utilisez, et les journaux de Homeport sans leurs secrets. Rien n’est envoyé nulle part : c’est vous qui décidez de partager le fichier.
settings-diagnostics-includeNames = Inclure les noms et les adresses
settings-diagnostics-includeNamesHelp = Si la case est décochée, les hôtes et les clusters apparaissent sous la forme host-1, cluster-1… dans le résumé et dans les journaux.
settings-diagnostics-files = Fichiers du zip
settings-diagnostics-save = Enregistrer…
settings-diagnostics-saveTitle = Enregistrer le diagnostic
settings-diagnostics-fileType = Archive zip
settings-diagnostics-done = Diagnostic enregistré dans { $path }.
settings-licenses-title = Licences
settings-licenses-description = Homeport est un logiciel libre sous licence GNU Affero General Public License, version 3, construit sur les paquets open source ci-dessous.
settings-licenses-source = Le code source de cette version est publié sous la même licence.
settings-licenses-sourceLink = Source de la v{ $version }
settings-licenses-thirdParty = Logiciels tiers
settings-licenses-missing-title = Non inclus dans ce build
settings-licenses-missing-description = Les builds de version les intègrent ; exécutez <code>{ $command }</code> pour les générer dans un build de développement.
settings-palette-group = Paramètres
settings-palette-themeLight = Thème : clair
settings-palette-themeDark = Thème : sombre
settings-palette-themeSystem = Thème : suivre le système
