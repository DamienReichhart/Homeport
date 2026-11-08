import type { ComposeSupport } from '@/shared/api';
import { HintView, StatusDot } from '@/shared/ui';
import { t } from '@/shared/i18n';

/**
 * The compose CLI found (or the docker CLI, for swarm deploys), or what's
 * missing and how to install it, with what can't be done without it.
 */
export function ComposeStatus({
  compose,
  missing = t('settings.machine.composeMissing'),
}: {
  compose: ComposeSupport;
  missing?: string;
}) {
  if (compose.state === 'available') {
    const { program, prefix, version } = compose.cli;
    return (
      <span className="inline-flex items-center gap-1.5">
        <StatusDot tone="success" />
        {[program, ...prefix, version].join(' ')}
      </span>
    );
  }
  return (
    <div className="grid gap-2">
      <span className="inline-flex items-center gap-1.5">
        <StatusDot tone="warning" />
        {compose.problem.message}
      </span>
      <span className="text-muted-foreground">{missing}</span>
      {compose.problem.hint && <HintView hint={compose.problem.hint} />}
    </div>
  );
}
