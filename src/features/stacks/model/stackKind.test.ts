import { t } from '@/shared/i18n';
import { KIND_BADGE, KIND_LABEL } from './stackKind';

describe('Stack kinds', () => {
  it('names every kind, Linked included', () => {
    expect(t(KIND_LABEL.managed)).toBe('Managed');
    expect(t(KIND_LABEL.linked)).toBe('Linked');
    expect(t(KIND_LABEL.external)).toBe('External');
  });

  it('tells the three kinds apart by their badge', () => {
    expect(new Set(Object.values(KIND_BADGE)).size).toBe(3);
  });
});
