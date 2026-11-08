import { screen, waitFor, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands } from '@/test/fixtures';
import { clusterCommands } from '@/test/clusters';
import { details, history, release } from '@/test/helm';
import { mockCommands, sendToChannel } from '@/test/ipc';
import { renderApp } from '@/test/render';

// CodeMirror and xterm.js don't run in jsdom: editors are text areas, a
// diff shows both sides, the terminal is plain text.
vi.mock('@/shared/ui', async (original) => ({
  ...(await original<object>()),
  XTerm: (await import('@/test/terminal')).FakeXTerm,
  YamlEditor: ({
    value,
    label,
    readOnly = false,
    onChange,
  }: {
    value: string;
    label: string;
    readOnly?: boolean;
    onChange?: (value: string) => void;
  }) => (
    <textarea
      aria-label={label}
      value={value}
      readOnly={readOnly}
      onChange={(e) => onChange?.(e.target.value)}
    />
  ),
  YamlDiff: ({ before, after, label }: { before: string; after: string; label: string }) => (
    <div aria-label={label}>
      <pre data-testid="before">{before}</pre>
      <pre data-testid="after">{after}</pre>
    </div>
  ),
}));

/** Commands of a Cluster with the release web in shop, and helm runs that succeed. */
function commands(runs: Record<string, unknown[]>) {
  const streamed = (kind: string) => (args: Record<string, unknown>) => {
    (runs[kind] ??= []).push(args.request);
    setTimeout(() =>
      sendToChannel(args.onEvent, { type: 'exit', success: true, code: 0, error: null }),
    );
    return 1;
  };
  return {
    ...baseCommands,
    ...clusterCommands,
    clusters_helm_releases: () => ({ releases: [release('web')], onlyNamespace: null }),
    clusters_helm_history: () => history,
    clusters_helm_release_details: ({ revision }: Record<string, unknown>) =>
      details(revision as number),
    clusters_api_resources: () => [],
    helm_chart_candidates: () => ['bitnami/nginx', 'mirror/nginx'],
    helm_chart_versions: () => [
      { name: 'bitnami/nginx', version: '18.2.5', appVersion: '1.27.3', description: '' },
      { name: 'bitnami/nginx', version: '18.2.4', appVersion: '1.27.2', description: '' },
    ],
    clusters_helm_upgrade_subscribe: streamed('upgrade'),
    clusters_helm_rollback_subscribe: streamed('rollback'),
    clusters_helm_uninstall_subscribe: streamed('uninstall'),
  };
}

describe('changing a release', () => {
  it('rolls back to an earlier revision once both revisions are named', async () => {
    const runs: Record<string, unknown[]> = {};
    mockCommands(commands(runs));
    const user = userEvent.setup();
    renderApp('/helm/shop/web');
    expect(
      screen.queryByRole('button', { name: 'Roll back to revision 2' }),
    ).not.toBeInTheDocument();
    await user.click(await screen.findByRole('button', { name: 'Roll back to revision 1' }));
    const dialog = screen.getByRole('dialog');
    expect(within(dialog).getByText('Chart nginx-18.2.4, app 1.27.2')).toBeInTheDocument();
    expect(within(dialog).getByText('Chart nginx-18.2.3, app 1.27.1')).toBeInTheDocument();
    expect(runs.rollback).toBeUndefined();
    await user.click(within(dialog).getByRole('button', { name: 'Roll back to revision 1' }));
    await waitFor(() =>
      expect(runs.rollback).toEqual([
        { name: 'web', namespace: 'shop', revision: 1, wait: false, timeoutSeconds: null },
      ]),
    );
    expect(await screen.findByText('Done.')).toBeInTheDocument();
  });

  it('upgrades from the release’s values, showing what changed in them', async () => {
    const runs: Record<string, unknown[]> = {};
    mockCommands(commands(runs));
    const user = userEvent.setup();
    renderApp('/helm/shop/web');
    await user.click(await screen.findByRole('button', { name: 'Upgrade' }));
    const values = await screen.findByLabelText('Values to install with');
    expect(values).toHaveValue('replicaCount: 2\n');
    await user.clear(values);
    await user.type(values, 'replicaCount: 4');
    await user.click(screen.getByRole('radio', { name: 'Changes to the values' }));
    expect(screen.getByTestId('before')).toHaveTextContent('replicaCount: 2');
    expect(screen.getByTestId('after')).toHaveTextContent('replicaCount: 4');

    await user.click(within(screen.getByRole('dialog')).getByRole('button', { name: 'Upgrade' }));
    await waitFor(() =>
      expect(runs.upgrade).toEqual([
        {
          name: 'web',
          namespace: 'shop',
          chart: 'bitnami/nginx',
          version: null,
          values: 'replicaCount: 4',
          wait: false,
          timeoutSeconds: null,
        },
      ]),
    );
  });

  it('uninstalls after listing what goes with the release', async () => {
    const runs: Record<string, unknown[]> = {};
    mockCommands(commands(runs));
    const user = userEvent.setup();
    renderApp('/helm/shop/web');
    await user.click(await screen.findByRole('button', { name: 'Uninstall' }));
    const dialog = screen.getByRole('dialog');
    expect(await within(dialog).findByText('1 object will be deleted:')).toBeInTheDocument();
    expect(within(dialog).getByLabelText('Objects that will be deleted')).toHaveTextContent(
      'Service/web',
    );
    expect(within(dialog).getByText(/from the Namespace shop on kind-dev/)).toBeInTheDocument();
    await user.click(within(dialog).getByLabelText('Keep its history, so it can be rolled back'));
    await user.click(within(dialog).getByRole('button', { name: 'Uninstall web' }));
    await waitFor(() =>
      expect(runs.uninstall).toEqual([
        { name: 'web', namespace: 'shop', keepHistory: true, wait: false, timeoutSeconds: null },
      ]),
    );
    await user.click(await screen.findByRole('button', { name: 'Back to releases' }));
    expect(await screen.findByText('1 release')).toBeInTheDocument();
  });
});
