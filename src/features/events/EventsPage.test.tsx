import { act, screen } from '@testing-library/react';
import { emit } from '@tauri-apps/api/event';
import type { EngineEvent } from '@/shared/api';
import { baseCommands } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';
import { actorName, filterEvents } from './EventsPage';

const event = (kind: string, action: string, name: string): EngineEvent => ({
  time: 1_700_000_000,
  kind,
  action,
  actorId: 'sha256:0123456789abcdef',
  attributes: { name },
});

describe('events', () => {
  it('filters by type and search', () => {
    const list = [
      event('container', 'start', 'web'),
      event('image', 'pull', 'nginx'),
      event('container', 'die', 'db'),
    ];
    expect(filterEvents(list, 'container', '').map(actorName)).toEqual(['web', 'db']);
    expect(filterEvents(list, 'all', 'pul').map(actorName)).toEqual(['nginx']);
  });

  it('appends events pushed by the backend', async () => {
    mockCommands({
      ...baseCommands,
      hosts_events: () => [event('container', 'start', 'web')],
      containers_list: () => [],
    });
    renderApp('/events');
    expect(await screen.findByText('web')).toBeInTheDocument();
    await act(() =>
      emit('engine-event-received', { hostId: 'h1', event: event('network', 'create', 'backend') }),
    );
    expect(await screen.findByText('backend')).toBeInTheDocument();
  });
});
