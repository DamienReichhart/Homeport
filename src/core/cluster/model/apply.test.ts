import { describe, expect, it } from 'vitest';
import type { ApplyDocument } from '@/shared/api';
import { previewSummary } from './apply';

const doc = (action: ApplyDocument['action'], conflicts = 0): ApplyDocument => ({
  apiVersion: 'v1',
  kind: 'ConfigMap',
  namespace: 'shop',
  name: 'a',
  action,
  before: null,
  after: null,
  conflicts: Array.from({ length: conflicts }, () => ({ field: '.data.k', message: 'conflict' })),
  error: null,
});

describe('previewSummary', () => {
  it('is ready when every document applies and one changes something', () => {
    expect(previewSummary({ documents: [doc('created'), doc('unchanged')] })).toMatchObject({
      ready: true,
      changes: 1,
    });
  });

  it('has nothing to do when nothing changes', () => {
    expect(previewSummary({ documents: [doc('unchanged')] })).toMatchObject({
      ready: false,
      nothingToDo: true,
    });
  });

  it('can be forced only when conflicts are all that stand in the way', () => {
    expect(previewSummary({ documents: [doc('changed'), doc('failed', 1)] })).toMatchObject({
      ready: false,
      forceable: true,
    });
    expect(previewSummary({ documents: [doc('failed', 1), doc('failed')] }).forceable).toBe(false);
  });
});
