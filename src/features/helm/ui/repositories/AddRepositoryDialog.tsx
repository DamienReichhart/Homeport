import { useState } from 'react';
import {
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
} from '@/shared/ui';
import { useAddRepository } from '../../api/mutations';
import { repositoryProblem } from '../../model/repository';
import { t } from '@/shared/i18n';

/** Adds a public chart repository to helm's list on this machine. */
export function AddRepositoryDialog({ onClose }: { onClose: () => void }) {
  const [name, setName] = useState('');
  const [url, setUrl] = useState('');
  const [touched, setTouched] = useState(false);
  const add = useAddRepository(onClose);
  const problem = repositoryProblem(name, url);
  return (
    <Dialog open onOpenChange={(o) => !o && !add.isPending && onClose()}>
      <DialogContent>
        <form
          className="grid gap-4"
          onSubmit={(e) => {
            e.preventDefault();
            setTouched(true);
            if (!problem) add.mutate({ name: name.trim(), url: url.trim() });
          }}
        >
          <DialogHeader>
            <DialogTitle>{t('helm.repos.addTitle')}</DialogTitle>
            <DialogDescription>{t('helm.repos.addDescription')}</DialogDescription>
          </DialogHeader>
          <div className="grid gap-2">
            <Label htmlFor="repo-name">{t('helm.repos.name')}</Label>
            <Input
              id="repo-name"
              value={name}
              placeholder="bitnami"
              autoFocus
              onChange={(e) => setName(e.target.value)}
            />
          </div>
          <div className="grid gap-2">
            <Label htmlFor="repo-url">{t('helm.repos.url')}</Label>
            <Input
              id="repo-url"
              value={url}
              placeholder="https://charts.bitnami.com/bitnami"
              onChange={(e) => setUrl(e.target.value)}
            />
          </div>
          {touched && problem && (
            <p role="alert" className="text-destructive text-sm">
              {t(problem)}
            </p>
          )}
          {add.error && <ErrorMessage error={add.error} />}
          <DialogFooter>
            <Button type="button" variant="outline" onClick={onClose} disabled={add.isPending}>
              {t('common.cancel')}
            </Button>
            <Button type="submit" disabled={add.isPending}>
              {t('helm.repos.add')}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}
