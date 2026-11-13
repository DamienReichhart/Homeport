/** Arabic text of the `format` namespace. */
import type en from '../en/format';
import type { Translation } from '../types';

export default {
  'format.bytes.b': '{value} بايت',
  'format.bytes.kb': '{value} كيلوبايت',
  'format.bytes.mb': '{value} ميغابايت',
  'format.bytes.gb': '{value} غيغابايت',
  'format.bytes.tb': '{value} تيرابايت',
  'format.perSecond': '{value}/ث',
  'format.bits.b': '{value} بت/ث',
  'format.bits.kb': '{value} كيلوبت/ث',
  'format.bits.mb': '{value} ميغابت/ث',
  'format.bits.gb': '{value} غيغابت/ث',
  'format.bits.tb': '{value} تيرابت/ث',
} satisfies Translation<typeof en>;
