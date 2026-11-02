import { render, screen } from '@testing-library/react';
import { RouterProvider } from 'react-router-dom';
import { mockCommands } from '@/test/ipc';
import { createAppRouter } from './routes/router';

describe('app routing', () => {
  beforeEach(() =>
    mockCommands({
      get_app_info: () => ({ name: 'Homeport', version: '0.1.0', os: 'linux', arch: 'x86_64' }),
      get_settings: () => ({ theme: 'system' }),
    }),
  );

  it('renders the home page with backend info', async () => {
    render(<RouterProvider router={createAppRouter(['/'])} />);
    expect(await screen.findByText(/Homeport v0.1.0 on linux\/x86_64/)).toBeInTheDocument();
  });

  it('renders the settings page', async () => {
    render(<RouterProvider router={createAppRouter(['/settings'])} />);
    expect(await screen.findByLabelText(/theme/i)).toHaveValue('system');
  });
});
