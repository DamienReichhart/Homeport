#!/bin/sh
# postinst of the Homeport .deb.
#
# The first time Homeport is installed on this machine, register the Homeport
# APT repository, so a .deb downloaded by hand still gets updates from apt.
# A marker records that this was done: if the user deletes the source later,
# no upgrade adds it back. `apt purge homeport` removes all of it (postrm).
set -e

SHARE=/usr/share/homeport
SOURCES=/etc/apt/sources.list.d/homeport.sources
KEYRING=/etc/apt/keyrings/homeport.asc
MARKER=/var/lib/homeport/repository-registered

if [ "$1" = configure ]; then
  if [ ! -e "$MARKER" ]; then
    # The quick install script or a manual setup may have added it already.
    if [ ! -e "$SOURCES" ]; then
      install -d -m 0755 /etc/apt/keyrings /etc/apt/sources.list.d
      install -m 0644 "$SHARE/homeport.asc" "$KEYRING"
      install -m 0644 "$SHARE/homeport.sources" "$SOURCES"
    fi
    install -d -m 0755 "${MARKER%/*}"
    : >"$MARKER"
  elif [ -e "$KEYRING" ]; then
    # Keep the repository key current, so a new key reaches existing installs.
    install -m 0644 "$SHARE/homeport.asc" "$KEYRING"
  fi
fi

exit 0
