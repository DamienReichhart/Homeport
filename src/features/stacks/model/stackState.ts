import type { StackState } from '@/shared/api';
import type { Tone } from '@/shared/ui';
import type { MessageKey } from '@/shared/i18n';

export const STATE_LABEL: Record<StackState, MessageKey> = {
  running: 'stacks.state.running',
  partial: 'stacks.state.partial',
  stopped: 'stacks.state.stopped',
  notDeployed: 'stacks.state.notDeployed',
};

export const STATE_TONE: Record<StackState, Tone> = {
  running: 'success',
  partial: 'warning',
  stopped: 'muted',
  notDeployed: 'info',
};
