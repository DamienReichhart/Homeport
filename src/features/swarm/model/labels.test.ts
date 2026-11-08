import { labelMap, labelRows } from './labels';

describe('node labels', () => {
  it('round-trips through rows, sorted by key', () => {
    const rows = labelRows({ zone: 'b', disk: 'ssd' });
    expect(rows).toEqual([
      { key: 'disk', value: 'ssd' },
      { key: 'zone', value: 'b' },
    ]);
    expect(labelMap(rows)).toEqual({ zone: 'b', disk: 'ssd' });
  });

  it('drops blank rows and trims keys, keeping a value without a key to be refused', () => {
    expect(
      labelMap([
        { key: ' zone ', value: 'a' },
        { key: '', value: '' },
        { key: '', value: 'orphan' },
      ]),
    ).toEqual({ zone: 'a', '': 'orphan' });
  });
});
