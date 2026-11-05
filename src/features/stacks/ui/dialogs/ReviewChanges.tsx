import { useQuery } from '@tanstack/react-query';
import type { Candidate, DeployDiff, SourceKind } from '@/shared/api';
import { formatDateTime } from '@/shared/lib';
import {
  Alert,
  AlertDescription,
  AlertTitle,
  Badge,
  Button,
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  ErrorMessage,
  Spinner,
  YamlDiff,
} from '@/shared/ui';
import { stackQueries } from '../../api/queries';
import { fileName } from '../../model/linkedEdit';
import { ENV_CHANGE_LABEL, FILE_CHANGE_LABEL, fileChange, reviewState } from '../../model/review';
import { t } from '@/shared/i18n';

/**
 * The review step before a redeploy, inside a dialog: the deployed
 * definition against the one about to be, with Deploy and Back.
 */
export function ReviewChanges({
  kind,
  stackId,
  name,
  candidate,
  onDeploy,
  onBack,
}: {
  kind: SourceKind;
  stackId: string;
  name: string;
  candidate: Candidate | null;
  onDeploy: () => void;
  onBack: () => void;
}) {
  const diff = useQuery(stackQueries.diff(kind, stackId, candidate));
  const state = diff.data ? reviewState(diff.data) : null;
  return (
    <>
      <DialogHeader>
        <DialogTitle>{t('stacks.review.title', { name })}</DialogTitle>
        {diff.data?.deployedAt && (
          <DialogDescription>
            {t('stacks.review.description', {
              when: formatDateTime(Date.parse(diff.data.deployedAt) / 1000),
            })}
          </DialogDescription>
        )}
      </DialogHeader>
      <div className="grid max-h-[60vh] gap-3 overflow-auto">
        {diff.isPending && <Spinner />}
        {diff.error && <ErrorMessage error={diff.error} />}
        {state === 'unchanged' && <p className="text-sm">{t('stacks.review.noChanges')}</p>}
        {state === 'unknown' && (
          <Alert>
            <AlertTitle>{t('stacks.review.unknownTitle')}</AlertTitle>
            <AlertDescription>{t('stacks.review.unknownDescription')}</AlertDescription>
          </Alert>
        )}
        {diff.data && state !== 'unchanged' && <Changes diff={diff.data} />}
      </div>
      <DialogFooter>
        <Button variant="outline" onClick={onBack}>
          {t('stacks.review.back')}
        </Button>
        <Button disabled={diff.isPending} onClick={onDeploy}>
          {state === 'unchanged' ? t('stacks.review.redeployAnyway') : t('stacks.review.deploy')}
        </Button>
      </DialogFooter>
    </>
  );
}

function Changes({ diff }: { diff: DeployDiff }) {
  const changedEnv = diff.env.filter((e) => e.change !== 'unchanged');
  const keptEnv = diff.env.length - changedEnv.length;
  const profilesChanged = diff.deployedProfiles.join() !== diff.candidateProfiles.join();
  const list = (profiles: string[]) =>
    profiles.length > 0 ? profiles.join(', ') : t('stacks.review.noProfile');
  return (
    <>
      {diff.reordered && <p className="text-sm">{t('stacks.review.reordered')}</p>}
      {diff.files.map((file) => {
        const change = diff.known ? fileChange(file) : 'added';
        const header = (
          <span className="flex items-center gap-2 text-sm">
            <span className="font-mono" title={file.name}>
              {fileName(file.name)}
            </span>
            {diff.known && <Badge variant="outline">{t(FILE_CHANGE_LABEL[change])}</Badge>}
          </span>
        );
        if (change === 'unchanged')
          return (
            <Collapsible key={file.name}>
              <CollapsibleTrigger className="hover:underline">{header}</CollapsibleTrigger>
              <CollapsibleContent className="mt-2">
                <YamlDiff
                  before={file.deployed ?? ''}
                  after={file.candidate ?? ''}
                  label={file.name}
                />
              </CollapsibleContent>
            </Collapsible>
          );
        return (
          <div key={file.name} className="grid gap-2">
            {header}
            {change === 'removed' ? (
              <p className="text-muted-foreground text-sm">{t('stacks.review.removedFile')}</p>
            ) : (
              <YamlDiff
                before={diff.known ? (file.deployed ?? '') : (file.candidate ?? '')}
                after={file.candidate ?? ''}
                label={file.name}
              />
            )}
          </div>
        );
      })}
      {changedEnv.length > 0 && (
        <div className="grid gap-1">
          <span className="text-sm font-medium">{t('stacks.review.environment')}</span>
          <ul className="grid gap-1 text-sm">
            {changedEnv.map((e) => (
              <li key={e.key} className="flex items-center gap-2">
                <span className="font-mono">{e.key}</span>
                <Badge variant="outline">{t(ENV_CHANGE_LABEL[e.change])}</Badge>
              </li>
            ))}
          </ul>
          {keptEnv > 0 && (
            <p className="text-muted-foreground text-xs">
              {t('stacks.review.envUnchanged', { count: keptEnv })}
            </p>
          )}
        </div>
      )}
      {diff.known && profilesChanged && (
        <p className="text-sm">
          {t('stacks.review.profiles', {
            before: list(diff.deployedProfiles),
            after: list(diff.candidateProfiles),
          })}
        </p>
      )}
    </>
  );
}
