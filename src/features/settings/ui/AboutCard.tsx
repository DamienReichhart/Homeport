import { useQuery } from '@tanstack/react-query';
import { FolderOpenIcon } from 'lucide-react';
import { useState } from 'react';
import { appQueries } from '@/shared/api';
import { Button, Card, CardContent, LogoMark } from '@/shared/ui';
import { openLogFolder } from '../api/mutations';
import { LicensesDialog } from './dialogs/LicensesDialog';
import { t } from '@/shared/i18n';

/** Homeport's version, its log folder and its licenses. */
export function AboutCard() {
  const info = useQuery(appQueries.info());
  const [licensesOpen, setLicensesOpen] = useState(false);
  return (
    <>
      <Card>
        <CardContent className="flex items-center gap-4">
          <LogoMark className="size-12" />
          <div className="min-w-0">
            <p className="font-semibold tracking-tight">Homeport</p>
            <p className="text-muted-foreground text-sm">{t('settings.about.tagline')}</p>
            <p className="text-muted-foreground font-mono text-xs">
              {info.data ? `v${info.data.version} · ${info.data.os}/${info.data.arch}` : '…'}
            </p>
          </div>
          <Button variant="outline" size="sm" className="ms-auto" onClick={openLogFolder}>
            <FolderOpenIcon /> {t('settings.about.openLogFolder')}
          </Button>
          <Button variant="outline" size="sm" onClick={() => setLicensesOpen(true)}>
            {t('settings.about.licenses')}
          </Button>
        </CardContent>
      </Card>
      <LicensesDialog open={licensesOpen} onOpenChange={setLicensesOpen} />
    </>
  );
}
