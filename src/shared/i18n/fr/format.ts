/** French text of the `format` namespace. */
import type en from '../en/format';
import type { Translation } from '../types';

export default {
  'format.bytes.b': '{value} o',
  'format.bytes.kb': '{value} Ko',
  'format.bytes.mb': '{value} Mo',
  'format.bytes.gb': '{value} Go',
  'format.bytes.tb': '{value} To',
  'format.perSecond': '{value}/s',
  'format.bits.b': '{value} bit/s',
  'format.bits.kb': '{value} kbit/s',
  'format.bits.mb': '{value} Mbit/s',
  'format.bits.gb': '{value} Gbit/s',
  'format.bits.tb': '{value} Tbit/s',
} satisfies Translation<typeof en>;
