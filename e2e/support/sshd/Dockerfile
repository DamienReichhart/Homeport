# An SSH server for the end-to-end tests: root logs in with the key the test
# makes, and may forward to the Engine socket mounted at /var/run/docker.sock,
# which is all an SSH Host needs (Homeport installs nothing on a Host).
ARG BASE=mirror.gcr.io/library/alpine:3.22
FROM ${BASE}
RUN apk add --no-cache openssh-server \
 && ssh-keygen -A \
 && mkdir -p /root/.ssh && chmod 700 /root/.ssh \
 && printf '%s\n' \
      'PermitRootLogin prohibit-password' \
      'PasswordAuthentication no' \
      'AllowTcpForwarding yes' \
      'AllowStreamLocalForwarding yes' \
      > /etc/ssh/sshd_config.d/homeport-e2e.conf
EXPOSE 22
CMD ["/usr/sbin/sshd", "-D", "-e"]
