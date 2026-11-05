import { useQuery } from '@tanstack/react-query';
import { PencilIcon, PlusIcon, Trash2Icon, TriangleAlertIcon } from 'lucide-react';
import { useState } from 'react';
import { useCapabilities, type Registry } from '@/shared/api';
import {
  Alert,
  AlertDescription,
  AlertTitle,
  Button,
  Card,
  CardAction,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
  ErrorMessage,
  Spinner,
  useConfirm,
} from '@/shared/ui';
import { useRemoveRegistry } from '../api/mutations';
import { registryQueries } from '../api/queries';
import { registryLabel } from '../model/login';
import { RegistryDialog } from './RegistryDialog';
import { t } from '@/shared/i18n';

/** The Registries card in Settings: the stored logins and the way to add one. */
export function RegistriesCard() {
  const registries = useQuery(registryQueries.list());
  const capabilities = useCapabilities();
  const remove = useRemoveRegistry();
  const [dialog, confirm] = useConfirm();
  const [editing, setEditing] = useState<Registry | 'new' | null>(null);
  const keychain = capabilities.data?.keychain ?? true;

  return (
    <Card>
      <CardHeader>
        <CardTitle>{t('registries.title')}</CardTitle>
        <CardDescription>{t('registries.description')}</CardDescription>
        {keychain && (
          <CardAction>
            <Button variant="outline" size="sm" onClick={() => setEditing('new')}>
              <PlusIcon /> {t('registries.add')}
            </Button>
          </CardAction>
        )}
      </CardHeader>
      <CardContent className="grid gap-3">
        {!keychain && (
          <Alert variant="destructive">
            <TriangleAlertIcon />
            <AlertTitle>{t('registries.noKeychain.title')}</AlertTitle>
            <AlertDescription>{t('registries.noKeychain.description')}</AlertDescription>
          </Alert>
        )}
        {registries.isPending && <Spinner />}
        {registries.error && (
          <ErrorMessage error={registries.error} onRetry={() => void registries.refetch()} />
        )}
        {registries.data?.length === 0 && (
          <p className="text-muted-foreground text-sm">{t('registries.empty')}</p>
        )}
        {registries.data && registries.data.length > 0 && (
          <ul className="divide-border divide-y">
            {registries.data.map((r) => (
              <li key={r.id} className="flex items-center gap-2 py-2">
                <div className="min-w-0 flex-1">
                  <p className="truncate font-medium">{registryLabel(r.address)}</p>
                  <p className="text-muted-foreground truncate text-xs">{r.username}</p>
                </div>
                <Button
                  variant="ghost"
                  size="icon-sm"
                  aria-label={t('registries.editOne', { registry: r.address })}
                  disabled={!keychain}
                  onClick={() => setEditing(r)}
                >
                  <PencilIcon />
                </Button>
                <Button
                  variant="ghost"
                  size="icon-sm"
                  aria-label={t('registries.removeOne', { registry: r.address })}
                  onClick={() =>
                    confirm({
                      title: t('registries.remove.title', { registry: r.address }),
                      description: t('registries.remove.description', { registry: r.address }),
                      confirmLabel: t('common.remove'),
                      destructive: true,
                      onConfirm: () => remove.mutateAsync(r),
                    })
                  }
                >
                  <Trash2Icon />
                </Button>
              </li>
            ))}
          </ul>
        )}
      </CardContent>
      {editing && (
        <RegistryDialog
          registry={editing === 'new' ? null : editing}
          onClose={() => setEditing(null)}
        />
      )}
      {dialog}
    </Card>
  );
}
