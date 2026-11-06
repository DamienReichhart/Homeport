import { act, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { emit } from '@tauri-apps/api/event';
import type { EngineEvent } from '@/shared/api';
import { baseCommands } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

const event = (kind: string, action: string, name: string, at = 1_700_000_000): EngineEvent => ({
  time: at,
  timeNano: at * 1_000_000_000,
  kind,
  action,
  actorId: 'sha256:0123456789abcdef',
  attributes: { name },
});

describe('events page', () => {
  it('appends events pushed by the backend, once each', async () => {
    mockCommands({
      ...baseCommands,
      hosts_events: () => [event('container', 'start', 'web')],
      containers_list: () => [],
    });
    renderApp('/events');
    expect(await screen.findByText('web')).toBeInTheDocument();
    expect(
      screen.getByText('What the Engine reported, kept for 30 days, up to 10,000.'),
    ).toBeInTheDocument();
    await act(() =>
      emit('engine-event-received', { hostId: 'h1', event: event('network', 'create', 'backend') }),
    );
    expect(await screen.findByText('backend')).toBeInTheDocument();
    // A reconnection asks again from the last kept event.
    await act(() =>
      emit('engine-event-received', { hostId: 'h1', event: event('container', 'start', 'web') }),
    );
    expect(screen.getAllByText('web')).toHaveLength(1);
  });

  it('loads older pages from the oldest event listed', async () => {
    const latest = Array.from({ length: 1000 }, (_, i) =>
      event('container', 'start', `recent-${i}`, 1_700_001_000 + i),
    );
    const asked: unknown[] = [];
    mockCommands({
      ...baseCommands,
      containers_list: () => [],
      hosts_events: ({ before }) => {
        asked.push(before);
        return before ? [event('container', 'die', 'ancient', 1_600_000_000)] : latest;
      },
    });
    const user = userEvent.setup();
    renderApp('/events');
    await user.click(await screen.findByRole('button', { name: 'Load older events' }));
    expect(await screen.findByText('ancient')).toBeInTheDocument();
    expect(asked[1]).toEqual({
      timeNano: 1_700_001_000 * 1_000_000_000,
      kind: 'container',
      action: 'start',
      actorId: 'sha256:0123456789abcdef',
    });
    // A short page was the last one.
    expect(screen.queryByRole('button', { name: 'Load older events' })).toBeNull();
  });

  it('clears the history after asking', async () => {
    const cleared: unknown[] = [];
    mockCommands({
      ...baseCommands,
      containers_list: () => [],
      hosts_events: () => [event('container', 'start', 'web')],
      hosts_events_clear: ({ id }) => {
        cleared.push(id);
        return null;
      },
    });
    const user = userEvent.setup();
    renderApp('/events');
    await user.click(await screen.findByRole('button', { name: 'Clear history' }));
    expect(
      await screen.findByText(/Every event kept for this Host is forgotten/),
    ).toBeInTheDocument();
    const buttons = screen.getAllByRole('button', { name: 'Clear history' });
    await user.click(buttons[buttons.length - 1]!);
    await waitFor(() => expect(cleared).toEqual(['h1']));
    expect(await screen.findByText('No events yet')).toBeInTheDocument();
  });
});
