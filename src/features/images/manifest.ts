import { DownloadIcon, LayersIcon } from 'lucide-react';
import { defineFeature } from '@/core/feature';
import { ImagesPage } from './ui/ImagesPage';

export const imagesFeature = defineFeature({
  routes: [{ path: 'images', Component: ImagesPage }],
  nav: { path: '/images', label: 'images.nav', icon: LayersIcon },
  commands: [
    {
      id: 'images:pull',
      label: 'images.palette.pull',
      icon: DownloadIcon,
      keywords: ['download', 'registry'],
      needsHost: true,
      run: (ctx) => ctx.open('/images', 'pull'),
    },
  ],
});
