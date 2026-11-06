import type { SyncedStack } from '@/shared/api';
import { formatRelative } from '@/shared/lib';
import { ErrorMessage, Label, Switch } from '@/shared/ui';
import { useSetSync } from '../../api/synced';
import { t } from '@/shared/i18n';

/** A Synced Stack's polling and auto-deploy, and how its last check went. */
export function SyncSettings({ hostId, synced }: { hostId: string; synced: SyncedStack }) {
  const set = useSetSync(hostId, synced.id);
  return (
    <div className="grid gap-3">
      <div className="flex items-start gap-2">
        <Switch
          id="synced-poll"
          checked={synced.poll}
          disabled={set.isPending}
          onCheckedChange={(poll) => set.mutate({ poll, autoDeploy: poll && synced.autoDeploy })}
        />
        <div className="grid gap-0.5">
          <Label htmlFor="synced-poll">{t('stacks.syncSettings.poll')}</Label>
          <span className="text-muted-foreground text-xs">
            {synced.lastPolledAt
              ? t('stacks.syncSettings.lastChecked', {
                  when: formatRelative(Date.parse(synced.lastPolledAt) / 1000),
                })
              : t('stacks.syncSettings.notChecked')}
          </span>
        </div>
      </div>
      <div className="flex items-start gap-2">
        <Switch
          id="synced-auto-deploy"
          checked={synced.autoDeploy}
          disabled={!synced.poll || set.isPending}
          onCheckedChange={(autoDeploy) => set.mutate({ poll: synced.poll, autoDeploy })}
        />
        <div className="grid gap-0.5">
          <Label htmlFor="synced-auto-deploy">{t('stacks.syncSettings.autoDeploy')}</Label>
          <span className="text-muted-foreground text-xs">
            {t('stacks.syncSettings.autoDeployHelp')}
          </span>
        </div>
      </div>
      <p className="text-muted-foreground text-xs">{t('stacks.synced.whileOpen')}</p>
      {synced.lastPollError && <ErrorMessage error={synced.lastPollError} />}
      {synced.autoDeployError && (
        <div className="grid gap-1">
          <p className="text-sm">
            {t('stacks.syncSettings.autoDeployFailed', {
              commit: synced.autoDeployFailedCommit?.slice(0, 7) ?? '',
            })}
          </p>
          <ErrorMessage error={synced.autoDeployError} />
        </div>
      )}
      {set.error && <ErrorMessage error={set.error} />}
    </div>
  );
}
