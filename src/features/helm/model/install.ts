import type { MessageKey } from '@/shared/i18n';

/** Whether `chart` is an OCI reference, installed without a repository. */
export const isOci = (chart: string) => chart.trim().toLowerCase().startsWith('oci://');

/** A release name to suggest for `chart`: its last part, made valid. */
export function suggestedName(chart: string): string {
  const last = chart.trim().split('/').pop() ?? '';
  return last
    .toLowerCase()
    .replace(/[^a-z0-9-]+/g, '-')
    .replace(/^-+|-+$/g, '')
    .slice(0, 53);
}

const DNS_PART = /^[a-z0-9]([-a-z0-9]*[a-z0-9])?$/;

/** Why `name` isn't a release name helm takes, as a catalogue key; `null` when it is. */
export function releaseNameProblem(name: string): MessageKey | null {
  if (!name) return 'helm.install.problem.noName';
  if (name.length > 53) return 'helm.install.problem.nameLong';
  if (!name.split('.').every((p) => DNS_PART.test(p))) return 'helm.install.problem.name';
  return null;
}

/** Why `namespace` isn't a Namespace name, as a catalogue key; `null` when it is. */
export function namespaceProblem(namespace: string): MessageKey | null {
  if (!namespace) return 'helm.install.problem.noNamespace';
  if (namespace.length > 63 || !DNS_PART.test(namespace)) return 'helm.install.problem.namespace';
  return null;
}

/** Why `chart` can't be installed from Homeport, as a catalogue key; `null` when it can. */
export function ociProblem(chart: string): MessageKey | null {
  const c = chart.trim();
  if (!isOci(c) || !/^oci:\/\/[^\s/]+\/\S+$/i.test(c)) return 'helm.install.problem.oci';
  return null;
}

/** helm's `--timeout` from the minutes typed: `null` for helm's default. */
export function timeoutSeconds(minutes: string): number | null {
  const n = Number(minutes);
  return Number.isFinite(n) && n > 0 ? Math.round(n * 60) : null;
}
