import type { Stack, StackOwner } from '@/shared/api';
import { Label, Switch } from '@/shared/ui';
import { useRegistryAuth } from '../api/mutations';
import { t } from '@/shared/i18n';

/**
 * A Swarm Stack's **Send my registry credentials**: `--with-registry-auth`,
 * so the nodes can pull private images with this computer's docker logins.
 */
export function RegistryAuthSwitch({ owner, stack }: { owner: StackOwner; stack: Stack }) {
  const set = useRegistryAuth(owner);
  if (!stack.id || stack.kind === 'external') return null;
  const id = stack.id;
  const kind = stack.kind;
  return (
    <div className="grid gap-1">
      <div className="flex items-center gap-2">
        <Switch
          id="registry-auth"
          checked={stack.registryAuth}
          disabled={set.isPending}
          onCheckedChange={(on) => set.mutate({ kind, id, on })}
        />
        <Label htmlFor="registry-auth">{t('stacks.registryAuth.label')}</Label>
      </div>
      <p className="text-muted-foreground text-xs">{t('stacks.registryAuth.description')}</p>
    </div>
  );
}
