import { useQuery } from '@tanstack/react-query';
import { useEffect, type ReactNode } from 'react';
import { useLocation } from 'react-router-dom';
import { settingsQuery } from '@/core/settings';
import { RegistriesCard } from '@/features/registries';
import { RepositoriesCard } from '@/features/repositories';
import { UpdatesCard } from '@/features/updates';
import type { Settings } from '@/shared/api';
import { ErrorMessage, PageHeader, Spinner } from '@/shared/ui';
import { useSaveSettings } from '../api/mutations';
import { AboutCard } from './AboutCard';
import { AppearanceCard } from './AppearanceCard';
import { DataCard } from './DataCard';
import { DesktopCard } from './DesktopCard';
import { MachineCard } from './MachineCard';
import { NotificationsCard } from './NotificationsCard';
import { goToSection, sectionId, type SettingsSection } from './sections';
import { SettingsIndex } from './SettingsIndex';
import { SyncCard } from './SyncCard';
import { t } from '@/shared/i18n';

type Save = (next: Settings) => void;

interface Section extends SettingsSection {
  render: (settings: Settings, save: Save, error: Error | null) => ReactNode;
}

/** The sections, most often changed first; the page's index lists them in this order. */
const SECTIONS: Section[] = [
  {
    id: 'appearance',
    label: 'settings.appearance.title',
    render: (s, save, error) => <AppearanceCard settings={s} onChange={save} error={error} />,
  },
  { id: 'registries', label: 'registries.title', render: () => <RegistriesCard /> },
  { id: 'repositories', label: 'repositories.title', render: () => <RepositoriesCard /> },
  {
    id: 'sync',
    label: 'settings.sync.title',
    render: (s, save) => <SyncCard settings={s} onChange={save} />,
  },
  {
    id: 'notifications',
    label: 'settings.notifications.title',
    render: (s, save) => <NotificationsCard settings={s} onChange={save} />,
  },
  {
    id: 'desktop',
    label: 'settings.desktop.title',
    render: (s, save) => <DesktopCard settings={s} onChange={save} />,
  },
  {
    id: 'updates',
    label: 'updates.title',
    render: (s, save) => <UpdatesCard settings={s} onChange={save} />,
  },
  {
    id: 'data',
    label: 'settings.data.title',
    render: (s, save) => <DataCard settings={s} onChange={save} />,
  },
  { id: 'machine', label: 'settings.machine.title', render: () => <MachineCard /> },
  { id: 'about', label: 'settings.about.title', render: () => <AboutCard /> },
];

/**
 * Every setting on one page, with an index beside it on a wide window so any
 * section is one click away. `/settings#updates` opens at that section.
 */
export function SettingsPage() {
  const settings = useQuery(settingsQuery());
  const save = useSaveSettings();
  const { hash } = useLocation();
  const loaded = !!settings.data;
  useEffect(() => {
    const id = hash.replace(/^#/, '');
    if (loaded && SECTIONS.some((s) => s.id === id)) goToSection(id);
  }, [hash, loaded]);

  if (settings.isPending) return <Spinner />;
  if (settings.error)
    return <ErrorMessage error={settings.error} onRetry={() => void settings.refetch()} />;

  const onChange: Save = (next) => save.mutate(next);
  return (
    <div>
      <PageHeader title={t('settings.title')} />
      <div className="flex items-start gap-10">
        <SettingsIndex sections={SECTIONS} variant="aside" />
        <div className="grid max-w-2xl min-w-0 flex-1 gap-4">
          <SettingsIndex sections={SECTIONS} variant="row" />
          {SECTIONS.map(({ id, label, render }) => (
            <section
              key={id}
              id={sectionId(id)}
              aria-label={t(label)}
              tabIndex={-1}
              className="scroll-mt-2 outline-none"
            >
              {render(settings.data, onChange, save.error)}
            </section>
          ))}
        </div>
      </div>
    </div>
  );
}
