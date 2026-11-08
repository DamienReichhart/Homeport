import { useQueryClient } from '@tanstack/react-query';
import { useNavigate } from 'react-router-dom';
import type { NavEntry } from '@/core/feature';
import { hostKeys, useHosts, useHostSelection } from '@/core/host';
import { useShortcut } from '@/shared/hooks';
import { useNavEntries } from './navShortcuts';
import { t } from '@/shared/i18n';

/** The shortcuts that work on every page. */
export function AppShortcuts() {
  const navigate = useNavigate();
  const client = useQueryClient();
  const hosts = useHosts();
  const { hostId, select } = useHostSelection();
  const { shortcuts } = useNavEntries();

  const step = (by: number) => {
    const list = hosts.data ?? [];
    if (list.length < 2) return;
    const at = list.findIndex((h) => h.host.id === hostId);
    const next = list[(at + by + list.length) % list.length];
    if (next) select(next.host.id);
  };

  useShortcut({
    keys: 'Mod+,',
    label: t('app.shortcuts.settings'),
    group: t('app.shortcuts.general'),
    handler: () => void navigate('/settings'),
  });
  useShortcut({
    keys: 'Alt+ArrowUp',
    label: t('app.shortcuts.previousHost'),
    group: t('app.shortcuts.hosts'),
    handler: () => step(-1),
  });
  useShortcut({
    keys: 'Alt+ArrowDown',
    label: t('app.shortcuts.nextHost'),
    group: t('app.shortcuts.hosts'),
    handler: () => step(1),
  });
  useShortcut({
    keys: 'Mod+R',
    label: t('app.shortcuts.refresh'),
    group: t('app.shortcuts.hosts'),
    enabled: hostId !== null,
    handler: () => {
      void client.invalidateQueries({ queryKey: hostKeys.list });
      if (hostId) void client.invalidateQueries({ queryKey: hostKeys.scope(hostId) });
    },
  });
  return (
    <>
      {shortcuts.map(({ entry, keys }) => (
        <NavShortcut key={entry.path} entry={entry} keys={keys} />
      ))}
    </>
  );
}

function NavShortcut({ entry, keys }: { entry: NavEntry; keys: string }) {
  const navigate = useNavigate();
  useShortcut({
    keys,
    label: t('app.shortcuts.goTo', { page: t(entry.label) }),
    group: t('app.shortcuts.navigation'),
    handler: () => void navigate(entry.path),
  });
  return null;
}
