import { commands, type Settings } from '@/shared/api';

export const settingsApi = {
  get: () => commands.getSettings(),
  update: (settings: Settings) => commands.updateSettings(settings),
};
