import type { RepoAuth, Repository, RepositoryDraft } from '@/shared/api';
import { t } from '@/shared/i18n';

/** How a URL reaches its repository, by the backend's rules; `null` when it can't be used. */
export function transportOf(url: string): 'ssh' | 'https' | null {
  const u = url.trim();
  if (u === '' || /\s/.test(u)) return null;
  if (u.startsWith('https://')) return /^https:\/\/[^/@]+(@[^/]+)?\/.+/.test(u) ? 'https' : null;
  if (u.startsWith('ssh://')) return /^ssh:\/\/[^/-][^/]*\/.+/.test(u) ? 'ssh' : null;
  if (u.includes('://')) return null;
  // `[user@]host:path`, the form git takes for SSH without a scheme.
  const scp = /^(?:[^@/\\:-][^@/\\:]*@)?([A-Za-z0-9._-]{2,}):(?!\/\/)(.+)$/.exec(u);
  return scp && !scp[1]?.startsWith('-') ? 'ssh' : null;
}

/** What the add dialog holds. */
export interface RepositoryForm {
  url: string;
  auth: RepoAuth;
  username: string;
  token: string;
}

export const emptyRepository: RepositoryForm = {
  url: '',
  auth: 'sshKey',
  username: '',
  token: '',
};

/** The kinds of access a URL can use: a deploy key over SSH, a token or nothing over HTTPS. */
export function authsFor(url: string): RepoAuth[] {
  const transport = transportOf(url);
  if (transport === 'ssh') return ['sshKey'];
  if (transport === 'https') return ['httpsToken', 'none'];
  return ['sshKey', 'httpsToken', 'none'];
}

/** The form with its access switched to one its URL can use. */
export function withUrl(form: RepositoryForm, url: string): RepositoryForm {
  const allowed = authsFor(url);
  return { ...form, url, auth: allowed.includes(form.auth) ? form.auth : (allowed[0] ?? 'none') };
}

/** What's still missing before the form can be saved; `null` when complete. */
export function missingField(form: RepositoryForm): string | null {
  if (!form.url.trim()) return t('repositories.missing.url');
  if (transportOf(form.url) === null) return t('repositories.missing.validUrl');
  if (!authsFor(form.url).includes(form.auth)) return t('repositories.missing.auth');
  if (form.auth === 'httpsToken') {
    if (!form.username.trim()) return t('repositories.missing.username');
    if (!form.token.trim()) return t('repositories.missing.token');
  }
  return null;
}

export const toDraft = (form: RepositoryForm): RepositoryDraft => ({
  url: form.url.trim(),
  auth: form.auth,
  username: form.auth === 'httpsToken' ? form.username.trim() : null,
  token: form.auth === 'httpsToken' ? form.token.trim() : null,
});

/** `owner/repo` and its server, from any accepted URL. */
export function repositoryParts(url: string): { host: string; path: string } {
  const u = url.trim().replace(/\/+$/, '');
  const scheme = /^[a-z]+:\/\/(?:[^@/]+@)?([^/:]+|\[[^\]]+\])(?::\d+)?\/(.+)$/.exec(u);
  if (scheme) return { host: scheme[1] ?? '', path: (scheme[2] ?? '').replace(/\.git$/, '') };
  const scp = /^(?:[^@]+@)?([^:]+):\/?(.+)$/.exec(u);
  if (scp) return { host: scp[1] ?? '', path: (scp[2] ?? '').replace(/\.git$/, '') };
  return { host: '', path: u };
}

/** How a repository is named in lists: `owner/repo`. */
export const repositoryLabel = (repository: Pick<Repository, 'url'>) =>
  repositoryParts(repository.url).path;

/** How Homeport reaches it, for a list's second line. */
export function describeAuth(repository: Repository): string {
  const { host } = repositoryParts(repository.url);
  switch (repository.auth) {
    case 'sshKey':
      return t('repositories.auth.withKey', { host });
    case 'httpsToken':
      return t('repositories.auth.withToken', { host, username: repository.username ?? '' });
    case 'none':
      return t('repositories.auth.public', { host });
  }
}
