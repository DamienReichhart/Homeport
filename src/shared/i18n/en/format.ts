/**
 * Units the format helpers put after a number (`src/shared/lib/format.ts`);
 * every key starts with `format.`. Sizes count in powers of 1024, like the
 * Docker CLI, whatever the unit is called. Bit rates count in powers of
 * 1000, as networks do.
 */
export default {
  'format.bytes.b': '{value} B',
  'format.bytes.kb': '{value} KB',
  'format.bytes.mb': '{value} MB',
  'format.bytes.gb': '{value} GB',
  'format.bytes.tb': '{value} TB',
  'format.perSecond': '{value}/s',
  'format.bits.b': '{value} bit/s',
  'format.bits.kb': '{value} kbit/s',
  'format.bits.mb': '{value} Mbit/s',
  'format.bits.gb': '{value} Gbit/s',
  'format.bits.tb': '{value} Tbit/s',
} as const;
