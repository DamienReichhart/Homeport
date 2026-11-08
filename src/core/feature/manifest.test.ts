import { BoxIcon } from 'lucide-react';
import { defineFeature, featureScope, visibleNavEntries, type NavEntry } from './manifest';

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

describe('featureScope', () => {
  it('is the sidebar entry’s, or the app’s without one', () => {
    expect(featureScope(defineFeature({ routes: [], nav: entry('/x', 'host') }))).toBe('host');
    expect(featureScope(defineFeature({ routes: [] }))).toBe('app');
  });
});
