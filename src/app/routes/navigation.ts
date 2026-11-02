export interface NavItem {
  path: string;
  label: string;
}

/** Sidebar entries. Add one line per new top-level feature. */
export const navItems: NavItem[] = [
  { path: '/', label: 'Home' },
  { path: '/settings', label: 'Settings' },
];
