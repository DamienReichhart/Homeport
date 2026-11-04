import type { UpdateCheck } from '@/shared/api';
import { describeMethod } from './updates';

const check = (method: UpdateCheck['method']): UpdateCheck => ({
  currentVersion: '0.7.0',
  available: null,
  method,
});

describe('update method', () => {
  it('names where package-managed updates come from', () => {
    expect(
      describeMethod(
        check({ kind: 'packageManager', manager: 'scoop', command: 'scoop update homeport' }),
      ),
    ).toBe('Installed with Scoop: updates come from the Homeport bucket.');
    expect(
      describeMethod(
        check({ kind: 'packageManager', manager: 'aur', command: 'yay -Syu homeport-bin' }),
      ),
    ).toMatch(/AUR/);
    expect(
      describeMethod(check({ kind: 'packageManager', manager: 'apt', command: 'sudo apt …' })),
    ).toMatch(/Homeport package repository/);
  });
});
