import { act, screen, waitFor } from '@testing-library/react';
import { emit } from '@tauri-apps/api/event';
import type { Settings } from '@/shared/api';
import { baseCommands, connectedHost, hostView } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

describe('tray choices', () => {
  it('opens the Overview of the Host picked in the tray', async () => {
    const saved: Settings[] = [];
    mockCommands({
      ...baseCommands,
      hosts_list: () => [connectedHost, hostView('prod', { id: 'h2' })],
      update_settings: ({ settings }) => {
        saved.push(settings as Settings);
        return settings;
      },
      system_info: () => {
        throw new Error('not needed');
      },
      system_disk_usage: () => {
        throw new Error('not needed');
      },
      hosts_events: () => [],
      containers_list: () => [],
      usage_subscribe: () => null,
    });
    renderApp('/settings');
    expect(await screen.findByRole('heading', { name: 'Settings' })).toBeInTheDocument();
    await act(() => emit('tray-host-chosen', { hostId: 'h2' }));
    expect(await screen.findByRole('heading', { name: 'Overview' })).toBeInTheDocument();
    await waitFor(() => expect(saved.at(-1)?.lastHostId).toBe('h2'));
  });

  it('opens Add host from the tray', async () => {
    mockCommands({ ...baseCommands, hosts_detect_local: () => [] });
    renderApp('/settings');
    expect(await screen.findByRole('heading', { name: 'Settings' })).toBeInTheDocument();
    await act(() => emit('tray-add-host-chosen', null));
    expect(await screen.findByRole('heading', { name: 'Add host' })).toBeInTheDocument();
  });
});
