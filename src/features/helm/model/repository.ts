import type { MessageKey } from '@/shared/i18n';

/** Why a chart repository can't be added as typed, as a catalogue key; `null` when it can. */
export function repositoryProblem(name: string, url: string): MessageKey | null {
  const n = name.trim();
  const u = url.trim();
  if (!n) return 'helm.repos.problem.noName';
  if (!/^[A-Za-z0-9._-]+$/.test(n)) return 'helm.repos.problem.name';
  if (!u) return 'helm.repos.problem.noUrl';
  if (u.toLowerCase().startsWith('oci://')) return 'helm.repos.problem.oci';
  if (!/^https?:\/\/\S+$/i.test(u)) return 'helm.repos.problem.url';
  if (/^https?:\/\/[^/]*@/i.test(u)) return 'helm.repos.problem.credentials';
  return null;
}
