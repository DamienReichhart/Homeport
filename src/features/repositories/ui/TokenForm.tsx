import { useState } from 'react';
import type { Repository } from '@/shared/api';
import { Button, ErrorMessage, Input, Label } from '@/shared/ui';
import { useSetToken } from '../api/mutations';
import { t } from '@/shared/i18n';

/** Replaces the user name and token of an HTTPS repository. */
export function TokenForm({
  repository,
  onSaved,
}: {
  repository: Repository;
  onSaved: () => void;
}) {
  const [username, setUsername] = useState(repository.username ?? '');
  const [token, setToken] = useState('');
  const save = useSetToken(repository.id, () => {
    setToken('');
    onSaved();
  });
  const ready = username.trim() !== '' && token.trim() !== '';
  return (
    <form
      className="grid gap-3 rounded-md border p-3"
      onSubmit={(e) => {
        e.preventDefault();
        if (ready) save.mutate({ username, token });
      }}
    >
      <span className="text-sm font-medium">{t('repositories.token.replace')}</span>
      <div className="grid gap-1.5">
        <Label htmlFor="repository-token-username">{t('repositories.dialog.username')}</Label>
        <Input
          id="repository-token-username"
          value={username}
          onChange={(e) => setUsername(e.target.value)}
          autoComplete="off"
          spellCheck={false}
        />
      </div>
      <div className="grid gap-1.5">
        <Label htmlFor="repository-token-new">{t('repositories.dialog.token')}</Label>
        <Input
          id="repository-token-new"
          type="password"
          value={token}
          onChange={(e) => setToken(e.target.value)}
          autoComplete="new-password"
        />
      </div>
      {save.error && <ErrorMessage error={save.error} />}
      <div>
        <Button type="submit" size="sm" disabled={!ready || save.isPending}>
          {t('repositories.token.save')}
        </Button>
      </div>
    </form>
  );
}
