import { screen, waitFor, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import type { DeploySummary, Stack } from '@/shared/api';
import { baseCommands } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

const shop: Stack = {
  name: 'shop',
  kind: 'managed',
  id: 'm1',
  state: 'running',
  services: [],
  running: 1,
  total: 1,
  configFiles: null,
  workingDir: null,
  problem: null,
  updateAvailable: false,
};

const summary = (id: string, success: boolean | null, at: string): DeploySummary => ({
  id,
  stackId: 'm1',
  kind: 'managed',
  commit: null,
  automatic: false,
  action: 'up',
  startedAt: at,
  finishedAt: success === null ? null : at,
  success,
  exitCode: success === null ? null : success ? 0 : 1,
  hasEnv: true,
});

function mock(rolledBack: unknown[]) {
  mockCommands({
    ...baseCommands,
    get_capabilities: () => ({
      keychain: true,
      git: { state: 'available', cli: { program: 'git', version: '2.43.0' } },
      compose: {
        state: 'available',
        cli: { program: 'docker', prefix: ['compose'], version: '5' },
      },
    }),
    containers_list: () => [],
    images_freshness: () => [],
    stacks_list: () => [shop],
    stacks_history: () => [
      summary('d3', false, '2026-11-05T12:00:00.000Z'),
      summary('d2', true, '2026-11-05T11:00:00.000Z'),
      summary('d1', true, '2026-11-05T10:00:00.000Z'),
    ],
    stacks_deploy: ({ id }) => ({
      deploy: {
        summary: summary(id as string, true, '2026-11-05T10:00:00.000Z'),
        files: [{ name: 'compose.yaml', content: 'services:\n  web:\n    image: nginx:1.26\n' }],
        profiles: [],
      },
      envKeys: ['DB_PASSWORD'],
    }),
    stacks_rollback_subscribe: ({ deployId }) => {
      rolledBack.push(deployId);
      return 9;
    },
  });
}

describe('a Stack’s deploy history', () => {
  it('lists deploys, marks the one deployed now, and rolls back to another', async () => {
    const rolledBack: unknown[] = [];
    mock(rolledBack);
    renderApp('/stacks/shop');
    await userEvent.click(await screen.findByRole('tab', { name: 'History' }));
    const rows = await screen.findAllByRole('row');
    // Header, then newest first.
    expect(within(rows[1]!).getByText('Failed (exit code 1)')).toBeInTheDocument();
    expect(within(rows[2]!).getByText('Deployed now')).toBeInTheDocument();

    await userEvent.click(within(rows[3]!).getAllByRole('button')[0]!);
    expect(await screen.findByText(/image: nginx:1.26/)).toBeInTheDocument();
    expect(screen.getByText('Environment: DB_PASSWORD')).toBeInTheDocument();
    await userEvent.click(screen.getByRole('button', { name: 'Roll back' }));
    await userEvent.click(
      within(await screen.findByRole('alertdialog')).getByRole('button', { name: 'Roll back' }),
    );
    await waitFor(() => expect(rolledBack).toEqual(['d1']));
    expect(await screen.findByText('Rolling back shop')).toBeInTheDocument();
  });
});
