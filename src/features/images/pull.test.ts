import { initialPull, reducePull } from './pull';

describe('pull progress', () => {
  it('tracks each layer and the overall messages', () => {
    let s = reducePull(initialPull, {
      type: 'progress',
      id: 'alpine',
      status: 'Pulling from library/alpine',
      current: null,
      total: null,
    });
    s = reducePull(s, {
      type: 'progress',
      id: '0123456789ab',
      status: 'Downloading',
      current: 10,
      total: 100,
    });
    s = reducePull(s, {
      type: 'progress',
      id: '0123456789ab',
      status: 'Downloading',
      current: 60,
      total: 100,
    });
    s = reducePull(s, {
      type: 'progress',
      id: null,
      status: 'Digest: sha256:abc',
      current: null,
      total: null,
    });
    s = reducePull(s, { type: 'done' });
    expect(s.layers).toEqual([
      { id: '0123456789ab', status: 'Downloading', current: 60, total: 100 },
    ]);
    expect(s.messages).toEqual(['alpine: Pulling from library/alpine', 'Digest: sha256:abc']);
    expect(s.done).toBe(true);
  });

  it('keeps the failure message', () => {
    expect(reducePull(initialPull, { type: 'error', message: 'manifest unknown' }).error).toBe(
      'manifest unknown',
    );
  });
});
