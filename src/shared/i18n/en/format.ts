/**
 * Units the format helpers put after a number (`src/shared/lib/format.ts`);
 * every key starts with `format.`. Sizes count in powers of 1024, like the
 * Docker CLI, whatever the unit is called.
 */
export default {
  'format.bytes.b': '{value} B',
  'format.bytes.kb': '{value} KB',
  'format.bytes.mb': '{value} MB',
  'format.bytes.gb': '{value} GB',
  'format.bytes.tb': '{value} TB',
} as const;
