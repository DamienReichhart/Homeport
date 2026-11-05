import type { DeployDiff } from '@/shared/api';
import { fileChange, reviewState } from './review';

const diff = (known: boolean, unchanged: boolean): DeployDiff => ({
  known,
  unchanged,
  deployedAt: known ? '2026-11-05T10:00:00.000Z' : null,
  files: [],
  reordered: false,
  env: [],
  deployedProfiles: [],
  candidateProfiles: [],
});

describe('reviewing a redeploy', () => {
  it('tells an unknown deploy, no change and changes apart', () => {
    expect(reviewState(diff(false, false))).toBe('unknown');
    expect(reviewState(diff(true, true))).toBe('unchanged');
    expect(reviewState(diff(true, false))).toBe('changes');
  });

  it('names what happened to each file', () => {
    expect(fileChange({ name: 'a', deployed: null, candidate: 'x' })).toBe('added');
    expect(fileChange({ name: 'a', deployed: 'x', candidate: null })).toBe('removed');
    expect(fileChange({ name: 'a', deployed: 'x', candidate: 'y' })).toBe('changed');
    expect(fileChange({ name: 'a', deployed: 'x', candidate: 'x' })).toBe('unchanged');
  });
});
