import { defineFeature } from '@/core/feature';
import { SwarmPage } from './ui/page/SwarmPage';

// No sidebar entry of its own: the sidebar lists each Swarm (SwarmNav).
export const swarmFeature = defineFeature({
  routes: [{ path: 'swarms/:swarmId/:tab?', Component: SwarmPage }],
});
