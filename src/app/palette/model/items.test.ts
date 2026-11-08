import { BoxIcon } from 'lucide-react';
import { defineFeature } from '@/core/feature';
import { connectedHost, hostView } from '@/test/fixtures';
import { paletteItems, recentItems, remember, score, type PaletteSource } from './items';

const containers = defineFeature({
  routes: [],
  nav: { path: '/containers', label: 'containers.nav', icon: BoxIcon, scope: 'host' },
  commands: [
    {
      id: 'containers:run',
      label: 'containers.palette.run',
      needsHost: true,
      run: () => {},
    },
  ],
});
const volumes = defineFeature({
  routes: [],
  nav: { path: '/volumes', label: 'volumes.nav', icon: BoxIcon, scope: 'host' },
  commands: [{ id: 'volumes:create', label: 'volumes.create', run: () => {} }],
});
const unreachable = hostView('prod-server', {
  id: 'h2',
  status: {
    state: 'unreachable',
    retryInSecs: 4,
    error: { code: 'timeout', key: '', params: {}, message: 'Timed out', hint: null, detail: null },
  },
});

const source = (over: Partial<PaletteSource> = {}): PaletteSource => ({
  features: [containers, volumes],
  appCommands: [],
  hosts: [connectedHost, unreachable],
  selected: connectedHost,
  navKeys: new Map([['/containers', 'Mod+2']]),
  ...over,
});

describe('palette items', () => {
  it('lists the pages, every host with its status, then the commands', () => {
    const items = paletteItems(source());
    expect(items.map((i) => [i.group, i.label])).toEqual([
      ['Go to', 'Go to Containers'],
      ['Go to', 'Go to Volumes'],
      ['Hosts', 'Switch to local'],
      ['Hosts', 'Switch to prod-server'],
      ['Actions', 'Run a container'],
      ['Actions', 'Create volume'],
    ]);
    expect(items[0]?.shortcut).toBe('Mod+2');
    expect(items[3]?.status?.label).toBe('Unreachable');
  });

  it('hides commands that need a connected host when the selected one isn’t', () => {
    const labels = paletteItems(source({ selected: unreachable })).map((i) => i.label);
    expect(labels).not.toContain('Run a container');
    expect(labels).toContain('Create volume');
  });

  it('ranks pages first, so "vol" opens Volumes', () => {
    const items = paletteItems(source());
    const best = [...items].sort((a, b) => score(b, 'vol') - score(a, 'vol'))[0];
    expect(best?.label).toBe('Go to Volumes');
    expect(score(items[0]!, 'vol')).toBe(0);
  });

  it('finds a host by its name', () => {
    const items = paletteItems(source());
    const best = [...items].sort((a, b) => score(b, 'prod') - score(a, 'prod'))[0];
    expect(best?.label).toBe('Switch to prod-server');
  });

  it('remembers the last few items, most recent first', () => {
    let recent: string[] = [];
    for (const id of ['a', 'b', 'c', 'a', 'd', 'e', 'f']) recent = remember(recent, id);
    expect(recent).toEqual(['f', 'e', 'd', 'a', 'c']);
    const items = paletteItems(source());
    expect(recentItems(items, ['nav:/volumes', 'gone']).map((i) => i.label)).toEqual([
      'Go to Volumes',
    ]);
  });
});
