import { useState } from 'react';
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { UpdatesCard } from '@/features/updates';
import { commands, keys, type Settings } from '@/shared/api';
import { applyTheme, type Theme } from '@/shared/lib';
import { Button } from '@/shared/ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/shared/ui/card';
import { Label } from '@/shared/ui/label';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/shared/ui/select';
import { ErrorMessage, KeyValue, LogoMark, PageHeader, Spinner, StatusDot } from '@/shared/ui';
import { settingsApi } from '../api/settingsApi';
import { LicensesDialog } from './LicensesDialog';

const THEMES: { value: Theme; label: string }[] = [
  { value: 'system', label: 'Match the system' },
  { value: 'light', label: 'Light' },
  { value: 'dark', label: 'Dark' },
];

export function SettingsPage() {
  const client = useQueryClient();
  const settings = useQuery({ queryKey: keys.settings, queryFn: settingsApi.get });
  const capabilities = useQuery({
    queryKey: keys.capabilities,
    queryFn: () => commands.getCapabilities(),
  });
  const info = useQuery({ queryKey: keys.appInfo, queryFn: () => commands.getAppInfo() });
  const [licensesOpen, setLicensesOpen] = useState(false);
  const save = useMutation({
    meta: { silent: true },
    mutationFn: (next: Settings) => settingsApi.update(next),
    onSuccess: (saved) => {
      applyTheme(saved.theme ?? 'system');
      client.setQueryData(keys.settings, saved);
    },
  });

  if (settings.isPending) return <Spinner />;
  if (settings.error)
    return <ErrorMessage error={settings.error} onRetry={() => void settings.refetch()} />;

  return (
    <div className="grid max-w-2xl gap-4">
      <PageHeader title="Settings" />
      <Card>
        <CardHeader>
          <CardTitle>Appearance</CardTitle>
        </CardHeader>
        <CardContent className="grid gap-1.5">
          <Label htmlFor="theme">Theme</Label>
          <Select
            value={settings.data.theme ?? 'system'}
            onValueChange={(theme) => save.mutate({ ...settings.data, theme: theme as Theme })}
          >
            <SelectTrigger id="theme" className="w-56" aria-label="Theme">
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              {THEMES.map((t) => (
                <SelectItem key={t.value} value={t.value}>
                  {t.label}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
          {save.error && <ErrorMessage error={save.error} />}
        </CardContent>
      </Card>
      <UpdatesCard settings={settings.data} onChange={(next) => save.mutate(next)} />
      <Card>
        <CardHeader>
          <CardTitle>This machine</CardTitle>
          <CardDescription>What Homeport can use here.</CardDescription>
        </CardHeader>
        <CardContent>
          {capabilities.data && (
            <KeyValue
              items={[
                [
                  'OS keychain',
                  <span className="inline-flex items-center gap-1.5">
                    <StatusDot tone={capabilities.data.keychain ? 'success' : 'danger'} />
                    {capabilities.data.keychain
                      ? 'Available — SSH keys, certificates and stack environments are stored encrypted.'
                      : 'Unavailable — SSH keys, TLS certificates and stack environments can’t be stored.'}
                  </span>,
                ],
                [
                  'Compose CLI',
                  <span className="inline-flex items-center gap-1.5">
                    <StatusDot tone={capabilities.data.compose ? 'success' : 'warning'} />
                    {capabilities.data.compose
                      ? `${capabilities.data.compose.program} ${capabilities.data.compose.prefix.join(' ')} ${capabilities.data.compose.version}`
                      : 'Not found — Stacks can be started and stopped, but not deployed.'}
                  </span>,
                ],
              ]}
            />
          )}
        </CardContent>
      </Card>
      <Card>
        <CardContent className="flex items-center gap-4">
          <LogoMark className="size-12" />
          <div className="min-w-0">
            <p className="font-semibold tracking-tight">Homeport</p>
            <p className="text-muted-foreground text-sm">
              Every Docker Engine, docked in one place.
            </p>
            <p className="text-muted-foreground font-mono text-xs">
              {info.data ? `v${info.data.version} · ${info.data.os}/${info.data.arch}` : '…'}
            </p>
          </div>
          <Button
            variant="outline"
            size="sm"
            className="ml-auto"
            onClick={() => setLicensesOpen(true)}
          >
            Licenses
          </Button>
        </CardContent>
      </Card>
      <LicensesDialog open={licensesOpen} onOpenChange={setLicensesOpen} />
    </div>
  );
}
