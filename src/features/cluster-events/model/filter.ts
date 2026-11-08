import type { EventView } from '@/shared/api';

export type EventType = 'all' | 'Warning' | 'Normal';

/** The Events of `type`, in the backend's order (warnings first, newest first). */
export function byType(events: EventView[], type: EventType): EventView[] {
  return type === 'all' ? events : events.filter((e) => e.type === type);
}
