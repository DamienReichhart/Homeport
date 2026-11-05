import { describe, expect, it } from 'vitest';
import { forgeFileUrl, forgeOf } from './forge';

describe('links to a file on its forge', () => {
  it('builds GitHub, GitLab and Gitea-style pages', () => {
    expect(forgeFileUrl('git@github.com:acme/shop.git', 'abc123', 'deploy/compose.yaml')).toBe(
      'https://github.com/acme/shop/blob/abc123/deploy/compose.yaml',
    );
    expect(forgeFileUrl('https://gitlab.com/group/sub/shop.git', 'abc123', 'compose.yaml')).toBe(
      'https://gitlab.com/group/sub/shop/-/blob/abc123/compose.yaml',
    );
    expect(forgeFileUrl('ssh://git@codeberg.org:22/acme/shop.git', 'abc123', 'compose.yaml')).toBe(
      'https://codeberg.org/acme/shop/src/commit/abc123/compose.yaml',
    );
    expect(forgeOf('https://gitea.example.com/acme/shop')).toBe('gitea');
    expect(forgeOf('git@gitlab.example.com:acme/shop.git')).toBe('gitlab');
  });

  it('encodes each part of the path, and gives up on unknown forges', () => {
    expect(forgeFileUrl('git@github.com:a/b.git', 'c', 'my app/compose.yaml')).toBe(
      'https://github.com/a/b/blob/c/my%20app/compose.yaml',
    );
    expect(forgeFileUrl('git@example.com:a/b.git', 'c', 'compose.yaml')).toBeNull();
  });
});
