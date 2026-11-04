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
        <CardTitle>Registries</CardTitle>
        <CardDescription>
          Logins for container registries, used to pull private images, push and build on every
          Host.
        </CardDescription>
        {keychain && (
          <CardAction>
            <Button variant="outline" size="sm" onClick={() => setEditing('new')}>
              <PlusIcon /> Add login
            </Button>
          </CardAction>
        )}
      </CardHeader>
      <CardContent className="grid gap-3">
        {!keychain && (
          <Alert variant="destructive">
            <TriangleAlertIcon />
            <AlertTitle>No OS keychain</AlertTitle>
            <AlertDescription>
              Registry passwords and tokens are encrypted with a key kept in the OS keychain, and
              none is available. Images can still be pulled anonymously.
            </AlertDescription>
          </Alert>
        )}
        {registries.isPending && <Spinner />}
        {registries.error && (
          <ErrorMessage error={registries.error} onRetry={() => void registries.refetch()} />
        )}
        {registries.data?.length === 0 && (
          <p className="text-muted-foreground text-sm">
            No logins yet: images are pulled anonymously.
          </p>
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
                  aria-label={`Edit the login for ${r.address}`}
                  disabled={!keychain}
                  onClick={() => setEditing(r)}
                >
                  <PencilIcon />
                </Button>
                <Button
                  variant="ghost"
                  size="icon-sm"
                  aria-label={`Remove the login for ${r.address}`}
                  onClick={() =>
                    confirm({
                      title: `Remove the login for ${r.address}?`,
                      description: `Images from ${r.address} will be pulled anonymously until a login is added again.`,
                      confirmLabel: 'Remove',
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
