/** Russian text of the `format` namespace. */
import type en from '../en/format';
import type { Translation } from '../types';

export default {
  'format.bytes.b': '{value} Б',
  'format.bytes.kb': '{value} КБ',
  'format.bytes.mb': '{value} МБ',
  'format.bytes.gb': '{value} ГБ',
  'format.bytes.tb': '{value} ТБ',
  'format.perSecond': '{value}/с',
  'format.bits.b': '{value} бит/с',
  'format.bits.kb': '{value} кбит/с',
  'format.bits.mb': '{value} Мбит/с',
  'format.bits.gb': '{value} Гбит/с',
  'format.bits.tb': '{value} Тбит/с',
} satisfies Translation<typeof en>;
