import { columnCount, columnsFor } from './columns';

describe('containers table columns', () => {
  it('shows every column while the width is unknown', () => {
    expect(columnsFor(0)).toEqual({ image: true, ports: true, created: true, usage: true });
  });

  it('drops Created first, then Ports, Image, and CPU and memory as the table narrows', () => {
    expect(columnsFor(1200)).toEqual({ image: true, ports: true, created: true, usage: true });
    expect(columnsFor(990)).toEqual({ image: true, ports: true, created: false, usage: true });
    expect(columnsFor(810)).toEqual({ image: true, ports: false, created: false, usage: true });
    expect(columnsFor(650)).toEqual({ image: false, ports: false, created: false, usage: true });
    expect(columnsFor(500)).toEqual({ image: false, ports: false, created: false, usage: false });
  });

  it('counts the columns a row spans', () => {
    expect(columnCount(columnsFor(0))).toBe(9);
    expect(columnCount(columnsFor(500))).toBe(4);
  });
});
