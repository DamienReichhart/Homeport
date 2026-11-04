import type { AppError, UpdateCheck } from '@/shared/api';

/** Where the website lives; the changelog and download page are under it. */
export const WEBSITE_URL = 'https://damienreichhart.github.io/Homeport';

/** How often Homeport looks for a new version while it's open. */
export const CHECK_INTERVAL_MS = 6 * 60 * 60 * 1000;

export type InstallState =
  | { step: 'idle' }
  | { step: 'downloading'; percent: number | null }
  | { step: 'installing' }
  | { step: 'failed'; error: AppError };

export function describeProgress(state: InstallState): string | undefined {
  if (state.step === 'downloading')
    return state.percent === null ? 'Downloading…' : `Downloading… ${state.percent}%`;
  if (state.step === 'installing') return 'Installing…';
  return undefined;
}

/** One line on how this copy gets updates. */
export function describeMethod(check: UpdateCheck): string {
  switch (check.method.kind) {
    case 'inApp':
      return 'Updates are downloaded, checked against Homeport’s signing key and installed when you choose.';
    case 'packageManager':
      return `Installed with ${check.method.manager}: updates come from the Homeport package repository.`;
    case 'manual':
      return 'This is a development build: download new versions from the website.';
  }
}
