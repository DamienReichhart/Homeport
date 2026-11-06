import { LayoutGridIcon } from 'lucide-react';
import { defineFeature } from '@/core/feature';
import { BoardPage } from './ui/BoardPage';

// Not behind HostGate: it shows every Host at once.
export const boardFeature = defineFeature({
  routes: [{ path: 'board', Component: BoardPage }],
  nav: { path: '/board', label: 'board.nav', icon: LayoutGridIcon },
});
