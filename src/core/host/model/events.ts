import type { EngineEvent, EventCursor } from '@/shared/api';

/** How many events a page of the history holds, as the backend lists them. */
export const EVENT_PAGE = 1000;

/** What tells two events of a Host apart: when, what, and to what. */
export function eventKey(e: EngineEvent): string {
  return `${e.timeNano}|${e.kind}|${e.action}|${e.actorId}`;
}

/** Where the page of events older than `e` starts. */
export function cursorOf(e: EngineEvent): EventCursor {
  return { timeNano: e.timeNano, kind: e.kind, action: e.action, actorId: e.actorId };
}

/**
 * `list` with `event` appended, unless it's already there (a reconnection
 * asks the Engine again from the last kept event), keeping the latest page.
 */
export function appendEvent(list: EngineEvent[], event: EngineEvent): EngineEvent[] {
  const key = eventKey(event);
  if (list.some((e) => eventKey(e) === key)) return list;
  return [...list.slice(-(EVENT_PAGE - 1)), event];
}
