import { DownloadIcon, LayersIcon } from 'lucide-react';
import { defineFeature } from '@/core/feature';
import { ImagesPage } from './ui/ImagesPage';

export const containerdImagesFeature = defineFeature({
  requires: 'containerd',
  routes: [{ path: 'containerd/images', Component: ImagesPage }],
  nav: {
    path: '/containerd/images',
    label: 'containerd.images.nav',
    icon: LayersIcon,
    scope: 'host',
  },
  commands: [
    {
      id: 'containerd-images:pull',
      label: 'containerd.images.pull',
      icon: DownloadIcon,
      keywords: ['containerd', 'image', 'pull'],
      needsHost: true,
      run: (ctx) => ctx.open('/containerd/images', 'pull'),
    },
  ],
});
