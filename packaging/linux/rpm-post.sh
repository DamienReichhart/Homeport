# %post of the Homeport .rpm (run by /bin/sh; $1 is 1 on install, 2 on upgrade).
#
# The first time Homeport is installed on this machine, register the Homeport
# repository for dnf or zypper, so an .rpm downloaded by hand still gets
# updates. A marker records that this was done: if the user deletes the
# repository later, no upgrade adds it back. Removing the package removes all
# of it (%postun). dnf and zypper import the key from the repository file the
# first time they use it; rpm can't import it here, during its own transaction.

SHARE=/usr/share/homeport
MARKER=/var/lib/homeport/repository-registered

if [ ! -e "$MARKER" ]; then
  if [ -d /etc/zypp/repos.d ]; then
    REPOS=/etc/zypp/repos.d
  elif [ -d /etc/yum.repos.d ]; then
    REPOS=/etc/yum.repos.d
  fi
  # The quick install script or a manual setup may have added it already.
  if [ -n "$REPOS" ] && [ ! -e "$REPOS/homeport.repo" ]; then
    install -m 0644 "$SHARE/homeport.repo" "$REPOS/homeport.repo" || :
  fi
  mkdir -p "${MARKER%/*}" && : >"$MARKER" || :
fi

exit 0
