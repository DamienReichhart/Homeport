//! The error messages of this crate, by catalogue key and English template
//! (ADR 0011). `bun run bindings` writes them to the frontend's catalogue.

homeport_core::messages! {
    // Managed Stacks

    /// A Stack name Compose wouldn't take as a project name.
    STACK_NAME = "errors.stacks.name" => "Stack names use lowercase letters, digits, - and _, and start with a letter or digit.";
    /// A Managed Stack saved without a compose file.
    NO_FILES = "errors.stacks.noFiles" => "A stack needs at least one compose file.";
    /// A compose file name with a folder in it, or without a YAML extension.
    FILE_NAME = "errors.stacks.fileName" => "\"{name}\" isn't a compose file name. Use a name such as compose.prod.yaml, without folders.";
    /// Two compose files of one Stack with the same name.
    FILE_NAME_TWICE = "errors.stacks.fileNameTwice" => "Two compose files are named {name}.";
    /// A Compose profile name with characters Compose doesn't allow.
    PROFILE_NAME = "errors.stacks.profileName" => "\"{name}\" isn't a profile name. Profile names use letters, digits, _, . and -, and start with a letter or digit.";
    /// A recorded deploy dropped from the history while it was in use.
    DEPLOY_GONE = "errors.stacks.deployGone" => "This deploy is no longer in the stack's history.";
    /// Rolling back a Linked Stack, whose folder is its source.
    ROLLBACK_LINKED = "errors.stacks.rollbackLinked" => "A linked stack can't be rolled back from Homeport.";
    ROLLBACK_LINKED_HINT = "errors.stacks.rollbackLinked.hint" => "Restore the earlier files in its folder, with your version control for example, then redeploy.";
    /// A line of the Stack's environment without `=`.
    ENV_NOT_KEY_VALUE = "errors.stacks.envNotKeyValue" => "Line {line} isn't KEY=value.";
    /// A line of the Stack's environment whose name has other characters than letters, digits and `_`.
    ENV_BAD_NAME = "errors.stacks.envBadName" => "Line {line} has an invalid variable name.";
    /// A Stack removed while it was in use.
    GONE = "errors.stacks.gone" => "This stack no longer exists.";
    /// Creating a Stack under a name the Host already has.
    NAME_IN_USE = "errors.stacks.nameInUse" => "This host already has a stack named \"{name}\".";
    /// Creating a Stack for a Host that was removed.
    HOST_GONE = "errors.stacks.hostGone" => "This host no longer exists.";
    /// Running compose for a Host that isn't connected.
    HOST_NOT_CONNECTED = "errors.stacks.hostNotConnected" => "{name} isn't connected.";
    HOST_NOT_CONNECTED_HINT = "errors.stacks.hostNotConnected.hint" => "Wait for it to connect, then try again.";
    /// The local proxy compose talks to a remote Host through couldn't open.
    PROXY_FAILED = "errors.stacks.proxyFailed" => "Homeport couldn't open the private connection compose uses to reach {name}.";
    /// Pulling a Stack's image failed; the reason is in Details.
    PULL_FAILED = "errors.stacks.pullFailed" => "Homeport couldn't pull {image} for {services}.";

    // Linked Stacks

    /// A Linked Stack's folder is no longer on this machine.
    FOLDER_MISSING = "errors.stacks.folderMissing" => "The folder {folder} is no longer on this computer.";
    FOLDER_MISSING_HINT = "errors.stacks.folderMissing.hint" => "Put it back, or forget this Stack and link it again from where it is now.";
    /// One of a Linked Stack's compose files is no longer in its folder.
    FILE_MISSING = "errors.stacks.fileMissing" => "{file} is no longer on this computer.";
    FILE_MISSING_HINT = "errors.stacks.fileMissing.hint" => "Put it back, or remove it from the Stack's compose files.";
    /// A Linked Stack's compose file Homeport may not read or write.
    FILE_UNREADABLE = "errors.stacks.fileUnreadable" => "Homeport can't read or write {file}.";
    FILE_UNREADABLE_HINT = "errors.stacks.fileUnreadable.hint" => "Check that your user may read and write it, then try again.";
    /// Saving a compose file that was edited elsewhere since it was opened.
    FILES_CHANGED = "errors.stacks.filesChanged" => "{file} changed outside Homeport since you opened it.";
    FILES_CHANGED_HINT = "errors.stacks.filesChanged.hint" => "Reload it to see the change, or overwrite it with your version.";
    /// A folder picked for a Linked Stack that has no compose file Compose would use.
    NO_COMPOSE_FILE = "errors.stacks.noComposeFile" => "{folder} has no compose file. Homeport looks for compose.yaml, compose.yml, docker-compose.yml or docker-compose.yaml.";
    /// A compose file picked for a Linked Stack outside its folder.
    NOT_IN_FOLDER = "errors.stacks.notInFolder" => "{file} isn't in the stack's folder, {folder}.";
    /// A path that isn't one of the Linked Stack's compose files.
    NOT_A_STACK_FILE = "errors.stacks.notAStackFile" => "{file} isn't one of this Stack's compose files.";

    // Synced Stacks

    /// A compose file path outside the repository, or not a plain relative path.
    SYNCED_PATH = "errors.stacks.syncedPath" => "\"{path}\" isn't a path inside the repository. Use one like compose.yaml or deploy/compose.yaml.";
    /// A compose file path that doesn't end in .yaml or .yml.
    SYNCED_PATH_NOT_YAML = "errors.stacks.syncedPathNotYaml" => "{path} isn't a compose file. Its name ends in .yaml or .yml.";
    /// Creating a Synced Stack without a branch or tag.
    SYNCED_REF_EMPTY = "errors.stacks.syncedRefEmpty" => "Choose the branch or tag to deploy.";
    /// The compose file isn't at that path in the commit.
    SYNCED_FILE_MISSING = "errors.stacks.syncedFileMissing" => "The repository has no compose file at {path} in commit {commit}.";
    SYNCED_FILE_MISSING_HINT = "errors.stacks.syncedFileMissing.hint" => "Check the path, relative to the repository's root, and the branch or tag.";
    /// Deploying a Synced Stack whose ref was never fetched.
    SYNCED_NOT_FETCHED = "errors.stacks.syncedNotFetched" => "This synced stack hasn't been fetched yet. Sync it first.";
    /// Rolling back a Synced Stack, whose repository is its source.
    ROLLBACK_SYNCED = "errors.stacks.rollbackSynced" => "A synced stack can't be rolled back from Homeport.";
    ROLLBACK_SYNCED_HINT = "errors.stacks.rollbackSynced.hint" => "Revert the change in its repository, or point the stack at an earlier tag, then sync.";

    // Compose definitions

    /// A definition that isn't YAML; the parser's words are in Details.
    COMPOSE_UNPARSABLE = "errors.compose.unparsable" => "The YAML can't be parsed.";
    /// A definition without `services:`.
    COMPOSE_NO_SERVICES_SECTION = "errors.compose.noServicesSection" => "The file needs a services: section.";
    /// A definition with an empty `services:`.
    COMPOSE_NO_SERVICE = "errors.compose.noService" => "Define at least one service.";
    /// A service built from local files.
    COMPOSE_BUILD = "errors.compose.build" => "Service \"{service}\" uses build:, which would build from files on this machine. Use a pushed image instead.";
    /// A service built from local files, for a Swarm whose nodes can't use the image.
    COMPOSE_BUILD_SWARM = "errors.compose.buildSwarm" => "Service \"{service}\" is built from files on this machine, but a Swarm's nodes pull their images. Push the image to a registry and name it in image:.";
    /// A config or secret read from a file, in a Managed Stack for a Swarm.
    COMPOSE_FILE_OBJECT_SWARM = "errors.compose.fileObjectSwarm" => "{section} \"{name}\" is read from a file, which a Managed Stack doesn't have. Use a Linked or Synced Stack, or an external one created on the Swarm.";
    /// Starting or stopping a swarm stack, which Docker can't do.
    SWARM_NO_START_STOP = "errors.compose.swarmNoStartStop" => "A swarm stack can't be started or stopped as a whole. Scale its services instead.";
    /// The docker CLI isn't installed, needed for a swarm deploy.
    DOCKER_CLI_MISSING = "errors.compose.dockerCliMissing" => "Deploying to a Swarm needs the docker command on this computer.";
    DOCKER_CLI_MISSING_HINT = "errors.compose.dockerCliMissing.hint" => "Install the Docker CLI (Docker Desktop, or the docker-ce-cli package on Linux). Homeport runs docker stack deploy with it; no Engine is needed here.";
    /// A swarm deploy whose config or secret changed since the last one.
    SWARM_OBJECT_CHANGED = "errors.compose.swarmObjectChanged" => "A config or secret of this Stack changed, and the Swarm can't change one in place.";
    SWARM_OBJECT_CHANGED_HINT = "errors.compose.swarmObjectChanged.hint" => "Give it a new name: for example name: app_conf_$'{'CONF_VERSION'}' under configs:, and change CONF_VERSION in the environment each time its file changes.";
    /// Pulling a swarm stack's images, which its nodes do.
    SWARM_NO_PULL = "errors.compose.swarmNoPull" => "A Swarm's nodes pull their images themselves when a Stack is deployed. Redeploy it instead.";
    /// Linking a swarm stack to a folder that describes other services.
    SWARM_LINK_MISMATCH = "errors.compose.swarmLinkMismatch" => "This folder doesn't describe this Stack: it runs {running}, and the folder's compose files declare {declared}.";
    /// A service without an image.
    COMPOSE_NO_IMAGE = "errors.compose.noImage" => "Service \"{service}\" needs an image.";

    // The compose CLI

    /// Only the standalone Compose v1 is installed.
    COMPOSE_OUTDATED = "errors.compose.outdated" => "docker-compose {version} is too old: Homeport needs Compose v2.";
    /// The Docker CLI is there, its compose plugin isn't.
    COMPOSE_PLUGIN_MISSING = "errors.compose.pluginMissing" => "The Docker CLI is installed without its compose plugin.";
    /// Neither the Docker CLI nor compose is installed.
    COMPOSE_MISSING = "errors.compose.missing" => "The docker compose CLI isn't installed on this computer.";
    /// How to add the plugin on Linux, followed by the command.
    INSTALL_PLUGIN_HINT = "errors.compose.installPlugin.hint" => "Install the compose plugin. On Debian or Ubuntu with Docker's repository:";
    /// How to add the CLI and plugin on Linux, followed by the command.
    INSTALL_CLI_HINT = "errors.compose.installCli.hint" => "Install the Docker CLI with its compose plugin. On Debian or Ubuntu with Docker's repository:";
    /// How to get compose on macOS, followed by the command.
    INSTALL_MACOS_HINT = "errors.compose.installMacos.hint" => "Docker Desktop and OrbStack include it. Otherwise, with Homebrew:";
    /// How to get compose on Windows, followed by the command.
    INSTALL_WINDOWS_HINT = "errors.compose.installWindows.hint" => "Docker Desktop includes it:";
    /// The compose program couldn't be started.
    SPAWN_FAILED = "errors.compose.spawnFailed" => "Homeport couldn't start {program}.";
    /// …because it's no longer on the PATH.
    SPAWN_FAILED_NOT_FOUND_HINT = "errors.compose.spawnFailed.hintNotFound" => "It disappeared from the PATH. Reinstall it, then restart Homeport.";
    /// …because Homeport may not run it.
    SPAWN_FAILED_PERMISSION_HINT = "errors.compose.spawnFailed.hintPermission" => "Homeport isn't allowed to run it. Check its file permissions.";
    /// …for another reason, given in Details.
    SPAWN_FAILED_HINT = "errors.compose.spawnFailed.hint" => "Check that it runs from a terminal, then try again.";
    /// A variable the definition requires has no value.
    MISSING_VARIABLE = "errors.compose.missingVariable" => "The variable {name} has no value.";
    MISSING_VARIABLE_HINT = "errors.compose.missingVariable.hint" => "Add {name}=… to the stack's environment, then try again.";
    /// Compose found the definition invalid.
    INVALID_DEFINITION = "errors.compose.invalidDefinition" => "Compose rejected the stack's definition.";
    INVALID_DEFINITION_HINT = "errors.compose.invalidDefinition.hint" => "Fix the problem reported in the output, in the definition editor.";
    /// Compose couldn't talk to the Host's Engine.
    ENGINE_UNREACHABLE = "errors.compose.engineUnreachable" => "Compose couldn't reach the Docker Engine of {host}.";
    ENGINE_UNREACHABLE_HINT = "errors.compose.engineUnreachable.hint" => "Check that {host} is connected, then try again.";
    /// Compose exited with an error Homeport has no better words for.
    EXITED = "errors.compose.exited" => "docker compose {verb} exited with code {code}.";
    /// Compose was stopped by a signal.
    STOPPED = "errors.compose.stopped" => "docker compose {verb} was stopped.";
    /// For both of the above.
    FAILED_HINT = "errors.compose.failed.hint" => "The output above says why.";
}
