import { Outlet } from 'react-router-dom';
import { UpdateNotice } from '@/features/updates';
import { KeychainWarning } from './KeychainWarning';
import { AppShortcuts } from '../shortcuts/AppShortcuts';
import { ShortcutsHelpProvider } from '../shortcuts/ShortcutsHelp';
import { Sidebar } from './Sidebar';
import { TrayChoices } from './TrayChoices';

export function AppLayout() {
  return (
    <ShortcutsHelpProvider>
      <div className="flex h-screen overflow-hidden">
        <TrayChoices />
        <AppShortcuts />
        <Sidebar />
        <main className="flex min-w-0 flex-1 flex-col overflow-hidden">
          <UpdateNotice />
          <KeychainWarning />
          <div className="min-h-0 flex-1 overflow-auto p-6">
            <Outlet />
          </div>
        </main>
      </div>
    </ShortcutsHelpProvider>
  );
}
