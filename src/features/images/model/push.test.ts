import { describePush, initialPush, reducePush } from './push';

const progress = (status: string, current: number | null = null, total: number | null = null) =>
  ({ type: 'progress', id: null, status, current, total }) as const;

describe('push progress', () => {
  it('counts layers and keeps the image-level lines', () => {
    let s = reducePush(initialPush, progress('The push refers to repository [ghcr.io/org/app]'));
    s = reducePush(s, progress('Preparing'));
    s = reducePush(s, progress('Waiting'));
    s = reducePush(s, progress('Layer already exists'));
    s = reducePush(s, progress('Pushing', 10, 100));
    expect([s.current, s.total]).toEqual([10, 100]);
    s = reducePush(s, progress('Pushed'));
    s = reducePush(s, progress('v1: digest: sha256:abc size: 528'));
    s = reducePush(s, { type: 'done' });
    expect(s.messages).toEqual([
      'The push refers to repository [ghcr.io/org/app]',
      'v1: digest: sha256:abc size: 528',
    ]);
    expect(describePush(s)).toBe('1 layer pushed, 1 already in the registry');
    expect(s.done).toBe(true);
    expect(s.current).toBeNull();
  });

  it('keeps the failure', () => {
    const error = {
      code: 'registry_auth_required',
      message: 'ghcr.io refused the push.',
      hint: null,
      detail: null,
    } as const;
    expect(reducePush(initialPush, { type: 'error', error }).error).toMatchObject(error);
  });
});
