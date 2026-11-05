import type { Settings } from '@/shared/api';
import {
  Button,
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
  Label,
  Switch,
} from '@/shared/ui';
import { useUpdateCheck } from '../api/queries';
import { describeMethod } from '../model/updates';
import { UpdateActions } from './UpdateActions';
import { UpdateStatus } from './UpdateStatus';
import { t } from '@/shared/i18n';

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
        <CardTitle>{t('updates.title')}</CardTitle>
        <CardDescription>
          {check.data ? describeMethod(check.data) : t('updates.description')}
        </CardDescription>
      </CardHeader>
      <CardContent className="grid gap-4">
        <div className="flex items-center gap-2">
          <Switch
            id="check-for-updates"
            checked={enabled}
            onCheckedChange={(checkForUpdates) => onChange({ ...settings, checkForUpdates })}
          />
          <Label htmlFor="check-for-updates">{t('updates.automatic')}</Label>
        </div>
        <div className="flex flex-wrap items-center gap-3">
          <Button
            variant="outline"
            size="sm"
            disabled={check.isFetching}
            onClick={() => void check.refetch()}
          >
            {check.isFetching ? t('updates.checking') : t('updates.checkNow')}
          </Button>
          <UpdateStatus check={check} />
        </div>
        {check.data?.available && <UpdateActions check={check.data} />}
      </CardContent>
    </Card>
  );
}
