import type { GitSupport } from '@/shared/api';
import { HintView, StatusDot } from '@/shared/ui';
import { t } from '@/shared/i18n';

/** The git CLI found, or what's missing and how to install it. */
export function GitStatus({ git }: { git: GitSupport }) {
  if (git.state === 'available') {
    return (
      <span className="inline-flex items-center gap-1.5">
        <StatusDot tone="success" />
        {`${git.cli.program} ${git.cli.version}`}
      </span>
    );
  }
  return (
    <div className="grid gap-2">
      <span className="inline-flex items-center gap-1.5">
        <StatusDot tone="warning" />
        {git.problem.message}
      </span>
      <span className="text-muted-foreground">{t('settings.machine.gitMissing')}</span>
      {git.problem.hint && <HintView hint={git.problem.hint} />}
    </div>
  );
}
