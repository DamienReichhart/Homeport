import { StringStream } from '@codemirror/language';
import { jobSpecParser as parser } from './jobSpec';

/** The styled tokens the job specification language gives `line`, spaces left out. */
function tokens(line: string, state = parser.startState!(2)): [string, string | null][] {
  const stream = new StringStream(line, 4, 2);
  const out: [string, string | null][] = [];
  while (!stream.eol()) {
    const style = parser.token(stream, state);
    if (stream.current().trim()) out.push([stream.current(), style]);
    stream.start = stream.pos;
  }
  return out;
}

describe('job specification highlighting', () => {
  it('tells HCL blocks, attributes, strings and numbers apart', () => {
    expect(tokens('job "web" {')).toEqual([
      ['job', 'keyword'],
      ['"web"', 'string'],
      ['{', null],
    ]);
    expect(tokens('  count = 3 # three')).toEqual([
      ['count', 'propertyName'],
      ['=', null],
      ['3', 'number'],
      ['# three', 'comment'],
    ]);
  });

  it('reads JSON keys and atoms', () => {
    expect(tokens('"Stop": false,')).toEqual([
      ['"Stop"', 'propertyName'],
      [':', null],
      ['false', 'atom'],
      [',', null],
    ]);
  });

  it('carries a block comment over lines', () => {
    const state = parser.startState!(2);
    expect(tokens('/* start', state)).toEqual([
      ['/*', 'comment'],
      [' start', 'comment'],
    ]);
    expect(state.comment).toBe(true);
    expect(tokens('end */ count = 1', state)[0]).toEqual(['end */', 'comment']);
    expect(state.comment).toBe(false);
  });
});
