/** French text of the `format` namespace. */
import type en from '../en/format';
import type { Translation } from '../types';

export default {
  'format.bytes.b': '{value} o',
  'format.bytes.kb': '{value} Ko',
  'format.bytes.mb': '{value} Mo',
  'format.bytes.gb': '{value} Go',
  'format.bytes.tb': '{value} To',
} satisfies Translation<typeof en>;
