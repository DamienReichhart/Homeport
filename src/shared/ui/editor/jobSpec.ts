import { StreamLanguage, type StreamParser, type StringStream } from '@codemirror/language';

interface JobSpecState {
  /** Inside a `/* … *\/` comment. */
  comment: boolean;
}

/**
 * Nomad job specifications, HCL2 or JSON: comments (`#`, `//`, `/* *\/`),
 * strings, numbers, `true`/`false`/`null`, attribute names before `=` or
 * `:`, and block types such as `job`, `group` or `task`.
 */
export const jobSpecParser: StreamParser<JobSpecState> = {
  name: 'nomad-job',
  startState: () => ({ comment: false }),
  token(stream: StringStream, state: JobSpecState) {
    if (state.comment) {
      if (stream.match(/^.*?\*\//)) state.comment = false;
      else stream.skipToEnd();
      return 'comment';
    }
    if (stream.eatSpace()) return null;
    if (stream.match(/^(#|\/\/).*/)) return 'comment';
    if (stream.match('/*')) {
      state.comment = true;
      return 'comment';
    }
    if (stream.match(/^"(?:[^"\\]|\\.)*"?/)) {
      // A JSON key is a string followed by a colon.
      return stream.match(/^\s*:/, false) ? 'propertyName' : 'string';
    }
    if (stream.match(/^-?\d+(\.\d+)?([eE][+-]?\d+)?\b/)) return 'number';
    if (stream.match(/^(true|false|null)\b/)) return 'atom';
    if (stream.match(/^[A-Za-z_][\w-]*/)) {
      if (stream.match(/^\s*=/, false)) return 'propertyName';
      return 'keyword';
    }
    stream.next();
    return null;
  },
};

export const jobSpec = StreamLanguage.define(jobSpecParser);
