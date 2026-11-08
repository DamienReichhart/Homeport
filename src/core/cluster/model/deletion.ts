import type { ApiType, OwnerView, ResourceKind } from '@/shared/api';
import { t } from '@/shared/i18n';
import { kindName } from './paths';

/** An object to delete: one of the kinds Homeport has pages for, or any API type. */
export interface DeleteTarget {
  type: { kind: ResourceKind } | { api: ApiType };
  namespace: string | null;
  name: string;
  /** What controls it, for a Pod a controller would recreate. */
  owner?: OwnerView | null;
}

/** The Kubernetes kind of a target (`Deployment`). */
export function targetKind(target: DeleteTarget): string {
  return 'kind' in target.type ? kindName(target.type.kind) : target.type.api.kind;
}

const WORKLOADS = new Set([
  'Deployment',
  'StatefulSet',
  'DaemonSet',
  'ReplicaSet',
  'Job',
  'CronJob',
]);

/** What deleting `target` sets off, beyond the object itself. */
function consequence(target: DeleteTarget): string | null {
  const kind = targetKind(target);
  if (kind === 'Namespace') return t('core.delete.namespace');
  if (kind === 'Pod' && target.owner && target.owner.kind !== 'Node')
    return t('core.delete.recreated', { kind: target.owner.kind, name: target.owner.name });
  if (kind === 'CronJob') return t('core.delete.cronJob');
  if (WORKLOADS.has(kind)) return t('core.delete.pods');
  if (kind === 'Node') return t('core.delete.node');
  return null;
}

/**
 * The confirmation of a delete: its title, what goes, named with the
 * Cluster, Namespace, kind and name, and what happens next.
 */
export function deleteConfirmation(
  cluster: string,
  targets: DeleteTarget[],
): { title: string; description: string; consequences: string[] } {
  const [first] = targets;
  if (!first) return { title: '', description: '', consequences: [] };
  const kinds = new Set(targets.map(targetKind));
  const namespaces = new Set(targets.map((x) => x.namespace));
  const consequences = [
    ...new Set(targets.map(consequence).filter((c): c is string => c !== null)),
  ];
  if (targets.length === 1) {
    const kind = targetKind(first);
    return {
      title: t('core.delete.title', { kind, name: first.name }),
      description: first.namespace
        ? t('core.delete.oneIn', { kind, name: first.name, namespace: first.namespace, cluster })
        : t('core.delete.one', { kind, name: first.name, cluster }),
      consequences,
    };
  }
  const kind = kinds.size === 1 ? targetKind(first) : null;
  const names = targets.map((x) => (kind ? x.name : `${targetKind(x)} ${x.name}`)).join(', ');
  const namespace = namespaces.size === 1 ? first.namespace : null;
  return {
    title: kind
      ? t('core.delete.titleMany', { count: targets.length, kind })
      : t('core.delete.titleObjects', { count: targets.length }),
    description: namespace
      ? t('core.delete.manyIn', { names, namespace, cluster })
      : t('core.delete.many', { names, cluster }),
    consequences,
  };
}
