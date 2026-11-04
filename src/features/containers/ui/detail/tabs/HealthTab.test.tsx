import { screen, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import type { Health } from '@/shared/api';
import { baseCommands, container, inspectDocument } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { clientWithoutRetries, renderApp } from '@/test/render';

const unhealthy: Health = {
  status: 'unhealthy',
  failingStreak: 4,
  config: {
    command: 'curl -fsS localhost/health || exit 1',
    intervalS: 10,
    timeoutS: null,
    retries: 3,
    startPeriodS: null,
  },
  log: [
    {
      start: '2026-11-04T10:00:10.000Z',
      end: '2026-11-04T10:00:10.300Z',
      exitCode: 7,
      output: 'curl: (7) Failed to connect to localhost port 80',
    },
    { start: '2026-11-04T10:00:00Z', end: '2026-11-04T10:00:00.1Z', exitCode: 0, output: '' },
  ],
};

function page(health: string | null, view: Health | null) {
  const web = { ...container('web', null), health };
  mockCommands({
    ...baseCommands,
    containers_list: () => [web],
    containers_inspect: () => inspectDocument('web'),
    containers_health: () => view,
    logs_subscribe: () => 1,
  });
  return web;
}

describe('health tab', () => {
  it("shows an unhealthy container's failing output and exit code", async () => {
    const web = page('unhealthy', unhealthy);
    renderApp(`/containers/${web.id}/health`, clientWithoutRetries());
    expect(
      await screen.findByText('curl: (7) Failed to connect to localhost port 80'),
    ).toBeInTheDocument();
    expect(screen.getByText('exit 7')).toBeInTheDocument();
    expect(screen.getByText('unhealthy')).toBeInTheDocument();
    expect(
      screen.getByText('every 10s, timeout 30s, 3 retries, start period 0s'),
    ).toBeInTheDocument();
    expect(screen.getByText('No output.')).toBeInTheDocument();
  });

  it('opens from its tab when the container has a healthcheck', async () => {
    const web = page('healthy', { ...unhealthy, status: 'healthy', failingStreak: 0, log: [] });
    const user = userEvent.setup();
    renderApp(`/containers/${web.id}/inspect`, clientWithoutRetries());
    await user.click(await screen.findByRole('tab', { name: 'Health' }));
    const panel = await screen.findByRole('tabpanel');
    expect(await within(panel).findByText('No check has run yet.')).toBeInTheDocument();
    expect(within(panel).getByText('healthy')).toBeInTheDocument();
  });

  it('is hidden for a container without a healthcheck', async () => {
    const web = page(null, null);
    renderApp(`/containers/${web.id}/logs`, clientWithoutRetries());
    expect(await screen.findByRole('tab', { name: 'Logs' })).toBeInTheDocument();
    expect(screen.queryByRole('tab', { name: 'Health' })).toBeNull();
  });
});
