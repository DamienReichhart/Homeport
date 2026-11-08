import { screen, waitFor, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands } from '@/test/fixtures';
import { clusterCommands } from '@/test/clusters';
import { history, release } from '@/test/helm';
import { mockCommands, sendToChannel } from '@/test/ipc';
import { renderApp } from '@/test/render';

// CodeMirror and xterm.js don't run in jsdom: the editor is a text area, the terminal plain text.
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
}));

const nginx = (version: string) => ({
  name: 'bitnami/nginx',
  version,
  appVersion: '1.27.2',
  description: 'NGINX Open Source is a web server',
});

describe('installing a chart', () => {
  it('installs a repository chart with edited values after a preview', async () => {
    const previews: unknown[] = [];
    const installs: Record<string, unknown>[] = [];
    mockCommands({
      ...baseCommands,
      ...clusterCommands,
      clusters_helm_releases: () => ({ releases: [], onlyNamespace: null }),
      helm_charts_search: ({ versions }) =>
        versions ? [nginx('18.2.4'), nginx('18.2.3')] : [nginx('18.2.4')],
      helm_chart_versions: () => [nginx('18.2.4'), nginx('18.2.3')],
      helm_chart_values: () => 'replicaCount: 1\nimage:\n  tag: 1.27\n',
      clusters_helm_preview_install: ({ request }) => {
        previews.push(request);
        return {
          manifest: 'kind: Deployment\nspec:\n  replicas: 3\n',
          notes: 'Visit http://nginx',
        };
      },
      clusters_helm_install_subscribe: (args) => {
        installs.push(args.request as Record<string, unknown>);
        setTimeout(() =>
          sendToChannel(
            args.onEvent,
            { type: 'line', text: 'NAME: nginx' },
            { type: 'exit', success: true, code: 0, error: null },
          ),
        );
        return 1;
      },
      clusters_helm_history: () => history,
    });
    const user = userEvent.setup();
    renderApp('/helm');
    await user.click(await screen.findByRole('button', { name: 'Install chart' }));
    await user.type(screen.getByPlaceholderText('Search charts, such as postgres'), 'nginx');
    await user.click(await screen.findByRole('button', { name: 'Use' }));

    const values = await screen.findByLabelText('Values to install with');
    await waitFor(() => expect(values).toHaveValue('replicaCount: 1\nimage:\n  tag: 1.27\n'));
    expect(screen.getByLabelText('Release name')).toHaveValue('nginx');
    expect(within(screen.getByRole('dialog')).getByLabelText('Namespace')).toHaveValue('default');
    await user.clear(values);
    await user.type(values, 'replicaCount: 3');

    await user.click(screen.getByRole('button', { name: 'Preview' }));
    expect(await screen.findByLabelText('Rendered manifest')).toHaveValue(
      'kind: Deployment\nspec:\n  replicas: 3\n',
    );
    await user.click(screen.getByRole('button', { name: 'Install' }));

    expect(await screen.findByText(/NAME: nginx/)).toBeInTheDocument();
    expect(installs).toEqual([
      {
        name: 'nginx',
        namespace: 'default',
        chart: 'bitnami/nginx',
        version: null,
        values: 'replicaCount: 3',
        createNamespace: true,
        wait: false,
        timeoutSeconds: null,
      },
    ]);
    expect(previews).toEqual(installs);
    await user.click(await screen.findByRole('button', { name: 'Open release' }));
    expect(await screen.findByText('Upgrade complete')).toBeInTheDocument();
  });

  it('installs an OCI chart without a repository, and checks the release name first', async () => {
    const installs: Record<string, unknown>[] = [];
    mockCommands({
      ...baseCommands,
      ...clusterCommands,
      clusters_helm_releases: () => ({ releases: [release('web')], onlyNamespace: null }),
      helm_chart_values: () => '',
      clusters_helm_install_subscribe: (args) => {
        installs.push(args.request as Record<string, unknown>);
        return 1;
      },
    });
    const user = userEvent.setup();
    renderApp('/helm');
    await user.click(await screen.findByRole('button', { name: 'Install chart' }));
    await user.click(screen.getByRole('tab', { name: 'From an OCI registry' }));
    await user.type(
      screen.getByLabelText('OCI reference'),
      'oci://registry-1.docker.io/bitnamicharts/nginx',
    );
    await user.click(screen.getByRole('button', { name: 'Next' }));
    const name = await screen.findByLabelText('Release name');
    await user.clear(name);
    await user.type(name, 'Web');
    await user.click(screen.getByRole('button', { name: 'Install' }));
    expect(screen.getByRole('alert')).toHaveTextContent(/lowercase letters/);
    expect(installs).toEqual([]);

    await user.clear(name);
    await user.type(name, 'web2');
    await user.type(screen.getByLabelText('Chart version'), '18.2.4');
    await user.click(screen.getByRole('button', { name: 'Install' }));
    await waitFor(() =>
      expect(installs).toEqual([
        expect.objectContaining({
          name: 'web2',
          chart: 'oci://registry-1.docker.io/bitnamicharts/nginx',
          version: '18.2.4',
        }),
      ]),
    );
  });
});
