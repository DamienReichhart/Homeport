/** Hindi text of the `format` namespace. */
import type en from '../en/format';
import type { Translation } from '../types';

export default {
  'format.bytes.b': '{value} B',
  'format.bytes.kb': '{value} KB',
  'format.bytes.mb': '{value} MB',
  'format.bytes.gb': '{value} GB',
  'format.bytes.tb': '{value} TB',
  'format.perSecond': '{value}/से',
  'format.bits.b': '{value} bit/s',
  'format.bits.kb': '{value} kbit/s',
  'format.bits.mb': '{value} Mbit/s',
  'format.bits.gb': '{value} Gbit/s',
  'format.bits.tb': '{value} Tbit/s',
} satisfies Translation<typeof en>;
