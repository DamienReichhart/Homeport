import { screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands, container, inspectDocument } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { clientWithoutRetries, renderApp } from '@/test/render';

const web = container('web', null);
const pageCommands = {
  ...baseCommands,
  containers_list: () => [web],
};

describe('inspect tab', () => {
  it('shows the inspect document as sections, and raw on demand', async () => {
    mockCommands({ ...pageCommands, containers_inspect: () => inspectDocument('web') });
    const user = userEvent.setup();
    renderApp(`/containers/${web.id}/inspect`);

    expect(await screen.findByText('nginx:1.27')).toBeInTheDocument();
    expect(screen.getByText('MODE')).toBeInTheDocument();
    expect(screen.getByText('0.0.0.0:8080')).toBeInTheDocument();
    expect(screen.getByText('web-data')).toBeInTheDocument();
    expect(screen.getByText('org.example.team')).toBeInTheDocument();

    await user.click(screen.getByRole('tab', { name: 'Raw JSON' }));
    expect(await screen.findByText(/"NetworkMode"/)).toBeInTheDocument();
  });

  it('explains a failed inspect and retries it', async () => {
    let calls = 0;
    mockCommands({
      ...pageCommands,
      containers_inspect: () => {
        calls += 1;
        if (calls === 1)
          throw {
            code: 'connection_lost',
            message: 'The connection to local was lost.',
            hint: null,
            detail: null,
          };
        return inspectDocument('web');
      },
    });
    const user = userEvent.setup();
    renderApp(`/containers/${web.id}/inspect`, clientWithoutRetries());

    expect(await screen.findByText('The connection to local was lost.')).toBeInTheDocument();
    await user.click(screen.getByRole('button', { name: 'Retry' }));
    expect(await screen.findByText('nginx:1.27')).toBeInTheDocument();
  });
});
