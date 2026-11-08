import { screen, waitFor, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderWithProviders } from '@/test/render';
import { ApplyFlow } from './ApplyFlow';

const yaml = 'apiVersion: v1\nkind: ConfigMap\nmetadata:\n  name: settings\ndata:\n  mode: fast\n';

const doc = (action: string, extra: Record<string, unknown> = {}) => ({
  apiVersion: 'v1',
  kind: 'ConfigMap',
  namespace: 'shop',
  name: 'settings',
  action,
  before: action === 'created' ? null : 'data:\n  mode: slow\n',
  after: 'data:\n  mode: fast\n',
  conflicts: [],
  error: null,
  ...extra,
});

describe('apply flow', () => {
  it('previews, then applies only after a confirmation naming the cluster', async () => {
    const applied: unknown[] = [];
    mockCommands({
      ...baseCommands,
      clusters_apply_preview: () => ({ documents: [doc('changed')] }),
      clusters_apply: (args) => {
        applied.push(args);
        return { documents: [doc('changed')] };
      },
    });
    const user = userEvent.setup();
    renderWithProviders(
      <ApplyFlow clusterId="c1" clusterName="kind-dev" yaml={yaml} namespace="shop" />,
    );
    expect(screen.getByRole('button', { name: 'Apply' })).toBeDisabled();
    await user.click(screen.getByRole('button', { name: 'Preview changes' }));
    expect(await screen.findByText('Changed')).toBeInTheDocument();
    expect(screen.getByText('ConfigMap shop/settings')).toBeInTheDocument();
    expect(applied).toEqual([]);

    await user.click(screen.getByRole('button', { name: 'Apply' }));
    const dialog = await screen.findByRole('alertdialog');
    expect(within(dialog).getByText('Apply 1 change to kind-dev?')).toBeInTheDocument();
    await user.click(within(dialog).getByRole('button', { name: 'Apply' }));
    await waitFor(() =>
      expect(applied).toEqual([{ id: 'c1', yaml, namespace: 'shop', force: false }]),
    );
  });

  it('says when there is nothing to change', async () => {
    mockCommands({
      ...baseCommands,
      clusters_apply_preview: () => ({ documents: [doc('unchanged')] }),
    });
    const user = userEvent.setup();
    renderWithProviders(
      <ApplyFlow clusterId="c1" clusterName="kind-dev" yaml={yaml} namespace={null} />,
    );
    await user.click(screen.getByRole('button', { name: 'Preview changes' }));
    expect(
      await screen.findByText(/No changes: the cluster already holds this/),
    ).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Apply' })).toBeDisabled();
  });

  it('lists the conflicting fields and takes them over after a second confirmation', async () => {
    const applied: unknown[] = [];
    mockCommands({
      ...baseCommands,
      clusters_apply_preview: () => ({
        documents: [
          doc('failed', {
            conflicts: [
              {
                field: '.data.mode',
                message: 'conflict with "kubectl-client-side-apply" using v1',
              },
            ],
            error: {
              code: 'conflict',
              message:
                'Other tools manage some fields of ConfigMap settings that this would change.',
            },
          }),
        ],
      }),
      clusters_apply: (args) => {
        applied.push(args);
        return { documents: [doc('changed')] };
      },
    });
    const user = userEvent.setup();
    renderWithProviders(
      <ApplyFlow clusterId="c1" clusterName="kind-dev" yaml={yaml} namespace="shop" />,
    );
    await user.click(screen.getByRole('button', { name: 'Preview changes' }));
    expect(await screen.findByText('.data.mode')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Apply' })).toBeDisabled();
    await user.click(screen.getByRole('button', { name: 'Apply anyway' }));
    const dialog = await screen.findByRole('alertdialog');
    await user.click(within(dialog).getByRole('button', { name: 'Apply anyway' }));
    await waitFor(() => expect(applied).toMatchObject([{ force: true }]));
  });
});
