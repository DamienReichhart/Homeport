import { useEffect } from 'react';
import type { Commit, SyncReview } from '@/shared/api';
import { formatRelative } from '@/shared/lib';
import {
  Alert,
  AlertDescription,
  AlertTitle,
  Button,
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  ErrorMessage,
  Spinner,
  YamlDiff,
} from '@/shared/ui';
import { useSyncReview } from '../../api/synced';
import type { ComposeJob } from '../../model/composeJob';
import { shortCommit } from '../../model/newSynced';
import { t } from '@/shared/i18n';

/**
 * "Sync": fetches a Synced Stack's branch or tag, then shows the commits and
 * the changes to its compose file's folder before deploying the new commit.
 */
export function SyncDialog({
  stackId,
  name,
  onDeploy,
  onClose,
}: {
  stackId: string;
  name: string;
  onDeploy: (job: ComposeJob) => void;
  onClose: () => void;
}) {
  const sync = useSyncReview(stackId);
  const { mutate } = sync;
  useEffect(() => mutate(), [mutate]);
  const deploy = (review: SyncReview) => {
    onClose();
    onDeploy({
      kind: 'synced',
      stackId,
      name,
      action: review.builds ? 'rebuild' : 'up',
      commit: review.fetched.hash,
    });
  };
  return (
    <Dialog open onOpenChange={(o) => !o && onClose()}>
      <DialogContent className="sm:max-w-3xl">
        <DialogHeader>
          <DialogTitle>{t('stacks.sync.title', { name })}</DialogTitle>
          <DialogDescription>{t('stacks.synced.whileOpen')}</DialogDescription>
        </DialogHeader>
        <div className="grid max-h-[60vh] gap-3 overflow-auto">
          {sync.isPending && (
            <p className="text-muted-foreground flex items-center gap-2 text-sm">
              <Spinner /> {t('stacks.sync.fetching')}
            </p>
          )}
          {sync.error && <ErrorMessage error={sync.error} onRetry={() => mutate()} />}
          {sync.data && <Review review={sync.data} />}
        </div>
        <DialogFooter>
          <Button variant="ghost" onClick={onClose}>
            {t('common.cancel')}
          </Button>
          {sync.data && (
            <Button
              variant={sync.data.upToDate ? 'outline' : 'default'}
              onClick={() => sync.data && deploy(sync.data)}
            >
              {sync.data.upToDate
                ? t('stacks.sync.redeployAnyway')
                : t('stacks.sync.deploy', { commit: sync.data.fetched.shortHash })}
            </Button>
          )}
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}

function Review({ review }: { review: SyncReview }) {
  return (
    <>
      {review.tagMoved && (
        <Alert>
          <AlertTitle>{t('stacks.sync.tagMoved.title', { tag: review.stack.refName })}</AlertTitle>
          <AlertDescription>{t('stacks.sync.tagMoved.description')}</AlertDescription>
        </Alert>
      )}
      {review.upToDate ? (
        <p className="text-sm">
          {t('stacks.sync.upToDate', { commit: shortCommit(review.deployed?.hash) })}
        </p>
      ) : (
        <p className="text-sm">
          {review.deployed
            ? t('stacks.sync.from', {
                from: review.deployed.shortHash,
                to: review.fetched.shortHash,
              })
            : t('stacks.sync.first', { commit: review.fetched.shortHash })}
        </p>
      )}
      {review.commits.length > 0 && (
        <section className="grid gap-1">
          <h3 className="text-sm font-medium">{t('stacks.sync.commits')}</h3>
          <ul className="grid gap-1 text-sm">
            {review.commits.map((c) => (
              <CommitLine key={c.hash} commit={c} />
            ))}
          </ul>
        </section>
      )}
      {review.files.map((f) => (
        <section key={f.path} className="grid gap-1">
          <h3 className="font-mono text-xs">
            {f.path}{' '}
            <span className="text-muted-foreground font-sans">
              {f.before === null && !f.binary
                ? t('stacks.sync.added')
                : f.after === null && !f.binary
                  ? t('stacks.sync.deleted')
                  : ''}
            </span>
          </h3>
          {f.binary ? (
            <p className="text-muted-foreground text-xs">{t('stacks.sync.binary')}</p>
          ) : (
            <YamlDiff
              before={f.before ?? ''}
              after={f.after ?? ''}
              label={f.path}
              maxHeight="20rem"
            />
          )}
        </section>
      ))}
    </>
  );
}

function CommitLine({ commit }: { commit: Commit }) {
  return (
    <li className="flex gap-2">
      <span className="text-muted-foreground font-mono text-xs leading-5">{commit.shortHash}</span>
      <span className="min-w-0 flex-1 truncate">{commit.subject}</span>
      <span className="text-muted-foreground shrink-0 text-xs leading-5">
        {t('stacks.sync.by', {
          author: commit.author,
          when: formatRelative(Date.parse(commit.date) / 1000),
        })}
      </span>
    </li>
  );
}
