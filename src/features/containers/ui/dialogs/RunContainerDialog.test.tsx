import { screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { MemoryRouter } from 'react-router-dom';
import type { RunSpec } from '@/shared/api';
import { baseCommands, podmanHost } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderWithProviders } from '@/test/render';
import { RunContainerDialog } from './RunContainerDialog';

function renderDialog() {
  const sent: RunSpec[] = [];
  mockCommands({
    ...baseCommands,
    images_list: () => [],
    networks_list: () => [],
    containers_list: () => [],
    containers_run: ({ spec }) => {
      sent.push(spec as RunSpec);
      return 'new-id';
    },
  });
  renderWithProviders(
    <MemoryRouter>
      <RunContainerDialog hostId="h1" image="nginx:1.27" open onOpenChange={() => {}} />
    </MemoryRouter>,
  );
  return sent;
}

function renderOnPodman() {
  const sent: { spec: RunSpec; pod: unknown }[] = [];
  mockCommands({
    ...baseCommands,
    hosts_list: () => [podmanHost],
    images_list: () => [],
    networks_list: () => [],
    containers_list: () => [],
    pods_list: () => [
      {
        id: 'p1',
        name: 'shop',
        status: 'running',
        created: '',
        infraId: null,
        labels: {},
        networks: [],
        containers: [],
      },
    ],
    containers_run: ({ spec, pod }) => {
      sent.push({ spec: spec as RunSpec, pod });
      return 'new-id';
    },
  });
  renderWithProviders(
    <MemoryRouter>
      <RunContainerDialog hostId="h1" image="nginx:1.27" open onOpenChange={() => {}} />
    </MemoryRouter>,
  );
  return sent;
}

describe('run dialog in a pod', () => {
  it('isn’t offered on a Docker Engine', async () => {
    renderDialog();
    expect(await screen.findByText('Advanced options')).toBeInTheDocument();
    expect(screen.queryByRole('combobox', { name: 'Pod' })).toBeNull();
    expect(screen.getByText('Ports')).toBeInTheDocument();
  });

  it('runs into the pod picked on Podman, leaving ports to it', async () => {
    const sent = renderOnPodman();
    const user = userEvent.setup();
    await user.click(await screen.findByRole('combobox', { name: 'Pod' }));
    await user.click(await screen.findByRole('option', { name: 'shop' }));
    expect(screen.queryByText('Ports')).toBeNull();
    expect(screen.getByText(/Containers in shop share its network/)).toBeInTheDocument();
    await user.click(screen.getByRole('button', { name: 'Run' }));
    await waitFor(() => expect(sent).toHaveLength(1));
    expect(sent[0]!.pod).toBe('shop');
    expect(sent[0]!.spec).toMatchObject({ image: 'nginx:1.27', ports: [], network: null });
  });

  it('runs outside any pod unless one is picked', async () => {
    const sent = renderOnPodman();
    const user = userEvent.setup();
    await screen.findByRole('combobox', { name: 'Pod' });
    await user.click(screen.getByRole('button', { name: 'Run' }));
    await waitFor(() => expect(sent).toHaveLength(1));
    expect(sent[0]!.pod).toBeNull();
  });
});

describe('run dialog', () => {
  it('keeps the advanced options out of the way until asked', async () => {
    renderDialog();
    expect(await screen.findByText('Advanced options')).toBeInTheDocument();
    expect(screen.queryByLabelText('CPU limit')).toBeNull();
  });

  it('sends limits and labels set in the advanced area', async () => {
    const sent = renderDialog();
    const user = userEvent.setup();
    await user.click(await screen.findByText('Advanced options'));
    await user.type(screen.getByLabelText('CPU limit'), '0.5');
    await user.type(screen.getByLabelText('Memory limit'), '512m');
    await user.click(screen.getByRole('tab', { name: 'Labels' }));
    await user.type(screen.getByRole('textbox', { name: 'Labels' }), 'team=web{Enter}tier=front');
    await user.click(screen.getByRole('button', { name: 'Run' }));

    await waitFor(() => expect(sent).toHaveLength(1));
    expect(sent[0]).toMatchObject({
      image: 'nginx:1.27',
      cpus: 0.5,
      memory: 512 * 1024 ** 2,
      labels: ['team=web', 'tier=front'],
    });
  });

  it('explains a size it cannot read and holds the run back', async () => {
    const sent = renderDialog();
    const user = userEvent.setup();
    await user.click(await screen.findByText('Advanced options'));
    await user.type(screen.getByLabelText('Memory limit'), 'lots');

    expect(screen.getByText('Write a size like 512m or 2g.')).toBeInTheDocument();
    expect(screen.getByLabelText('Memory limit')).toHaveAttribute('aria-invalid', 'true');
    expect(screen.getByRole('button', { name: 'Run' })).toBeDisabled();
    expect(sent).toHaveLength(0);
  });

  it('adds capabilities as tokens and warns about privileged mode', async () => {
    const sent = renderDialog();
    const user = userEvent.setup();
    await user.click(await screen.findByText('Advanced options'));
    await user.click(screen.getByRole('tab', { name: 'Security' }));
    await user.type(screen.getByLabelText('Add capabilities'), 'cap_net_admin{Enter}');
    expect(screen.getByText('NET_ADMIN')).toBeInTheDocument();

    await user.click(screen.getByRole('switch', { name: 'Privileged' }));
    expect(screen.getByText('The container gets full access to the host')).toBeInTheDocument();
    await user.click(screen.getByRole('button', { name: 'Run' }));
    await waitFor(() => expect(sent).toHaveLength(1));
    expect(sent[0]).toMatchObject({ capAdd: ['NET_ADMIN'], privileged: true });
  });

  it('sends a healthcheck with its durations in seconds', async () => {
    const sent = renderDialog();
    const user = userEvent.setup();
    await user.click(await screen.findByText('Advanced options'));
    await user.click(screen.getByRole('tab', { name: 'Healthcheck' }));
    await user.click(screen.getByRole('combobox', { name: 'Healthcheck' }));
    await user.click(await screen.findByRole('option', { name: 'A command' }));
    await user.type(screen.getByLabelText('Check command'), 'true');
    await user.type(screen.getByLabelText('Interval'), '1m30s');
    await user.click(screen.getByRole('button', { name: 'Run' }));

    await waitFor(() => expect(sent).toHaveLength(1));
    expect(sent[0]?.healthcheck).toMatchObject({ command: 'true', intervalS: 90, disabled: false });
  });
});
