import { RefreshCwIcon } from 'lucide-react';
import { defineFeature } from '@/core/feature';
import { checkForUpdatesNow } from './api/queries';

// No pages: the updates live in Settings and the bar at the top.
export const updatesFeature = defineFeature({
  routes: [],
  commands: [
    {
      id: 'updates:check',
      label: 'updates.palette.check',
      icon: RefreshCwIcon,
      keywords: ['upgrade', 'version', 'new version'],
      run: async (ctx) => {
        ctx.open('/settings');
        await checkForUpdatesNow(ctx.client);
      },
    },
  ],
});
