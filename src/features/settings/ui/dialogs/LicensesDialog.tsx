import { useQuery } from '@tanstack/react-query';
import { openUrl } from '@tauri-apps/plugin-opener';
import { ExternalLinkIcon } from 'lucide-react';
import { appQueries } from '@/shared/api';
import {
  Button,
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  EmptyState,
  ErrorMessage,
  Spinner,
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
  toastError,
} from '@/shared/ui';
import { sourceUrl } from '../../model/source';
import { licensesQuery } from '../../api/queries';
import { t, tRich } from '@/shared/i18n';

const TEXT = 'bg-muted h-[60vh] overflow-auto rounded-md p-3 font-mono text-xs whitespace-pre-wrap';

/** Homeport's license and the notices of every package it bundles. */
export function LicensesDialog({
  open,
  onOpenChange,
}: {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}) {
  const licenses = useQuery({ ...licensesQuery(), enabled: open });
  const info = useQuery({ ...appQueries.info(), enabled: open });
  const version = info.data?.version;

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-3xl">
        <DialogHeader>
          <DialogTitle>{t('settings.licenses.title')}</DialogTitle>
          <DialogDescription>{t('settings.licenses.description')}</DialogDescription>
        </DialogHeader>
        {version && (
          <div className="flex flex-wrap items-center gap-2 text-sm">
            <span>{t('settings.licenses.source', { version })}</span>
            <Button
              variant="outline"
              size="sm"
              onClick={() => openUrl(sourceUrl(version)).catch(toastError)}
            >
              <ExternalLinkIcon /> {t('settings.licenses.sourceLink', { version })}
            </Button>
          </div>
        )}
        {licenses.isPending && <Spinner />}
        {licenses.error && (
          <ErrorMessage error={licenses.error} onRetry={() => void licenses.refetch()} />
        )}
        {licenses.data && (
          <Tabs defaultValue="app" className="min-w-0">
            <TabsList>
              <TabsTrigger value="app">Homeport</TabsTrigger>
              <TabsTrigger value="third-party">{t('settings.licenses.thirdParty')}</TabsTrigger>
            </TabsList>
            <TabsContent value="app">
              <pre className={TEXT}>{licenses.data.app}</pre>
            </TabsContent>
            <TabsContent value="third-party">
              {licenses.data.third_party ? (
                <pre className={TEXT}>{licenses.data.third_party}</pre>
              ) : (
                <EmptyState
                  title={t('settings.licenses.missing.title')}
                  description={tRich('settings.licenses.missing.description', {
                    command: 'bun run licenses',
                    code: (chunks) => <code>{chunks}</code>,
                  })}
                />
              )}
            </TabsContent>
          </Tabs>
        )}
        <p className="text-muted-foreground text-xs">{t('settings.licenses.trademarks')}</p>
      </DialogContent>
    </Dialog>
  );
}
