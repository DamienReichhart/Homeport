import type { QueryClient } from '@tanstack/react-query';
import type { LucideIcon } from 'lucide-react';
import type { MessageKey } from '@/shared/i18n';

/** What a palette command can do when it runs. */
export interface CommandContext {
  /**
   * Opens `path`. With `action`, the page there starts it, as if its button
   * was pressed (see `useActionRequest`).
   */
  open: (path: string, action?: string) => void;
  /** The selected Host, `null` when there's none. */
  hostId: string | null;
  client: QueryClient;
}

/** An action a feature offers in the command palette. */
export interface PaletteCommand {
  /** Unique across features, such as `containers:run`. */
  id: string;
  label: MessageKey;
  /** The palette's heading it's listed under; Actions by default. */
  group?: MessageKey;
  icon?: LucideIcon;
  /** More words it's found by, in English. */
  keywords?: string[];
  /** Its keyboard shortcut, shown next to it (see `useShortcut`). */
  shortcut?: string;
  /** Hidden unless the selected Host is connected. */
  needsHost?: boolean;
  run: (ctx: CommandContext) => void | Promise<unknown>;
}
