import { BoxIcon } from 'lucide-react';
import {
  defineFeature,
  featureScope,
  navEntriesOf,
  visibleNavEntries,
  type NavEntry,
} from './manifest';

const entry = (path: string, scope: NavEntry['scope']): NavEntry => ({
  path,
  label: 'overview.nav',
  icon: BoxIcon,
  scope,
});
const entries = [entry('/', 'app'), entry('/containers', 'host'), entry('/pods', 'cluster')];

describe('visibleNavEntries', () => {
  it('shows the app’s and the Host’s entries while a Host is selected', () => {
    expect(visibleNavEntries(entries, 'host').map((e) => e.path)).toEqual(['/', '/containers']);
  });

  it('shows the app’s and the Cluster’s entries while a Cluster is selected', () => {
    expect(visibleNavEntries(entries, 'cluster').map((e) => e.path)).toEqual(['/', '/pods']);
  });

  it('shows the Host entries while nothing is selected, as before Clusters', () => {
    expect(visibleNavEntries(entries, null).map((e) => e.path)).toEqual(['/', '/containers']);
  });
});

describe('what a Host page requires', () => {
  const podman = { ...entry('/podman-pods', 'host'), requires: 'podman' as const };
  const all = [...entries, podman];

  it('hides the entries the selected Host doesn’t support', () => {
    const docker = (c: string) => c === 'docker';
    expect(visibleNavEntries(all, 'host', docker).map((e) => e.path)).toEqual(['/', '/containers']);
    expect(visibleNavEntries(all, 'host', () => true).map((e) => e.path)).toContain('/podman-pods');
  });

  it('leaves the other scopes alone', () => {
    expect(visibleNavEntries(all, 'cluster', () => false).map((e) => e.path)).toEqual([
      '/',
      '/pods',
    ]);
  });

  it('carries each feature’s requirement onto its sidebar entry', () => {
    const navs = navEntriesOf([
      defineFeature({ routes: [], nav: entry('/a', 'host'), requires: 'podman' }),
      defineFeature({ routes: [], nav: entry('/b', 'host') }),
      defineFeature({ routes: [] }),
    ]);
    expect(navs.map((e) => [e.path, e.requires])).toEqual([
      ['/a', 'podman'],
      ['/b', undefined],
    ]);
  });
});

describe('featureScope', () => {
  it('is the sidebar entry’s, or the app’s without one', () => {
    expect(featureScope(defineFeature({ routes: [], nav: entry('/x', 'host') }))).toBe('host');
    expect(featureScope(defineFeature({ routes: [] }))).toBe('app');
  });
});
