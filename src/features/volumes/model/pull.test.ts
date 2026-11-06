import { initialPull, pullPercent, reducePull } from './pull';

describe('helper image pull', () => {
  it('adds up the layers, and is complete when done', () => {
    let state = reducePull(initialPull, {
      type: 'progress',
      id: null,
      status: 'Pulling from library/busybox',
      current: null,
      total: null,
    });
    expect(pullPercent(state)).toBeNull();
    state = reducePull(state, {
      type: 'progress',
      id: 'a',
      status: 'Downloading',
      current: 500,
      total: 1000,
    });
    state = reducePull(state, {
      type: 'progress',
      id: 'b',
      status: 'Downloading',
      current: 0,
      total: 1000,
    });
    expect(pullPercent(state)).toBe(25);
    state = reducePull(state, { type: 'done' });
    expect(pullPercent(state)).toBe(100);
  });
});
