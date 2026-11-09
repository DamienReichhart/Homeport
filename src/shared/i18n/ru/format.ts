/** Russian text of the `format` namespace. */
import type en from '../en/format';
import type { Translation } from '../types';

export default {
  'format.bytes.b': '{value} Б',
  'format.bytes.kb': '{value} КБ',
  'format.bytes.mb': '{value} МБ',
  'format.bytes.gb': '{value} ГБ',
  'format.bytes.tb': '{value} ТБ',
} satisfies Translation<typeof en>;
