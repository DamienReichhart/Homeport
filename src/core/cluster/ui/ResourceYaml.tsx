import { useQuery } from '@tanstack/react-query';
import { PencilIcon } from 'lucide-react';
import { useState } from 'react';
import type { ResourceKind } from '@/shared/api';
import { Button, CopyButton, ErrorMessage, Spinner } from '@/shared/ui';
import { useClusters } from '../api/queries';
import { clusterResourceQueries } from '../api/resources';
import { kindName } from '../model/paths';
import { EditYamlDialog } from './apply/EditYamlDialog';
import { t } from '@/shared/i18n';

/** Kinds Homeport doesn't edit: Secrets (never shown whole) and Events (written by Kubernetes). */
const NOT_EDITABLE: ResourceKind[] = ['secret', 'event'];

/**
 * An object's YAML, without managedFields (and Secret values hidden), with
 * Edit to change it through a preview and server-side apply.
 */
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
  const clusters = useClusters();
  const clusterName = clusters.data?.find((c) => c.cluster.id === clusterId)?.cluster.name ?? '';
  const [editing, setEditing] = useState(false);
  if (yaml.isPending) return <Spinner />;
  if (yaml.error) return <ErrorMessage error={yaml.error} onRetry={() => void yaml.refetch()} />;
  return (
    <div className="relative">
      <div className="absolute top-2 end-2 flex items-center gap-1">
        {!NOT_EDITABLE.includes(kind) && (
          <Button variant="outline" size="sm" onClick={() => setEditing(true)}>
            <PencilIcon /> {t('core.yaml.edit')}
          </Button>
        )}
        <CopyButton text={yaml.data} label={t('core.yaml.copy')} />
      </div>
      <pre
        aria-label={t('core.yaml.label')}
        className="bg-muted max-h-[70vh] overflow-auto rounded-md p-3 font-mono text-xs"
      >
        {yaml.data}
      </pre>
      {editing && (
        <EditYamlDialog
          clusterId={clusterId}
          clusterName={clusterName}
          target={{ kind }}
          kindName={kindName(kind)}
          namespace={namespace}
          name={name}
          onClose={() => {
            setEditing(false);
            void yaml.refetch();
          }}
        />
      )}
    </div>
  );
}
