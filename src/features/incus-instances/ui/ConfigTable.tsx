import { Badge } from '@/shared/ui';
import { t } from '@/shared/i18n';

/**
 * An Instance's configuration keys, its own and those its profiles set,
 * with `volatile.` keys (Incus's bookkeeping) last.
 */
export function ConfigTable({
  own,
  expanded,
}: {
  own: Record<string, string>;
  expanded: Record<string, string>;
}) {
  const keys = Object.keys(expanded).sort((a, b) => {
    const va = a.startsWith('volatile.');
    const vb = b.startsWith('volatile.');
    return va === vb ? a.localeCompare(b) : va ? 1 : -1;
  });
  if (keys.length === 0)
    return <p className="text-muted-foreground text-sm">{t('incus.instances.noConfig')}</p>;
  return (
    <table className="w-full text-sm">
      <thead className="text-muted-foreground text-left text-xs">
        <tr>
          <th className="py-1 pr-4 font-normal">{t('incus.instances.config.key')}</th>
          <th className="py-1 pr-4 font-normal">{t('incus.instances.config.value')}</th>
          <th className="py-1 font-normal" />
        </tr>
      </thead>
      <tbody>
        {keys.map((key) => (
          <tr key={key} className="border-t align-top">
            <td className="py-1.5 pr-4 font-mono text-xs">{key}</td>
            <td className="py-1.5 pr-4 font-mono text-xs break-all">{expanded[key]}</td>
            <td className="py-1.5 text-right">
              {!(key in own) && (
                <Badge variant="outline">{t('incus.instances.config.fromProfile')}</Badge>
              )}
            </td>
          </tr>
        ))}
      </tbody>
    </table>
  );
}

/** An Instance's devices, each with its settings, and whether a profile adds it. */
export function DevicesList({
  own,
  expanded,
}: {
  own: Record<string, Record<string, string>>;
  expanded: Record<string, Record<string, string>>;
}) {
  const names = Object.keys(expanded).sort();
  if (names.length === 0)
    return <p className="text-muted-foreground text-sm">{t('incus.instances.noDevices')}</p>;
  return (
    <div className="grid gap-3">
      {names.map((name) => {
        const device = expanded[name] ?? {};
        return (
          <div key={name} className="rounded-md border p-3">
            <p className="mb-2 flex items-center gap-2 text-sm font-medium">
              {name}
              <Badge variant="secondary">{device.type ?? '—'}</Badge>
              {!(name in own) && (
                <Badge variant="outline">{t('incus.instances.config.fromProfile')}</Badge>
              )}
            </p>
            <dl className="grid grid-cols-[max-content_1fr] gap-x-6 gap-y-1 text-xs">
              {Object.entries(device)
                .filter(([k]) => k !== 'type')
                .map(([k, v]) => (
                  <div key={k} className="contents">
                    <dt className="text-muted-foreground font-mono">{k}</dt>
                    <dd className="font-mono break-all">{v}</dd>
                  </div>
                ))}
            </dl>
          </div>
        );
      })}
    </div>
  );
}
