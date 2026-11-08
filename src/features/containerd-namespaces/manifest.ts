import { FolderTreeIcon, PlusIcon } from 'lucide-react';
import { defineFeature } from '@/core/feature';
import { NamespacesPage } from './ui/NamespacesPage';

export const containerdNamespacesFeature = defineFeature({
  requires: 'containerd',
  routes: [{ path: 'containerd/namespaces', Component: NamespacesPage }],
  nav: {
    path: '/containerd/namespaces',
    label: 'containerd.namespaces.nav',
    icon: FolderTreeIcon,
    scope: 'host',
  },
  commands: [
    {
      id: 'containerd-namespaces:new',
      label: 'containerd.namespaces.new',
      icon: PlusIcon,
      keywords: ['containerd', 'namespace', 'create'],
      needsHost: true,
      run: (ctx) => ctx.open('/containerd/namespaces', 'new'),
    },
  ],
});
