import { commands } from '@/shared/api';
import { nativeTemplates, resolveLocale, setLocale, SYSTEM_LANGUAGE } from '@/shared/i18n';

/**
 * Shows the app in the Settings value `language` (`system` follows the OS),
 * without a restart, and hands the tray and notifications the same language.
 */
export async function applyLanguage(language: string | undefined): Promise<void> {
  await setLocale(resolveLocale(language ?? SYSTEM_LANGUAGE));
  await commands.setNativeText(nativeTemplates()).catch((e: unknown) => {
    // The window still switched; only the tray stays in the previous language.
    console.warn('couldn’t hand the language to the tray', e);
  });
}
