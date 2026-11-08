import { EyeIcon, EyeOffIcon } from 'lucide-react';
import { useState } from 'react';
import { formatBytes } from '@/shared/lib';
import type { DataKey, RevealedValue } from '@/shared/api';
import { Badge, Button, CopyButton, toastError } from '@/shared/ui';
import { revealSecretValue } from '../api/queries';
import { t } from '@/shared/i18n';

/**
 * A Secret's keys, each masked until revealed. A revealed value lives in
 * this component's state only: leaving the page hides it again.
 */
export function SecretValues({
  clusterId,
  namespace,
  name,
  keys,
}: {
  clusterId: string;
  namespace: string;
  name: string;
  keys: DataKey[];
}) {
  const [shown, setShown] = useState<Partial<Record<string, RevealedValue>>>({});
  const [busy, setBusy] = useState<string | null>(null);
  const reveal = async (key: string) => {
    setBusy(key);
    try {
      const value = await revealSecretValue(clusterId, namespace, name, key);
      setShown((s) => ({ ...s, [key]: value }));
    } catch (e) {
      toastError(e, t('config.reveal.failed', { key }));
    } finally {
      setBusy(null);
    }
  };
  const hide = (key: string) =>
    setShown((s) => {
      const next = { ...s };
      delete next[key];
      return next;
    });
  if (keys.length === 0)
    return <p className="text-muted-foreground text-sm">{t('config.noKeys')}</p>;
  return (
    <ul className="divide-border grid divide-y rounded-md border">
      {keys.map((k) => {
        const value = shown[k.key];
        return (
          <li key={k.key} className="grid gap-2 p-3">
            <div className="flex items-center justify-between gap-2">
              <span className="flex items-center gap-2">
                <span className="font-mono text-sm font-medium">{k.key}</span>
                <span className="text-muted-foreground text-xs">{formatBytes(k.size)}</span>
                {value?.binary && <Badge variant="outline">{t('config.base64')}</Badge>}
              </span>
              <span className="flex items-center gap-1">
                {value ? (
                  <>
                    <CopyButton text={value.value} label={t('config.copy', { key: k.key })} />
                    <Button size="sm" variant="ghost" onClick={() => hide(k.key)}>
                      <EyeOffIcon /> {t('config.hide')}
                    </Button>
                  </>
                ) : (
                  <Button
                    size="sm"
                    variant="outline"
                    disabled={busy === k.key}
                    aria-label={t('config.revealOne', { key: k.key })}
                    onClick={() => void reveal(k.key)}
                  >
                    <EyeIcon /> {t('config.reveal')}
                  </Button>
                )}
              </span>
            </div>
            <pre className="bg-muted overflow-auto rounded p-2 font-mono text-xs whitespace-pre-wrap break-all">
              {value ? value.value : '••••••••'}
            </pre>
          </li>
        );
      })}
    </ul>
  );
}
