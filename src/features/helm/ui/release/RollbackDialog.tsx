import { useState } from 'react';
import type { ReleaseRevision, RollbackRequest } from '@/shared/api';
import {
  Button,
  Checkbox,
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  KeyValue,
  Label,
} from '@/shared/ui';
import { t } from '@/shared/i18n';

/** Confirms rolling a release back from its current revision to `target`, naming both. */
export function RollbackDialog({
  clusterName,
  namespace,
  name,
  current,
  target,
  onConfirm,
  onClose,
}: {
  clusterName: string;
  namespace: string;
  name: string;
  current: ReleaseRevision;
  target: ReleaseRevision;
  onConfirm: (request: RollbackRequest) => void;
  onClose: () => void;
}) {
  const [wait, setWait] = useState(false);
  const versions = (r: ReleaseRevision) =>
    t('helm.rollback.versions', { chart: r.chart, app: r.appVersion || '—' });
  return (
    <Dialog open onOpenChange={(o) => !o && onClose()}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>{t('helm.rollback.title', { name, revision: target.revision })}</DialogTitle>
          <DialogDescription>
            {t('helm.rollback.description', {
              name,
              namespace,
              cluster: clusterName,
              revision: target.revision,
            })}
          </DialogDescription>
        </DialogHeader>
        <KeyValue
          items={[
            [t('helm.rollback.from', { revision: current.revision }), versions(current)],
            [t('helm.rollback.to', { revision: target.revision }), versions(target)],
          ]}
        />
        <div className="flex items-center gap-2">
          <Checkbox
            id="helm-rollback-wait"
            checked={wait}
            onCheckedChange={(v) => setWait(v === true)}
          />
          <Label htmlFor="helm-rollback-wait" className="font-normal">
            {t('helm.install.wait')}
          </Label>
        </div>
        <DialogFooter>
          <Button variant="outline" onClick={onClose}>
            {t('common.cancel')}
          </Button>
          <Button
            onClick={() =>
              onConfirm({ name, namespace, revision: target.revision, wait, timeoutSeconds: null })
            }
          >
            {t('helm.rollback.confirm', { revision: target.revision })}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
