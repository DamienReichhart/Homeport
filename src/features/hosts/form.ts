import type { Host, HostInput, SshAuth } from '@/shared/api';

export type HostKind = 'local' | 'tcp' | 'tls' | 'ssh';

/** Everything the Host form edits, as plain strings. */
export interface HostFormState {
  name: string;
  kind: HostKind;
  socketPath: string;
  address: string;
  sshHost: string;
  sshPort: string;
  sshUser: string;
  sshSocketPath: string;
  sshAuth: SshAuth;
  caPem: string;
  certPem: string;
  keyPem: string;
  privateKey: string;
  passphrase: string;
}

export const emptyForm = (kind: HostKind = 'ssh'): HostFormState => ({
  name: '',
  kind,
  socketPath: '/var/run/docker.sock',
  address: '',
  sshHost: '',
  sshPort: '22',
  sshUser: '',
  sshSocketPath: '/var/run/docker.sock',
  sshAuth: 'generated',
  caPem: '',
  certPem: '',
  keyPem: '',
  privateKey: '',
  passphrase: '',
});

export function formFromHost(host: Host): HostFormState {
  const form = { ...emptyForm(host.connection.kind), name: host.name };
  const c = host.connection;
  switch (c.kind) {
    case 'local':
      return { ...form, socketPath: c.socketPath };
    case 'tcp':
    case 'tls':
      return { ...form, address: c.address };
    case 'ssh':
      return {
        ...form,
        sshHost: c.host,
        sshPort: String(c.port),
        sshUser: c.user,
        sshSocketPath: c.socketPath,
        sshAuth: c.auth,
      };
  }
}

/**
 * The request for creating or updating a Host. When editing, credentials
 * left blank mean "keep the stored ones".
 */
export function toHostInput(form: HostFormState, editing: boolean): HostInput {
  const name = form.name.trim();
  switch (form.kind) {
    case 'local':
      return {
        name,
        connection: { kind: 'local', socketPath: form.socketPath.trim() },
        credentials: null,
      };
    case 'tcp':
      return { name, connection: { kind: 'tcp', address: form.address.trim() }, credentials: null };
    case 'tls': {
      const blank = !form.caPem.trim() && !form.certPem.trim() && !form.keyPem.trim();
      return {
        name,
        connection: { kind: 'tls', address: form.address.trim() },
        credentials:
          editing && blank
            ? null
            : { kind: 'tls', caPem: form.caPem, certPem: form.certPem, keyPem: form.keyPem },
      };
    }
    case 'ssh': {
      const importing = form.sshAuth === 'imported' && form.privateKey.trim() !== '';
      return {
        name,
        connection: {
          kind: 'ssh',
          host: form.sshHost.trim(),
          port: Number.parseInt(form.sshPort, 10) || 0,
          user: form.sshUser.trim(),
          socketPath: form.sshSocketPath.trim(),
          auth: form.sshAuth,
          publicKey: null,
        },
        credentials: importing
          ? { kind: 'sshKey', privateKey: form.privateKey, passphrase: form.passphrase || null }
          : null,
      };
    }
  }
}
