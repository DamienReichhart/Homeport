import { commands } from '@/shared/api';
import { pickFile } from '@/shared/lib';
import { t } from '@/shared/i18n';

/** Asks for a job specification on this computer and reads it; `null` when cancelled. */
export async function openJobSpec(): Promise<string | null> {
  const path = await pickFile(t('nomad.run.openTitle'), [
    { name: t('nomad.run.specFiles'), extensions: ['hcl', 'nomad', 'json'] },
  ]);
  return path ? commands.readTextFile(path) : null;
}
