import { screen, waitFor, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands } from '@/test/fixtures';
import { clusterCommands } from '@/test/clusters';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

const type = (
  group: string,
  kind: string,
  plural: string,
  shortNames: string[],
  extra: Record<string, unknown> = {},
) => ({
  api: { group, version: 'v1', kind, plural, namespaced: true },
  shortNames,
  verbs: ['get', 'list', 'watch', 'delete', 'patch'],
  categories: [],
  listable: true,
  ...extra,
});

const types = [
  type('', 'PersistentVolumeClaim', 'persistentvolumeclaims', ['pvc']),
  type('', 'Secret', 'secrets', [], { listable: false }),
  type('cert-manager.io', 'Certificate', 'certificates', ['cert', 'certs']),
];

function commands(listed: unknown[] = []) {
  mockCommands({
    ...baseCommands,
    ...clusterCommands,
    clusters_api_resources: () => types,
    clusters_api_watch_subscribe: () => 1,
    clusters_api_list: (args) => {
      listed.push(args);
      return {
        columns: [
          { name: 'Ready', type: 'string', description: '', priority: 0 },
          { name: 'Secret', type: 'string', description: '', priority: 0 },
          { name: 'Issuer', type: 'string', description: '', priority: 1 },
        ],
        rows: [
          {
            name: 'web-tls',
            namespace: 'default',
            cells: ['True', 'web-tls-secret', 'letsencrypt'],
            created: '2026-11-05T10:00:00Z',
          },
        ],
      };
    },
  });
}

describe('API resources', () => {
  it('finds a type by its short name', async () => {
    commands();
    const user = userEvent.setup();
    renderApp('/api-resources');
    const nav = await screen.findByRole('navigation', { name: 'Resource types' });
    expect(within(nav).getByText('Certificate')).toBeInTheDocument();
    await user.type(within(nav).getByPlaceholderText('Kind or short name (pvc)'), 'pvc');
    expect(within(nav).getByText('PersistentVolumeClaim')).toBeInTheDocument();
    expect(within(nav).queryByText('Certificate')).not.toBeInTheDocument();
  });

  it('marks the types the credentials can’t list', async () => {
    commands();
    renderApp('/api-resources');
    const nav = await screen.findByRole('navigation', { name: 'Resource types' });
    expect(within(nav).getByLabelText('Your credentials can’t list it')).toBeInTheDocument();
  });

  it('lists a custom resource with the columns kubectl get shows', async () => {
    const listed: unknown[] = [];
    commands(listed);
    renderApp('/api-resources/cert-manager.io/v1/certificates');
    expect(await screen.findByText('web-tls')).toBeInTheDocument();
    expect(screen.getByRole('columnheader', { name: /Ready/ })).toBeInTheDocument();
    expect(screen.getByRole('columnheader', { name: /Secret/ })).toBeInTheDocument();
    // Columns kubectl shows only with -o wide stay out.
    expect(screen.queryByRole('columnheader', { name: /Issuer/ })).not.toBeInTheDocument();
    expect(screen.getByText('web-tls-secret')).toBeInTheDocument();
    await waitFor(() =>
      expect(listed[0]).toMatchObject({
        id: 'c1',
        api: { group: 'cert-manager.io', plural: 'certificates' },
        namespace: 'default',
      }),
    );
  });
});
