import {
  flexRender,
  getCoreRowModel,
  getFilteredRowModel,
  getSortedRowModel,
  useReactTable,
  type ColumnDef,
  type RowSelectionState,
  type SortingState,
} from '@tanstack/react-table';
import { ArrowDownIcon, ArrowUpIcon } from 'lucide-react';
import { useState, type ReactNode } from 'react';
import { Checkbox } from '../primitives/checkbox';
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '../primitives/table';
import { cn } from '@/shared/lib/utils';
import { t } from '@/shared/i18n';

export type { ColumnDef } from '@tanstack/react-table';

/** Brings a newly marked row into view (jsdom has no scrolling). */
function scrollIntoView(row: HTMLTableRowElement | null) {
  row?.scrollIntoView?.({ block: 'nearest' });
}

/**
 * A sortable, filterable table. `filter` is matched against every column
 * with an accessor; `onSelectionChange` enables row checkboxes;
 * `selectedId` marks one row, such as one just made, and scrolls to it.
 */
export function DataTable<T>({
  columns,
  data,
  getRowId,
  filter = '',
  empty,
  onRowClick,
  onSelectionChange,
  selectedId,
  initialSorting = [],
}: {
  columns: ColumnDef<T, unknown>[];
  data: T[];
  getRowId: (row: T) => string;
  filter?: string;
  empty?: ReactNode;
  onRowClick?: (row: T) => void;
  onSelectionChange?: (rows: T[]) => void;
  selectedId?: string | null;
  initialSorting?: SortingState;
}) {
  const [sorting, setSorting] = useState<SortingState>(initialSorting);
  const [selection, setSelection] = useState<RowSelectionState>({});

  const allColumns: ColumnDef<T, unknown>[] = onSelectionChange
    ? [
        {
          id: 'select',
          enableSorting: false,
          header: ({ table }) => (
            <Checkbox
              aria-label={t('ui.table.selectAll')}
              checked={
                table.getIsAllRowsSelected() || (table.getIsSomeRowsSelected() && 'indeterminate')
              }
              onCheckedChange={(v) => table.toggleAllRowsSelected(v === true)}
            />
          ),
          cell: ({ row }) => (
            <Checkbox
              aria-label={t('ui.table.selectRow')}
              checked={row.getIsSelected()}
              onClick={(e) => e.stopPropagation()}
              onCheckedChange={(v) => row.toggleSelected(v === true)}
            />
          ),
        },
        ...columns,
      ]
    : columns;

  // The table instance changes in place, so the React Compiler must skip this
  // component, which is what this rule reports.
  // eslint-disable-next-line react-hooks/incompatible-library -- skipping is the intent
  const table = useReactTable({
    data,
    columns: allColumns,
    getRowId,
    state: { sorting, globalFilter: filter, rowSelection: selection },
    onSortingChange: setSorting,
    onRowSelectionChange: (updater) => {
      const next = typeof updater === 'function' ? updater(selection) : updater;
      setSelection(next);
      onSelectionChange?.(data.filter((row) => next[getRowId(row)]));
    },
    getCoreRowModel: getCoreRowModel(),
    getSortedRowModel: getSortedRowModel(),
    getFilteredRowModel: getFilteredRowModel(),
  });

  const rows = table.getRowModel().rows;
  return (
    <div className="rounded-md border">
      <Table>
        <TableHeader>
          {table.getHeaderGroups().map((group) => (
            <TableRow key={group.id}>
              {group.headers.map((header) => {
                const sorted = header.column.getIsSorted();
                return (
                  <TableHead
                    key={header.id}
                    className={cn(
                      header.column.getCanSort() && 'cursor-pointer select-none',
                      header.id === 'select' && 'w-8',
                    )}
                    onClick={header.column.getToggleSortingHandler()}
                  >
                    <span className="inline-flex items-center gap-1">
                      {flexRender(header.column.columnDef.header, header.getContext())}
                      {sorted === 'asc' && <ArrowUpIcon className="size-3" />}
                      {sorted === 'desc' && <ArrowDownIcon className="size-3" />}
                    </span>
                  </TableHead>
                );
              })}
            </TableRow>
          ))}
        </TableHeader>
        <TableBody>
          {rows.length === 0 ? (
            <TableRow>
              <TableCell
                colSpan={allColumns.length}
                className="text-muted-foreground h-24 text-center"
              >
                {empty ?? t('ui.table.empty')}
              </TableCell>
            </TableRow>
          ) : (
            rows.map((row) => (
              <TableRow
                key={row.id}
                data-state={row.getIsSelected() || row.id === selectedId ? 'selected' : undefined}
                aria-selected={row.id === selectedId || undefined}
                ref={row.id === selectedId ? scrollIntoView : undefined}
                className={cn(onRowClick && 'cursor-pointer')}
                onClick={() => onRowClick?.(row.original)}
              >
                {row.getVisibleCells().map((cell) => (
                  <TableCell key={cell.id}>
                    {flexRender(cell.column.columnDef.cell, cell.getContext())}
                  </TableCell>
                ))}
              </TableRow>
            ))
          )}
        </TableBody>
      </Table>
    </div>
  );
}
