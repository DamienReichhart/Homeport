import { defineFeature } from '@/core/feature';
import { AddHostPage } from './components/AddHostPage';
import { HostDetailPage } from './components/HostDetailPage';
import { HostsPage } from './components/HostsPage';

// No sidebar entry: the Host switcher leads here.
export const hostsFeature = defineFeature({
  routes: [
    { path: 'hosts', Component: HostsPage },
    { path: 'hosts/new', Component: AddHostPage },
    { path: 'hosts/:hostId', Component: HostDetailPage },
  ],
});
