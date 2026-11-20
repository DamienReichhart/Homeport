import { SlidersHorizontalIcon } from 'lucide-react';
import { defineFeature, splitRoutes } from '@/core/feature';
import { ProfilePage } from './ui/ProfilePage';
import { ProfilesPage } from './ui/ProfilesPage';

export const incusProfilesFeature = defineFeature({
  requires: 'incus',
  routes: [
    splitRoutes('incus/profiles', ProfilesPage, [{ path: ':name', Component: ProfilePage }]),
  ],
  nav: {
    path: '/incus/profiles',
    label: 'incus.profiles.nav',
    icon: SlidersHorizontalIcon,
    scope: 'host',
  },
});
