import { describe, expect, it } from 'vitest';
import { nameFromRepository, refChoices, shortCommit } from './newSynced';

describe('a new Synced Stack', () => {
  it('is named after its repository the way Compose takes names', () => {
    expect(nameFromRepository('acme/Shop Front.git')).toBe('shop-front');
    expect(nameFromRepository('group/sub/api_v2')).toBe('api_v2');
  });

  it('offers the default branch first, and tags from last to first', () => {
    const refs = {
      branches: ['dev', 'main', 'release'],
      tags: ['v1', 'v2'],
      defaultBranch: 'main',
    };
    expect(refChoices(refs, 'branch')).toEqual(['main', 'dev', 'release']);
    expect(refChoices(refs, 'tag')).toEqual(['v2', 'v1']);
    expect(refChoices(undefined, 'branch')).toEqual([]);
  });

  it('shortens commits', () => {
    expect(shortCommit('0123456789abcdef')).toBe('0123456');
    expect(shortCommit(null)).toBe('');
  });
});
