/** The shortest passphrase a backup accepts, as the backend checks it. */
export const MIN_PASSPHRASE = 12;

/** Backups' extension: the format of Hosts exports, which Homeport opens too. */
export const BACKUP_EXTENSION = 'homeport';

/** Why the passphrase typed twice can't protect a backup, if it can't. */
export function passphraseProblem(
  passphrase: string,
  confirmation: string,
): 'settings.backup.problem.short' | 'settings.backup.problem.mismatch' | null {
  if ([...passphrase].length < MIN_PASSPHRASE) return 'settings.backup.problem.short';
  if (passphrase !== confirmation) return 'settings.backup.problem.mismatch';
  return null;
}

/** A backup's default file name, dated: `homeport-backup-2026-11-06.homeport`. */
export function backupFileName(now = new Date()): string {
  return `homeport-backup-${now.toISOString().slice(0, 10)}.${BACKUP_EXTENSION}`;
}
