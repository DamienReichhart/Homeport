import { useQueryClient } from '@tanstack/react-query';
import { useState } from 'react';
import {
  commands,
  keys,
  subscribe,
  toAppError,
  type AppError,
  type ComposeAction,
  type ComposeEvent,
} from '@/shared/api';
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

const TITLES: Record<ComposeAction, string> = {
  up: 'Deploying',
  pull: 'Pulling images for',
  down: 'Taking down',
  downWithVolumes: 'Taking down (with volumes)',
};

export interface ComposeJob {
  stackId: string;
  name: string;
  action: ComposeAction;
}

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
  const client = useQueryClient();
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
          <DialogTitle>
            {TITLES[job.action]} {job.name}
          </DialogTitle>
        </DialogHeader>
        {failure && <ErrorMessage error={failure} />}
        <div className="h-80">
          <XTerm
            readOnly
            className="h-full"
            onReady={({ terminal }) =>
              subscribe<ComposeEvent>(
                (ch) => commands.stacksComposeSubscribe(job.stackId, job.action, ch),
                (e) => {
                  if (e.type === 'line') terminal.writeln(e.text);
                  else if (e.type === 'error') {
                    terminal.writeln(`\x1b[31m${e.error.message}\x1b[39m`);
                    fail(e.error);
                  } else {
                    terminal.writeln(
                      e.success
                        ? '\x1b[32m✓ Done\x1b[39m'
                        : `\x1b[31m✗ Failed with exit code ${e.code ?? '?'}\x1b[39m`,
                    );
                    if (e.error) fail(e.error);
                    else setResult(e.success ? 'ok' : 'failed');
                    void client.invalidateQueries({ queryKey: keys.host(hostId) });
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
            {result === 'running' ? 'Cancel' : 'Close'}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
