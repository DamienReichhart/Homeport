import { SlidersHorizontalIcon } from 'lucide-react';
import { defineFeature } from '@/core/feature';
import { ProfilePage } from './ui/ProfilePage';
import { ProfilesPage } from './ui/ProfilesPage';

export const incusProfilesFeature = defineFeature({
  requires: 'incus',
  routes: [
    { path: 'incus/profiles', Component: ProfilesPage },
    { path: 'incus/profiles/:name', Component: ProfilePage },
  ],
  nav: {
    path: '/incus/profiles',
    label: 'incus.profiles.nav',
    icon: SlidersHorizontalIcon,
    scope: 'host',
  },
});
