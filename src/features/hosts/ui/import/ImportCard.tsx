import { ContainerIcon, FileKeyIcon } from 'lucide-react';
import { useState } from 'react';
import { Button, Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/shared/ui';
import type { ImportSource } from '../../model/importRows';
import { ImportHostsDialog } from './ImportHostsDialog';
import { t } from '@/shared/i18n';

/** Offers to add the Hosts this computer already describes. */
export function ImportCard() {
  const [source, setSource] = useState<ImportSource | null>(null);
  return (
    <Card>
      <CardHeader>
        <CardTitle>{t('hosts.import.title')}</CardTitle>
        <CardDescription>{t('hosts.import.description')}</CardDescription>
      </CardHeader>
      <CardContent className="flex flex-wrap gap-2">
        <Button variant="outline" onClick={() => setSource('sshConfig')}>
          <FileKeyIcon /> {t('hosts.import.sshConfig.button')}
        </Button>
        <Button variant="outline" onClick={() => setSource('dockerContexts')}>
          <ContainerIcon /> {t('hosts.import.dockerContexts.button')}
        </Button>
      </CardContent>
      {source && <ImportHostsDialog source={source} onClose={() => setSource(null)} />}
    </Card>
  );
}
