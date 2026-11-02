import type { Theme } from '@/shared/lib';

/** Mirrors `Settings` in src-tauri/src/services/settings.rs. */
export interface Settings {
  theme: Theme;
}
