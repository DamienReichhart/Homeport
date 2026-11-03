import { defineFeature } from '@/core/feature';
import { AddHostPage } from './ui/AddHostPage';
import { HostDetailPage } from './ui/detail/HostDetailPage';
import { HostsPage } from './ui/HostsPage';

// No sidebar entry: the Host switcher leads here.
export const hostsFeature = defineFeature({
  routes: [
    { path: 'hosts', Component: HostsPage },
    { path: 'hosts/new', Component: AddHostPage },
    { path: 'hosts/:hostId', Component: HostDetailPage },
  ],
});
