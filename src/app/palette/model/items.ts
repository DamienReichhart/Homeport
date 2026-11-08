import type { LucideIcon } from 'lucide-react';
import { ServerIcon } from 'lucide-react';
import type { CommandContext, FeatureManifest, PaletteCommand } from '@/core/feature';
import { describeStatus } from '@/core/host';
import type { HostView } from '@/shared/api';
import type { Tone } from '@/shared/ui';
import { t } from '@/shared/i18n';

/** What the palette runs an item with: a command's context, and switching Host. */
export interface PaletteContext extends CommandContext {
  selectHost: (hostId: string) => void;
}

/** One row of the palette. */
export interface PaletteItem {
  id: string;
  /** Its heading, translated. */
  group: string;
  label: string;
  /** What it's found by besides its label. */
  keywords: string[];
  icon?: LucideIcon;
  shortcut?: string;
  /** A Host's status, next to its name. */
  status?: { label: string; tone: Tone };
  run: (ctx: PaletteContext) => void | Promise<unknown>;
}

export interface PaletteSource {
  features: FeatureManifest[];
  /** Commands of the shell itself, such as Show keyboard shortcuts. */
  appCommands: PaletteCommand[];
  hosts: HostView[];
  /** The selected Host, whose status hides commands that need it connected. */
  selected: HostView | null;
  /** The sidebar entries' keys, by path. */
  navKeys: Map<string, string>;
}

/** Every item the palette lists, in its groups' order: Go to, Hosts, then commands. */
export function paletteItems(source: PaletteSource): PaletteItem[] {
  const connected = source.selected?.status.state === 'connected';
  const goTo = t('app.palette.goTo');
  const nav = source.features.flatMap((f) => (f.nav ? [f.nav] : []));
  const items: PaletteItem[] = nav.map((entry) => ({
    id: `nav:${entry.path}`,
    group: goTo,
    label: t('app.shortcuts.goTo', { page: t(entry.label) }),
    keywords: [t(entry.label)],
    icon: entry.icon,
    shortcut: source.navKeys.get(entry.path),
    run: (ctx) => ctx.open(entry.path),
  }));
  const hostsGroup = t('app.palette.hosts');
  for (const view of source.hosts) {
    const status = describeStatus(view.status);
    items.push({
      id: `host:${view.host.id}`,
      group: hostsGroup,
      label: t('app.palette.switchTo', { name: view.host.name }),
      keywords: [view.host.name],
      icon: ServerIcon,
      status: { label: status.label, tone: status.tone },
      run: (ctx) => ctx.selectHost(view.host.id),
    });
  }
  const commands = [...source.features.flatMap((f) => f.commands ?? []), ...source.appCommands];
  for (const command of commands) {
    if (command.needsHost && !connected) continue;
    items.push({
      id: `command:${command.id}`,
      group: t(command.group ?? 'app.palette.actions'),
      label: t(command.label),
      keywords: command.keywords ?? [],
      icon: command.icon,
      shortcut: command.shortcut,
      run: command.run,
    });
  }
  return items;
}

/**
 * How well `item` matches `query`, from 0 (hidden) to 1: its label or a
 * keyword starting with it first, then a word in it starting with it, then
 * containing it. Pages win a tie, so "vol" opens Volumes.
 */
export function score(item: PaletteItem, query: string): number {
  const q = query.trim().toLowerCase();
  if (!q) return 1;
  const label = item.label.toLowerCase();
  const keywords = item.keywords.map((k) => k.toLowerCase());
  const best = Math.max(rank(label, q), ...keywords.map((k) => rank(k, q) * 0.95));
  if (best === 0) return 0;
  return item.id.startsWith('nav:') ? Math.min(1, best + 0.02) : best;
}

function rank(text: string, q: string): number {
  if (text.startsWith(q)) return 0.9;
  if (text.split(/[\s\-_/.:]+/).some((word) => word.startsWith(q))) return 0.8;
  if (text.includes(q)) return 0.5;
  return 0;
}

/** At most this many recently used items lead the palette while it's empty. */
export const RECENT_LIMIT = 5;

/** `ids` of recently run items, most recent first, with `id` moved to the front. */
export function remember(recent: string[], id: string): string[] {
  return [id, ...recent.filter((r) => r !== id)].slice(0, RECENT_LIMIT);
}

/** The recently used items that are still listed, most recent first. */
export function recentItems(items: PaletteItem[], recent: string[]): PaletteItem[] {
  const byId = new Map(items.map((item) => [item.id, item]));
  return recent.flatMap((id) => {
    const item = byId.get(id);
    return item ? [item] : [];
  });
}
