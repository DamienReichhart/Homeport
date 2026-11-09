import type { AppErrorPayload, ClusterKind, ClusterStatus } from '@/shared/api';
import type { Tone } from '@/shared/ui';
import { t } from '@/shared/i18n';

export interface ClusterStatusSummary {
  label: string;
  tone: Tone;
  detail?: string;
}

/** The classified cause of a status that needs the user, with its hint and detail. */
export function clusterStatusError(status: ClusterStatus): AppErrorPayload | null {
  return 'error' in status ? status.error : null;
}

/** What a connected Cluster of `kind` runs: its version, and a Nomad Cluster's region. */
function connectedDetail(
  status: Extract<ClusterStatus, { state: 'connected' }>,
  kind: ClusterKind,
): string | undefined {
  if (kind === 'kubernetes')
    return t('core.cluster.serverVersion', { version: status.serverVersion });
  const region = status.region ?? 'global';
  return status.serverVersion
    ? t('core.cluster.nomadVersion', { version: status.serverVersion, region })
    : t('core.cluster.nomadRegion', { region });
}

/** A short label, a colour and the reason, for showing the status of a Cluster of `kind`. */
export function describeClusterStatus(
  status: ClusterStatus,
  kind: ClusterKind = 'kubernetes',
): ClusterStatusSummary {
  switch (status.state) {
    case 'connected':
      return {
        label: t('core.status.connected'),
        tone: 'success',
        detail: connectedDetail(status, kind),
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
    case 'credentialsLost':
      return {
        label: t('core.status.credentialsLost'),
        tone: 'danger',
        detail: t('core.cluster.credentialsLostDetail'),
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
