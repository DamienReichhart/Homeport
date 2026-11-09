import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { DataTable, type ColumnDef } from './DataTable';

interface Row {
  id: string;
  name: string;
  size: number;
}

const rows: Row[] = [
  { id: 'a', name: 'alpine', size: 3 },
  { id: 'b', name: 'busybox', size: 1 },
];
const columns: ColumnDef<Row, unknown>[] = [
  { accessorKey: 'name', header: 'Name' },
  { accessorKey: 'size', header: 'Size' },
];

describe('DataTable from the keyboard', () => {
  it('opens a row with Tab and Enter, through its primary cell', async () => {
    const opened: string[] = [];
    render(
      <DataTable
        columns={columns}
        data={rows}
        getRowId={(r) => r.id}
        onRowClick={(r) => opened.push(r.name)}
      />,
    );
    const user = userEvent.setup();
    await user.tab(); // the Name header
    await user.tab(); // the Size header
    await user.tab();
    expect(screen.getByRole('button', { name: 'alpine' })).toHaveFocus();
    await user.keyboard('{Enter}');
    await user.tab();
    await user.keyboard(' ');
    expect(opened).toEqual(['alpine', 'busybox']);
  });

  it('opens a row once on a mouse click, on the cell or elsewhere on the row', async () => {
    const opened: string[] = [];
    render(
      <DataTable
        columns={columns}
        data={rows}
        getRowId={(r) => r.id}
        onRowClick={(r) => opened.push(r.name)}
      />,
    );
    const user = userEvent.setup();
    await user.click(screen.getByRole('button', { name: 'alpine' }));
    await user.click(screen.getByText('1'));
    expect(opened).toEqual(['alpine', 'busybox']);
  });

  it('sorts from a header button and says so with aria-sort', async () => {
    render(<DataTable columns={columns} data={rows} getRowId={(r) => r.id} />);
    const header = screen.getByRole('columnheader', { name: 'Name' });
    expect(header).toHaveAttribute('aria-sort', 'none');
    const user = userEvent.setup();
    screen.getByRole('button', { name: 'Name' }).focus();
    await user.keyboard('{Enter}');
    expect(header).toHaveAttribute('aria-sort', 'ascending');
    await user.keyboard(' ');
    expect(header).toHaveAttribute('aria-sort', 'descending');
    expect(screen.getAllByRole('row')[1]).toHaveTextContent('busybox');
  });

  it('adds no buttons to rows that open nothing', () => {
    render(<DataTable columns={columns} data={rows} getRowId={(r) => r.id} />);
    expect(screen.queryByRole('button', { name: 'alpine' })).not.toBeInTheDocument();
  });
});
