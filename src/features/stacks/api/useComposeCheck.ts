import { useEffect, useState } from 'react';
import { commands, type ComposeCheck, type SourceKind } from '@/shared/api';

/**
 * Re-checks a definition made of `files` (merged in order), against the
 * rules of its `kind`, shortly after the user stops typing.
 */
export function useComposeCheck(files: string[], kind: SourceKind): ComposeCheck | null {
  const [check, setCheck] = useState<ComposeCheck | null>(null);
  // One string, so a new array with the same files doesn't check again.
  const key = JSON.stringify(files);
  useEffect(() => {
    const timer = setTimeout(() => {
      commands
        .stacksCheck(JSON.parse(key) as string[], kind)
        .then(setCheck)
        .catch(() => setCheck(null));
    }, 400);
    return () => clearTimeout(timer);
  }, [key, kind]);
  return check;
}
