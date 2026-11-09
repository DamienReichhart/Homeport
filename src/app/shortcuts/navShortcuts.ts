import { useMemo } from 'react';
import { visibleNavEntries, type HostCapability, type NavEntry } from '@/core/feature';
import { supports, useHostSelection } from '@/core/host';
import { useSelection } from '@/core/selection';
import { navEntries } from '../router/features';

export interface NavShortcut {
  entry: NavEntry;
  keys: string;
}

/** Mod+1…9 open the sidebar's entries in order. */
export const navShortcuts = (entries: NavEntry[]): NavShortcut[] =>
  entries.slice(0, 9).map((entry, i) => ({ entry, keys: `Mod+${i + 1}` }));

/** The sidebar's entries for the current selection, and their shortcuts. */
export function useNavEntries(): { entries: NavEntry[]; shortcuts: NavShortcut[] } {
  const kind = useSelection().selection?.kind ?? null;
  const { host } = useHostSelection();
  // With no Host selected, the Docker pages show, each explaining how to add one.
  const docker = !host || supports(host, 'docker');
  const podman = supports(host, 'podman');
  const containerd = supports(host, 'containerd');
  const incus = supports(host, 'incus');
  return useMemo(() => {
    const offered: Record<HostCapability, boolean> = { docker, podman, containerd, incus };
    const entries = visibleNavEntries(navEntries, kind, (c) => offered[c]);
    return { entries, shortcuts: navShortcuts(entries) };
  }, [kind, docker, podman, containerd, incus]);
}
