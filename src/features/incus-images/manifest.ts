import { DiscIcon, DownloadIcon } from 'lucide-react';
import { defineFeature } from '@/core/feature';
import { ImagesPage } from './ui/ImagesPage';

export const incusImagesFeature = defineFeature({
  requires: 'incus',
  routes: [{ path: 'incus/images', Component: ImagesPage }],
  nav: { path: '/incus/images', label: 'incus.images.nav', icon: DiscIcon, scope: 'host' },
  commands: [
    {
      id: 'incus-images:get',
      label: 'incus.images.get',
      icon: DownloadIcon,
      keywords: ['incus', 'image', 'download', 'copy'],
      needsHost: true,
      run: (ctx) => ctx.open('/incus/images', 'get'),
    },
  ],
});
