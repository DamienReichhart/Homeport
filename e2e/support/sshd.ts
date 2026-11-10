/**
 * A throwaway SSH Host: an `sshd` container that lets in one key made for
 * the run and reaches this machine's Engine through its mounted socket.
 */
import { join } from 'node:path';
import { docker, tryDocker } from './docker';

export const SSHD = 'homeport-e2e-sshd';
const IMAGE = 'homeport-e2e-sshd:latest';
export const SSH_PORT = 2222;

export interface SshHost {
  /** The private key the server accepts, as a user would paste it. */
  privateKey: string;
  stop: () => void;
}

/**
 * Builds and starts the server, with a key made inside it for this run.
 * `HOMEPORT_E2E_SSHD_IMAGE` names an image built beforehand from
 * `support/sshd/Dockerfile` instead, for machines whose containers can't
 * download Alpine's packages; `HOMEPORT_E2E_SSHD_BASE` changes its base image.
 */
export function startSshHost(): SshHost {
  const prebuilt = process.env.HOMEPORT_E2E_SSHD_IMAGE;
  const image = prebuilt ?? IMAGE;
  if (!prebuilt) {
    const base = process.env.HOMEPORT_E2E_SSHD_BASE;
    const args = ['build', '--quiet', '--tag', IMAGE];
    if (base) args.push('--build-arg', `BASE=${base}`);
    docker(...args, join(import.meta.dirname, 'sshd'));
  }
  tryDocker('rm', '--force', SSHD);
  docker(
    'run',
    '--detach',
    '--name',
    SSHD,
    '--publish',
    `127.0.0.1:${SSH_PORT}:22`,
    '--volume',
    '/var/run/docker.sock:/var/run/docker.sock',
    image,
  );
  docker(
    'exec',
    SSHD,
    'sh',
    '-c',
    "ssh-keygen -q -t ed25519 -N '' -C homeport-e2e -f /tmp/key && " +
      'mv /tmp/key.pub /root/.ssh/authorized_keys && chmod 600 /root/.ssh/authorized_keys',
  );
  const privateKey = `${docker('exec', SSHD, 'cat', '/tmp/key')}\n`;
  return { privateKey, stop: () => void tryDocker('rm', '--force', SSHD) };
}
