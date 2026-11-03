import { useEffect, useState } from 'react';
import { commands, type ComposeCheck } from '@/shared/api';

/** Re-checks a compose definition shortly after the user stops typing. */
export function useComposeCheck(compose: string): ComposeCheck | null {
  const [check, setCheck] = useState<ComposeCheck | null>(null);
  useEffect(() => {
    const timer = setTimeout(() => {
      commands
        .stacksCheck(compose)
        .then(setCheck)
        .catch(() => setCheck(null));
    }, 400);
    return () => clearTimeout(timer);
  }, [compose]);
  return check;
}
