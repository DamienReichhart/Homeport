import { openUrl } from '@tauri-apps/plugin-opener';
import type { UpdateCheck } from '@/shared/api';
import { Button, CopyButton, ErrorMessage, toastError } from '@/shared/ui';
import { useInstallUpdate } from '../api/mutations';
import { describeProgress, WEBSITE_URL } from '../model/updates';
import { t } from '@/shared/i18n';

/**
 * What the user can do about an available update, depending on how this copy
 * was installed: install it here, run the package manager, or download it.
 */
export function UpdateActions({ check }: { check: UpdateCheck }) {
  const { state, install, busy } = useInstallUpdate();
  const { method } = check;
  const progress = describeProgress(state);

  return (
    <div className="flex flex-wrap items-center gap-2">
      {method.kind === 'packageManager' && (
        <>
          <code className="bg-muted rounded px-1.5 py-0.5 font-mono text-xs">{method.command}</code>
          <CopyButton text={method.command} label={t('updates.copyCommand')} />
        </>
      )}
      <Button
        variant="outline"
        size="sm"
        onClick={() => openUrl(`${WEBSITE_URL}/changelog/`).catch(toastError)}
      >
        {t('updates.whatsNew')}
      </Button>
      {method.kind === 'inApp' && (
        <Button size="sm" disabled={busy} onClick={install}>
          {progress ?? t('updates.install')}
        </Button>
      )}
      {method.kind === 'manual' && (
        <Button size="sm" onClick={() => openUrl(`${WEBSITE_URL}/download/`).catch(toastError)}>
          {t('updates.download')}
        </Button>
      )}
      {state.step === 'failed' && <ErrorMessage error={state.error} className="basis-full" />}
    </div>
  );
}
