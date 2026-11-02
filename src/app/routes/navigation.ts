import { BoxIcon, LayoutDashboardIcon, SettingsIcon, type LucideIcon } from 'lucide-react';

export interface NavItem {
  path: string;
  label: string;
  icon: LucideIcon;
}

/** Sidebar entries. Add one line per new top-level feature. */
export const navItems: NavItem[] = [
  { path: '/', label: 'Overview', icon: LayoutDashboardIcon },
  { path: '/containers', label: 'Containers', icon: BoxIcon },
  { path: '/settings', label: 'Settings', icon: SettingsIcon },
];
