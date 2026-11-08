import { CheckIcon, EyeIcon } from 'lucide-react';
import { useState } from 'react';
import { toast } from 'sonner';
import type { ApplyResult } from '@/shared/api';
import { Alert, AlertDescription, Button, ErrorMessage, useConfirmedAction } from '@/shared/ui';
import { useApply, useApplyPreview } from '../../api/apply';
import { previewSummary } from '../../model/apply';
import { ApplyPreview } from './ApplyPreview';
import { t } from '@/shared/i18n';

/**
 * Preview, then apply: a dry run shows what each document would change,
 * and nothing is applied until the user confirms after it. Conflicts with
 * other field managers can be taken over after a second confirmation.
 */
export function ApplyFlow({
  clusterId,
  clusterName,
  yaml,
  namespace,
  onApplied,
}: {
  clusterId: string;
  clusterName: string;
  yaml: string;
  /** Where namespaced documents without one go. */
  namespace: string | null;
  onApplied?: (result: ApplyResult) => void;
}) {
  const preview = useApplyPreview(clusterId);
  const apply = useApply(clusterId);
  const [dialog, confirm] = useConfirmedAction();
  // A preview is only good for the YAML it was made from.
  const [previewed, setPreviewed] = useState<string | null>(null);
  const [applied, setApplied] = useState<ApplyResult | null>(null);
  const current = previewed === yaml ? preview.data : undefined;
  const summary = current && previewSummary(current);

  const run = (force: boolean) =>
    apply.mutateAsync({ yaml, namespace, force }).then((result) => {
      setApplied(result);
      const failed = result.documents.filter((d) => d.action === 'failed').length;
      if (failed === 0) {
        toast.success(t('core.apply.done', { cluster: clusterName }));
        onApplied?.(result);
      }
      return result;
    });

  return (
    <div className="grid gap-3">
      <div className="flex flex-wrap items-center gap-2">
        <Button
          variant="outline"
          disabled={!yaml.trim() || preview.isPending}
          onClick={() => {
            setApplied(null);
            setPreviewed(yaml);
            preview.mutate({ yaml, namespace });
          }}
        >
          <EyeIcon /> {t('core.apply.preview')}
        </Button>
        <Button
          disabled={!summary?.ready || apply.isPending}
          onClick={() =>
            confirm({
              title: t('core.apply.confirmTitle', {
                count: summary?.changes ?? 0,
                cluster: clusterName,
              }),
              description: t('core.apply.confirmDescription', { cluster: clusterName }),
              confirmLabel: t('core.apply.apply'),
              action: () => run(false),
            })
          }
        >
          <CheckIcon /> {t('core.apply.apply')}
        </Button>
        {summary?.forceable && (
          <Button
            variant="destructive"
            disabled={apply.isPending}
            onClick={() =>
              confirm({
                title: t('core.apply.forceTitle'),
                description: t('core.apply.forceDescription', { cluster: clusterName }),
                confirmLabel: t('core.apply.force'),
                destructive: true,
                action: () => run(true),
              })
            }
          >
            {t('core.apply.force')}
          </Button>
        )}
        {previewed !== null && previewed !== yaml && (
          <span className="text-muted-foreground text-sm">{t('core.apply.stale')}</span>
        )}
      </div>
      {preview.error && previewed === yaml && <ErrorMessage error={preview.error} />}
      {apply.error && <ErrorMessage error={apply.error} />}
      {summary?.nothingToDo && !applied && (
        <Alert>
          <AlertDescription>{t('core.apply.noChanges')}</AlertDescription>
        </Alert>
      )}
      {applied ? <ApplyPreview result={applied} /> : current && <ApplyPreview result={current} />}
      {dialog}
    </div>
  );
}
