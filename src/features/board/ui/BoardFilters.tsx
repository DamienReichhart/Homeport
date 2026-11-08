import { ChevronDownIcon } from 'lucide-react';
import type { HostView } from '@/shared/api';
import {
  Button,
  DropdownMenu,
  DropdownMenuCheckboxItem,
  DropdownMenuContent,
  DropdownMenuTrigger,
  SearchInput,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/shared/ui';
import type { BoardFilter, StateFilter } from '../model/board';
import { t, type MessageKey } from '@/shared/i18n';

const STATES: { value: StateFilter; label: MessageKey }[] = [
  { value: 'all', label: 'board.state.all' },
  { value: 'running', label: 'board.state.running' },
  { value: 'stopped', label: 'board.state.stopped' },
  { value: 'unhealthy', label: 'board.state.unhealthy' },
];
const ANY = '__any__';

/** The board's filters: which Hosts, state, Stack, and a search. */
export function BoardFilters({
  filter,
  onChange,
  hosts,
  stacks,
}: {
  filter: BoardFilter;
  onChange: (next: BoardFilter) => void;
  hosts: HostView[];
  stacks: string[];
}) {
  const toggleHost = (id: string, on: boolean) =>
    onChange({
      ...filter,
      hosts: on ? [...filter.hosts, id] : filter.hosts.filter((h) => h !== id),
    });
  const filtered =
    filter.hosts.length > 0 || filter.state !== 'all' || filter.stack !== null || filter.query;
  return (
    <div className="mb-3 flex flex-wrap items-center gap-2">
      <SearchInput
        value={filter.query}
        onChange={(query) => onChange({ ...filter, query })}
        placeholder={t('board.search')}
      />
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button variant="outline" aria-label={t('board.filter.hosts')}>
            {filter.hosts.length
              ? t('board.filter.someHosts', { count: filter.hosts.length })
              : t('board.filter.allHosts')}
            <ChevronDownIcon />
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent align="start">
          {hosts.map(({ host }) => (
            <DropdownMenuCheckboxItem
              key={host.id}
              checked={filter.hosts.includes(host.id)}
              onCheckedChange={(on) => toggleHost(host.id, on === true)}
              onSelect={(e) => e.preventDefault()}
            >
              {host.name}
            </DropdownMenuCheckboxItem>
          ))}
        </DropdownMenuContent>
      </DropdownMenu>
      <Select
        value={filter.state}
        onValueChange={(state) => onChange({ ...filter, state: state as StateFilter })}
      >
        <SelectTrigger className="w-36" aria-label={t('board.filter.state')}>
          <SelectValue />
        </SelectTrigger>
        <SelectContent>
          {STATES.map((s) => (
            <SelectItem key={s.value} value={s.value}>
              {t(s.label)}
            </SelectItem>
          ))}
        </SelectContent>
      </Select>
      <Select
        value={filter.stack ?? ANY}
        onValueChange={(stack) => onChange({ ...filter, stack: stack === ANY ? null : stack })}
      >
        <SelectTrigger className="w-40" aria-label={t('board.filter.stack')}>
          <SelectValue />
        </SelectTrigger>
        <SelectContent>
          <SelectItem value={ANY}>{t('board.filter.anyStack')}</SelectItem>
          {stacks.map((stack) => (
            <SelectItem key={stack} value={stack}>
              {stack}
            </SelectItem>
          ))}
        </SelectContent>
      </Select>
      {filtered && (
        <Button
          variant="ghost"
          onClick={() => onChange({ hosts: [], state: 'all', stack: null, query: '' })}
        >
          {t('board.filter.clear')}
        </Button>
      )}
    </div>
  );
}
