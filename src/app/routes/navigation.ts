import {
  ActivityIcon,
  BoxIcon,
  BoxesIcon,
  HardDriveIcon,
  LayersIcon,
  LayoutDashboardIcon,
  NetworkIcon,
  SettingsIcon,
  type LucideIcon,
} from 'lucide-react';

export interface NavItem {
  path: string;
  label: string;
  icon: LucideIcon;
}

/** Sidebar entries. Add one line per new top-level feature. */
export const navItems: NavItem[] = [
  { path: '/', label: 'Overview', icon: LayoutDashboardIcon },
  { path: '/containers', label: 'Containers', icon: BoxIcon },
  { path: '/images', label: 'Images', icon: LayersIcon },
  { path: '/volumes', label: 'Volumes', icon: HardDriveIcon },
  { path: '/networks', label: 'Networks', icon: NetworkIcon },
  { path: '/stacks', label: 'Stacks', icon: BoxesIcon },
  { path: '/events', label: 'Events', icon: ActivityIcon },
  { path: '/settings', label: 'Settings', icon: SettingsIcon },
];
