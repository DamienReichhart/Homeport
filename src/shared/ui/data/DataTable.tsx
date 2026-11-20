import {
  flexRender,
  getCoreRowModel,
  getFilteredRowModel,
  getPaginationRowModel,
  getSortedRowModel,
  useReactTable,
  type ColumnDef,
  type RowSelectionState,
  type SortingState,
} from '@tanstack/react-table';
import { ArrowDownIcon, ArrowUpIcon, ChevronLeftIcon, ChevronRightIcon } from 'lucide-react';
import { useState, type KeyboardEvent, type ReactNode } from 'react';
import { Button } from '../primitives/button';
import { Checkbox } from '../primitives/checkbox';
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '../primitives/table';
import { useSplitPane } from '../layout/SplitView';
import { cn } from '@/shared/lib/utils';
import { t } from '@/shared/i18n';

export type { ColumnDef } from '@tanstack/react-table';

declare module '@tanstack/react-table' {
  // eslint-disable-next-line @typescript-eslint/no-unused-vars -- the generics must match TanStack's
  interface ColumnMeta<TData, TValue> {
    /** Kept when the table is a split view's narrow list, beside the primary column. */
    compact?: boolean;
  }
}

/** The keyboard focus ring of the table's own controls. */
const FOCUS =
  'rounded-sm outline-none focus-visible:ring-[3px] focus-visible:ring-ring/50 cursor-pointer';

const safeDecode = (value: string) => {
  try {
    return decodeURIComponent(value);
  } catch {
    return value;
  }
};

/** Brings a newly marked row into view (jsdom has no scrolling). */
function scrollIntoView(row: HTMLTableRowElement | null) {
  row?.scrollIntoView?.({ block: 'nearest' });
}

/**
 * A sortable, filterable table. With `onRowClick`, the primary column's cell
 * is a button, so rows open with Tab and Enter as well as a click; sortable
 * headers are buttons with `aria-sort`. `filter` is matched against every column
 * with an accessor; `onSelectionChange` enables row checkboxes;
 * `selectedId` marks one row, such as one just made, and scrolls to it;
 * `pageSize` shows long lists a page at a time, so thousands of rows stay fast.
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
  pageSize,
  primaryColumn,
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
  pageSize?: number;
  /** The id of the column whose cell opens the row from the keyboard; the first data column by default. */
  primaryColumn?: string;
}) {
  const [sorting, setSorting] = useState<SortingState>(initialSorting);
  const [selection, setSelection] = useState<RowSelectionState>({});
  const pane = useSplitPane();
  // Beside an open item, the list shows what names each row and marks the open one.
  const compact = pane.compact && !!onRowClick;
  const openId = compact ? pane.selected : (selectedId ?? null);
  const isOpen = (id: string) =>
    openId !== null &&
    (id === openId || id === safeDecode(openId) || (openId.length >= 12 && id.startsWith(openId)));

  const allColumns: ColumnDef<T, unknown>[] =
    onSelectionChange && !compact
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

  const idOf = (c: ColumnDef<T, unknown>) =>
    c.id ?? (c as { accessorKey?: string }).accessorKey ?? '';
  const firstData = allColumns.map(idOf).find((id) => id && id !== 'select' && id !== 'actions');
  const columnVisibility = compact
    ? Object.fromEntries(
        allColumns.map((c) => [
          idOf(c),
          idOf(c) === (primaryColumn ?? firstData) || c.meta?.compact === true,
        ]),
      )
    : {};

  // The table instance changes in place, so the React Compiler must skip this
  // component, which is what this rule reports.
  // eslint-disable-next-line react-hooks/incompatible-library -- skipping is the intent
  const table = useReactTable({
    data,
    columns: allColumns,
    getRowId,
    state: { sorting, globalFilter: filter, rowSelection: selection, columnVisibility },
    onSortingChange: setSorting,
    onRowSelectionChange: (updater) => {
      const next = typeof updater === 'function' ? updater(selection) : updater;
      setSelection(next);
      onSelectionChange?.(data.filter((row) => next[getRowId(row)]));
    },
    getCoreRowModel: getCoreRowModel(),
    getSortedRowModel: getSortedRowModel(),
    getFilteredRowModel: getFilteredRowModel(),
    ...(pageSize
      ? {
          getPaginationRowModel: getPaginationRowModel(),
          initialState: { pagination: { pageIndex: 0, pageSize } },
          autoResetPageIndex: true,
        }
      : {}),
  });
  const pages = pageSize ? table.getPageCount() : 1;
  /** The column whose cell opens the row from the keyboard: the first one with data. */
  const primary =
    primaryColumn ??
    table.getAllLeafColumns().find((c) => c.id !== 'select' && c.id !== 'actions')?.id;

  const rows = table.getRowModel().rows;
  /** Up and Down open the row above or below the open one. */
  const onKeyDown = (event: KeyboardEvent<HTMLDivElement>) => {
    if (!compact || !onRowClick || (event.key !== 'ArrowDown' && event.key !== 'ArrowUp')) return;
    const at = rows.findIndex((row) => isOpen(row.id));
    const next = rows[at + (event.key === 'ArrowDown' ? 1 : -1)];
    if (!next) return;
    event.preventDefault();
    onRowClick(next.original);
  };
  return (
    <div className="bg-card rounded-md border" onKeyDown={onKeyDown}>
      <Table>
        <TableHeader className={cn(compact && 'sr-only')}>
          {table.getHeaderGroups().map((group) => (
            <TableRow key={group.id}>
              {group.headers.map((header) => {
                const sorted = header.column.getIsSorted();
                const label = (
                  <>
                    {flexRender(header.column.columnDef.header, header.getContext())}
                    {sorted === 'asc' && <ArrowUpIcon className="size-3" />}
                    {sorted === 'desc' && <ArrowDownIcon className="size-3" />}
                  </>
                );
                return (
                  <TableHead
                    key={header.id}
                    className={cn(header.id === 'select' && 'w-8')}
                    aria-sort={
                      !header.column.getCanSort()
                        ? undefined
                        : sorted === 'asc'
                          ? 'ascending'
                          : sorted === 'desc'
                            ? 'descending'
                            : 'none'
                    }
                  >
                    {header.column.getCanSort() ? (
                      // A real button, so Tab reaches it and Enter or Space sorts.
                      <button
                        type="button"
                        className={cn('inline-flex items-center gap-1 select-none', FOCUS)}
                        onClick={header.column.getToggleSortingHandler()}
                      >
                        {label}
                      </button>
                    ) : (
                      <span className="inline-flex items-center gap-1">{label}</span>
                    )}
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
                data-state={row.getIsSelected() || isOpen(row.id) ? 'selected' : undefined}
                aria-selected={isOpen(row.id) || undefined}
                aria-current={compact && isOpen(row.id) ? 'true' : undefined}
                ref={isOpen(row.id) ? scrollIntoView : undefined}
                className={cn(onRowClick && 'cursor-pointer')}
                onClick={() => onRowClick?.(row.original)}
              >
                {row.getVisibleCells().map((cell) => {
                  const content = flexRender(cell.column.columnDef.cell, cell.getContext());
                  return (
                    <TableCell key={cell.id}>
                      {onRowClick && cell.column.id === primary ? (
                        // The row's one keyboard stop: Enter or Space clicks it, and
                        // the click reaches the row's handler like a mouse click.
                        <button type="button" className={cn('max-w-full text-start', FOCUS)}>
                          {content}
                        </button>
                      ) : (
                        content
                      )}
                    </TableCell>
                  );
                })}
              </TableRow>
            ))
          )}
        </TableBody>
      </Table>
      {pages > 1 && (
        <div className="text-muted-foreground flex items-center justify-end gap-2 border-t px-3 py-2 text-sm">
          {t('ui.table.page', {
            page: table.getState().pagination.pageIndex + 1,
            pages,
            rows: table.getFilteredRowModel().rows.length,
          })}
          <Button
            variant="ghost"
            size="icon-sm"
            aria-label={t('ui.table.previous')}
            disabled={!table.getCanPreviousPage()}
            onClick={() => table.previousPage()}
          >
            <ChevronLeftIcon className="rtl:-scale-x-100" />
          </Button>
          <Button
            variant="ghost"
            size="icon-sm"
            aria-label={t('ui.table.next')}
            disabled={!table.getCanNextPage()}
            onClick={() => table.nextPage()}
          >
            <ChevronRightIcon className="rtl:-scale-x-100" />
          </Button>
        </div>
      )}
    </div>
  );
}
