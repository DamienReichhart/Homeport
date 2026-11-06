import { backupFileName, passphraseProblem } from './backup';

describe('backups', () => {
  it('asks for a long passphrase, typed twice the same', () => {
    expect(passphraseProblem('short', 'short')).toBe('settings.backup.problem.short');
    expect(passphraseProblem('long enough phrase', 'long enough phrase!')).toBe(
      'settings.backup.problem.mismatch',
    );
    expect(passphraseProblem('long enough phrase', 'long enough phrase')).toBeNull();
  });

  it('names a backup after its day', () => {
    expect(backupFileName(new Date('2026-11-06T10:00:00Z'))).toBe(
      'homeport-backup-2026-11-06.homeport',
    );
  });
});
