import type { LucideIcon } from 'lucide-react';
import type { RouteObject } from 'react-router-dom';
import type { MessageKey } from '@/shared/i18n';

/** A sidebar entry. */
export interface NavEntry {
  path: string;
  /** Its name in the sidebar, from the catalogue. */
  label: MessageKey;
  icon: LucideIcon;
}

/**
 * What a feature plugs into the shell: its pages, and its sidebar entry if
 * it has a top-level one. The app lists the manifests in
 * `src/app/router/features.ts`; nothing else needs editing.
 */
export interface FeatureManifest {
  routes: RouteObject[];
  nav?: NavEntry;
}

export const defineFeature = (manifest: FeatureManifest): FeatureManifest => manifest;
