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
import { useComposeRun } from '../../api/streams';
import { COMPOSE_TITLES, type ComposeJob } from '../../model/composeJob';
import { t } from '@/shared/i18n';

/** Runs one compose command for a Managed Stack and shows its output live. */
export function ComposeRunDialog({
  hostId,
  job,
  onClose,
}: {
  hostId: string;
  job: ComposeJob | null;
  onClose: () => void;
}) {
  const run = useComposeRun(hostId);
  const [result, setResult] = useState<'running' | 'ok' | 'failed'>('running');
  const [failure, setFailure] = useState<AppError | null>(null);
  const fail = (error: unknown) => {
    setFailure(toAppError(error));
    setResult('failed');
  };
  if (!job) return null;
  return (
    <Dialog open onOpenChange={(o) => !o && onClose()}>
      <DialogContent className="sm:max-w-3xl">
        <DialogHeader>
          <DialogTitle>{t(COMPOSE_TITLES[job.action], { name: job.name })}</DialogTitle>
        </DialogHeader>
        {failure && <ErrorMessage error={failure} />}
        <div className="h-80">
          <XTerm
            readOnly
            className="h-full"
            onReady={({ terminal }) =>
              run(
                job,
                (e) => {
                  if (e.type === 'line') terminal.writeln(e.text);
                  else if (e.type === 'error') {
                    terminal.writeln(`\x1b[31m${e.error.message}\x1b[39m`);
                    fail(e.error);
                  } else {
                    terminal.writeln(
                      e.success
                        ? `\x1b[32m${t('stacks.compose.done')}\x1b[39m`
                        : `\x1b[31m${t('stacks.compose.failed', { code: e.code ?? '?' })}\x1b[39m`,
                    );
                    if (e.error) fail(e.error);
                    else setResult(e.success ? 'ok' : 'failed');
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
