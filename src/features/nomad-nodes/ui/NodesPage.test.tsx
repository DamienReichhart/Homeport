import { screen, waitFor, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { nomadAllocation, nomadCommands, nomadNode } from '@/test/nomad';
import { renderApp } from '@/test/render';

const NODE = nomadNode('client-1');

function setup(detail: Record<string, unknown> = {}, extra = {}) {
  const calls: { command: string; args: Record<string, unknown> }[] = [];
  const record = (command: string) => (args: Record<string, unknown>) => {
    calls.push({ command, args });
    return null;
  };
  mockCommands({
    ...baseCommands,
    ...nomadCommands,
    clusters_nomad_nodes: () => [
      NODE,
      nomadNode('client-22', { draining: true, eligible: false, running: 1 }),
    ],
    clusters_nomad_node: () => ({
      node: NODE,
      drain: null,
      lastDrain: null,
      drivers: [{ name: 'docker', detected: true, healthy: true }],
      attributes: [['os.name', 'ubuntu']],
      allocations: [nomadAllocation('web.api[0]')],
      ...detail,
    }),
    clusters_nomad_node_drain: record('drain'),
    clusters_nomad_node_eligibility: record('eligibility'),
    ...extra,
  });
  return calls;
}

describe('Nomad nodes', () => {
  it('lists each node with its status, scheduling and running allocations', async () => {
    setup();
    renderApp('/nomad/nodes');
    const draining = await screen.findByRole('row', { name: /client-22/ });
    expect(within(draining).getByText('Draining')).toBeInTheDocument();
    expect(within(draining).getByText('Ineligible')).toBeInTheDocument();
    const ready = screen.getByRole('row', { name: /client-1 / });
    expect(within(ready).getByText('Ready')).toBeInTheDocument();
  });

  it('drains a node after asking, with an hour’s deadline by default', async () => {
    const calls = setup();
    const user = userEvent.setup();
    renderApp(`/nomad/nodes/${NODE.id}`);
    expect(await screen.findByText('docker')).toBeInTheDocument();
    await user.click(screen.getByRole('button', { name: 'Drain' }));
    const dialog = await screen.findByRole('dialog');
    expect(within(dialog).getByLabelText('Deadline')).toHaveValue('1h');
    await user.click(
      within(dialog).getByRole('checkbox', { name: 'Leave system jobs running on it' }),
    );
    await user.click(within(dialog).getByRole('button', { name: 'Drain' }));
    await waitFor(() => expect(calls).toHaveLength(1));
    expect(calls[0]?.args).toMatchObject({
      nodeId: NODE.id,
      spec: { deadlineSecs: 3600, ignoreSystemJobs: true },
      markEligible: false,
    });
  });

  it('shows a drain’s progress and cancels it, making the node eligible again', async () => {
    const calls = setup({
      node: { ...NODE, draining: true, eligible: false, running: 1 },
      drain: { deadline: null, forced: false, ignoreSystemJobs: false, started: null },
    });
    const user = userEvent.setup();
    renderApp(`/nomad/nodes/${NODE.id}`);
    expect(await screen.findByText('1 allocation still runs here.')).toBeInTheDocument();
    await user.click(screen.getByRole('button', { name: 'Cancel drain' }));
    const dialog = await screen.findByRole('alertdialog');
    await user.click(within(dialog).getByRole('button', { name: 'Cancel drain' }));
    await waitFor(() => expect(calls).toHaveLength(1));
    expect(calls[0]?.args).toMatchObject({ spec: null, markEligible: true });
  });

  it('switches eligibility both ways', async () => {
    const calls = setup();
    const user = userEvent.setup();
    renderApp(`/nomad/nodes/${NODE.id}`);
    await user.click(await screen.findByRole('switch', { name: 'Eligible for new allocations' }));
    await waitFor(() => expect(calls).toHaveLength(1));
    expect(calls[0]).toMatchObject({ command: 'eligibility', args: { eligible: false } });
  });

  it('says which permission a refused drain needs', async () => {
    setup(
      {},
      {
        clusters_nomad_node_drain: () => {
          throw {
            code: 'nomad_permission_denied',
            key: 'errors.nomad.nodeWriteDenied',
            params: { name: 'nomad-dev' },
            message: "nomad-dev refused: the ACL token can't change nodes.",
            hint: null,
            detail: null,
          };
        },
      },
    );
    const user = userEvent.setup();
    renderApp(`/nomad/nodes/${NODE.id}`);
    await user.click(await screen.findByRole('button', { name: 'Drain' }));
    const dialog = await screen.findByRole('dialog');
    await user.click(within(dialog).getByRole('button', { name: 'Drain' }));
    expect(
      await within(dialog).findByText("nomad-dev refused: the ACL token can't change nodes."),
    ).toBeInTheDocument();
  });
});
