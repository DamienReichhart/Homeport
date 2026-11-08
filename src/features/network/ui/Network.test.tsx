import { screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands } from '@/test/fixtures';
import { clusterCommands } from '@/test/clusters';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

const service = {
  namespace: 'default',
  name: 'web',
  type: 'NodePort',
  clusterIp: '10.96.0.12',
  external: [],
  ports: ['80:30080/TCP'],
  selector: 'app=web',
  created: null,
};

describe('network', () => {
  it('lists Services and shows the Pods one selects', async () => {
    const pods: unknown[] = [];
    mockCommands({
      ...baseCommands,
      ...clusterCommands,
      clusters_services_list: () => [service],
      clusters_service_get: () => ({
        view: service,
        labels: {},
        annotations: {},
        ports: [{ name: 'http', port: 80, targetPort: 'http', nodePort: 30080, protocol: 'TCP' }],
        sessionAffinity: 'None',
      }),
      clusters_pods_list: (args) => {
        pods.push(args);
        return [];
      },
    });
    const user = userEvent.setup();
    renderApp('/services');
    expect(await screen.findByText('80:30080/TCP')).toBeInTheDocument();
    await user.click(screen.getByText('web'));
    expect(await screen.findByText('30080')).toBeInTheDocument();
    await user.click(screen.getByRole('tab', { name: 'Pods' }));
    await waitFor(() =>
      expect(pods).toEqual([{ id: 'c1', namespace: 'default', selector: 'app=web', node: null }]),
    );
  });

  it('links an Ingress’s paths to their Services', async () => {
    mockCommands({
      ...baseCommands,
      ...clusterCommands,
      clusters_ingress_get: () => ({
        view: {
          namespace: 'default',
          name: 'web',
          class: 'nginx',
          hosts: ['shop.example.com'],
          addresses: [],
          tls: false,
          created: null,
        },
        labels: {},
        annotations: {},
        paths: [
          {
            host: 'shop.example.com',
            path: '/api',
            pathType: 'Prefix',
            service: 'api',
            port: 'http',
            resource: null,
          },
        ],
        tls: [],
      }),
    });
    const user = userEvent.setup();
    renderApp('/ingresses/default/web');
    await user.click(await screen.findByRole('tab', { name: 'Rules' }));
    expect(await screen.findByRole('link', { name: 'api:http' })).toHaveAttribute(
      'href',
      '/services/default/api',
    );
  });
});
