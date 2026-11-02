import { PlusIcon, ServerCogIcon } from 'lucide-react';
import { useNavigate } from 'react-router-dom';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectSeparator,
  SelectTrigger,
  SelectValue,
} from '@/shared/ui/select';
import { StatusDot } from '@/shared/ui';
import { useHosts } from '../api';
import { useHostSelection } from '../context';
import { describeStatus } from '../status';

const ADD = '__add__';
const MANAGE = '__manage__';

/** The sidebar's Host picker; every resource page shows the selected Host. */
export function HostSwitcher() {
  const hosts = useHosts();
  const { hostId, select } = useHostSelection();
  const navigate = useNavigate();

  return (
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
          <SelectItem key={host.id} value={host.id}>
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
  );
}
