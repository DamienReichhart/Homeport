import { useVirtualizer } from '@tanstack/react-virtual';
import { ArrowDownIcon, ArrowUpIcon } from 'lucide-react';
import { useRef } from 'react';
import type { ContainerUsage } from '@/shared/api';
import {
  Checkbox,
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/shared/ui';
import type { BoardRow, BoardSort, SortKey } from '../model/board';
import { BoardRowView } from './BoardRowView';
import { t, type MessageKey } from '@/shared/i18n';

const COLUMNS: { key: SortKey | null; label: MessageKey; className?: string }[] = [
  { key: 'host', label: 'board.column.host' },
  { key: 'name', label: 'containers.column.name' },
  { key: 'image', label: 'containers.column.image' },
  { key: 'status', label: 'containers.column.status' },
  { key: 'stack', label: 'board.column.stack' },
  { key: null, label: 'containers.column.ports' },
  { key: 'cpu', label: 'containers.column.cpu', className: 'text-right' },
  { key: 'memory', label: 'containers.column.memory', className: 'text-right' },
  { key: 'created', label: 'containers.column.created' },
];
/** Past this many rows, only those in view are drawn. */
export const VIRTUAL_FROM = 300;
const ROW_HEIGHT = 53;

/** Every Host's containers, sortable by any column but the ports. */
export function BoardTable({
  rows,
  sort,
  onSort,
  usage,
  selected,
  onSelect,
  onOpen,
}: {
  rows: BoardRow[];
  sort: BoardSort;
  onSort: (sort: BoardSort) => void;
  usage: (row: BoardRow) => ContainerUsage | undefined;
  selected: Set<string>;
  onSelect: (keys: string[], on: boolean) => void;
  onOpen: (row: BoardRow) => void;
}) {
  const scroller = useRef<HTMLDivElement>(null);
  const virtual = rows.length > VIRTUAL_FROM;
  // The virtualizer measures in place, which the React Compiler must skip.
  // eslint-disable-next-line react-hooks/incompatible-library -- skipping is the intent
  const virtualizer = useVirtualizer({
    count: virtual ? rows.length : 0,
    getScrollElement: () => scroller.current,
    estimateSize: () => ROW_HEIGHT,
    overscan: 12,
  });
  const items = virtualizer.getVirtualItems();
  const shown = virtual ? items.flatMap((item) => rows[item.index] ?? []) : rows;
  const before = virtual ? (items[0]?.start ?? 0) : 0;
  const after = virtual ? virtualizer.getTotalSize() - (items.at(-1)?.end ?? 0) : 0;
  const all = rows.length > 0 && rows.every((r) => selected.has(r.key));
  const some = rows.some((r) => selected.has(r.key));

  return (
    <div ref={scroller} className="max-h-[calc(100vh-15rem)] overflow-auto rounded-md border">
      <Table>
        <TableHeader className="bg-background sticky top-0 z-10">
          <TableRow>
            <TableHead className="w-8">
              <Checkbox
                aria-label={t('board.selectAll')}
                checked={all || (some && 'indeterminate')}
                onCheckedChange={(on) =>
                  onSelect(
                    rows.map((r) => r.key),
                    on === true,
                  )
                }
              />
            </TableHead>
            {COLUMNS.map(({ key, label, className }) => (
              <TableHead
                key={label}
                className={className}
                aria-sort={
                  key && sort.key === key ? (sort.desc ? 'descending' : 'ascending') : undefined
                }
              >
                {key ? (
                  <button
                    type="button"
                    className="inline-flex items-center gap-1"
                    aria-label={t('board.sortBy', { column: t(label) })}
                    onClick={() => onSort({ key, desc: sort.key === key ? !sort.desc : false })}
                  >
                    {t(label)}
                    {sort.key === key &&
                      (sort.desc ? (
                        <ArrowDownIcon className="size-3" />
                      ) : (
                        <ArrowUpIcon className="size-3" />
                      ))}
                  </button>
                ) : (
                  t(label)
                )}
              </TableHead>
            ))}
            <TableHead className="w-20" />
          </TableRow>
        </TableHeader>
        <TableBody>
          {before > 0 && <tr style={{ height: before }} aria-hidden />}
          {shown.map((row) => (
            <BoardRowView
              key={row.key}
              row={row}
              usage={usage(row)}
              selected={selected.has(row.key)}
              onSelect={(on) => onSelect([row.key], on)}
              onOpen={() => onOpen(row)}
            />
          ))}
          {after > 0 && <tr style={{ height: after }} aria-hidden />}
          {rows.length === 0 && (
            <TableRow>
              <TableCell
                colSpan={COLUMNS.length + 2}
                className="text-muted-foreground h-20 text-center"
              >
                {t('board.noMatch')}
              </TableCell>
            </TableRow>
          )}
        </TableBody>
      </Table>
    </div>
  );
}
