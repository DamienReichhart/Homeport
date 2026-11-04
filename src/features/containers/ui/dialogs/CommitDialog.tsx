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
          <DialogTitle>Commit {container.name} to an image</DialogTitle>
          <DialogDescription>
            The new image holds the container&apos;s files as they are now, including changes made
            inside it, and its settings. Volumes aren&apos;t included.
          </DialogDescription>
        </DialogHeader>
        <form
          className="grid gap-3"
          onSubmit={(e) => {
            e.preventDefault();
            commit.mutate(toCommitSpec(form));
          }}
        >
          <Field id="commit-name" label="Image name" hint="Like repo:tag.">
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
                An image is already named {form.reference.trim()}. The name moves to the new image;
                the old one keeps its other names, or becomes untagged.
              </AlertDescription>
            </Alert>
          )}
          <div className="grid gap-3 sm:grid-cols-2">
            <Field id="commit-comment" label="Message">
              <Input
                id="commit-comment"
                value={form.comment}
                onChange={(e) => set('comment', e.target.value)}
                placeholder="Before the upgrade"
              />
            </Field>
            <Field id="commit-author" label="Author">
              <Input
                id="commit-author"
                value={form.author}
                onChange={(e) => set('author', e.target.value)}
                placeholder="Optional"
              />
            </Field>
          </div>
          <Field
            id="commit-changes"
            label="Changes"
            hint="Dockerfile instructions, one per line: CMD, ENTRYPOINT, ENV, EXPOSE, LABEL, ONBUILD, USER, VOLUME or WORKDIR."
          >
            <Textarea
              id="commit-changes"
              value={form.changes}
              onChange={(e) => set('changes', e.target.value)}
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
              <Label htmlFor="commit-pause">Pause the container while committing</Label>
            </div>
            <p className="text-muted-foreground text-xs">
              {form.pause
                ? 'Its processes are frozen for the few seconds the commit takes, so files being written are consistent.'
                : 'It keeps running, so files being written may be caught half-way.'}
            </p>
          </div>
          {commit.error && <ErrorMessage error={commit.error} />}
          <DialogFooter>
            <Button type="button" variant="ghost" onClick={onClose} disabled={commit.isPending}>
              Cancel
            </Button>
            <Button type="submit" disabled={commit.isPending || !form.reference.trim()}>
              {commit.isPending ? 'Committing…' : 'Commit'}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}
