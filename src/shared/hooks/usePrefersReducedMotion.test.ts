import { act, renderHook } from '@testing-library/react';
import { usePrefersReducedMotion } from './usePrefersReducedMotion';
import { mockReducedMotion } from '@/test/motion';

describe('usePrefersReducedMotion', () => {
  it('follows the OS setting as it changes', () => {
    const motion = mockReducedMotion(false);
    const { result } = renderHook(() => usePrefersReducedMotion());
    expect(result.current).toBe(false);
    act(() => motion.set(true));
    expect(result.current).toBe(true);
    motion.restore();
  });
});
