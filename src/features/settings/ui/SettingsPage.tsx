import { useQuery } from '@tanstack/react-query';
import { settingsQuery } from '@/core/settings';
import { RegistriesCard } from '@/features/registries';
import { UpdatesCard } from '@/features/updates';
import { ErrorMessage, PageHeader, Spinner } from '@/shared/ui';
import { useSaveSettings } from '../api/mutations';
import { AboutCard } from './AboutCard';
import { AppearanceCard } from './AppearanceCard';
import { MachineCard } from './MachineCard';

export function SettingsPage() {
  const settings = useQuery(settingsQuery());
  const save = useSaveSettings();

  if (settings.isPending) return <Spinner />;
  if (settings.error)
    return <ErrorMessage error={settings.error} onRetry={() => void settings.refetch()} />;

  return (
    <div className="grid max-w-2xl gap-4">
      <PageHeader title="Settings" />
      <AppearanceCard
        settings={settings.data}
        onChange={(next) => save.mutate(next)}
        error={save.error}
      />
      <UpdatesCard settings={settings.data} onChange={(next) => save.mutate(next)} />
      <RegistriesCard />
      <MachineCard />
      <AboutCard />
    </div>
  );
}
