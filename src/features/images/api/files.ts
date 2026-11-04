import { pickFile, pickSavePath } from '@/shared/lib';
import { LOAD_FILTERS, SAVE_FILTERS } from '../model/transfer';

/** Asks where to save an image archive; `null` when cancelled. */
export const chooseArchiveToSave = (defaultName: string) =>
  pickSavePath('Save images as', defaultName, SAVE_FILTERS);

/** Asks for an image archive to load; `null` when cancelled. */
export const chooseArchiveToLoad = () => pickFile('Load images from', LOAD_FILTERS);
