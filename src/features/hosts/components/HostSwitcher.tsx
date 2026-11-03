import { toAppError, type HostStatus } from '@/shared/api';
import { PlusIcon, ServerCogIcon, TriangleAlertIcon } from 'lucide-react';
import { useNavigate } from 'react-router-dom';
import {
  Button,
  Select,
  SelectContent,
  SelectItem,
  SelectSeparator,
  SelectTrigger,
  SelectValue,
  showErrorDetails,
  StatusDot,
} from '@/shared/ui';
import { useHosts } from '../api';
import { useHostSelection } from '../context';
import { describeStatus, statusError } from '../status';

/** The status and its reason, shown when hovering a host in the list. */
function statusTitle(status: HostStatus): string {
  const { label, detail } = describeStatus(status);
  return detail ? `${label}: ${detail}` : label;
}

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
        <SelectTrigger className="w-full" aria-label="Host">
          <SelectValue placeholder="No host" />
        </SelectTrigger>
        <SelectContent>
          {hosts.data?.map(({ host, status }) => (
            <SelectItem key={host.id} value={host.id} title={statusTitle(status)}>
              <StatusDot tone={describeStatus(status).tone} />
              <span className="truncate">{host.name}</span>
            </SelectItem>
          ))}
          {hosts.data?.length ? <SelectSeparator /> : null}
          <SelectItem value={ADD}>
            <PlusIcon /> Add host…
          </SelectItem>
          <SelectItem value={MANAGE}>
            <ServerCogIcon /> Manage hosts
          </SelectItem>
        </SelectContent>
      </Select>
      {selected && error && (
        <Button
          variant="ghost"
          size="icon"
          className="text-destructive shrink-0"
          aria-label={`Why ${selected.host.name} can’t be used`}
          title={`${describeStatus(selected.status).label}: ${error.message}`}
          onClick={() => showErrorDetails(toAppError(error))}
        >
          <TriangleAlertIcon />
        </Button>
      )}
    </div>
  );
}
