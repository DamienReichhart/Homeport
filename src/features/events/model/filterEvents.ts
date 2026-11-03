import type { EngineEvent } from '@/shared/api';

/** The type filter's value that keeps every event. */
export const ALL_KINDS = 'all';

/** What an event is about, in words: the actor's name, else its short id. */
export function actorName(e: EngineEvent): string {
  return e.attributes.name ?? e.attributes.image ?? e.actorId.replace(/^sha256:/, '').slice(0, 12);
}

export function filterEvents(events: EngineEvent[], kind: string, query: string): EngineEvent[] {
  const q = query.trim().toLowerCase();
  return events.filter(
    (e) =>
      (kind === ALL_KINDS || e.kind === kind) &&
      (!q || e.action.toLowerCase().includes(q) || actorName(e).toLowerCase().includes(q)),
  );
}
