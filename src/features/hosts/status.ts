import type { HostConnection, HostStatus } from '@/shared/api';
import type { Tone } from '@/shared/ui';

export interface StatusSummary {
  label: string;
  tone: Tone;
  detail?: string;
}

/** A short label, a colour and the reason, for showing a Host's status. */
export function describeStatus(status: HostStatus): StatusSummary {
  switch (status.state) {
    case 'connected':
      return { label: 'Connected', tone: 'success', detail: `Engine ${status.engineVersion}` };
    case 'connecting':
      return { label: 'Connecting', tone: 'info' };
    case 'disabled':
      return { label: 'Disabled', tone: 'muted' };
    case 'unreachable':
      return {
        label: 'Unreachable',
        tone: 'danger',
        detail: `${status.message} — retrying in ${status.retryInSecs}s`,
      };
    case 'unauthorized':
      return { label: 'Access refused', tone: 'danger', detail: status.message };
    case 'hostKeyUntrusted':
      return {
        label: 'Server key not trusted',
        tone: 'warning',
        detail: `Check the fingerprint ${status.fingerprint} and trust it to connect.`,
      };
    case 'hostKeyMismatch':
      return {
        label: 'Server key changed',
        tone: 'danger',
        detail: `Expected ${status.expectedFingerprint}, got ${status.fingerprint}.`,
      };
    case 'credentialsLost':
      return {
        label: 'Credentials lost',
        tone: 'danger',
        detail: 'The stored credentials can’t be decrypted. Re-enter them or regenerate the key.',
      };
    case 'keychainUnavailable':
      return { label: 'Keychain unavailable', tone: 'danger', detail: status.message };
    case 'invalid':
      return { label: 'Misconfigured', tone: 'danger', detail: status.message };
  }
}

export function connectionLabel(kind: string): string {
  return { local: 'Local', tcp: 'TCP', tls: 'TLS', ssh: 'SSH' }[kind] ?? kind;
}

/** Where a Host lives, in one line: a socket path, an address or user@server. */
export function describeEndpoint(c: HostConnection): string {
  switch (c.kind) {
    case 'local':
      return c.socketPath;
    case 'tcp':
    case 'tls':
      return c.address;
    case 'ssh':
      return `${c.user}@${c.host}${c.port === 22 ? '' : `:${c.port}`}`;
  }
}
