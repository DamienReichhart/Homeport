import { StreamLanguage, type StreamParser, type StringStream } from '@codemirror/language';

interface UnitState {
  /** Inside a line, after the `=` of a key. */
  value: boolean;
}

/**
 * systemd unit files and Quadlet's (`.container`, `.pod`…): `[Section]`
 * headers, `Key=value` lines and `#` or `;` comments.
 */
export const unitFileParser: StreamParser<UnitState> = {
  name: 'unit-file',
  startState: () => ({ value: false }),
  token(stream: StringStream, state: UnitState) {
    if (stream.sol()) {
      state.value = false;
      stream.eatSpace();
      if (stream.match(/^[#;].*/)) return 'comment';
      if (stream.match(/^\[[^\]]*\]/)) return 'header';
      if (stream.match(/^[^=\s]+(?=\s*=)/)) return 'keyword';
    }
    if (!state.value && stream.match(/^\s*=/)) {
      state.value = true;
      return 'operator';
    }
    stream.skipToEnd();
    return state.value ? 'string' : null;
  },
};

export const unitFile = StreamLanguage.define(unitFileParser);
