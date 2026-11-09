import { screen, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { useState } from 'react';
import { emptyForm, type HostFormState } from '../../model/form';
import { HostForm } from './HostForm';
import { baseCommands } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderWithProviders } from '@/test/render';

/** The form with its own state, reporting each change. */
function Harness({
  initial = emptyForm('ssh'),
  editing = false,
  keychain = true,
  onChange = () => {},
}: {
  initial?: HostFormState;
  editing?: boolean;
  keychain?: boolean;
  onChange?: (form: HostFormState) => void;
}) {
  const [form, setForm] = useState(initial);
  return (
    <HostForm
      form={form}
      editing={editing}
      keychain={keychain}
      onChange={(next) => {
        setForm(next);
        onChange(next);
      }}
    />
  );
}

describe('host form', () => {
  beforeEach(() => mockCommands(baseCommands));

  it('adds Incus over HTTPS with a trust token, or with an existing certificate', async () => {
    const user = userEvent.setup();
    let last: HostFormState | null = null;
    mockCommands({
      ...baseCommands,
      incus_trust_token_read: () => ({
        clientName: 'homeport',
        fingerprint: '8d6a1f0a3c',
        addresses: ['10.0.0.5:8443'],
        expiresAt: null,
      }),
    });
    renderWithProviders(
      <Harness initial={{ ...emptyForm('tls'), engine: 'incus' }} onChange={(f) => (last = f)} />,
    );
    // The token names the addresses; there's nothing else to fill in.
    expect(screen.queryByText('Address')).toBeNull();
    await user.type(screen.getByRole('textbox', { name: 'Trust token' }), 'eyJjbGllbnRfbmFtZSI6');
    expect(await screen.findByText('For homeport, at 10.0.0.5:8443')).toBeInTheDocument();
    expect(screen.getByText('Server certificate 8d6a1f0a3c')).toBeInTheDocument();
    expect(last!.trustToken).toBe('eyJjbGllbnRfbmFtZSI6');

    await user.click(screen.getByRole('radio', { name: 'Existing certificate' }));
    expect(screen.getByText('Address')).toBeInTheDocument();
    expect(screen.getByText('Server certificate to trust')).toBeInTheDocument();
    expect(screen.getByText('Client key')).toBeInTheDocument();
    // Plain TCP isn't offered for Incus.
    expect(screen.getByRole('radio', { name: 'TCP' })).toBeDisabled();
  });

  it('shows the fields of the kind of Host picked', async () => {
    const user = userEvent.setup();
    renderWithProviders(<Harness />);
    expect(screen.getByText('Server')).toBeInTheDocument();
    expect(screen.getByText('Remote socket path')).toBeInTheDocument();

    await user.click(screen.getByRole('radio', { name: 'TLS' }));
    expect(screen.queryByText('Server')).toBeNull();
    expect(screen.getByText('Address')).toBeInTheDocument();
    expect(screen.getByText('CA certificate')).toBeInTheDocument();

    await user.click(screen.getByRole('radio', { name: 'TCP' }));
    expect(screen.getByText('Unencrypted and unauthenticated')).toBeInTheDocument();
    expect(screen.queryByText('CA certificate')).toBeNull();

    await user.click(screen.getByRole('radio', { name: 'Local socket' }));
    expect(screen.getByText('Socket path')).toBeInTheDocument();
    expect(screen.queryByText('Address')).toBeNull();
  });

  it('offers Docker-compatible Engines, containerd and Incus, preset from the socket', async () => {
    const user = userEvent.setup();
    let last: HostFormState | null = null;
    renderWithProviders(<Harness initial={emptyForm('local')} onChange={(f) => (last = f)} />);
    const engine = screen.getByRole('combobox', { name: 'Engine' });
    expect(engine).toHaveTextContent('Docker-compatible');
    await user.click(engine);
    expect(await screen.findByRole('option', { name: 'Incus' })).not.toHaveAttribute(
      'aria-disabled',
      'true',
    );
    expect(screen.getByRole('option', { name: 'containerd' })).not.toHaveAttribute(
      'aria-disabled',
      'true',
    );
    await user.keyboard('{Escape}');
    const socket = screen.getByDisplayValue('/var/run/docker.sock');
    await user.clear(socket);
    await user.type(socket, '/run/containerd/containerd.sock');
    expect(last!.engine).toBe('containerd');
    await user.clear(socket);
    await user.type(socket, '/var/lib/incus/unix.socket');
    expect(last!.engine).toBe('incus');
  });

  it('reaches containerd only through its socket, here or over SSH', async () => {
    const user = userEvent.setup();
    let last: HostFormState | null = null;
    renderWithProviders(<Harness initial={emptyForm('tcp')} onChange={(f) => (last = f)} />);
    await user.click(screen.getByRole('combobox', { name: 'Engine' }));
    await user.click(await screen.findByRole('option', { name: 'containerd' }));
    expect(last!.kind).toBe('ssh');
    expect(last!.sshSocketPath).toBe('/run/containerd/containerd.sock');
    expect(screen.getByRole('radio', { name: 'TCP' })).toBeDisabled();
    expect(screen.getByRole('radio', { name: 'TLS' })).toBeDisabled();
  });

  it('asks for a TLS Host’s three PEM files', async () => {
    const user = userEvent.setup();
    renderWithProviders(<Harness initial={emptyForm('tls')} />);
    for (const label of ['CA certificate', 'Client certificate', 'Client key']) {
      expect(screen.getByText(label)).toBeInTheDocument();
    }
    expect(screen.getAllByRole('button', { name: /Load from file/ })).toHaveLength(3);
    await user.click(screen.getByRole('radio', { name: 'SSH' }));
    expect(screen.queryByText('Client key')).toBeNull();
  });

  it('offers a generated key, an imported key or ssh-agent for SSH', async () => {
    const changes: HostFormState[] = [];
    const user = userEvent.setup();
    renderWithProviders(<Harness onChange={(f) => changes.push(f)} />);
    expect(screen.getByText(/creates an Ed25519 key for this host/)).toBeInTheDocument();
    expect(screen.queryByText('Private key')).toBeNull();

    await user.click(screen.getByRole('radio', { name: 'Existing key' }));
    expect(screen.getByText('Private key')).toBeInTheDocument();
    expect(screen.getByText('Passphrase')).toBeInTheDocument();

    await user.click(screen.getByRole('radio', { name: 'ssh-agent' }));
    expect(screen.queryByText('Private key')).toBeNull();
    expect(changes.at(-1)?.sshAuth).toBe('agent');
  });

  it('hides the kind picker when editing and keeps stored credentials by default', () => {
    renderWithProviders(<Harness initial={emptyForm('tls')} editing />);
    expect(screen.queryByRole('radio', { name: 'SSH' })).toBeNull();
    expect(screen.getAllByText('Leave blank to keep the stored value.')).toHaveLength(3);
  });

  it('leaves only ssh-agent when there is no keychain', async () => {
    const user = userEvent.setup();
    renderWithProviders(<Harness keychain={false} />);
    expect(screen.getByText('No OS keychain')).toBeInTheDocument();
    expect(screen.getByRole('radio', { name: /New key for this host/ })).toBeDisabled();
    expect(screen.getByRole('radio', { name: 'Existing key' })).toBeDisabled();
    await user.click(screen.getByRole('radio', { name: 'ssh-agent' }));
    expect(screen.queryByText('No OS keychain')).toBeNull();

    await user.click(screen.getByRole('radio', { name: 'TLS' }));
    expect(screen.getByText('No OS keychain')).toBeInTheDocument();
    expect(screen.queryByText('CA certificate')).toBeNull();
  });

  it('adds, reorders and removes bastions', async () => {
    const changes: HostFormState[] = [];
    const user = userEvent.setup();
    renderWithProviders(<Harness onChange={(f) => changes.push(f)} />);
    const add = screen.getByRole('button', { name: 'Add bastion' });
    await user.click(add);
    await user.click(add);
    await user.type(screen.getByLabelText('Server of bastion 1'), 'outer');
    await user.type(screen.getByLabelText('Server of bastion 2'), 'inner');
    expect(screen.getByLabelText('Move bastion 1 up')).toBeDisabled();
    expect(screen.getByLabelText('Move bastion 2 down')).toBeDisabled();

    await user.click(screen.getByLabelText('Move bastion 2 up'));
    expect(changes.at(-1)?.bastions.map((b) => b.host)).toEqual(['inner', 'outer']);

    await user.click(screen.getByLabelText('Remove bastion 1'));
    expect(changes.at(-1)?.bastions.map((b) => b.host)).toEqual(['outer']);
  });

  it('asks for a bastion’s own key only when it uses one', async () => {
    const user = userEvent.setup();
    renderWithProviders(<Harness />);
    await user.click(screen.getByRole('button', { name: 'Add bastion' }));
    const bastion = screen.getByRole('group', { name: 'Bastion 1' });
    expect(within(bastion).queryByText('Private key')).toBeNull();
    await user.click(within(bastion).getByRole('radio', { name: 'Its own key' }));
    expect(within(bastion).getByText('Private key')).toBeInTheDocument();
  });
});
