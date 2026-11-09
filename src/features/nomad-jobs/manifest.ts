import { BriefcaseIcon } from 'lucide-react';
import { defineFeature } from '@/core/feature';
import { AllocationPage } from './ui/allocation/AllocationPage';
import { JobPage } from './ui/JobPage';
import { JobsPage } from './ui/JobsPage';

export const nomadJobsFeature = defineFeature({
  routes: [
    { path: 'nomad/jobs', Component: JobsPage },
    { path: 'nomad/jobs/:namespace/:jobId', Component: JobPage },
    { path: 'nomad/allocations/:allocId/:tab?', Component: AllocationPage },
  ],
  nav: { path: '/nomad/jobs', label: 'nomad.jobs.nav', icon: BriefcaseIcon, scope: 'cluster' },
  clusterKind: 'nomad',
});
