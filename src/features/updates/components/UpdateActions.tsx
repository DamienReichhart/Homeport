import { openUrl } from '@tauri-apps/plugin-opener';
import type { UpdateCheck } from '@/shared/api';
import { Button } from '@/shared/ui/button';
import { CopyButton } from '@/shared/ui';
import { describeProgress, useInstallUpdate, WEBSITE_URL } from '../api';

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
          <CopyButton text={method.command} label="Copy command" />
        </>
      )}
      <Button variant="outline" size="sm" onClick={() => void openUrl(`${WEBSITE_URL}/changelog/`)}>
        What’s new
      </Button>
      {method.kind === 'inApp' && (
        <Button size="sm" disabled={busy} onClick={install}>
          {progress ?? 'Install and restart'}
        </Button>
      )}
      {method.kind === 'manual' && (
        <Button size="sm" onClick={() => void openUrl(`${WEBSITE_URL}/download/`)}>
          Download
        </Button>
      )}
      {state.step === 'failed' && (
        <span role="alert" className="text-destructive text-xs">
          {state.error.message}
        </span>
      )}
    </div>
  );
}
