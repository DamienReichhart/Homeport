import { open, save } from '@tauri-apps/plugin-dialog';

/** Asks for one file to open; `null` when cancelled. */
export async function pickFile(title: string): Promise<string | null> {
  const picked = await open({ title, multiple: false, directory: false });
  return typeof picked === 'string' ? picked : null;
}

/** Asks for files to open; empty when cancelled. */
export async function pickFiles(title: string): Promise<string[]> {
  const picked = await open({ title, multiple: true, directory: false });
  if (!picked) return [];
  return Array.isArray(picked) ? picked : [picked];
}

/** Asks where to save a file; `null` when cancelled. */
export async function pickSavePath(title: string, defaultPath?: string): Promise<string | null> {
  return save({ title, defaultPath });
}

/** Asks for a folder; `null` when cancelled. */
export async function pickFolder(title: string): Promise<string | null> {
  const picked = await open({ title, multiple: false, directory: true });
  return typeof picked === 'string' ? picked : null;
}
