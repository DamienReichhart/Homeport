/** Arabic text of the `format` namespace. */
import type en from '../en/format';
import type { Translation } from '../types';

export default {
  'format.bytes.b': '{value} B',
  'format.bytes.kb': '{value} KB',
  'format.bytes.mb': '{value} MB',
  'format.bytes.gb': '{value} GB',
  'format.bytes.tb': '{value} TB',
} satisfies Translation<typeof en>;
