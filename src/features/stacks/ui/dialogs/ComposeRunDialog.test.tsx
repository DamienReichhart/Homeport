import { render, screen } from '@testing-library/react';
import { AppProviders } from '@/app/providers/AppProviders';
import { createQueryClient } from '@/app/providers/queryClient';
import { baseCommands } from '@/test/fixtures';
import { mockCommands, sendToChannel } from '@/test/ipc';
import { ComposeRunDialog } from './ComposeRunDialog';

describe('compose run dialog', () => {
  it('explains a failed deploy above the output', async () => {
    mockCommands({
      ...baseCommands,
      stacks_compose_subscribe: ({ onEvent }) => {
        setTimeout(() =>
          sendToChannel(
            onEvent,
            { type: 'line', text: ' Container shop-web-1  Starting' },
            {
              type: 'exit',
              success: false,
              code: 1,
              error: {
                code: 'port_in_use',
                message: 'Port 8080 is already used on the host.',
                hint: { text: 'Publish this one on another port.', commands: [] },
                detail: 'Bind for 0.0.0.0:8080 failed: port is already allocated',
              },
            },
          ),
        );
        return 7;
      },
    });
    render(
      <AppProviders client={createQueryClient()}>
        <ComposeRunDialog
          owner={{ kind: 'host', id: 'h1' }}
          job={{ kind: 'managed', stackId: 's1', name: 'shop', action: 'up' }}
          onClose={() => {}}
        />
      </AppProviders>,
    );

    expect(await screen.findByText('Port 8080 is already used on the host.')).toBeInTheDocument();
    expect(screen.getByText('Publish this one on another port.')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Details' })).toBeInTheDocument();
  });
});
