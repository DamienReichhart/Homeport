import { PlayIcon, RotateCwIcon, SquareIcon, Trash2Icon } from 'lucide-react';
import { Button, type ConfirmOptions } from '@/shared/ui';
import { useBoardActions } from '../api/mutations';
import { idsByHost, type BoardRow } from '../model/board';
import { t } from '@/shared/i18n';

/** Start, stop, restart and remove for a selection that may span Hosts. */
export function BoardSelection({
  selected,
  onRemoved,
  confirm,
}: {
  selected: BoardRow[];
  onRemoved: () => void;
  confirm: (options: ConfirmOptions) => void;
}) {
  const { act, remove } = useBoardActions();
  const byHost = idsByHost(selected);
  const run = (action: 'start' | 'stop' | 'restart') => act.mutate({ byHost, action });
  return (
    <div
      role="toolbar"
      aria-label={t('board.selection')}
      className="ms-auto flex items-center gap-1.5"
    >
      <span className="text-muted-foreground text-sm">
        {t('containers.selected', { count: selected.length })}
      </span>
      <Button size="sm" variant="outline" onClick={() => run('start')}>
        <PlayIcon /> {t('containers.action.start')}
      </Button>
      <Button size="sm" variant="outline" onClick={() => run('stop')}>
        <SquareIcon /> {t('containers.action.stop')}
      </Button>
      <Button size="sm" variant="outline" onClick={() => run('restart')}>
        <RotateCwIcon /> {t('containers.action.restart')}
      </Button>
      <Button
        size="sm"
        variant="destructive"
        onClick={() =>
          confirm({
            title: t('board.remove.title', { count: selected.length }),
            description: <RemoveSummary selected={selected} />,
            confirmLabel: t('common.remove'),
            destructive: true,
            onConfirm: async () => {
              await remove.mutateAsync(byHost);
              onRemoved();
            },
          })
        }
      >
        <Trash2Icon /> {t('common.remove')}
      </Button>
    </div>
  );
}

/** What's removed, Host by Host. */
function RemoveSummary({ selected }: { selected: BoardRow[] }) {
  const hosts = new Map<string, string[]>();
  for (const row of selected) {
    hosts.set(row.hostName, [...(hosts.get(row.hostName) ?? []), row.container.name]);
  }
  return (
    <span className="grid gap-1">
      <span>{t('board.remove.description')}</span>
      {[...hosts].map(([host, names]) => (
        <span key={host}>{t('board.remove.onHost', { host, names: names.join(', ') })}</span>
      ))}
    </span>
  );
}
