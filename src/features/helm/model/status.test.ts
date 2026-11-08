import { isPending, statusTone } from './status';

describe('release status', () => {
  it('colours what needs attention', () => {
    expect(statusTone('deployed')).toBe('success');
    expect(statusTone('failed')).toBe('danger');
    expect(statusTone('pending-upgrade')).toBe('warning');
    expect(statusTone('pending-install')).toBe('warning');
    expect(statusTone('uninstalling')).toBe('warning');
    expect(statusTone('superseded')).toBe('muted');
    expect(statusTone('uninstalled')).toBe('muted');
  });

  it('knows a release locked by a pending operation', () => {
    expect(isPending('pending-rollback')).toBe(true);
    expect(isPending('deployed')).toBe(false);
  });
});
