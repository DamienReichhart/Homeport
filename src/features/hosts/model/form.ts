import type {
  BastionAuth,
  BastionKeyInput,
  EngineKind,
  Host,
  HostInput,
  SshAuth,
} from '@/shared/api';
import type { MessageKey } from '@/shared/i18n';

export type HostKind = 'local' | 'tcp' | 'tls' | 'ssh';

/** One bastion of an SSH Host, as the form edits it. */
export interface BastionFormState {
  host: string;
  port: string;
  user: string;
  auth: BastionAuth;
  privateKey: string;
  passphrase: string;
  /** The public half of the key already stored for it, when editing. */
  storedPublicKey: string | null;
}

/** Everything the Host form edits, as plain strings. */
export interface HostFormState {
  name: string;
  kind: HostKind;
  /** The kind of Engine the Host points to; `kind` is the connection's. */
  engine: EngineKind;
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
  /** The servers to go through first, in order. */
  bastions: BastionFormState[];
}

export const emptyBastion = (): BastionFormState => ({
  host: '',
  port: '22',
  user: '',
  auth: 'sameAsHost',
  privateKey: '',
  passphrase: '',
  storedPublicKey: null,
});

export const emptyForm = (kind: HostKind = 'ssh'): HostFormState => ({
  name: '',
  kind,
  engine: 'docker',
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
  bastions: [],
});

export function formFromHost(host: Host): HostFormState {
  const form = { ...emptyForm(host.connection.kind), name: host.name, engine: host.engine };
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
        bastions: c.bastions.map((b) => ({
          ...emptyBastion(),
          host: b.host,
          port: String(b.port),
          user: b.user,
          auth: b.auth,
          storedPublicKey: b.publicKey,
        })),
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
        engine: form.engine,
        credentials: null,
      };
    case 'tcp':
      return {
        name,
        connection: { kind: 'tcp', address: form.address.trim() },
        engine: form.engine,
        credentials: null,
      };
    case 'tls': {
      const blank = !form.caPem.trim() && !form.certPem.trim() && !form.keyPem.trim();
      return {
        name,
        connection: { kind: 'tls', address: form.address.trim() },
        engine: form.engine,
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
          bastions: form.bastions.map((b) => ({
            host: b.host.trim(),
            port: Number.parseInt(b.port, 10) || 0,
            user: b.user.trim(),
            auth: b.auth,
            publicKey: null,
            hostKey: null,
          })),
        },
        engine: form.engine,
        credentials: importing
          ? { kind: 'sshKey', privateKey: form.privateKey, passphrase: form.passphrase || null }
          : null,
        bastionKeys: bastionKeys(form.bastions),
      };
    }
  }
}

/** The keys typed for bastions that use their own; blank ones keep the stored key. */
function bastionKeys(bastions: BastionFormState[]): BastionKeyInput[] {
  return bastions.flatMap((b, index) =>
    b.auth === 'imported' && b.privateKey.trim() !== ''
      ? [{ index, privateKey: b.privateKey, passphrase: b.passphrase || null }]
      : [],
  );
}

/** `bastions` with the one at `index` moved by `by` places, kept within the list. */
export function moveBastion(
  bastions: BastionFormState[],
  index: number,
  by: number,
): BastionFormState[] {
  const to = index + by;
  if (to < 0 || to >= bastions.length) return bastions;
  const next = [...bastions];
  const [moved] = next.splice(index, 1);
  next.splice(to, 0, moved!);
  return next;
}

/** The kinds of Host the form offers, in the order it shows them. */
/** The ways a bastion can log in, in the order the form shows them. */
export const BASTION_AUTHS: { auth: BastionAuth; label: MessageKey }[] = [
  { auth: 'sameAsHost', label: 'hosts.bastions.auth.sameAsHost' },
  { auth: 'imported', label: 'hosts.bastions.auth.imported' },
  { auth: 'agent', label: 'hosts.bastions.auth.agent' },
];

/**
 * The kind of Engine a socket path suggests: containerd's and Incus's own
 * sockets name them, every other one is a Docker-compatible Engine's.
 */
export function engineForSocket(path: string): EngineKind {
  const file = path.trim().toLowerCase();
  if (/(^|\/)containerd(\/containerd)?\.sock$/.test(file)) return 'containerd';
  if (/(^|\/)(incus|lxd)(\/unix)?\.socket$/.test(file)) return 'incus';
  return 'docker';
}

/** The kinds of Engine the form offers; only Docker-compatible ones connect for now. */
export const ENGINE_KINDS: { engine: EngineKind; available: boolean }[] = [
  { engine: 'docker', available: true },
  { engine: 'containerd', available: false },
  { engine: 'incus', available: false },
];

export const HOST_KINDS: { kind: HostKind; label: MessageKey }[] = [
  { kind: 'ssh', label: 'hosts.kind.ssh' },
  { kind: 'tls', label: 'hosts.kind.tls' },
  { kind: 'tcp', label: 'hosts.kind.tcp' },
  { kind: 'local', label: 'hosts.kind.local' },
];
