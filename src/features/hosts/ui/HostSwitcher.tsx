import { toAppError } from '@/shared/api';
import { PlusIcon, ServerCogIcon, TriangleAlertIcon } from 'lucide-react';
import { useNavigate } from 'react-router-dom';
import {
  Button,
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectSeparator,
  SelectTrigger,
  SelectValue,
  showErrorDetails,
  StatusDot,
} from '@/shared/ui';
import { describeStatus, statusError, useHosts, useHostSelection } from '@/core/host';
import { statusTitle } from '../model/statusTitle';
import { t } from '@/shared/i18n';

const ADD = '__add__';
const MANAGE = '__manage__';

/** The sidebar's Host picker; every resource page shows the selected Host. */
export function HostSwitcher() {
  const hosts = useHosts();
  const { hostId, select } = useHostSelection();
  const navigate = useNavigate();

  const selected = hosts.data?.find((h) => h.host.id === hostId);
  const error = selected && statusError(selected.status);
  return (
    <div className="flex items-center gap-1">
      <Select
        value={hostId ?? ''}
        onValueChange={(value) => {
          if (value === ADD) navigate('/hosts/new');
          else if (value === MANAGE) navigate('/hosts');
          else select(value);
        }}
      >
        <SelectTrigger className="w-full" aria-label={t('hosts.switcher.label')}>
          <SelectValue placeholder={t('hosts.switcher.placeholder')} />
        </SelectTrigger>
        <SelectContent>
          {hosts.data?.length ? (
            <SelectGroup>
              <SelectLabel>{t('hosts.switcher.hosts')}</SelectLabel>
              {hosts.data.map(({ host, status }) => (
                <SelectItem key={host.id} value={host.id} title={statusTitle(status)}>
                  <StatusDot tone={describeStatus(status).tone} />
                  <span className="truncate">{host.name}</span>
                </SelectItem>
              ))}
            </SelectGroup>
          ) : null}
          {hosts.data?.length ? <SelectSeparator /> : null}
          <SelectItem value={ADD}>
            <PlusIcon /> {t('hosts.switcher.add')}
          </SelectItem>
          <SelectItem value={MANAGE}>
            <ServerCogIcon /> {t('hosts.switcher.manage')}
          </SelectItem>
        </SelectContent>
      </Select>
      {selected && error && (
        <Button
          variant="ghost"
          size="icon"
          className="text-destructive shrink-0"
          aria-label={t('hosts.switcher.whyUnusable', { name: selected.host.name })}
          title={t('hosts.statusTitle', {
            label: describeStatus(selected.status).label,
            detail: error.message,
          })}
          onClick={() => showErrorDetails(toAppError(error))}
        >
          <TriangleAlertIcon />
        </Button>
      )}
    </div>
  );
}
