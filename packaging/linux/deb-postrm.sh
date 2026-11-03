#!/bin/sh
# postrm of the Homeport .deb. `apt purge homeport` also removes the Homeport
# APT repository and its key, whoever added them. A plain `apt remove` keeps
# them, like the rest of a package's configuration.
set -e

if [ "$1" = purge ]; then
  rm -f /etc/apt/sources.list.d/homeport.sources /etc/apt/keyrings/homeport.asc
  rm -rf /var/lib/homeport
fi

exit 0
