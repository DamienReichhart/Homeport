import { readFileSync } from 'node:fs';
import { join } from 'node:path';

/**
 * Licenses `about.toml` must never accept (ADR 0016): copyleft ones beyond a
 * file's scope, and those forbidding commercial use. One dependency under
 * them would keep Homeport from ever being offered under a commercial licence.
 */
const FORBIDDEN = [
  /^A?GPL/i,
  /^LGPL/i,
  /^SSPL/i,
  /^EUPL/i,
  /^OSL/i,
  /^CPAL/i,
  /^RPL/i,
  /^Sleepycat/i,
  /^CC-BY-(NC|SA)/i,
  /-NC(-|$)/i,
  /Commons-Clause/i,
  /^BUSL/i,
  /^Elastic/i,
];

/** The `accepted = [...]` list of `about.toml`, its comments left out. */
function accepted(): string[] {
  const toml = readFileSync(join(import.meta.dirname, '..', 'about.toml'), 'utf8');
  const list = /^accepted\s*=\s*\[([^\]]*)\]/m.exec(toml)?.[1];
  if (list === undefined) throw new Error('about.toml has no accepted list');
  return list
    .split('\n')
    .map((line) => line.replace(/#.*$/, ''))
    .flatMap((line) => [...line.matchAll(/"([^"]+)"/g)].map((m) => m[1] ?? ''));
}

describe('about.toml', () => {
  it('accepts no copyleft or non-commercial license', () => {
    expect(accepted()).toContain('MIT');
    const offending = accepted().filter((id) => FORBIDDEN.some((re) => re.test(id)));
    expect(offending).toEqual([]);
  });

  it.each(['GPL-3.0-only', 'LGPL-2.1-or-later', 'AGPL-3.0-only', 'SSPL-1.0', 'CC-BY-NC-4.0'])(
    'would refuse %s',
    (id) => {
      expect(FORBIDDEN.some((re) => re.test(id))).toBe(true);
    },
  );

  it.each(['MIT', 'Apache-2.0', 'MPL-2.0', 'OFL-1.1', 'BSD-3-Clause'])('allows %s', (id) => {
    expect(FORBIDDEN.some((re) => re.test(id))).toBe(false);
  });
});
