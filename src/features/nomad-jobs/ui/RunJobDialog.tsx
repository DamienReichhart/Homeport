import { FolderOpenIcon } from 'lucide-react';
import { useState } from 'react';
import type { NomadPlan, NomadRunResult } from '@/shared/api';
import {
  Badge,
  Button,
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  ErrorMessage,
  JobSpecEditor,
  toastError,
} from '@/shared/ui';
import { openJobSpec } from '../api/files';
import { usePlanJob, useRunJob } from '../api/mutations';
import { specFormat } from '../model/plan';
import { PlanView } from './PlanView';
import { t } from '@/shared/i18n';

/**
 * Runs a job from its specification: write, paste or open it, see Nomad's
 * plan, then run it. Running fails rather than overwrite a job someone
 * changed since the plan. `namespace` is where a job naming none goes.
 */
export function RunJobDialog({
  clusterId,
  clusterName,
  namespace,
  initial = '',
  title,
  onClose,
  onRan,
}: {
  clusterId: string;
  clusterName: string;
  namespace: string | null;
  initial?: string;
  title?: string;
  onClose: () => void;
  onRan: (result: NomadRunResult) => void;
}) {
  const [spec, setSpec] = useState(initial);
  const [plan, setPlan] = useState<NomadPlan | null>(null);
  const planJob = usePlanJob(clusterId);
  const run = useRunJob(clusterId, (result) => {
    onRan(result);
    onClose();
  });
  const busy = planJob.isPending || run.isPending;
  const format = specFormat(spec);
  const edit = (next: string) => {
    planJob.reset();
    run.reset();
    setSpec(next);
  };
  return (
    <Dialog open onOpenChange={(open) => !open && !busy && onClose()}>
      <DialogContent className="sm:max-w-3xl">
        <DialogHeader>
          <DialogTitle>{title ?? t('nomad.run.title')}</DialogTitle>
          <DialogDescription>
            {t('nomad.run.description', {
              cluster: clusterName,
              namespace: namespace && namespace !== '*' ? namespace : 'default',
            })}
          </DialogDescription>
        </DialogHeader>
        {plan ? (
          <div className="grid max-h-[60vh] gap-3 overflow-y-auto">
            <PlanView plan={plan} />
            {run.error && <ErrorMessage error={run.error} />}
          </div>
        ) : (
          <div className="grid gap-2">
            <div className="flex items-center justify-between gap-2">
              <Badge variant="outline">
                {format === 'json' ? t('nomad.run.formatJson') : t('nomad.run.formatHcl')}
              </Badge>
              <Button
                variant="outline"
                size="sm"
                disabled={busy}
                onClick={() =>
                  void openJobSpec()
                    .then((text) => text !== null && edit(text))
                    .catch((e: unknown) => toastError(e, t('nomad.run.openFailed')))
                }
              >
                <FolderOpenIcon /> {t('nomad.run.open')}
              </Button>
            </div>
            <JobSpecEditor
              value={spec}
              onChange={edit}
              label={t('nomad.run.editor')}
              height="20rem"
            />
            {planJob.error && <ErrorMessage error={planJob.error} />}
          </div>
        )}
        <DialogFooter>
          {plan ? (
            <>
              <Button variant="ghost" disabled={busy} onClick={() => setPlan(null)}>
                {t('nomad.run.back')}
              </Button>
              <Button disabled={busy} onClick={() => run.mutate({ plan, source: spec })}>
                {plan.placement ? t('nomad.run.runAnyway') : t('nomad.run.run')}
              </Button>
            </>
          ) : (
            <>
              <Button variant="ghost" disabled={busy} onClick={onClose}>
                {t('common.cancel')}
              </Button>
              <Button
                disabled={busy || !spec.trim()}
                onClick={() => planJob.mutate({ spec, namespace }, { onSuccess: setPlan })}
              >
                {planJob.isPending ? t('nomad.run.planning') : t('nomad.run.plan')}
              </Button>
            </>
          )}
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
