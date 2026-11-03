#!/bin/sh
# Installs Homeport on Linux from its package repository, so your package
# manager keeps it up to date:
#
#   curl -fsSL https://damienreichhart.github.io/Homeport/install.sh | sh
#
# On Debian and Ubuntu (apt), Fedora, RHEL and their relatives (dnf) and
# openSUSE (zypper), it adds the Homeport signing key and repository, then
# installs the `homeport` package. Running it again is harmless: it updates
# Homeport if a newer version is out. It asks for sudo only for those steps,
# after saying what it will do.
#
# Manual setup, without this script:
# https://damienreichhart.github.io/Homeport/docs/installation/#linux
set -eu

# Set HOMEPORT_REPO_URL to install from a copy of the repositories (for testing).
SITE=${HOMEPORT_REPO_URL:-https://damienreichhart.github.io/Homeport}
DOWNLOAD_PAGE=https://damienreichhart.github.io/Homeport/download/
MIN_GLIBC=2.35

say() { printf '%s\n' "$*"; }
fail() {
  printf 'Error: %s\n' "$*" >&2
  exit 1
}

fetch() { # url file
  if command -v curl >/dev/null 2>&1; then
    if [ -n "${HOMEPORT_REPO_URL:-}" ]; then
      curl -fsSL -o "$2" "$1" # a test copy may be served over plain HTTP
    else
      curl -fsSL --proto '=https' --tlsv1.2 -o "$2" "$1"
    fi
  elif command -v wget >/dev/null 2>&1; then
    wget -q -O "$2" "$1"
  else
    fail "curl or wget is needed to download Homeport's repository files."
  fi
}

# Whether version $1 is at least $2 (both x.y), in plain sh: minimal systems lack awk.
version_at_least() {
  have_major=${1%%.*} have_minor=${1#*.}
  need_major=${2%%.*} need_minor=${2#*.}
  have_minor=${have_minor%%.*} need_minor=${need_minor%%.*}
  [ "$have_major" -gt "$need_major" ] ||
    { [ "$have_major" -eq "$need_major" ] && [ "$have_minor" -ge "$need_minor" ]; }
}

check_system() {
  [ "$(uname -s)" = Linux ] ||
    fail "this script installs Homeport on Linux. For $(uname -s), download it from $DOWNLOAD_PAGE"

  case "$(uname -m)" in
    x86_64 | amd64 | aarch64 | arm64) ;;
    *) fail "Homeport is built for x64 and ARM64, not $(uname -m)." ;;
  esac

  glibc=$(getconf GNU_LIBC_VERSION 2>/dev/null) || glibc=
  glibc=${glibc#glibc }
  [ -n "$glibc" ] || fail "Homeport needs glibc $MIN_GLIBC or later, and this system doesn't use glibc."
  version_at_least "$glibc" "$MIN_GLIBC" ||
    fail "Homeport needs glibc $MIN_GLIBC or later (Ubuntu 22.04, Debian 12, Fedora 36 or newer); this system has $glibc."
}

detect_manager() {
  if command -v apt-get >/dev/null 2>&1; then
    echo apt
  elif command -v dnf >/dev/null 2>&1; then
    echo dnf
  elif command -v zypper >/dev/null 2>&1; then
    echo zypper
  else
    fail "no apt, dnf or zypper found. Download the AppImage from $DOWNLOAD_PAGE instead."
  fi
}

main() {
  check_system
  manager=$(detect_manager)

  if [ "$(id -u)" -eq 0 ]; then
    sudo=
  elif command -v sudo >/dev/null 2>&1; then
    sudo=sudo
  else
    fail "run this script as root, or install sudo."
  fi

  case "$manager" in
    apt)
      key_path=/etc/apt/keyrings/homeport.asc
      repo_path=/etc/apt/sources.list.d/homeport.sources
      repo_url=$SITE/apt/homeport.sources
      ;;
    dnf)
      key_path=
      repo_path=/etc/yum.repos.d/homeport.repo
      repo_url=$SITE/rpm/homeport.repo
      ;;
    zypper)
      key_path=
      repo_path=/etc/zypp/repos.d/homeport.repo
      repo_url=$SITE/rpm/homeport.repo
      ;;
  esac

  tmp=$(mktemp -d)
  trap 'rm -rf "$tmp"' EXIT
  fetch "$SITE/homeport.asc" "$tmp/homeport.asc"
  fetch "$repo_url" "$tmp/repository"
  # Point a test copy's repository file at that copy.
  if [ "$SITE" != https://damienreichhart.github.io/Homeport ]; then
    sed -i "s|https://damienreichhart.github.io/Homeport|$SITE|g" "$tmp/repository"
  fi

  say "Installing Homeport with $manager. This will:"
  if [ -n "$key_path" ]; then
    say "  1. add the Homeport signing key to $key_path"
  else
    say "  1. import the Homeport signing key into rpm"
  fi
  say "  2. add the Homeport repository to $repo_path"
  say "  3. install (or update) the homeport package"
  [ -z "$sudo" ] || say "sudo may ask for your password."
  say ""

  case "$manager" in
    apt)
      $sudo install -d -m 0755 /etc/apt/keyrings
      $sudo install -m 0644 "$tmp/homeport.asc" "$key_path"
      $sudo install -m 0644 "$tmp/repository" "$repo_path"
      $sudo apt-get update
      $sudo apt-get install -y homeport
      ;;
    dnf)
      $sudo rpm --import "$tmp/homeport.asc"
      $sudo install -m 0644 "$tmp/repository" "$repo_path"
      if rpm -q homeport >/dev/null 2>&1; then
        $sudo dnf upgrade -y --refresh homeport
      else
        $sudo dnf install -y homeport
      fi
      ;;
    zypper)
      $sudo rpm --import "$tmp/homeport.asc"
      $sudo install -m 0644 "$tmp/repository" "$repo_path"
      $sudo zypper --non-interactive refresh homeport
      if rpm -q homeport >/dev/null 2>&1; then
        $sudo zypper --non-interactive update homeport
      else
        $sudo zypper --non-interactive install homeport
      fi
      ;;
  esac

  say ""
  say "Homeport is installed: open it from your application menu. Your package"
  say "manager updates it along with the rest of the system."
}

# Everything runs from here, so a partly downloaded script does nothing.
main "$@"
