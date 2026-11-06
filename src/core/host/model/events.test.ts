import type { EngineEvent } from '@/shared/api';
import { appendEvent, cursorOf, EVENT_PAGE } from './events';

const event = (timeNano: number, action = 'start'): EngineEvent => ({
  time: Math.floor(timeNano / 1e9),
  timeNano,
  kind: 'container',
  action,
  actorId: 'abc',
  attributes: {},
});

describe('event history', () => {
  it('appends new events once', () => {
    const list = [event(1), event(2)];
    expect(appendEvent(list, event(2))).toBe(list);
    expect(appendEvent(list, event(2, 'die')).map((e) => e.action)).toEqual([
      'start',
      'start',
      'die',
    ]);
  });

  it('keeps the latest page', () => {
    const full = Array.from({ length: EVENT_PAGE }, (_, i) => event(i));
    const next = appendEvent(full, event(EVENT_PAGE));
    expect(next).toHaveLength(EVENT_PAGE);
    expect(next[0]?.timeNano).toBe(1);
  });

  it('starts older pages at an event', () => {
    expect(cursorOf(event(5))).toEqual({
      timeNano: 5,
      kind: 'container',
      action: 'start',
      actorId: 'abc',
    });
  });
});
