import type { AppErrorPayload, HostConnection, HostStatus } from '@/shared/api';
import type { Tone } from '@/shared/ui';
import { t } from '@/shared/i18n';

export interface StatusSummary {
  label: string;
  tone: Tone;
  detail?: string;
}

/** The classified cause of a status that needs the user, with its hint and detail. */
export function statusError(status: HostStatus): AppErrorPayload | null {
  return 'error' in status ? status.error : null;
}

/** A short label, a colour and the reason, for showing a Host's status. */
export function describeStatus(status: HostStatus): StatusSummary {
  switch (status.state) {
    case 'connected':
      return {
        label: t('core.status.connected'),
        tone: 'success',
        detail: status.podmanVersion
          ? t('core.host.podmanVersion', { version: status.podmanVersion })
          : status.containerdVersion
            ? t('core.host.containerdVersion', { version: status.containerdVersion })
            : status.incusVersion
              ? t('core.host.incusVersion', { version: status.incusVersion })
              : t('core.host.engineVersion', { version: status.engineVersion }),
      };
    case 'connecting':
      return { label: t('core.status.connecting'), tone: 'info' };
    case 'disabled':
      return { label: t('core.status.disabled'), tone: 'muted' };
    case 'unreachable':
      return {
        label: t('core.status.unreachable'),
        tone: 'danger',
        detail: t('core.status.unreachableDetail', {
          message: status.error.message,
          secs: status.retryInSecs,
        }),
      };
    case 'unauthorized':
      return { label: t('core.status.unauthorized'), tone: 'danger', detail: status.error.message };
    case 'hostKeyUntrusted':
      return {
        label: t('core.status.hostKeyUntrusted'),
        tone: 'warning',
        detail: t('core.status.hostKeyUntrustedDetail', { fingerprint: status.fingerprint }),
      };
    case 'hostKeyMismatch':
      return {
        label: t('core.status.hostKeyMismatch'),
        tone: 'danger',
        detail: t('core.status.hostKeyMismatchDetail', {
          expected: status.expectedFingerprint,
          actual: status.fingerprint,
        }),
      };
    case 'credentialsLost':
      return {
        label: t('core.status.credentialsLost'),
        tone: 'danger',
        detail: t('core.status.credentialsLostDetail'),
      };
    case 'keychainUnavailable':
      return {
        label: t('core.status.keychainUnavailable'),
        tone: 'danger',
        detail: status.error.message,
      };
    case 'invalid':
      return { label: t('core.status.invalid'), tone: 'danger', detail: status.error.message };
  }
}

export function connectionLabel(kind: string): string {
  return { local: t('core.connection.local'), tcp: 'TCP', tls: 'TLS', ssh: 'SSH' }[kind] ?? kind;
}

/** Where a Host lives, in one line: a socket path, an address or user@server. */
export function describeAddress(c: HostConnection): string {
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

/** The address published container ports are reachable on, for links. */
export function hostAddress(c: HostConnection): string {
  switch (c.kind) {
    case 'local':
      return 'localhost';
    case 'tcp':
    case 'tls':
      return c.address.replace(/:\d+$/, '').replace(/^\[|\]$/g, '');
    case 'ssh':
      return c.host;
  }
}
