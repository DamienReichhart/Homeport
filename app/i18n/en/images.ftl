### The `images` namespace, in English (en).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/en/images.ts:
### change the text there, then run it again.

images-nav = Images
images-title = Images
images-summary = { $count ->
        [one] { NUM($count) } image
       *[other] { NUM($count) } images
    } · { $size }
images-cleanUp = Clean up
images-cleanUp-dangling = Dangling images
images-cleanUp-all = All unused images
images-saveSelected = { $count ->
        [one] Save image…
       *[other] Save { NUM($count) } images…
    }
images-load = Load…
images-importTar = Import from tar…
images-build = Build
images-pull = Pull image
images-search = Search images
images-empty-title = No images
images-empty-description = Pull one from a registry to get started.
images-column-name = Name
images-column-id = ID
images-column-created = Created
images-column-size = Size
images-column-status = Status
images-moreTags = { $count ->
        [one] +{ NUM($count) } more tag
       *[other] +{ NUM($count) } more tags
    }
images-inUse = In use · { $count }
images-unused = Unused
images-runOne = Run { $name }
images-action-inspect = Inspect
images-action-tag = Tag
images-action-push = Push
images-action-save = Save…
images-remove-title = Remove { $name }?
images-remove-everyName = This removes every name of the image ({ $names }) and its layers that nothing else uses.
images-remove-description = The image and the layers no other image uses are deleted. It can be pulled again later.
images-prune-allTitle = Remove all unused images?
images-prune-danglingTitle = Remove dangling images?
images-prune-allDescription = Every image no container uses is deleted, tagged ones included.
images-prune-danglingDescription = Untagged images left behind by builds and pulls are deleted.
images-removed = Removed { $name }
images-pruned = Removed { $count ->
        [one] { NUM($count) } image
       *[other] { NUM($count) } images
    }, { $space } reclaimed
images-pulled = Pulled { $reference }
images-pushed = Pushed { $reference }
images-built = Built { $name }
images-builtUnnamed = Built the image
images-saved = Saved { $path }
images-loadedOne = Loaded { $name }
images-loaded = Loaded { $count ->
        [one] { NUM($count) } image
       *[other] { NUM($count) } images
    }
images-imported = Imported { $name }
images-files-save = Save images as
images-files-load = Load images from
images-files-import = Import a filesystem from
images-files-imageArchives = Image archives
images-files-imageArchive = Image archive
images-files-filesystemArchives = Filesystem archives
images-buildDialog-title = Build an image
images-buildDialog-description = From a folder on this computer. Homeport sends it to the Host, leaving out what its .dockerignore excludes, and the Host’s Engine builds it.
images-buildDialog-built = ✓ Built
images-buildDialog-folder = Folder
images-buildDialog-chooseFolder = Choose the folder to build
images-buildDialog-choose = Choose…
images-buildDialog-dockerfile = Dockerfile
images-buildDialog-target = Target stage
images-buildDialog-targetPlaceholder = The last stage
images-buildDialog-names = Names
images-buildDialog-buildArgs = Build arguments
images-buildDialog-buildArgsPlaceholder = VERSION=1.0, one per line
images-buildDialog-noCache = Don’t use the build cache
images-buildDialog-pull = Pull newer base images
images-buildForm-noFolder = Choose the folder to build.
images-buildForm-argNeedsName = "{ $line }" needs a name before "=".
images-hub-unavailable = Docker Hub can’t be searched right now: { $error } A full name like nginx:1.27 still pulls.
images-hub-noMatch = Nothing on Docker Hub matches “{ $term }”.
images-hub-results = Docker Hub results
images-hub-result = { $name }, { $stars } stars
images-hub-resultOfficial = { $name }, official image, { $stars } stars
images-hub-official = Official
images-hub-backToSearch = Search
images-hub-tagCount = { $count ->
        [one] { NUM($count) } tag
       *[other] { NUM($count) } tags
    }
images-hub-filterTags = Filter tags
images-hub-tags = Tags
images-hub-noTagMatch = No tag matches “{ $filter }”.
images-hub-moreTags = More tags
images-importDialog-title = Import a filesystem as an image
images-importDialog-description = { $path } becomes a one-layer image with no history or settings of its own; give it a command below to make it runnable.
images-importDialog-name = Image name
images-importDialog-changes = Changes
images-importDialog-changesHint = Dockerfile instructions, one per line: CMD, ENTRYPOINT, ENV, EXPOSE, LABEL, ONBUILD, USER, VOLUME or WORKDIR.
images-importDialog-import = Import
images-inspect-layers = Layers
images-inspect-raw = Raw JSON
images-inspect-created = Created
images-inspect-instruction = Instruction
images-inspect-size = Size
images-pullDialog-title = Pull an image
images-pullDialog-description = Search Docker Hub by name, or type any reference the host can reach, e.g. nginx:1.27 or ghcr.io/org/app:v2.
images-pullDialog-placeholder = postgres, or nginx:1.27
images-pullDialog-reference = Image reference
images-pullDialog-pull = Pull
images-done = Done.
images-pushDialog-title = Push { $name }
images-pushDialog-description = To the registry its name starts with, e.g. ghcr.io/org/app:v2, or Docker Hub for names like org/app:v2.
images-pushDialog-target = Name to push
images-pushDialog-push = Push
images-pushDialog-tagsFirst = The image is given this name first, as Tag does.
images-pushDialog-uploading = Uploading { $current } / { $total }
images-push-pushed = { $pushed ->
        [one] { NUM($pushed) } layer
       *[other] { NUM($pushed) } layers
    } pushed
images-push-pushedAndExisting = { $pushed ->
        [one] { NUM($pushed) } layer
       *[other] { NUM($pushed) } layers
    } pushed, { $existing } already in the registry
images-tagDialog-title = Tag { $name }
images-tagDialog-newName = New name
images-tagDialog-add = Add tag
images-transfer-savingOne = Saving { $name }
images-transfer-saving = Saving { $count ->
        [one] { NUM($count) } image
       *[other] { NUM($count) } images
    }
images-transfer-to = To { $path }
images-transfer-from = From { $path }
images-transfer-written = Written
images-transfer-sent = Sent
images-transfer-loading = Loading images
images-transfer-importing = Importing { $reference }
images-transfer-doneWith = Done: { $images }
images-palette-pull = Pull an image
