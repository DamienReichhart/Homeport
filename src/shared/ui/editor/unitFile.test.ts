import { StringStream } from '@codemirror/language';
import { unitFileParser as parser } from './unitFile';

/** The tokens the unit file language gives `line`, with their text. */
function tokens(line: string): [string, string | null][] {
  const state = parser.startState!(2);
  const stream = new StringStream(line, 4, 2);
  const out: [string, string | null][] = [];
  while (!stream.eol()) {
    const style = parser.token(stream, state);
    out.push([stream.current(), style]);
    stream.start = stream.pos;
  }
  return out;
}

describe('unit file highlighting', () => {
  it('tells sections, keys, values and comments apart', () => {
    expect(tokens('[Container]')).toEqual([['[Container]', 'header']]);
    expect(tokens('Image=docker.io/nginx')).toEqual([
      ['Image', 'keyword'],
      ['=', 'operator'],
      ['docker.io/nginx', 'string'],
    ]);
    expect(tokens('# PublishPort=80')).toEqual([['# PublishPort=80', 'comment']]);
  });
});
