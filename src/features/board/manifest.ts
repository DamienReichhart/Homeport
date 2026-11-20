import { LayoutGridIcon } from 'lucide-react';
import { defineFeature } from '@/core/feature';
import { BoardPage } from './ui/BoardPage';
import { OldBoardLink } from './ui/OldBoardLink';

// The home page, not behind HostGate: it shows every Host and Cluster at once.
// `/board` is where it lived before 1.1; links to it still land here.
export const boardFeature = defineFeature({
  routes: [
    { index: true, Component: BoardPage },
    { path: 'board', Component: OldBoardLink },
  ],
  nav: { path: '/', label: 'board.nav', icon: LayoutGridIcon, scope: 'app' },
});
