import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { Toaster } from '@/shared/ui';
import { baseCommands } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { useHostSelection } from './context';
import { HostProvider } from './HostProvider';

function Picker() {
  const { hostId, select } = useHostSelection();
  return (
    <button type="button" onClick={() => select('h2')}>
      Selected {hostId ?? 'none'}
    </button>
  );
}

function renderProvider() {
  const client = new QueryClient({ defaultOptions: { queries: { retry: false } } });
  return render(
    <QueryClientProvider client={client}>
      <HostProvider>
        <Picker />
      </HostProvider>
      <Toaster />
    </QueryClientProvider>,
  );
}

describe('host selection', () => {
  it('says so when the selected Host can’t be remembered', async () => {
    mockCommands({
      ...baseCommands,
      update_settings: () => {
        throw { code: 'db', message: 'Homeport couldn’t save its data.', hint: null, detail: null };
      },
    });
    const user = userEvent.setup();
    renderProvider();
    await user.click(await screen.findByRole('button', { name: /Selected/ }));
    expect(
      await screen.findByText('Homeport couldn’t remember the selected Host'),
    ).toBeInTheDocument();
    expect(screen.getByText('Homeport couldn’t save its data.')).toBeInTheDocument();
  });

  it('falls back to the first Host when the settings can’t be read', async () => {
    mockCommands({
      ...baseCommands,
      get_settings: () => {
        throw { code: 'db', message: 'Homeport couldn’t read its data.', hint: null, detail: null };
      },
    });
    renderProvider();
    expect(await screen.findByRole('button', { name: 'Selected h1' })).toBeInTheDocument();
  });
});
