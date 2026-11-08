import type { HelmSupport } from '@/shared/api';
import { HintView, StatusDot } from '@/shared/ui';
import { t } from '@/shared/i18n';

/** The helm CLI found, or what's missing and how to install it. */
export function HelmStatus({ helm }: { helm: HelmSupport }) {
  if (helm.state === 'available') {
    return (
      <span className="inline-flex items-center gap-1.5">
        <StatusDot tone="success" />
        {`${helm.cli.program} ${helm.cli.version}`}
      </span>
    );
  }
  return (
    <div className="grid gap-2">
      <span className="inline-flex items-center gap-1.5">
        <StatusDot tone="warning" />
        {helm.problem.message}
      </span>
      <span className="text-muted-foreground">{t('settings.machine.helmMissing')}</span>
      {helm.problem.hint && <HintView hint={helm.problem.hint} />}
    </div>
  );
}
