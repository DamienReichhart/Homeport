import type { ReactNode } from 'react';
import { useNavigate } from 'react-router-dom';
import { PageHeader, Tabs, TabsList, TabsTrigger } from '@/shared/ui';
import { sections } from './sections';
import { t } from '@/shared/i18n';

/** The Helm pages' title, their tabs (Releases, Chart repositories) and a toolbar. */
export function HelmPageHeader({
  current,
  description,
  actions,
  toolbar,
}: {
  current: string;
  description?: ReactNode;
  actions?: ReactNode;
  toolbar?: ReactNode;
}) {
  const navigate = useNavigate();
  return (
    <div>
      <PageHeader title={t('helm.title')} description={description} actions={actions} />
      <div className="flex flex-wrap items-center gap-3">
        <Tabs value={current} onValueChange={(path) => void navigate(path, { replace: true })}>
          <TabsList>
            {sections().map((s) => (
              <TabsTrigger
                key={s.path}
                value={s.path}
                // A tab that switches what one view shows, without a panel of its own.
                aria-controls={undefined}
              >
                {s.label}
              </TabsTrigger>
            ))}
          </TabsList>
        </Tabs>
        {toolbar}
      </div>
    </div>
  );
}
