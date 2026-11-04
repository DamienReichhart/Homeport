import { SettingsIcon } from 'lucide-react';
import { defineFeature } from '@/core/feature';
import { SettingsPage } from './ui/SettingsPage';

export const settingsFeature = defineFeature({
  routes: [{ path: 'settings', Component: SettingsPage }],
  nav: { path: '/settings', label: 'Settings', icon: SettingsIcon },
});
