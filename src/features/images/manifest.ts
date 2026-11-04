import { LayersIcon } from 'lucide-react';
import { defineFeature } from '@/core/feature';
import { ImagesPage } from './ui/ImagesPage';

export const imagesFeature = defineFeature({
  routes: [{ path: 'images', Component: ImagesPage }],
  nav: { path: '/images', label: 'Images', icon: LayersIcon },
});
