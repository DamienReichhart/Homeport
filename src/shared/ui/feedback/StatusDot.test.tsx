import { render } from '@testing-library/react';
import { mockReducedMotion } from '@/test/motion';
import { StatusDot } from './StatusDot';

describe('StatusDot', () => {
  it('pulses while connecting, unless the OS asks for reduced motion', () => {
    const motion = mockReducedMotion(false);
    const { container, rerender } = render(<StatusDot tone="info" pulse />);
    expect(container.firstElementChild).toHaveClass('animate-pulse');
    motion.set(true);
    rerender(<StatusDot tone="info" pulse />);
    expect(container.firstElementChild).not.toHaveClass('animate-pulse');
    expect(container.firstElementChild).toHaveAttribute('data-tone', 'info');
    motion.restore();
  });
});
