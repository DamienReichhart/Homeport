import { ActivityIcon } from 'lucide-react';
import { defineFeature } from '@/core/feature';
import { EventsPage } from './ui/EventsPage';

export const eventsFeature = defineFeature({
  routes: [{ path: 'events', Component: EventsPage }],
  nav: { path: '/events', label: 'Events', icon: ActivityIcon },
});
