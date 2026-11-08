import { defineFeature } from '@/core/feature';
import { SwarmPage } from './ui/page/SwarmPage';
import { ServiceDetailPage } from './ui/services/ServiceDetailPage';
import { SwarmStackDetailPage } from './ui/stacks/SwarmStackDetailPage';

// No sidebar entry of its own: the sidebar lists each Swarm (SwarmNav).
export const swarmFeature = defineFeature({
  routes: [
    { path: 'swarms/:swarmId/:tab?', Component: SwarmPage },
    { path: 'swarms/:swarmId/services/:serviceId/:tab?', Component: ServiceDetailPage },
    { path: 'swarms/:swarmId/stacks/:name', Component: SwarmStackDetailPage },
  ],
});
