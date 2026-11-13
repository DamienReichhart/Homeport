/** Japanese text of the `format` namespace. */
import type en from '../en/format';
import type { Translation } from '../types';

export default {
  'format.bytes.b': '{value} B',
  'format.bytes.kb': '{value} KB',
  'format.bytes.mb': '{value} MB',
  'format.bytes.gb': '{value} GB',
  'format.bytes.tb': '{value} TB',
  'format.perSecond': '{value}/秒',
  'format.bits.b': '{value} bps',
  'format.bits.kb': '{value} kbps',
  'format.bits.mb': '{value} Mbps',
  'format.bits.gb': '{value} Gbps',
  'format.bits.tb': '{value} Tbps',
} satisfies Translation<typeof en>;
