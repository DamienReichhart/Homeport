import type { StackState } from '@/shared/api';
import type { Tone } from '@/shared/ui';

export const STATE_LABEL: Record<StackState, string> = {
  running: 'Running',
  partial: 'Partly running',
  stopped: 'Stopped',
  notDeployed: 'Not deployed',
};

export const STATE_TONE: Record<StackState, Tone> = {
  running: 'success',
  partial: 'warning',
  stopped: 'muted',
  notDeployed: 'info',
};
