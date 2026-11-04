/**
 * The Linux package repositories and the commands that set them up, shared by
 * the download page and the docs. The repositories live on this site at a
 * permanent URL (ADR 0003), independent of where a preview build is served.
 */
export const REPOSITORY_URL = 'https://damienreichhart.github.io/Homeport';
export const KEY_URL = `${REPOSITORY_URL}/homeport.asc`;
export const INSTALL_SCRIPT_URL = `${REPOSITORY_URL}/install.sh`;
export const INSTALL_COMMAND = `curl -fsSL ${INSTALL_SCRIPT_URL} | sh`;

export interface ManualSetup {
  id: 'apt' | 'dnf' | 'zypper';
  label: string;
  distributions: string;
  commands: string;
}

/** What `install.sh` does, by hand: the same files at the same paths. */
export const MANUAL_SETUP: ManualSetup[] = [
  {
    id: 'apt',
    label: 'Debian, Ubuntu',
    distributions: 'Debian 12, Ubuntu 22.04, Linux Mint 21, Pop!_OS 22.04 or newer',
    commands: [
      'sudo install -d -m 0755 /etc/apt/keyrings',
      `curl -fsSL ${KEY_URL} | sudo tee /etc/apt/keyrings/homeport.asc > /dev/null`,
      `curl -fsSL ${REPOSITORY_URL}/apt/homeport.sources | sudo tee /etc/apt/sources.list.d/homeport.sources > /dev/null`,
      'sudo apt update',
      'sudo apt install homeport',
    ].join('\n'),
  },
  {
    id: 'dnf',
    label: 'Fedora, RHEL',
    distributions: 'Fedora 36, RHEL 10, Rocky Linux and AlmaLinux 10 or newer',
    commands: [
      `sudo rpm --import ${KEY_URL}`,
      `curl -fsSL ${REPOSITORY_URL}/rpm/homeport.repo | sudo tee /etc/yum.repos.d/homeport.repo > /dev/null`,
      'sudo dnf install homeport',
    ].join('\n'),
  },
  {
    id: 'zypper',
    label: 'openSUSE',
    distributions: 'openSUSE Tumbleweed and Leap 16',
    commands: [
      `sudo rpm --import ${KEY_URL}`,
      `curl -fsSL ${REPOSITORY_URL}/rpm/homeport.repo | sudo tee /etc/zypp/repos.d/homeport.repo > /dev/null`,
      'sudo zypper refresh',
      'sudo zypper install homeport',
    ].join('\n'),
  },
];

/** How each package manager updates Homeport, as the app shows it too. */
export const UPDATE_COMMANDS: Record<ManualSetup['id'], string> = {
  apt: 'sudo apt update && sudo apt install --only-upgrade homeport',
  dnf: 'sudo dnf upgrade --refresh homeport',
  zypper: 'sudo zypper refresh && sudo zypper update homeport',
};

/** Installers the in-app updater doesn't cover yet, each with its tracking issue. */
export const COMING_LATER: { name: string; what: string; issue: number }[] = [
  { name: 'Homebrew', what: 'brew install --cask homeport, on macOS', issue: 5 },
  { name: 'winget', what: 'winget install, on Windows', issue: 6 },
  { name: 'Scoop', what: 'a Homeport bucket, on Windows', issue: 7 },
  { name: 'AUR', what: 'homeport-bin, on Arch Linux', issue: 8 },
  { name: 'Flathub', what: 'a Flatpak, on any Linux distribution', issue: 9 },
  { name: 'Snap', what: 'the Snap Store, on Ubuntu and others', issue: 10 },
];
