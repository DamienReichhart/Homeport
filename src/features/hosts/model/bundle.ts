import type { BundleChoice, BundleHost } from '@/shared/api';
import type { MessageKey } from '@/shared/i18n';

/** The shortest passphrase an export accepts, as the backend checks it. */
export const MIN_PASSPHRASE = 12;

/** Export files' extension, offered in the save and open dialogs. */
export const BUNDLE_EXTENSION = 'homeport';

/** Why the passphrase typed twice can't protect an export, if it can't. */
export function passphraseProblem(passphrase: string, confirmation: string): MessageKey | null {
  if ([...passphrase].length < MIN_PASSPHRASE) return 'hosts.bundle.problem.short';
  if (passphrase !== confirmation) return 'hosts.bundle.problem.mismatch';
  return null;
}

/** One Host of an export file, with what the user picked and renamed. */
export interface BundleRow {
  host: BundleHost;
  selected: boolean;
  name: string;
}

/** Every Host is picked, under a free name. */
export const bundleRows = (hosts: BundleHost[]): BundleRow[] =>
  hosts.map((host) => ({ host, selected: true, name: host.suggestedName }));

/** What keeps a picked row from being imported, if anything. */
export function bundleRowProblem(row: BundleRow, rows: BundleRow[]): MessageKey | null {
  if (!row.selected) return null;
  const name = row.name.trim().toLowerCase();
  if (!name) return 'hosts.import.problem.nameEmpty';
  if (rows.filter((r) => r.selected && r.name.trim().toLowerCase() === name).length > 1) {
    return 'hosts.import.problem.nameTwice';
  }
  if (row.host.nameTaken && name === row.host.name.toLowerCase()) {
    return 'hosts.import.problem.nameTaken';
  }
  return null;
}

/** The picked rows, ready to send, or `null` while one of them has a problem. */
export function bundleChoices(rows: BundleRow[]): BundleChoice[] | null {
  const picked = rows.filter((r) => r.selected);
  if (picked.length === 0 || picked.some((r) => bundleRowProblem(r, rows))) return null;
  return picked.map((r) => ({ index: r.host.index, name: r.name.trim() }));
}
