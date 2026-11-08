import { screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands } from '@/test/fixtures';
import { clusterCommands } from '@/test/clusters';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

const event = (type: string, reason: string, kind: string, name: string) => ({
  namespace: 'default',
  name: `${name}.${reason}`,
  type,
  reason,
  message: `${reason} happened`,
  object: { kind, namespace: 'default', name },
  count: 1,
  firstSeen: null,
  lastSeen: null,
  source: null,
});

describe('a Cluster’s events', () => {
  it('lists them, filters warnings and links each to its object', async () => {
    mockCommands({
      ...baseCommands,
      ...clusterCommands,
      clusters_events_list: () => [
        event('Warning', 'BackOff', 'Pod', 'web-1'),
        event('Normal', 'ScalingReplicaSet', 'Deployment', 'web'),
      ],
    });
    const user = userEvent.setup();
    renderApp('/cluster-events');
    expect(await screen.findByRole('link', { name: 'Pod/web-1' })).toHaveAttribute(
      'href',
      '/pods/default/web-1',
    );
    expect(screen.getByRole('link', { name: 'Deployment/web' })).toHaveAttribute(
      'href',
      '/workloads/deployments/default/web',
    );
    await user.click(screen.getByRole('radio', { name: 'Warnings' }));
    await waitFor(() => expect(screen.queryByText('ScalingReplicaSet')).not.toBeInTheDocument());
    expect(screen.getByText('BackOff')).toBeInTheDocument();
  });
});
