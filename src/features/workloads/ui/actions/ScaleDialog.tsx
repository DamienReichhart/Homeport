import { useQuery } from '@tanstack/react-query';
import { useState } from 'react';
import {
  Alert,
  AlertDescription,
  Button,
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  Input,
  Label,
} from '@/shared/ui';
import { useScale, type WorkloadRef } from '../../api/mutations';
import { workloadQueries } from '../../api/queries';
import { parseReplicas } from '../../model/actions';
import { t } from '@/shared/i18n';

/** Sets how many Pods a Deployment or StatefulSet runs, warning when an autoscaler owns that. */
export function ScaleDialog({
  clusterId,
  clusterName,
  workload,
  current,
  onClose,
}: {
  clusterId: string;
  clusterName: string;
  workload: WorkloadRef;
  current: number;
  onClose: () => void;
}) {
  const [text, setText] = useState(String(current));
  const replicas = parseReplicas(text);
  const scale = useScale(clusterId, onClose);
  const autoscaler = useQuery(
    workloadQueries.autoscaler(clusterId, workload.kind, workload.namespace, workload.name),
  );
  return (
    <Dialog open onOpenChange={(open) => !open && !scale.isPending && onClose()}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle>{t('workloads.scale.title', { name: workload.name })}</DialogTitle>
          <DialogDescription>
            {t('workloads.scale.description', {
              namespace: workload.namespace,
              cluster: clusterName,
              current,
            })}
          </DialogDescription>
        </DialogHeader>
        <form
          className="grid gap-3"
          onSubmit={(e) => {
            e.preventDefault();
            if (replicas !== null) scale.mutate({ ref: workload, replicas });
          }}
        >
          <div className="grid gap-1.5">
            <Label htmlFor="scale-replicas">{t('workloads.scale.replicas')}</Label>
            <Input
              id="scale-replicas"
              inputMode="numeric"
              value={text}
              onChange={(e) => setText(e.target.value)}
              aria-invalid={replicas === null}
              className="w-32"
              autoFocus
            />
          </div>
          {autoscaler.data && (
            <Alert>
              <AlertDescription>
                {t('workloads.scale.autoscaled', { autoscaler: autoscaler.data })}
              </AlertDescription>
            </Alert>
          )}
          {replicas === 0 && current > 0 && (
            <p className="text-muted-foreground text-sm">{t('workloads.scale.toZero')}</p>
          )}
          <DialogFooter>
            <Button type="button" variant="ghost" onClick={onClose} disabled={scale.isPending}>
              {t('common.cancel')}
            </Button>
            <Button type="submit" disabled={scale.isPending || replicas === null}>
              {t('workloads.scale.submit')}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}
