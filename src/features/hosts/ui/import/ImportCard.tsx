import { ContainerIcon, FileKeyIcon } from 'lucide-react';
import { useId, useState } from 'react';
import { Button } from '@/shared/ui';
import type { ImportSource } from '../../model/importRows';
import { ImportHostsDialog } from './ImportHostsDialog';
import { t } from '@/shared/i18n';

/** Offers to add the Hosts this computer already describes, in one step each. */
export function ImportCard() {
  const [source, setSource] = useState<ImportSource | null>(null);
  const id = useId();
  return (
    <section aria-labelledby={id} className="grid gap-3">
      <div className="grid gap-1">
        <h2 id={id} className="text-base font-semibold">
          {t('hosts.import.title')}
        </h2>
        <p className="text-muted-foreground text-sm">{t('hosts.import.description')}</p>
      </div>
      <div className="flex flex-wrap gap-2">
        <Button variant="outline" onClick={() => setSource('sshConfig')}>
          <FileKeyIcon /> {t('hosts.import.sshConfig.button')}
        </Button>
        <Button variant="outline" onClick={() => setSource('dockerContexts')}>
          <ContainerIcon /> {t('hosts.import.dockerContexts.button')}
        </Button>
      </div>
      {source && <ImportHostsDialog source={source} onClose={() => setSource(null)} />}
    </section>
  );
}
