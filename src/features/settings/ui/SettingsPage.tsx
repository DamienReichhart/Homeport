import { useQuery } from '@tanstack/react-query';
import { settingsQuery } from '@/core/settings';
import { RegistriesCard } from '@/features/registries';
import { RepositoriesCard } from '@/features/repositories';
import { UpdatesCard } from '@/features/updates';
import { ErrorMessage, PageHeader, Spinner } from '@/shared/ui';
import { useSaveSettings } from '../api/mutations';
import { AboutCard } from './AboutCard';
import { AppearanceCard } from './AppearanceCard';
import { DataCard } from './DataCard';
import { DesktopCard } from './DesktopCard';
import { MachineCard } from './MachineCard';
import { SyncCard } from './SyncCard';
import { t } from '@/shared/i18n';

export function SettingsPage() {
  const settings = useQuery(settingsQuery());
  const save = useSaveSettings();

  if (settings.isPending) return <Spinner />;
  if (settings.error)
    return <ErrorMessage error={settings.error} onRetry={() => void settings.refetch()} />;

  return (
    <div className="grid max-w-2xl gap-4">
      <PageHeader title={t('settings.title')} />
      <AppearanceCard
        settings={settings.data}
        onChange={(next) => save.mutate(next)}
        error={save.error}
      />
      <DesktopCard settings={settings.data} onChange={(next) => save.mutate(next)} />
      <UpdatesCard settings={settings.data} onChange={(next) => save.mutate(next)} />
      <RegistriesCard />
      <RepositoriesCard />
      <SyncCard settings={settings.data} onChange={(next) => save.mutate(next)} />
      <DataCard settings={settings.data} onChange={(next) => save.mutate(next)} />
      <MachineCard />
      <AboutCard />
    </div>
  );
}
