import { screen, waitFor, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands } from '@/test/fixtures';
import { clusterCommands, workloadView } from '@/test/clusters';
import { mockCommands, sendToChannel } from '@/test/ipc';
import { renderApp } from '@/test/render';

describe('workloads', () => {
  it('lists the Deployments of the context’s namespace and follows their changes live', async () => {
    let channel: unknown = null;
    let ready = 1;
    const asked: unknown[] = [];
    mockCommands({
      ...baseCommands,
      ...clusterCommands,
      clusters_workloads_list: (args) => {
        asked.push(args);
        return [workloadView('web', { ready, health: ready === 3 ? 'ready' : 'progressing' })];
      },
      clusters_watch_subscribe: (args) => {
        channel = args.onEvent;
        return 1;
      },
    });
    renderApp('/workloads');
    expect(await screen.findByText('web')).toBeInTheDocument();
    expect(screen.getByText('1/3')).toBeInTheDocument();
    expect(screen.getByText('Progressing')).toBeInTheDocument();
    expect(asked[0]).toEqual({ id: 'c1', kind: 'deployment', namespace: 'default' });

    ready = 3;
    await waitFor(() => expect(channel).not.toBeNull());
    sendToChannel(channel, { kind: 'deployment' });
    expect(await screen.findByText('3/3')).toBeInTheDocument();
    expect(within(screen.getByRole('row', { name: /web/ })).getByText('Ready')).toBeInTheDocument();
  });

  it('says what the credentials can’t list, and keeps the page usable', async () => {
    mockCommands({
      ...baseCommands,
      ...clusterCommands,
      clusters_workloads_list: ({ kind }) => {
        if (kind === 'daemonSet')
          throw {
            code: 'forbidden',
            message: 'Your credentials can’t list daemonsets in default.',
          };
        return [];
      },
    });
    const user = userEvent.setup();
    renderApp('/workloads');
    await user.click(await screen.findByRole('tab', { name: 'DaemonSets' }));
    expect(
      await screen.findByText('Your credentials can’t list daemonsets in default.'),
    ).toBeInTheDocument();
    await user.click(screen.getByRole('tab', { name: 'Jobs' }));
    expect(await screen.findByText('None here.')).toBeInTheDocument();
  });

  it('opens a workload with its Pods, conditions and YAML', async () => {
    const podsAsked: unknown[] = [];
    mockCommands({
      ...baseCommands,
      ...clusterCommands,
      clusters_workload_get: () => ({
        view: workloadView('web'),
        labels: { app: 'web' },
        annotations: {},
        selector: 'app=web',
        conditions: [
          {
            type: 'Available',
            status: 'True',
            reason: 'MinimumReplicasAvailable',
            message: null,
            lastTransition: null,
          },
        ],
        containers: [{ name: 'web', image: 'nginx:1.27', ports: ['80/TCP'] }],
        strategy: 'RollingUpdate',
      }),
      clusters_pods_list: (args) => {
        podsAsked.push(args);
        return [];
      },
      clusters_resource_yaml: () => 'kind: Deployment\n',
    });
    const user = userEvent.setup();
    renderApp('/workloads/deployments/default/web');
    expect(await screen.findByText('RollingUpdate')).toBeInTheDocument();
    expect(screen.getByText('Deployment')).toBeInTheDocument();
    expect(screen.getByText('80/TCP')).toBeInTheDocument();
    await user.click(screen.getByRole('tab', { name: 'Pods' }));
    await waitFor(() =>
      expect(podsAsked).toEqual([
        { id: 'c1', namespace: 'default', selector: 'app=web', node: null },
      ]),
    );
    await user.click(screen.getByRole('tab', { name: 'Conditions' }));
    expect(await screen.findByText('MinimumReplicasAvailable')).toBeInTheDocument();
    await user.click(screen.getByRole('tab', { name: 'YAML' }));
    expect(await screen.findByText('kind: Deployment')).toBeInTheDocument();
  });

  it('scales a Deployment from its row, warning that its autoscaler will override it', async () => {
    const scaled: unknown[] = [];
    mockCommands({
      ...baseCommands,
      ...clusterCommands,
      clusters_workloads_list: () => [workloadView('web')],
      clusters_autoscaler: () => 'web-hpa',
      clusters_scale: (args) => {
        scaled.push(args);
        return null;
      },
    });
    const user = userEvent.setup();
    renderApp('/workloads');
    await user.click(await screen.findByRole('button', { name: 'Actions for web' }));
    await user.click(await screen.findByRole('menuitem', { name: 'Scale' }));
    expect(await screen.findByText(/The autoscaler web-hpa manages its Pods/)).toBeInTheDocument();
    const field = screen.getByLabelText('Pods');
    await user.clear(field);
    await user.type(field, '0');
    await user.click(screen.getByRole('button', { name: 'Scale' }));
    await waitFor(() =>
      expect(scaled).toEqual([
        { id: 'c1', kind: 'deployment', namespace: 'default', name: 'web', replicas: 0 },
      ]),
    );
  });

  it('deletes a workload only once the confirmation names it and the cluster', async () => {
    const deleted: unknown[] = [];
    mockCommands({
      ...baseCommands,
      ...clusterCommands,
      clusters_workloads_list: () => [workloadView('web')],
      clusters_delete: (args) => {
        deleted.push(args);
        return null;
      },
    });
    const user = userEvent.setup();
    renderApp('/workloads');
    await user.click(await screen.findByRole('button', { name: 'Actions for web' }));
    await user.click(await screen.findByRole('menuitem', { name: 'Delete' }));
    const dialog = await screen.findByRole('alertdialog');
    expect(
      within(dialog).getByText(/Deployment web will be deleted from the namespace default on/),
    ).toBeInTheDocument();
    expect(deleted).toEqual([]);
    await user.click(within(dialog).getByRole('button', { name: 'Delete' }));
    await waitFor(() =>
      expect(deleted).toEqual([
        { id: 'c1', kind: 'deployment', namespace: 'default', name: 'web', grace: null },
      ]),
    );
  });
});
