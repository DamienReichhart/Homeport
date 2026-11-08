import type { LucideIcon } from 'lucide-react';
import type { RouteObject } from 'react-router-dom';
import type { MessageKey } from '@/shared/i18n';
import type { PaletteCommand } from './command';

/**
 * What a feature's pages are about: the selected Host, the selected Cluster,
 * or the app as a whole (shown whatever is selected).
 */
export type NavScope = 'host' | 'cluster' | 'app';

/** A sidebar entry. */
export interface NavEntry {
  path: string;
  /** Its name in the sidebar, from the catalogue. */
  label: MessageKey;
  icon: LucideIcon;
  /** The feature's pages are about this; the sidebar hides the others. */
  scope: NavScope;
}

/**
 * The entries the sidebar shows for a selection of `kind`: the app's always,
 * and those of the selected kind. With nothing selected, the Host pages show,
 * each explaining how to add one.
 */
export function visibleNavEntries(entries: NavEntry[], kind: 'host' | 'cluster' | null) {
  const selected = kind ?? 'host';
  return entries.filter((e) => e.scope === 'app' || e.scope === selected);
}

/** The scope of a feature's pages: its sidebar entry's, else the app's. */
export const featureScope = (feature: FeatureManifest): NavScope => feature.nav?.scope ?? 'app';

/**
 * What a feature plugs into the shell: its pages, its sidebar entry if it
 * has a top-level one, and its command palette actions. The app lists the
 * manifests in `src/app/router/features.ts`; nothing else needs editing.
 */
export interface FeatureManifest {
  routes: RouteObject[];
  nav?: NavEntry;
  commands?: PaletteCommand[];
}

export const defineFeature = (manifest: FeatureManifest): FeatureManifest => manifest;
