import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { AppProviders } from '@/app/providers/AppProviders';
import { createQueryClient } from '@/app/providers/queryClient';
import type { DeployDiff } from '@/shared/api';
import { baseCommands } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { ComposeRunDialog } from './ComposeRunDialog';

const diff = (over: Partial<DeployDiff>): DeployDiff => ({
  known: true,
  deployedAt: '2026-11-05T10:00:00.000Z',
  files: [],
  reordered: false,
  env: [],
  deployedProfiles: [],
  candidateProfiles: [],
  unchanged: false,
  ...over,
});

function show(result: DeployDiff, started: unknown[] = []) {
  mockCommands({
    ...baseCommands,
    stacks_diff: () => result,
    stacks_compose_subscribe: (args) => {
      started.push(args);
      return 3;
    },
  });
  render(
    <AppProviders client={createQueryClient()}>
      <ComposeRunDialog
        owner={{ kind: 'host', id: 'h1' }}
        job={{ kind: 'managed', stackId: 's1', name: 'shop', action: 'up', review: null }}
        onClose={() => {}}
      />
    </AppProviders>,
  );
}

describe('reviewing a redeploy', () => {
  it('shows changed files and environment keys, never values, before running', async () => {
    const started: unknown[] = [];
    show(
      diff({
        files: [
          {
            name: 'compose.yaml',
            deployed: 'services:\n  web:\n    image: nginx:1.26\n',
            candidate: 'services:\n  web:\n    image: nginx:1.27\n',
          },
        ],
        env: [
          { key: 'DB_PASSWORD', change: 'changed' },
          { key: 'TAG', change: 'unchanged' },
        ],
      }),
      started,
    );
    expect(await screen.findByText('Review changes to shop')).toBeInTheDocument();
    expect(await screen.findByText('compose.yaml')).toBeInTheDocument();
    expect(screen.getByText('DB_PASSWORD')).toBeInTheDocument();
    expect(screen.getAllByText('Changed').length).toBeGreaterThan(0);
    expect(screen.getByText('1 other variable is unchanged.')).toBeInTheDocument();
    expect(started).toEqual([]);
    await userEvent.click(screen.getByRole('button', { name: 'Deploy' }));
    expect(await screen.findByText('Deploying shop')).toBeInTheDocument();
    expect(started).toHaveLength(1);
  });

  it('says when nothing changed and offers to redeploy anyway', async () => {
    show(diff({ unchanged: true }));
    expect(await screen.findByText('Nothing changed since the last deploy.')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Redeploy anyway' })).toBeInTheDocument();
  });

  it('says when what is deployed is unknown, and shows the new definition', async () => {
    show(
      diff({
        known: false,
        deployedAt: null,
        files: [{ name: 'compose.yaml', deployed: null, candidate: 'services: {}\n' }],
      }),
    );
    expect(await screen.findByText('What is deployed now is unknown')).toBeInTheDocument();
    expect(await screen.findByText('compose.yaml')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Deploy' })).toBeInTheDocument();
  });
});
