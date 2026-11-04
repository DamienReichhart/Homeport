import { useQuery } from '@tanstack/react-query';
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { resourceQueries } from '@/core/host';
import type { Container } from '@/shared/api';
import {
  Alert,
  AlertDescription,
  Button,
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  ErrorMessage,
  Input,
  Label,
  Switch,
  Textarea,
} from '@/shared/ui';
import { useCommitContainer } from '../../api/mutations';
import { emptyCommitForm, imageNamed, toCommitSpec, type CommitForm } from '../../model/commit';
import { Field } from './run/Field';
import { t } from '@/shared/i18n';

/** Commits a container's files and settings to a new image. */
export function CommitDialog({
  hostId,
  container,
  onClose,
}: {
  hostId: string;
  container: Container;
  onClose: () => void;
}) {
  const navigate = useNavigate();
  const [form, setForm] = useState<CommitForm>(() => emptyCommitForm(container.name));
  const images = useQuery(resourceQueries.images(hostId));
  const commit = useCommitContainer(hostId, container.id, onClose, () => navigate('/images'));
  const set = <K extends keyof CommitForm>(key: K, value: CommitForm[K]) =>
    setForm((f) => ({ ...f, [key]: value }));
  const existing = imageNamed(form.reference, images.data ?? []);

  return (
    <Dialog open onOpenChange={(open) => !open && !commit.isPending && onClose()}>
      <DialogContent className="sm:max-w-lg">
        <DialogHeader>
          <DialogTitle>{t('containers.commit.title', { name: container.name })}</DialogTitle>
          <DialogDescription>{t('containers.commit.description')}</DialogDescription>
        </DialogHeader>
        <form
          className="grid gap-3"
          onSubmit={(e) => {
            e.preventDefault();
            commit.mutate(toCommitSpec(form));
          }}
        >
          <Field
            id="commit-name"
            label={t('containers.commit.imageName')}
            hint={t('containers.commit.imageNameHint')}
          >
            <Input
              id="commit-name"
              value={form.reference}
              onChange={(e) => set('reference', e.target.value)}
              className="font-mono"
              autoFocus
            />
          </Field>
          {existing && (
            <Alert>
              <AlertDescription>
                {t('containers.commit.nameTaken', { name: form.reference.trim() })}
              </AlertDescription>
            </Alert>
          )}
          <div className="grid gap-3 sm:grid-cols-2">
            <Field id="commit-comment" label={t('containers.commit.message')}>
              <Input
                id="commit-comment"
                value={form.comment}
                onChange={(e) => set('comment', e.target.value)}
                placeholder={t('containers.commit.messagePlaceholder')}
              />
            </Field>
            <Field id="commit-author" label={t('containers.commit.author')}>
              <Input
                id="commit-author"
                value={form.author}
                onChange={(e) => set('author', e.target.value)}
                placeholder={t('common.optional')}
              />
            </Field>
          </div>
          <Field
            id="commit-changes"
            label={t('containers.commit.changes')}
            hint={t('containers.commit.changesHint')}
          >
            <Textarea
              id="commit-changes"
              value={form.changes}
              onChange={(e) => set('changes', e.target.value)}
              // eslint-disable-next-line homeport/no-literal-ui-string -- Dockerfile instructions
              placeholder={'ENV MODE=prod\nCMD ["nginx", "-g", "daemon off;"]'}
              className="h-20 font-mono text-xs"
              spellCheck={false}
            />
          </Field>
          <div className="grid gap-1">
            <div className="flex items-center gap-2">
              <Switch
                id="commit-pause"
                checked={form.pause}
                onCheckedChange={(pause) => set('pause', pause)}
              />
              <Label htmlFor="commit-pause">{t('containers.commit.pause')}</Label>
            </div>
            <p className="text-muted-foreground text-xs">
              {form.pause ? t('containers.commit.pauseOn') : t('containers.commit.pauseOff')}
            </p>
          </div>
          {commit.error && <ErrorMessage error={commit.error} />}
          <DialogFooter>
            <Button type="button" variant="ghost" onClick={onClose} disabled={commit.isPending}>
              {t('common.cancel')}
            </Button>
            <Button type="submit" disabled={commit.isPending || !form.reference.trim()}>
              {commit.isPending ? t('containers.commit.committing') : t('containers.commit.submit')}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}
