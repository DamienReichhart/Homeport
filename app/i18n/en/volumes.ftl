### The `volumes` namespace, in English (en).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/en/volumes.ts:
### change the text there, then run it again.

volumes-nav = Volumes
volumes-title = Volumes
volumes-summary = { $count ->
        [one] { NUM($count) } volume
       *[other] { NUM($count) } volumes
    } · { $unused } unused
volumes-cleanUp = Clean up
volumes-cleanUp-anonymous = Unused anonymous volumes
volumes-cleanUp-all = All unused volumes
volumes-create = Create volume
volumes-search = Search volumes
volumes-empty-title = No volumes
volumes-empty-description = Volumes keep container data across restarts and re-creations.
volumes-column-name = Name
volumes-column-stack = Stack
volumes-column-driver = Driver
volumes-column-size = Size
volumes-size-label = Size:
volumes-size-measuring = Measuring
volumes-size-unknown = This volume's driver doesn't report its size.
volumes-size-refresh = Refresh sizes
volumes-column-usedBy = Used by
volumes-column-created = Created
volumes-unused = Unused
volumes-inspectOne = Inspect { $name }
volumes-deleteOne = Delete { $name }
volumes-browseOne = Browse the files of { $name }
volumes-browse-title = Files of { $name }
volumes-browse-description = Read-only: nothing here can change the volume, even while containers use it.
volumes-browse-saveOne = Save { $name }
volumes-browse-saved = Saved { $name }
volumes-browse-savedArchive = Saved { $name } as a .tar archive
volumes-helper-needed = Homeport opens volumes with a short-lived helper container, which needs the { $image } image (about { $size }). This Host doesn't have it.
volumes-helper-removed = Homeport pulls it only if you agree, and removes it again when it's done, unless something else uses it.
volumes-helper-pull = Pull { $image }
volumes-helper-pulling = Pulling { $image }…
volumes-helper-progress = Pull progress
volumes-moreActions = More actions for { $name }
volumes-backupMenu = Back up to a file…
volumes-restoreMenu = Restore from a file…
volumes-cloneMenu = Clone…
volumes-clone-title = Clone { $volume }
volumes-clone-description = Copies its files, with their owners and permissions, into a new volume on this Host. The copy belongs to no Stack.
volumes-clone-name = Name of the copy
volumes-clone-start = Clone
volumes-clone-into = Into { $target }
volumes-clone-copying = Copying…
volumes-clone-done = Cloned { $source } into { $target }
volumes-restoreFromFile = Restore from file
volumes-backup-title = Back up { $volume }
volumes-backup-saveAs = Back up { $volume } to
volumes-backup-to = To { $path }
volumes-backup-done = Backed up { $volume } to { $path }
volumes-restore-pick = Restore a volume from
volumes-restore-title = Restore a volume
volumes-restore-titleInto = Restore { $volume }
volumes-restore-from = From { $path }
volumes-restore-into = Restore into
volumes-restore-newVolume = A new volume
volumes-restore-existingVolume = An existing volume
volumes-restore-volume = Volume
volumes-restore-chooseVolume = Choose a volume
volumes-restore-nameTaken = A volume named { $volume } already exists.
volumes-restore-start = Restore
volumes-restore-done = Restored { $volume }
volumes-restore-cancelled = The restore of { $volume } was cancelled; the volume may be partly written.
volumes-transfer-written = Written
volumes-transfer-sent = Sent
volumes-transfer-archives = Tar archives
volumes-flow-running-backup = { $containers } { $count ->
        [one] is
       *[other] are
    } running and { $count ->
        [one] uses
       *[other] use
    } { $volume }. A backup taken while they write to it may be inconsistent, such as a database caught mid-write.
volumes-flow-running-restore = { $containers } { $count ->
        [one] is
       *[other] are
    } running and { $count ->
        [one] uses
       *[other] use
    } { $volume }. Homeport stops { $count ->
        [one] it
       *[other] them
    } during the restore and starts { $count ->
        [one] it
       *[other] them
    } again afterwards.
volumes-flow-running-clone = { $containers } { $count ->
        [one] is
       *[other] are
    } running and { $count ->
        [one] uses
       *[other] use
    } { $volume }. A copy taken while they write to it may be inconsistent, such as a database caught mid-write.
volumes-flow-anyway-backup = Back up anyway
volumes-flow-anyway-clone = Copy anyway
volumes-flow-stopDuring-backup = Stop { $count ->
        [one] it
       *[other] them
    } during the backup
volumes-flow-stopDuring-restore = Stop and restore
volumes-flow-stopDuring-clone = Stop { $count ->
        [one] it
       *[other] them
    } during the copy
volumes-flow-replace = { $volume } already holds files. Restoring deletes them and puts the backup's in their place.
volumes-flow-replaceConfirm = Replace its contents
volumes-usedBy = Used by { $containers }.
volumes-notUsed = No container uses this volume.
volumes-createDialog-title = Create a volume
volumes-createDialog-name = Name
volumes-createDialog-driver = Driver
volumes-deleted = Deleted { $name }
volumes-pruned = Deleted { $count ->
        [one] { NUM($count) } volume
       *[other] { NUM($count) } volumes
    }, { $space } reclaimed
volumes-remove-title = Delete volume { $name }?
volumes-remove-warning = <strong>All data in this volume is permanently deleted.</strong>
volumes-remove-mounted = {" "}It is mounted by { $containers }; remove those containers first.
volumes-remove-confirm = Delete volume and data
volumes-prune-allTitle = Delete every unused volume?
volumes-prune-anonymousTitle = Delete unused anonymous volumes?
volumes-prune-allWarning = The data in every volume no container uses is permanently deleted, named volumes included.
volumes-prune-anonymousWarning = The data in unnamed volumes no container uses is permanently deleted.
volumes-prune-confirm = Delete volumes and data
