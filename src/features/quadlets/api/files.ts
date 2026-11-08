import { commands } from '@/shared/api';
import { pickFile, pickFiles } from '@/shared/lib';
import { baseName, type ExtraFile } from '../model/units';
import { t } from '@/shared/i18n';

/** Asks for a unit file on this computer and reads it; `null` when cancelled. */
export async function openUnitFile(): Promise<ExtraFile | null> {
  const path = await pickFile(t('quadlets.form.openTitle'), [
    {
      name: t('quadlets.form.unitFiles'),
      extensions: ['container', 'pod', 'volume', 'network', 'kube', 'image', 'build', 'artifact'],
    },
  ]);
  return path ? { name: baseName(path), content: await commands.readTextFile(path) } : null;
}

/** Asks for files to install beside a unit and reads them; empty when cancelled. */
export async function openExtraFiles(): Promise<ExtraFile[]> {
  const paths = await pickFiles(t('quadlets.form.extraTitle'));
  return Promise.all(
    paths.map(async (path) => ({
      name: baseName(path),
      content: await commands.readTextFile(path),
    })),
  );
}
