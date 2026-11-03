import type { Settings } from '@/shared/api';
import { ErrorMessage } from '@/shared/ui';
import { Button } from '@/shared/ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/shared/ui/card';
import { Label } from '@/shared/ui/label';
import { Switch } from '@/shared/ui/switch';
import { describeMethod, useUpdateCheck } from '../api';
import { UpdateActions } from './UpdateActions';

/** The Updates card in Settings: automatic checks, "Check now", and the result. */
export function UpdatesCard({
  settings,
  onChange,
}: {
  settings: Settings;
  onChange: (next: Settings) => void;
}) {
  const check = useUpdateCheck();
  const enabled = settings.checkForUpdates ?? true;

  return (
    <Card>
      <CardHeader>
        <CardTitle>Updates</CardTitle>
        <CardDescription>
          {check.data
            ? describeMethod(check.data)
            : 'Homeport can look for a new version when it starts and every few hours.'}
        </CardDescription>
      </CardHeader>
      <CardContent className="grid gap-4">
        <div className="flex items-center gap-2">
          <Switch
            id="check-for-updates"
            checked={enabled}
            onCheckedChange={(checkForUpdates) => onChange({ ...settings, checkForUpdates })}
          />
          <Label htmlFor="check-for-updates">Check for updates automatically</Label>
        </div>
        <div className="flex flex-wrap items-center gap-3">
          <Button
            variant="outline"
            size="sm"
            disabled={check.isFetching}
            onClick={() => void check.refetch()}
          >
            {check.isFetching ? 'Checking…' : 'Check now'}
          </Button>
          <UpdateStatus check={check} />
        </div>
        {check.data?.available && <UpdateActions check={check.data} />}
      </CardContent>
    </Card>
  );
}

function UpdateStatus({ check }: { check: ReturnType<typeof useUpdateCheck> }) {
  if (check.isFetching) return null;
  if (check.error) return <ErrorMessage error={check.error} className="basis-full" />;
  if (!check.data) return null;
  const { available, currentVersion } = check.data;
  return (
    <span className="text-muted-foreground text-sm">
      {available
        ? `Homeport ${available.version} is available (you have ${currentVersion}).`
        : `Homeport ${currentVersion} is the latest version.`}
    </span>
  );
}
