import type { BuildSpec } from '@/shared/api';
import { t } from '@/shared/i18n';

/** What the Build dialog holds; lists are typed as text, one item per line. */
export interface BuildForm {
  context: string;
  dockerfile: string;
  /** Names for the result, separated by spaces, commas or new lines. */
  tags: string;
  /** `NAME=value`, one per line. */
  buildArgs: string;
  target: string;
  noCache: boolean;
  pull: boolean;
}

export const emptyBuild: BuildForm = {
  context: '',
  dockerfile: 'Dockerfile',
  tags: '',
  buildArgs: '',
  target: '',
  noCache: false,
  pull: false,
};

/** The spec the backend builds, or what to fix first. */
export function toBuildSpec(form: BuildForm): { spec: BuildSpec } | { problem: string } {
  const context = form.context.trim();
  if (!context) return { problem: t('images.buildForm.noFolder') };
  const buildArgs = [];
  for (const line of form.buildArgs.split('\n')) {
    if (!line.trim()) continue;
    const at = line.indexOf('=');
    const name = (at === -1 ? line : line.slice(0, at)).trim();
    if (!name) return { problem: t('images.buildForm.argNeedsName', { line: line.trim() }) };
    buildArgs.push({ name, value: at === -1 ? '' : line.slice(at + 1) });
  }
  return {
    spec: {
      context,
      dockerfile: form.dockerfile.trim() || 'Dockerfile',
      tags: form.tags.split(/[\s,]+/).filter(Boolean),
      buildArgs,
      target: form.target.trim() || null,
      noCache: form.noCache,
      pull: form.pull,
    },
  };
}
