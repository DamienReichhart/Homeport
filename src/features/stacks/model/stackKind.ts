import type { StackKind } from '@/shared/api';
import type { MessageKey } from '@/shared/i18n';

/** The name of each kind of Stack, as badges and labels show it. */
export const KIND_LABEL: Record<StackKind, MessageKey> = {
  managed: 'stacks.kind.managed',
  linked: 'stacks.kind.linked',
  synced: 'stacks.kind.synced',
  external: 'stacks.kind.external',
};

/** The badge of each kind: Homeport holds Managed definitions, points at Linked and Synced ones. */
export const KIND_BADGE: Record<StackKind, 'default' | 'secondary' | 'outline'> = {
  managed: 'default',
  linked: 'secondary',
  synced: 'secondary',
  external: 'outline',
};
