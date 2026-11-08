import { commands } from '@/shared/api';
import { pickFile } from '@/shared/lib';
import { t } from '@/shared/i18n';

/** Asks for a manifest on this computer and reads it; `null` when cancelled. */
export async function openManifest(): Promise<string | null> {
  const path = await pickFile(t('apply.openTitle'), [
    { name: t('apply.manifests'), extensions: ['yaml', 'yml', 'json'] },
  ]);
  return path ? commands.readTextFile(path) : null;
}
