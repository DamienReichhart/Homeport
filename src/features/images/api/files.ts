import { pickFile, pickSavePath } from '@/shared/lib';
import { importFilters, loadFilters, saveFilters } from '../model/transfer';
import { t } from '@/shared/i18n';

/** Asks where to save an image archive; `null` when cancelled. */
export const chooseArchiveToSave = (defaultName: string) =>
  pickSavePath(t('images.files.save'), defaultName, saveFilters());

/** Asks for an image archive to load; `null` when cancelled. */
export const chooseArchiveToLoad = () => pickFile(t('images.files.load'), loadFilters());

/** Asks for a root filesystem tar to import; `null` when cancelled. */
export const chooseFilesystemToImport = () => pickFile(t('images.files.import'), importFilters());
