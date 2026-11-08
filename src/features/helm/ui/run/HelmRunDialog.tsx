import { useState } from 'react';
import { toAppError, type AppError } from '@/shared/api';
import {
  Button,
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  ErrorMessage,
  XTerm,
} from '@/shared/ui';
import type { HelmRun } from '../../api/streams';
import { t } from '@/shared/i18n';

/**
 * Runs one helm command and shows its output live, then how it ended.
 * Closing while it runs cancels it; `onDone` hears whether it succeeded.
 */
export function HelmRunDialog({
  title,
  run,
  onDone,
  onClose,
}: {
  title: string;
  run: HelmRun;
  onDone?: (success: boolean) => void;
  onClose: () => void;
}) {
  const [result, setResult] = useState<'running' | 'ok' | 'failed'>('running');
  const [failure, setFailure] = useState<AppError | null>(null);
  const fail = (error: unknown) => {
    setFailure(toAppError(error));
    setResult('failed');
    onDone?.(false);
  };
  return (
    <Dialog open onOpenChange={(o) => !o && onClose()}>
      <DialogContent className="sm:max-w-3xl">
        <DialogHeader>
          <DialogTitle>{title}</DialogTitle>
        </DialogHeader>
        {failure && <ErrorMessage error={failure} />}
        <div className="h-80">
          <XTerm
            readOnly
            className="h-full"
            onReady={({ terminal }) =>
              run(
                (e) => {
                  if (e.type === 'line') terminal.writeln(e.text);
                  else if (e.type === 'error') {
                    terminal.writeln(`\x1b[31m${e.error.message}\x1b[39m`);
                    fail(e.error);
                  } else if (e.success) {
                    terminal.writeln(`\x1b[32m${t('helm.run.done')}\x1b[39m`);
                    setResult('ok');
                    onDone?.(true);
                  } else {
                    terminal.writeln(
                      `\x1b[31m${t('helm.run.failed', { code: e.code ?? '?' })}\x1b[39m`,
                    );
                    fail(e.error ?? new Error(t('helm.run.failed', { code: e.code ?? '?' })));
                  }
                },
                (error) => {
                  terminal.writeln(`\x1b[31m${error.message}\x1b[39m`);
                  fail(error);
                },
              )
            }
          />
        </div>
        <DialogFooter>
          <Button variant={result === 'running' ? 'outline' : 'default'} onClick={onClose}>
            {result === 'running' ? t('common.cancel') : t('common.close')}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
