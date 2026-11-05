import { t } from '@/shared/i18n';
import { KIND_BADGE, KIND_LABEL } from './stackKind';

describe('Stack kinds', () => {
  it('names every kind, Linked and Synced included', () => {
    expect(t(KIND_LABEL.managed)).toBe('Managed');
    expect(t(KIND_LABEL.linked)).toBe('Linked');
    expect(t(KIND_LABEL.synced)).toBe('Synced');
    expect(t(KIND_LABEL.external)).toBe('External');
  });

  it('badges what Homeport holds, what it points at, and what it only finds', () => {
    expect(KIND_BADGE.linked).toBe(KIND_BADGE.synced);
    expect(new Set([KIND_BADGE.managed, KIND_BADGE.linked, KIND_BADGE.external]).size).toBe(3);
  });
});
