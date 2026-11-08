import { CheckCircle2Icon } from 'lucide-react';
import { ErrorMessage, Progress } from '@/shared/ui';
import type { OperationState } from '../model/progress';
import { t } from '@/shared/i18n';

/** How far an Incus download or unpack is, in Incus's own words. */
export function OperationProgress({ state, done }: { state: OperationState; done: string }) {
  if (state.error) return <ErrorMessage error={state.error} />;
  if (state.done)
    return (
      <p className="text-success flex items-center gap-2 text-sm">
        <CheckCircle2Icon className="size-4" /> {done}
      </p>
    );
  return (
    <div className="grid gap-1.5" role="status">
      <Progress value={state.percent ?? undefined} />
      <p className="text-muted-foreground font-mono text-xs">
        {state.status ?? t('incus.progress.waiting')}
      </p>
    </div>
  );
}
