import { render, screen } from '@testing-library/react';
import { RouterProvider } from 'react-router-dom';
import { vi } from 'vitest';
import { createAppRouter } from './routes/router';

vi.mock('@/shared/api/invoke', () => ({
  invokeCommand: vi.fn(async (cmd: string) => {
    if (cmd === 'get_app_info')
      return { name: 'Homeport', version: '0.1.0', os: 'linux', arch: 'x86_64' };
    if (cmd === 'get_settings') return { theme: 'system' };
    throw new Error(`unexpected command ${cmd}`);
  }),
}));

describe('app routing', () => {
  it('renders the home page with backend info', async () => {
    render(<RouterProvider router={createAppRouter(['/'])} />);
    expect(await screen.findByText(/Homeport v0.1.0 on linux\/x86_64/)).toBeInTheDocument();
  });

  it('renders the settings page', async () => {
    render(<RouterProvider router={createAppRouter(['/settings'])} />);
    expect(await screen.findByLabelText(/theme/i)).toHaveValue('system');
  });
});
