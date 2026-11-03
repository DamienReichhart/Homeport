# %postun of the Homeport .rpm (run by /bin/sh; $1 is 0 on removal, 1 after an
# upgrade). Removing Homeport also removes the Homeport repository, whoever
# added it. Its key stays in the rpm keyring: `rpm -e gpg-pubkey-<id>` removes it.

if [ "$1" = 0 ]; then
  rm -f /etc/yum.repos.d/homeport.repo /etc/zypp/repos.d/homeport.repo
  rm -rf /var/lib/homeport
fi

exit 0
