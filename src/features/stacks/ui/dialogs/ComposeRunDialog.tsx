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
import { ReviewChanges } from './ReviewChanges';
import { t } from '@/shared/i18n';

/**
 * Runs one compose command for a Managed or Linked Stack and shows its
 * output live, after showing what changes when the job asks for a review.
 */
export function ComposeRunDialog({
  hostId,
  job,
  onClose,
}: {
  hostId: string;
  job: ComposeJob | null;
  onClose: () => void;
}) {
  if (!job) return null;
  return <RunDialog hostId={hostId} job={job} onClose={onClose} />;
}

function RunDialog({
  hostId,
  job,
  onClose,
}: {
  hostId: string;
  job: ComposeJob;
  onClose: () => void;
}) {
  const [reviewed, setReviewed] = useState(job.review === undefined);
  return (
    <Dialog open onOpenChange={(o) => !o && onClose()}>
      <DialogContent className="sm:max-w-3xl">
        {reviewed ? (
          <Run hostId={hostId} job={job} onClose={onClose} />
        ) : (
          <ReviewChanges
            kind={job.kind}
            stackId={job.stackId}
            name={job.name}
            candidate={job.review ?? null}
            onDeploy={() => setReviewed(true)}
            onBack={onClose}
          />
        )}
      </DialogContent>
    </Dialog>
  );
}

function Run({ hostId, job, onClose }: { hostId: string; job: ComposeJob; onClose: () => void }) {
  const run = useComposeRun(hostId);
  const [result, setResult] = useState<'running' | 'ok' | 'failed'>('running');
  const [failure, setFailure] = useState<AppError | null>(null);
  const fail = (error: unknown) => {
    setFailure(toAppError(error));
    setResult('failed');
  };
  return (
    <>
      <DialogHeader>
        <DialogTitle>
          {t(job.rollbackTo ? 'stacks.compose.rollback' : COMPOSE_TITLES[job.action], {
            name: job.name,
          })}
        </DialogTitle>
      </DialogHeader>
      {job.images && job.images.length > 0 && (
        <p className="text-sm">{t('stacks.compose.updating', { images: job.images.join(', ') })}</p>
      )}
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
    </>
  );
}
