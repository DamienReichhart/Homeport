import type { LinkedFiles } from '@/shared/api';
import { changedOutside, dirtyFiles, fileName } from './linkedEdit';

const files = (fingerprints: [string, string], changed = false): LinkedFiles => ({
  workingDir: '/srv/site',
  fingerprint: 'all',
  changedOutside: changed,
  files: [
    { path: '/srv/site/compose.yaml', content: 'a', fingerprint: fingerprints[0] },
    { path: '/srv/site/compose.prod.yaml', content: 'b', fingerprint: fingerprints[1] },
  ],
});

describe('editing a Linked Stack', () => {
  it('knows which files were edited', () => {
    const base = files(['1', '2']);
    expect(dirtyFiles(base, {})).toEqual([]);
    expect(dirtyFiles(base, { '/srv/site/compose.prod.yaml': 'b' })).toEqual([]);
    expect(dirtyFiles(base, { '/srv/site/compose.prod.yaml': 'b2' }).map((f) => f.path)).toEqual([
      '/srv/site/compose.prod.yaml',
    ]);
  });

  it('notices a file changed on disk since the edit started', () => {
    const base = files(['1', '2']);
    expect(changedOutside(base, files(['1', '2']))).toBe(false);
    expect(changedOutside(base, files(['1', '3']))).toBe(true);
    expect(changedOutside(base, undefined)).toBe(false);
  });

  it('notices files that changed before Homeport opened them', () => {
    expect(changedOutside(files(['1', '2'], true), undefined)).toBe(true);
    expect(changedOutside(files(['1', '2']), files(['1', '2'], true))).toBe(true);
  });

  it('names files by their last path segment', () => {
    expect(fileName('/srv/site/compose.yaml')).toBe('compose.yaml');
    expect(fileName('C:\\site\\compose.yaml')).toBe('compose.yaml');
  });
});
