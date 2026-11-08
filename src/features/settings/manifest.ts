import { MonitorIcon, MoonIcon, SettingsIcon, SunIcon } from 'lucide-react';
import { defineFeature, type PaletteCommand } from '@/core/feature';
import type { Theme } from '@/shared/lib';
import type { MessageKey } from '@/shared/i18n';
import { setTheme } from './api/mutations';
import { SettingsPage } from './ui/SettingsPage';

const theme = (value: Theme, label: MessageKey, icon: PaletteCommand['icon']): PaletteCommand => ({
  id: `settings:theme:${value}`,
  label,
  group: 'settings.palette.group',
  icon,
  keywords: ['theme', 'appearance', value],
  run: (ctx) => setTheme(ctx.client, value),
});

export const settingsFeature = defineFeature({
  routes: [{ path: 'settings', Component: SettingsPage }],
  nav: { path: '/settings', label: 'settings.nav', icon: SettingsIcon, scope: 'app' },
  commands: [
    theme('light', 'settings.palette.themeLight', SunIcon),
    theme('dark', 'settings.palette.themeDark', MoonIcon),
    theme('system', 'settings.palette.themeSystem', MonitorIcon),
  ],
});
