import { screen, waitFor, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands } from '@/test/fixtures';
import { mockCommands, sendToChannel } from '@/test/ipc';
import { groupCounts, nomadCommands, nomadJob, nomadJobDetail } from '@/test/nomad';
import { renderApp } from '@/test/render';

describe('Nomad jobs', () => {
  it('lists the jobs with their counts and follows their changes through Nomad', async () => {
    let channel: unknown = null;
    let running = 2;
    const watched: unknown[] = [];
    mockCommands({
      ...baseCommands,
      ...nomadCommands,
      clusters_nomad_jobs: () => [
        nomadJob('web', { groups: [groupCounts('api', running, 3)] }),
        nomadJob('backup', { jobType: 'batch', groups: [groupCounts('run', 0, null)] }),
      ],
      clusters_nomad_watch_subscribe: (args) => {
        channel = args.onEvent;
        watched.push(args.topic);
        return 1;
      },
    });
    renderApp('/nomad/jobs');
    expect(await screen.findByText('2 of 3 running')).toBeInTheDocument();
    expect(screen.getByText('0 running')).toBeInTheDocument();
    expect(watched[0]).toEqual({ topic: 'jobs', namespace: null });

    running = 3;
    await waitFor(() => expect(channel).not.toBeNull());
    sendToChannel(channel, true);
    expect(await screen.findByText('3 of 3 running')).toBeInTheDocument();
  });

  it('shows the Jobs page only for a Nomad cluster in the sidebar', async () => {
    mockCommands({ ...baseCommands, ...nomadCommands, clusters_nomad_jobs: () => [] });
    renderApp('/nomad/jobs');
    expect(await screen.findByText('No jobs')).toBeInTheDocument();
    const nav = screen.getByRole('navigation', { name: 'Main' });
    expect(within(nav).getByRole('link', { name: 'Jobs' })).toBeInTheDocument();
    expect(within(nav).queryByRole('link', { name: 'Workloads' })).not.toBeInTheDocument();
  });
});

describe('a Nomad job', () => {
  function setup() {
    const calls: { command: string; args: Record<string, unknown> }[] = [];
    mockCommands({
      ...baseCommands,
      ...nomadCommands,
      clusters_nomad_job: () => nomadJobDetail(),
      clusters_nomad_job_stop: (args) => {
        calls.push({ command: 'stop', args });
        return null;
      },
      clusters_nomad_job_scale: (args) => {
        calls.push({ command: 'scale', args });
        return null;
      },
    });
    return calls;
  }

  it('shows its task groups and allocations', async () => {
    setup();
    renderApp('/nomad/jobs/default/web');
    expect(await screen.findByText('web.api[0]')).toBeInTheDocument();
    expect(screen.getByText('2 of 3')).toBeInTheDocument();
    expect(screen.getByText('1 to 5')).toBeInTheDocument();
    expect(screen.getByText('server (docker)')).toBeInTheDocument();
    expect(screen.getByText('a1b2c3d4')).toBeInTheDocument();
  });

  it('stops it after asking, purging only when ticked', async () => {
    const calls = setup();
    const user = userEvent.setup();
    renderApp('/nomad/jobs/default/web');
    await user.click(await screen.findByRole('button', { name: 'Stop' }));
    const dialog = await screen.findByRole('dialog');
    expect(within(dialog).getByRole('checkbox', { name: 'Purge it' })).not.toBeChecked();
    await user.click(within(dialog).getByRole('button', { name: 'Stop job' }));
    await waitFor(() => expect(calls).toHaveLength(1));
    expect(calls[0]?.args).toMatchObject({
      id: 'n1',
      namespace: 'default',
      jobId: 'web',
      purge: false,
    });
  });

  it('scales a task group within its policy', async () => {
    const calls = setup();
    const user = userEvent.setup();
    renderApp('/nomad/jobs/default/web');
    await user.click(await screen.findByRole('button', { name: 'Scale' }));
    const count = await screen.findByLabelText('Count');
    expect(screen.getByText('Its scaling policy allows 1 to 5.')).toBeInTheDocument();
    await user.clear(count);
    await user.type(count, '6');
    const submit = within(screen.getByRole('dialog')).getByRole('button', { name: 'Scale' });
    expect(submit).toBeDisabled();
    await user.clear(count);
    await user.type(count, '4');
    await user.click(submit);
    await waitFor(() => expect(calls).toHaveLength(1));
    expect(calls[0]?.args).toMatchObject({ jobId: 'web', group: 'api', count: 4 });
  });
});
