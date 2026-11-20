import type { StackKind } from '@/shared/api';
import type { MessageKey } from '@/shared/i18n';

/** The name of each kind of Stack, as badges and labels show it. */
export const KIND_LABEL: Record<StackKind, MessageKey> = {
  managed: 'stacks.kind.managed',
  linked: 'stacks.kind.linked',
  synced: 'stacks.kind.synced',
  external: 'stacks.kind.external',
};

/** What each kind means, for the badge's tooltip. */
export const KIND_HINT: Record<StackKind, MessageKey> = {
  managed: 'stacks.kind.managedHint',
  linked: 'stacks.kind.linkedHint',
  synced: 'stacks.kind.syncedHint',
  external: 'stacks.kind.externalHint',
};

/**
 * The badge of each kind. A kind is a fact, not a state that needs anyone,
 * so every badge is quiet (DESIGN.md): outline, and muted for External.
 */
export const KIND_BADGE: Record<StackKind, 'default' | 'secondary' | 'outline'> = {
  managed: 'outline',
  linked: 'outline',
  synced: 'outline',
  external: 'secondary',
};
