import { repositoryParts } from './repository';

/** The forges whose web pages Homeport can link a file to. */
export type Forge = 'github' | 'gitlab' | 'gitea';

/** The forge a repository URL is on, from its server's name; `null` when unknown. */
export function forgeOf(url: string): Forge | null {
  const host = repositoryParts(url).host.toLowerCase();
  if (host === 'github.com' || host.endsWith('.github.com') || host.startsWith('github.'))
    return 'github';
  if (host === 'gitlab.com' || host.startsWith('gitlab.') || host.includes('.gitlab.'))
    return 'gitlab';
  if (
    host === 'codeberg.org' ||
    /(^|\.)(gitea|forgejo)\./.test(`${host}.`) ||
    host.startsWith('git.')
  )
    return 'gitea';
  return null;
}

/**
 * The web page of `path` at `commit` on the repository's forge: GitHub's
 * `blob/`, GitLab's `-/blob/`, Gitea and Forgejo's `src/commit/`. `null` for a
 * forge Homeport doesn't know.
 */
export function forgeFileUrl(url: string, commit: string, path: string): string | null {
  const forge = forgeOf(url);
  if (!forge) return null;
  const { host, path: repo } = repositoryParts(url);
  const file = path.split('/').map(encodeURIComponent).join('/');
  const base = `https://${host}/${repo}`;
  switch (forge) {
    case 'github':
      return `${base}/blob/${commit}/${file}`;
    case 'gitlab':
      return `${base}/-/blob/${commit}/${file}`;
    case 'gitea':
      return `${base}/src/commit/${commit}/${file}`;
  }
}
