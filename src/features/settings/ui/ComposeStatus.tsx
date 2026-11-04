import type { ComposeSupport } from '@/shared/api';
import { HintView, StatusDot } from '@/shared/ui';
import { t } from '@/shared/i18n';

/** The compose CLI found, or what's missing and how to install it. */
export function ComposeStatus({ compose }: { compose: ComposeSupport }) {
  if (compose.state === 'available') {
    const { program, prefix, version } = compose.cli;
    return (
      <span className="inline-flex items-center gap-1.5">
        <StatusDot tone="success" />
        {`${program} ${prefix.join(' ')} ${version}`}
      </span>
    );
  }
  return (
    <div className="grid gap-2">
      <span className="inline-flex items-center gap-1.5">
        <StatusDot tone="warning" />
        {compose.problem.message}
      </span>
      <span className="text-muted-foreground">{t('settings.machine.composeMissing')}</span>
      {compose.problem.hint && <HintView hint={compose.problem.hint} />}
    </div>
  );
}
