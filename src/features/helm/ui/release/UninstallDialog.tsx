import { useQuery } from '@tanstack/react-query';
import { useState } from 'react';
import type { UninstallRequest } from '@/shared/api';
import {
  Button,
  Checkbox,
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  ErrorMessage,
  Label,
  Spinner,
} from '@/shared/ui';
import { helmQueries } from '../../api/queries';
import { t } from '@/shared/i18n';

/**
 * Confirms uninstalling a release, naming it, its Namespace and Cluster
 * and listing the objects that go with it, with Keep history.
 */
export function UninstallDialog({
  clusterId,
  clusterName,
  namespace,
  name,
  revision,
  onConfirm,
  onClose,
}: {
  clusterId: string;
  clusterName: string;
  namespace: string;
  name: string;
  revision: number;
  onConfirm: (request: UninstallRequest) => void;
  onClose: () => void;
}) {
  const details = useQuery(helmQueries.details(clusterId, namespace, name, revision));
  const [keepHistory, setKeepHistory] = useState(false);
  const [wait, setWait] = useState(false);
  return (
    <Dialog open onOpenChange={(o) => !o && onClose()}>
      <DialogContent className="sm:max-w-xl">
        <DialogHeader>
          <DialogTitle className="text-destructive">
            {t('helm.uninstall.title', { name })}
          </DialogTitle>
          <DialogDescription>
            {t('helm.uninstall.description', { name, namespace, cluster: clusterName })}
          </DialogDescription>
        </DialogHeader>
        {details.isPending ? (
          <Spinner />
        ) : details.error ? (
          <ErrorMessage error={details.error} onRetry={() => void details.refetch()} />
        ) : (
          <div className="grid gap-2">
            <p className="text-sm font-medium">
              {t('helm.uninstall.resources', { count: details.data.resources.length })}
            </p>
            <ul
              aria-label={t('helm.uninstall.resourcesLabel')}
              className="bg-muted max-h-48 overflow-auto rounded-md p-2 font-mono text-xs"
            >
              {details.data.resources.map((r) => (
                <li key={`${r.apiVersion}/${r.kind}/${r.namespace ?? ''}/${r.name}`}>
                  {r.kind}/{r.name}
                </li>
              ))}
            </ul>
            <p className="text-muted-foreground text-xs">{t('helm.uninstall.kept')}</p>
          </div>
        )}
        <div className="grid gap-2">
          <div className="flex items-center gap-2">
            <Checkbox
              id="helm-keep-history"
              checked={keepHistory}
              onCheckedChange={(v) => setKeepHistory(v === true)}
            />
            <Label htmlFor="helm-keep-history" className="font-normal">
              {t('helm.uninstall.keepHistory')}
            </Label>
          </div>
          <div className="flex items-center gap-2">
            <Checkbox
              id="helm-uninstall-wait"
              checked={wait}
              onCheckedChange={(v) => setWait(v === true)}
            />
            <Label htmlFor="helm-uninstall-wait" className="font-normal">
              {t('helm.uninstall.wait')}
            </Label>
          </div>
        </div>
        <DialogFooter>
          <Button variant="outline" onClick={onClose}>
            {t('common.cancel')}
          </Button>
          <Button
            variant="destructive"
            className="bg-destructive hover:bg-destructive/90 text-white"
            disabled={details.isPending}
            onClick={() => onConfirm({ name, namespace, keepHistory, wait, timeoutSeconds: null })}
          >
            {t('helm.uninstall.confirm', { name })}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
