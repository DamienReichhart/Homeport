import { useQuery } from '@tanstack/react-query';
import { useState } from 'react';
import { commands, type ApiType, type ResourceKind } from '@/shared/api';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  ErrorMessage,
  Spinner,
  YamlEditor,
} from '@/shared/ui';
import { clusterKeys } from '../../api/keys';
import { ApplyFlow } from './ApplyFlow';
import { t } from '@/shared/i18n';

/** An object of one of Homeport's pages, or of any API type. */
export type EditTarget = { kind: ResourceKind } | { api: ApiType };

function EditForm({
  clusterId,
  clusterName,
  namespace,
  live,
  onClose,
}: {
  clusterId: string;
  clusterName: string;
  namespace: string | null;
  live: string;
  onClose: () => void;
}) {
  const [yaml, setYaml] = useState(live);
  return (
    <div className="grid min-h-0 gap-3 overflow-y-auto">
      <YamlEditor value={yaml} onChange={setYaml} label={t('core.yaml.label')} height="22rem" />
      <ApplyFlow
        clusterId={clusterId}
        clusterName={clusterName}
        yaml={yaml}
        namespace={namespace}
        onApplied={onClose}
      />
    </div>
  );
}

/**
 * Edits an object's YAML like `kubectl edit`: the live object without what
 * the server manages, then a preview of what changed, then server-side apply.
 */
export function EditYamlDialog({
  clusterId,
  clusterName,
  target,
  kindName,
  namespace,
  name,
  onClose,
}: {
  clusterId: string;
  clusterName: string;
  target: EditTarget;
  kindName: string;
  namespace: string | null;
  name: string;
  onClose: () => void;
}) {
  const live = useQuery({
    queryKey: [...clusterKeys.scope(clusterId), 'edit', kindName, namespace, name],
    queryFn: () =>
      'kind' in target
        ? commands.clustersResourceEditYaml(clusterId, target.kind, namespace, name)
        : commands.clustersApiEditYaml(clusterId, target.api, namespace, name),
    gcTime: 0,
    staleTime: Infinity,
    retry: false,
  });
  return (
    <Dialog open onOpenChange={(open) => !open && onClose()}>
      <DialogContent className="flex max-h-[90vh] flex-col sm:max-w-4xl">
        <DialogHeader>
          <DialogTitle>{t('core.yaml.editTitle', { kind: kindName, name })}</DialogTitle>
          <DialogDescription>
            {t('core.yaml.editDescription', { cluster: clusterName })}
          </DialogDescription>
        </DialogHeader>
        {live.isPending ? (
          <Spinner />
        ) : live.error ? (
          <ErrorMessage error={live.error} onRetry={() => void live.refetch()} />
        ) : (
          <EditForm
            clusterId={clusterId}
            clusterName={clusterName}
            namespace={namespace}
            live={live.data}
            onClose={onClose}
          />
        )}
      </DialogContent>
    </Dialog>
  );
}
