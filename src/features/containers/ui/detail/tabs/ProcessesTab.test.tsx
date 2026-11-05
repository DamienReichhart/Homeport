import { screen } from '@testing-library/react';
import { baseCommands } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderWithProviders } from '@/test/render';
import { ProcessesTab } from './ProcessesTab';

describe('processes tab', () => {
  it('lists the processes under the titles the Engine sends', async () => {
    mockCommands({
      ...baseCommands,
      containers_processes: () => ({
        titles: ['UID', 'PID', 'CMD'],
        processes: [
          ['root', '1', 'nginx: master process'],
          ['101', '29', 'nginx: worker process'],
        ],
      }),
    });
    renderWithProviders(<ProcessesTab hostId="h1" containerId="web-1" running />);
    expect(await screen.findByRole('columnheader', { name: 'CMD' })).toBeInTheDocument();
    expect(screen.getByText('nginx: worker process')).toBeInTheDocument();
  });

  it('explains a failed listing', async () => {
    mockCommands({
      ...baseCommands,
      containers_processes: () => {
        throw {
          code: 'container_not_running',
          message: 'web isn’t running.',
          hint: null,
          detail: null,
        };
      },
    });
    renderWithProviders(<ProcessesTab hostId="h1" containerId="web-1" running />);
    expect(await screen.findByText('web isn’t running.')).toBeInTheDocument();
  });

  it('says a stopped container has no processes', () => {
    mockCommands({ ...baseCommands });
    renderWithProviders(<ProcessesTab hostId="h1" containerId="web-1" running={false} />);
    expect(screen.getByText("The container isn't running.")).toBeInTheDocument();
  });
});
