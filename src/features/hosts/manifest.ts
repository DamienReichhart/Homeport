import { PlusIcon, ServerCogIcon } from 'lucide-react';
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
  commands: [
    {
      id: 'hosts:add',
      label: 'hosts.palette.add',
      icon: PlusIcon,
      keywords: ['new', 'connect', 'ssh', 'engine'],
      run: (ctx) => ctx.open('/hosts/new'),
    },
    {
      id: 'hosts:manage',
      label: 'hosts.palette.manage',
      icon: ServerCogIcon,
      keywords: ['hosts', 'export', 'import'],
      run: (ctx) => ctx.open('/hosts'),
    },
  ],
});
