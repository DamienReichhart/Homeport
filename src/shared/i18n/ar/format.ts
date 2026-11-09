/** Arabic text of the `format` namespace. */
import type en from '../en/format';
import type { Translation } from '../types';

export default {
  'format.bytes.b': '{value} بايت',
  'format.bytes.kb': '{value} كيلوبايت',
  'format.bytes.mb': '{value} ميغابايت',
  'format.bytes.gb': '{value} غيغابايت',
  'format.bytes.tb': '{value} تيرابايت',
} satisfies Translation<typeof en>;
