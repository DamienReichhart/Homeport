import { commands, type Settings_Deserialize } from '@/shared/api';

export const settingsApi = {
  get: () => commands.getSettings(),
  update: (settings: Settings_Deserialize) => commands.updateSettings(settings),
};
