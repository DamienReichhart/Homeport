import { useQueryClient } from '@tanstack/react-query';
import { KeyboardIcon } from 'lucide-react';
import { useMemo, useRef, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import type { PaletteCommand } from '@/core/feature';
import { useHosts, useHostSelection } from '@/core/host';
import { useShortcut } from '@/shared/hooks';
import {
  Command,
  CommandDialog,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
  CommandShortcut,
  Keys,
  StatusDot,
  toastError,
} from '@/shared/ui';
import { features } from '../router/features';
import { useOpenShortcutsHelp } from '../shortcuts/helpContext';
import { NAV_SHORTCUTS } from '../shortcuts/navShortcuts';
import {
  paletteItems,
  recentItems,
  remember,
  score,
  type PaletteContext,
  type PaletteItem,
} from './model/items';
import { t } from '@/shared/i18n';

const navKeys = new Map(NAV_SHORTCUTS.map(({ entry, keys }) => [entry.path, keys]));

/** The command palette, opened with Mod+K from any page. */
export function CommandPalette() {
  const [open, setOpen] = useState(false);
  const [query, setQuery] = useState('');
  // Kept for the session, most recent first.
  const [recent, setRecent] = useState<string[]>([]);
  const navigate = useNavigate();
  const client = useQueryClient();
  const hosts = useHosts();
  const { hostId, host, select } = useHostSelection();
  const openHelp = useOpenShortcutsHelp();
  // Opened from the keyboard, it has no trigger to give focus back to.
  const returnFocus = useRef<HTMLElement | null>(null);
  const show = (next: boolean) => {
    if (next && !open && document.activeElement instanceof HTMLElement) {
      returnFocus.current = document.activeElement;
    }
    setOpen(next);
    if (!next) setQuery('');
  };

  useShortcut({
    keys: 'Mod+K',
    label: t('app.palette.open'),
    group: t('app.shortcuts.general'),
    handler: () => show(!open),
  });

  const appCommands = useMemo<PaletteCommand[]>(
    () => [
      {
        id: 'app:shortcuts',
        label: 'app.shortcuts.help',
        icon: KeyboardIcon,
        keywords: ['keys', 'keyboard', 'help'],
        shortcut: '?',
        run: openHelp,
      },
    ],
    [openHelp],
  );
  const items = useMemo(
    () =>
      paletteItems({
        features,
        appCommands,
        hosts: hosts.data ?? [],
        selected: host,
        navKeys,
      }),
    [appCommands, hosts.data, host],
  );

  const ctx: PaletteContext = {
    open: (path, action) => void navigate(path, action ? { state: { action } } : undefined),
    hostId,
    client,
    selectHost: select,
  };
  const run = (item: PaletteItem) => {
    // What it runs decides where focus goes: a page, a dialog.
    returnFocus.current = null;
    show(false);
    setRecent((list) => remember(list, item.id));
    Promise.resolve(item.run(ctx)).catch((e: unknown) => toastError(e));
  };

  const groups = query.trim() ? matching(items, query) : browsing(items, recent);
  return (
    <CommandDialog
      open={open}
      onOpenChange={show}
      onCloseAutoFocus={(event) => {
        if (!returnFocus.current?.isConnected) return;
        event.preventDefault();
        returnFocus.current.focus();
        returnFocus.current = null;
      }}
      title={t('app.palette.title')}
      description={t('app.palette.description')}
    >
      {/* The items are filtered and ranked here, so pages lead and hosts match by name. */}
      <Command shouldFilter={false}>
        <CommandInput
          value={query}
          onValueChange={setQuery}
          placeholder={t('app.palette.placeholder')}
          aria-label={t('app.palette.placeholder')}
        />
        <CommandList>
          <CommandEmpty>{t('app.palette.empty')}</CommandEmpty>
          {groups.map(({ heading, items: list, prefix }) => (
            <CommandGroup key={heading} heading={heading}>
              {list.map((item) => (
                <PaletteRow key={item.id} item={item} value={`${prefix}${item.id}`} onRun={run} />
              ))}
            </CommandGroup>
          ))}
        </CommandList>
      </Command>
    </CommandDialog>
  );
}

function PaletteRow({
  item,
  value,
  onRun,
}: {
  item: PaletteItem;
  value: string;
  onRun: (item: PaletteItem) => void;
}) {
  const Icon = item.icon;
  return (
    <CommandItem value={value} onSelect={() => onRun(item)}>
      {Icon && <Icon />}
      <span className="truncate">{item.label}</span>
      {item.status && (
        <span className="text-muted-foreground inline-flex items-center gap-1.5 text-xs">
          <StatusDot tone={item.status.tone} />
          {item.status.label}
        </span>
      )}
      {item.shortcut && (
        <CommandShortcut>
          <Keys keys={item.shortcut} />
        </CommandShortcut>
      )}
    </CommandItem>
  );
}

interface Group {
  heading: string;
  items: PaletteItem[];
  /** Makes an item's value unique when it's listed twice (in Recent). */
  prefix: string;
}

/** Everything, grouped, with the recently used first. */
function browsing(items: PaletteItem[], recent: string[]): Group[] {
  const groups: Group[] = [];
  const last = recentItems(items, recent);
  if (last.length)
    groups.push({ heading: t('app.palette.recent'), items: last, prefix: 'recent:' });
  return [...groups, ...byGroup(items)];
}

/** The items matching `query`, best first, grouped in the order their best appears. */
function matching(items: PaletteItem[], query: string): Group[] {
  const ranked = items
    .map((item, i) => ({ item, i, score: score(item, query) }))
    .filter((r) => r.score > 0)
    .sort((a, b) => b.score - a.score || a.i - b.i)
    .map((r) => r.item);
  return byGroup(ranked);
}

function byGroup(items: PaletteItem[]): Group[] {
  const groups = new Map<string, PaletteItem[]>();
  for (const item of items) groups.set(item.group, [...(groups.get(item.group) ?? []), item]);
  return [...groups].map(([heading, list]) => ({ heading, items: list, prefix: '' }));
}
