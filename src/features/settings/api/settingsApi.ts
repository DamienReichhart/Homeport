import { invokeCommand } from '@/shared/api';
import type { Settings } from '../types';

export const settingsApi = {
  get: () => invokeCommand<Settings>('get_settings'),
  update: (settings: Settings) => invokeCommand<Settings>('update_settings', { settings }),
};
