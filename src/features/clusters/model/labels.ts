import type { AuthMethod, Cluster } from '@/shared/api';
import { t } from '@/shared/i18n';

/** How a Cluster's user authenticates, in a few words. */
export function authLabel(auth: AuthMethod | null | undefined): string {
  switch (auth?.method ?? 'none') {
    case 'clientCertificate':
      return t('clusters.auth.clientCertificate');
    case 'token':
      return t('clusters.auth.token');
    case 'basic':
      return t('clusters.auth.basic');
    case 'exec':
      return t('clusters.auth.exec', { command: auth?.method === 'exec' ? auth.command : '' });
    case 'oidc':
      return t('clusters.auth.oidc');
    default:
      return t('clusters.auth.none');
  }
}

/** How Homeport authenticates to a Cluster of any kind, in a few words. */
export function clusterAuthLabel(cluster: Cluster): string {
  const c = cluster.connection;
  if (c.kind === 'kubernetes') return authLabel(c.auth);
  return c.aclToken ? t('clusters.auth.aclToken') : t('clusters.auth.none');
}
