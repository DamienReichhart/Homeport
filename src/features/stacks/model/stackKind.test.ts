import { t } from '@/shared/i18n';
import { KIND_BADGE, KIND_LABEL } from './stackKind';

describe('Stack kinds', () => {
  it('names every kind, Linked and Synced included', () => {
    expect(t(KIND_LABEL.managed)).toBe('Managed');
    expect(t(KIND_LABEL.linked)).toBe('Linked');
    expect(t(KIND_LABEL.synced)).toBe('Synced');
    expect(t(KIND_LABEL.external)).toBe('External');
  });

  it('keeps every kind quiet, and sets apart what Homeport only finds', () => {
    expect(Object.values(KIND_BADGE)).not.toContain('default');
    expect(KIND_BADGE.managed).toBe(KIND_BADGE.linked);
    expect(KIND_BADGE.linked).toBe(KIND_BADGE.synced);
    expect(KIND_BADGE.external).not.toBe(KIND_BADGE.managed);
  });
});
