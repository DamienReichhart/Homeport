import { useQuery } from '@tanstack/react-query';
import { useState } from 'react';
import { useCapabilities, type RepoAuth, type Repository } from '@/shared/api';
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
  ToggleGroup,
  ToggleGroupItem,
} from '@/shared/ui';
import { useCreateRepository } from '../api/mutations';
import { repositoryQueries } from '../api/queries';
import {
  authsFor,
  emptyRepository,
  missingField,
  repositoryLabel,
  toDraft,
  withUrl,
  type RepositoryForm,
} from '../model/repository';
import { RepositoryAccess } from './RepositoryAccess';
import { t } from '@/shared/i18n';

const AUTH_LABELS = {
  sshKey: 'repositories.dialog.authKey',
  httpsToken: 'repositories.dialog.authToken',
  none: 'repositories.dialog.authNone',
} as const satisfies Record<RepoAuth, string>;

/**
 * Adds a repository (`repositoryId` null), then shows how to give Homeport
 * access to it; or shows a stored one's access.
 */
export function RepositoryDialog({
  repositoryId,
  onClose,
  onAdded,
}: {
  repositoryId: string | null;
  onClose: () => void;
  onAdded?: (repository: Repository) => void;
}) {
  const [createdId, setCreatedId] = useState<string | null>(null);
  const id = repositoryId ?? createdId;
  const list = useQuery(repositoryQueries.list());
  const repository = list.data?.find((r) => r.id === id) ?? null;

  return (
    <Dialog open onOpenChange={(open) => !open && onClose()}>
      <DialogContent className="sm:max-w-lg">
        <DialogHeader>
          <DialogTitle>
            {repository ? repositoryLabel(repository) : t('repositories.dialog.addTitle')}
          </DialogTitle>
          <DialogDescription>
            {repository ? repository.url : t('repositories.dialog.description')}
          </DialogDescription>
        </DialogHeader>
        {repository ? (
          <>
            <RepositoryAccess repository={repository} />
            <DialogFooter>
              <Button onClick={onClose}>{t('common.close')}</Button>
            </DialogFooter>
          </>
        ) : (
          <AddRepository
            onCancel={onClose}
            onCreated={(created) => {
              setCreatedId(created.id);
              onAdded?.(created);
            }}
          />
        )}
      </DialogContent>
    </Dialog>
  );
}

function AddRepository({
  onCancel,
  onCreated,
}: {
  onCancel: () => void;
  onCreated: (repository: Repository) => void;
}) {
  const [form, setForm] = useState<RepositoryForm>(emptyRepository);
  const create = useCreateRepository(onCreated);
  const capabilities = useCapabilities();
  const keychain = capabilities.data?.keychain ?? true;
  const missing = missingField(form);
  const allowed = authsFor(form.url);

  return (
    <form
      className="grid gap-3"
      onSubmit={(e) => {
        e.preventDefault();
        if (!missing) create.mutate(toDraft(form));
      }}
    >
      <div className="grid gap-1.5">
        <Label htmlFor="repository-url">{t('repositories.dialog.url')}</Label>
        <Input
          id="repository-url"
          value={form.url}
          onChange={(e) => setForm((f) => withUrl(f, e.target.value))}
          // eslint-disable-next-line homeport/no-literal-ui-string -- example repository URLs
          placeholder="git@github.com:owner/repo.git, https://gitlab.com/owner/repo.git"
          autoFocus
          spellCheck={false}
        />
      </div>
      <div className="grid gap-1.5">
        <span className="text-sm font-medium">{t('repositories.dialog.auth')}</span>
        <ToggleGroup
          type="single"
          variant="outline"
          value={form.auth}
          onValueChange={(auth) => auth && setForm((f) => ({ ...f, auth: auth as RepoAuth }))}
          aria-label={t('repositories.dialog.auth')}
          className="justify-start"
        >
          {(['sshKey', 'httpsToken', 'none'] as const).map((auth) => (
            <ToggleGroupItem key={auth} value={auth} disabled={!allowed.includes(auth)}>
              {t(AUTH_LABELS[auth])}
            </ToggleGroupItem>
          ))}
        </ToggleGroup>
        <p className="text-muted-foreground text-xs">
          {form.auth === 'sshKey'
            ? t('repositories.dialog.authKeyHelp')
            : form.auth === 'httpsToken'
              ? t('repositories.dialog.authTokenHelp')
              : t('repositories.dialog.authNoneHelp')}
        </p>
      </div>
      {!keychain && form.auth !== 'none' && (
        <p className="text-destructive text-sm">{t('repositories.noKeychain')}</p>
      )}
      {form.auth === 'httpsToken' && (
        <>
          <div className="grid gap-1.5">
            <Label htmlFor="repository-username">{t('repositories.dialog.username')}</Label>
            <Input
              id="repository-username"
              value={form.username}
              onChange={(e) => setForm((f) => ({ ...f, username: e.target.value }))}
              autoComplete="off"
              spellCheck={false}
            />
          </div>
          <div className="grid gap-1.5">
            <Label htmlFor="repository-token">{t('repositories.dialog.token')}</Label>
            <Input
              id="repository-token"
              type="password"
              value={form.token}
              onChange={(e) => setForm((f) => ({ ...f, token: e.target.value }))}
              autoComplete="new-password"
            />
          </div>
        </>
      )}
      {create.error && <ErrorMessage error={create.error} />}
      <DialogFooter>
        <Button type="button" variant="ghost" onClick={onCancel}>
          {t('common.cancel')}
        </Button>
        <Button
          type="submit"
          disabled={missing !== null || create.isPending || (!keychain && form.auth !== 'none')}
        >
          {form.auth === 'sshKey'
            ? t('repositories.dialog.addWithKey')
            : t('repositories.dialog.add')}
        </Button>
      </DialogFooter>
    </form>
  );
}
