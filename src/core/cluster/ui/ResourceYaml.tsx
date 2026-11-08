import { useQuery } from '@tanstack/react-query';
import type { ResourceKind } from '@/shared/api';
import { CopyButton, ErrorMessage, Spinner } from '@/shared/ui';
import { clusterResourceQueries } from '../api/resources';
import { t } from '@/shared/i18n';

/** An object's YAML, read-only, without managedFields (and Secret values hidden). */
export function ResourceYaml({
  clusterId,
  kind,
  namespace,
  name,
}: {
  clusterId: string;
  kind: ResourceKind;
  namespace: string | null;
  name: string;
}) {
  const yaml = useQuery(clusterResourceQueries.yaml(clusterId, kind, namespace, name));
  if (yaml.isPending) return <Spinner />;
  if (yaml.error) return <ErrorMessage error={yaml.error} onRetry={() => void yaml.refetch()} />;
  return (
    <div className="relative">
      <div className="absolute top-2 right-2">
        <CopyButton text={yaml.data} label={t('core.yaml.copy')} />
      </div>
      <pre
        aria-label={t('core.yaml.label')}
        className="bg-muted max-h-[70vh] overflow-auto rounded-md p-3 font-mono text-xs"
      >
        {yaml.data}
      </pre>
    </div>
  );
}
