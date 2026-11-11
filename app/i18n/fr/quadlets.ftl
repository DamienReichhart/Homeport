### The `quadlets` namespace, in Français (fr).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/fr/quadlets.ts:
### change the text there, then run it again.

quadlets-nav = Quadlet
quadlets-title = Unités Quadlet
quadlets-summary = { $count ->
        [one] { NUM($count) } unité
       *[other] { NUM($count) } unités
    }
quadlets-search = Rechercher des unités
quadlets-empty = Aucune unité Quadlet
quadlets-emptyDescription = Les unités Quadlet sont des fichiers que Podman transforme en services systemd : les conteneurs démarrent ainsi avec l’hôte et systemd les maintient en marche. Installez-en une pour la voir ici.
quadlets-new = Nouvelle unité
quadlets-edit = Modifier
quadlets-notGenerated = pas encore générée
quadlets-gone = { $name } n’est plus installée
quadlets-column-name = Unité
quadlets-column-kind = Type
quadlets-column-service = Service
quadlets-column-status = État
quadlets-column-containers = Conteneurs
quadlets-status-running = En cours d’exécution
quadlets-status-starting = Démarrage
quadlets-status-stopping = Arrêt en cours
quadlets-status-stopped = Arrêtée
quadlets-status-failed = En échec
quadlets-status-notLoaded = Non chargée par systemd
quadlets-status-template = Modèle
quadlets-action-start = Démarrer
quadlets-action-stop = Arrêter
quadlets-action-restart = Redémarrer
quadlets-actionOne-start = Démarrer { $name }
quadlets-actionOne-stop = Arrêter { $name }
quadlets-actionOne-restart = Redémarrer { $name }
quadlets-done-start = { $name } démarrée
quadlets-done-stop = { $name } arrêtée
quadlets-done-restart = { $name } redémarrée
quadlets-installed = { $name } installée
quadlets-saved = { $name } enregistrée
quadlets-removed = { $name } supprimée
quadlets-blocked-noCommands = Homeport démarre et arrête les unités avec systemctl, qu’il ne peut exécuter que via SSH ou pour un Podman sur cet ordinateur.
quadlets-blocked-notLoaded = systemd n’a pas généré le service de cette unité : il n’y a donc rien à démarrer.
quadlets-blocked-podmanTooOld = Lire, installer, modifier et supprimer des unités nécessite Podman 5.8 ou ultérieur ; cet hôte exécute Podman { $version }.
quadlets-blocked-noFile = Cette unité n’est connue que par ses conteneurs : son fichier ne peut donc pas être modifié.
quadlets-note-labels = Podman { $version } ne peut pas lister les unités Quadlet : celles-ci proviennent donc des conteneurs qu’elles ont créés. Leurs fichiers, et les unités qui n’ont créé aucun conteneur, apparaissent avec Podman 5.7 et ultérieur.
quadlets-note-readOnly = Podman { $version } liste les unités mais ne peut pas les lire, les installer, les modifier ni les supprimer ; cela nécessite Podman 5.8 ou ultérieur.
quadlets-note-noCommands = Démarrer et arrêter des unités nécessite que cet hôte soit joint via SSH, ou un Podman sur cet ordinateur.
quadlets-detail-service = Service
quadlets-detail-unit = Unité systemd
quadlets-detail-scope = S’exécute en tant que
quadlets-detail-path = Fichier
quadlets-detail-app = Application
quadlets-detail-pod = Pod
quadlets-detail-containers = { $count ->
        [0] Conteneurs
        [one] { NUM($count) } conteneur
       *[other] { NUM($count) } conteneurs
    }
quadlets-detail-noContainers = Elle n’a créé aucun conteneur, ou son service est arrêté.
quadlets-scope-user = Une unité utilisateur (systemctl --user)
quadlets-scope-system = Une unité système
quadlets-file-title = Fichier de l’unité
quadlets-file-unknown = Cette unité n’est connue que par le label de ses conteneurs.
quadlets-file-tooOld = Podman { $version } ne peut pas lire les fichiers d’unité ; les afficher nécessite Podman 5.8 ou ultérieur.
quadlets-form-newTitle = Nouvelle unité Quadlet
quadlets-form-newDescription = Podman écrit le fichier là où vont les unités de son utilisateur et recharge systemd. Démarrez l’unité une fois installée.
quadlets-form-editTitle = Modifier { $name }
quadlets-form-editDescription = L’enregistrement remplace le fichier et recharge systemd ; redémarrez l’unité pour qu’un service en cours d’exécution l’utilise. Les fichiers installés à côté sont conservés.
quadlets-form-name = Nom du fichier
quadlets-form-kind = Type
quadlets-form-openFile = Ouvrir un fichier…
quadlets-form-openTitle = Ouvrir un fichier d’unité Quadlet
quadlets-form-unitFiles = Fichiers d’unité Quadlet
quadlets-form-content = Fichier de l’unité
quadlets-form-extra = Fichiers à côté
quadlets-form-extraHint = Un Containerfile, un YAML kube ou un fichier env que l’unité utilise, installé dans son dossier d’application.
quadlets-form-extraTitle = Choisir des fichiers à installer à côté de l’unité
quadlets-form-addExtra = Ajouter des fichiers…
quadlets-form-removeExtra = Retirer { $name }
quadlets-form-install = Installer
quadlets-form-save = Enregistrer
quadlets-form-overwrite = Écraser
quadlets-form-nameInvalid = Nommez-le comme web.container, avec l’extension .container, .pod, .volume, .network, .kube, .image, .build ou .artifact.
quadlets-form-sectionMissing = Le fichier doit contenir une section { $section } pour que Quadlet puisse le lire.
quadlets-form-extraInvalid = Chaque fichier à côté de l’unité doit avoir son propre nom simple, sans dossiers.
quadlets-remove-title = Supprimer { $name } ?
quadlets-remove-description = Podman supprime le fichier de l’unité et recharge systemd. Les conteneurs, volumes et images qu’elle a créés sont conservés.
quadlets-remove-stop = Arrêter son service s’il est en cours d’exécution
quadlets-remove-stopRunning = Arrêter d’abord son service en cours d’exécution
