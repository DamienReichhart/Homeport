#!/bin/sh
# Runs the end-to-end tests against the debug build of the app:
#
#   bun tauri build --debug --no-bundle   # once, and after each change
#   (cd e2e && bun install && ./run.sh [wdio arguments, e.g. --spec specs/ssh.e2e.ts])
#
# Each run gets a fresh app data folder, its own D-Bus session with an
# unlocked GNOME Keyring (so sealed secrets work, as on a desktop), and a
# virtual display. Needs: a running Docker Engine with the compose plugin,
# tauri-driver (cargo install tauri-driver --locked), webkit2gtk-driver,
# xvfb, dbus and gnome-keyring. Linux only: there is no WebDriver for
# macOS's WKWebView. Each run starts its own containers (homeport-e2e-*) on
# the local Engine and removes them; the SSH spec builds a small sshd image
# (support/sshd/), or uses HOMEPORT_E2E_SSHD_IMAGE when it's set.
set -eu

here=$(cd "$(dirname "$0")" && pwd)
home=$(mktemp -d "${TMPDIR:-/tmp}/homeport-e2e.XXXXXX")
# The document portal may have mounted itself in the runtime folder.
trap 'fusermount -u "$home/runtime/doc" 2>/dev/null; rm -rf "$home" 2>/dev/null || true' EXIT

# Tauri puts the app's data, config and logs under these.
export XDG_DATA_HOME="$home/data" XDG_CONFIG_HOME="$home/config" XDG_CACHE_HOME="$home/cache"
export XDG_RUNTIME_DIR="$home/runtime"
mkdir -p "$XDG_DATA_HOME/keyrings" "$XDG_CONFIG_HOME" "$XDG_CACHE_HOME" "$XDG_RUNTIME_DIR"
chmod 700 "$XDG_RUNTIME_DIR"
export HOMEPORT_E2E_HOME="$home"
# The specs read the app's English labels, so the OS's language is pinned:
# a C locale, as on CI runners, which WebKitGTK reports as the invalid tag
# "C" and the app must turn into English (src/shared/lib/languages.ts).
export LANG=C.UTF-8
unset LANGUAGE LC_ALL LC_MESSAGES
# Xvfb has no GPU: WebKit renders without compositing or DMA-BUF buffers.
export WEBKIT_DISABLE_COMPOSITING_MODE=1 WEBKIT_DISABLE_DMABUF_RENDERER=1

cd "$here"
dbus-run-session -- sh -c '
  # The login keyring, unlocked with a throwaway password.
  printf e2e | gnome-keyring-daemon --unlock --components=secrets >/dev/null
  exec xvfb-run --auto-servernum --server-args="-screen 0 1440x900x24" \
    node_modules/.bin/wdio run wdio.conf.ts "$@"
' sh "$@"
