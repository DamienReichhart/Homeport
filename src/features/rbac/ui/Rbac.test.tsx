import { screen, waitFor, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands } from '@/test/fixtures';
import { clusterCommands } from '@/test/clusters';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

const sa = { kind: 'ServiceAccount', name: 'deployer', namespace: 'prod' };

const binding = {
  kind: 'RoleBinding',
  name: 'deployer-edit',
  namespace: 'prod',
  roleKind: 'ClusterRole',
  roleName: 'edit',
  roleFound: true,
  subjects: [sa],
  created: null,
};

const index = {
  namespace: null,
  roles: [
    {
      kind: 'ClusterRole',
      name: 'edit',
      namespace: null,
      rules: [
        {
          verbs: ['get', 'delete'],
          apiGroups: [''],
          resources: ['secrets'],
          resourceNames: [],
          nonResourceUrls: [],
        },
      ],
      aggregation: ['rbac.authorization.k8s.io/aggregate-to-edit=true'],
      aggregatedFrom: ['system:aggregate-to-edit'],
      labels: {},
      created: null,
    },
  ],
  bindings: [binding],
  subjects: [{ subject: sa, bindings: 1 }],
  refused: [],
};

const own = {
  namespace: 'default',
  user: { username: 'system:admin', groups: ['system:masters'] },
  grants: {
    resources: [{ apiGroup: '*', resource: '*', resourceNames: [], verbs: ['*'], bindings: [] }],
    urls: [],
  },
  incomplete: true,
  evaluationError: null,
};

const canI = (args: Record<string, unknown>) => {
  const check = args.check as { verb: string; resource: string; namespace: string | null };
  return {
    allowed: check.verb === 'delete',
    denied: false,
    reason: null,
    evaluationError: null,
    resource: check.resource,
    known: true,
    command: `kubectl auth can-i ${check.verb} ${check.resource} --namespace ${check.namespace}`,
  };
};

describe('RBAC', () => {
  it('lists roles and opens a ServiceAccount with what it may do in its namespace', async () => {
    const asked: unknown[] = [];
    mockCommands({
      ...baseCommands,
      ...clusterCommands,
      clusters_rbac_index: () => index,
      clusters_rbac_subject: (args) => {
        asked.push(args.subject);
        return {
          subject: sa,
          namespace: null,
          groups: ['system:serviceaccounts', 'system:serviceaccounts:prod'],
          bindings: [{ binding, throughGroup: null }],
          scopes: [
            {
              namespace: 'prod',
              grants: {
                resources: [
                  {
                    apiGroup: '',
                    resource: 'secrets',
                    resourceNames: [],
                    verbs: ['get', 'delete'],
                    bindings: [0],
                  },
                ],
                urls: [],
              },
            },
          ],
          refused: [],
        };
      },
    });
    const user = userEvent.setup();
    renderApp('/rbac');
    expect(await screen.findByText('aggregated')).toBeInTheDocument();
    await user.click(screen.getByRole('tab', { name: 'Subjects' }));
    await user.click(await screen.findByText('deployer'));
    expect(await screen.findByText('In prod')).toBeInTheDocument();
    expect(asked).toEqual([sa]);
    expect(screen.queryByText('Cluster-wide', { selector: '[data-slot=card-title]' })).toBeNull();
    const row = screen.getByText('secrets').closest('tr');
    expect(row && within(row).getAllByLabelText('Allowed')).toHaveLength(2);
    expect(screen.getByText('Names it')).toBeInTheDocument();
  });

  it('shows only what the credentials may do when they can’t list RBAC objects', async () => {
    const checks: unknown[] = [];
    mockCommands({
      ...baseCommands,
      ...clusterCommands,
      clusters_rbac_index: () => {
        throw { code: 'forbidden', message: 'Your credentials don’t allow “list” on roles.' };
      },
      clusters_rbac_own: () => own,
      clusters_rbac_can_i: (args) => {
        checks.push(args.check);
        return canI(args);
      },
    });
    const user = userEvent.setup();
    renderApp('/rbac');
    expect(
      await screen.findByText('Your credentials don’t allow “list” on roles.'),
    ).toBeInTheDocument();
    expect(screen.queryByRole('tab', { name: 'Roles' })).toBeNull();
    expect(await screen.findByText('system:admin')).toBeInTheDocument();
    expect(screen.getByText(/may be incomplete/)).toBeInTheDocument();
    expect(screen.getAllByLabelText('Allowed: every verb (*)').length).toBeGreaterThan(0);

    const verb = screen.getByLabelText('Verb');
    await user.clear(verb);
    await user.type(verb, 'delete');
    await user.type(screen.getByLabelText('Resource'), 'secrets');
    const namespace = screen.getByLabelText('Namespace', { selector: 'input' });
    await user.clear(namespace);
    await user.type(namespace, 'prod');
    await user.click(screen.getByRole('button', { name: 'Ask' }));
    expect(await screen.findByText('Yes')).toBeInTheDocument();
    expect(
      screen.getByText('kubectl auth can-i delete secrets --namespace prod'),
    ).toBeInTheDocument();
    await waitFor(() =>
      expect(checks).toEqual([
        {
          verb: 'delete',
          resource: 'secrets',
          namespace: 'prod',
          group: null,
          subresource: null,
          name: null,
        },
      ]),
    );
  });
});
