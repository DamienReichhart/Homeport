import { useQueryClient } from '@tanstack/react-query';
import { useCallback, useEffect, useMemo, useState, type ReactNode } from 'react';
import { patchSettings, settingsQuery } from '@/core/settings';
import { toastError } from '@/shared/ui';
import { SelectionContext, type SelectionValue } from '../context';
import {
  rememberedSelection,
  resolveSelection,
  type Candidate,
  type Selection,
} from '../model/selection';
import { t } from '@/shared/i18n';

/**
 * The settings couldn't be read at start-up. The IPC gateway already wrote the
 * failure to the app log, and the provider falls back to the first Host, so
 * there is nothing more to tell the user here.
 */
function keepFallback() {}

/**
 * Tracks the Host or Cluster the resource pages show, remembered across
 * launches. The app hands it the Hosts and Clusters it may fall back to
 * (`undefined` while a list loads).
 */
export function SelectionProvider({
  hosts,
  clusters,
  children,
}: {
  hosts: Candidate[] | undefined;
  clusters: Candidate[] | undefined;
  children: ReactNode;
}) {
  const client = useQueryClient();
  const [chosen, setChosen] = useState<Selection | null>(null);

  useEffect(() => {
    client
      .fetchQuery(settingsQuery())
      .then((s) => setChosen((current) => current ?? rememberedSelection(s)))
      .catch(keepFallback);
  }, [client]);

  const selection = resolveSelection(chosen, hosts, clusters);

  const select = useCallback(
    (next: Selection) => {
      setChosen(next);
      // lastHostId keeps an older Homeport opening on the same Host.
      const patch =
        next.kind === 'host'
          ? { lastSelection: next, lastHostId: next.id }
          : { lastSelection: next };
      patchSettings(client, patch).catch((e: unknown) =>
        toastError(e, t('core.selection.rememberFailed')),
      );
    },
    [client],
  );

  const kind = selection?.kind;
  const id = selection?.id;
  const value = useMemo<SelectionValue>(
    () => ({ selection: kind && id ? ({ kind, id } as Selection) : null, select }),
    [kind, id, select],
  );
  return <SelectionContext.Provider value={value}>{children}</SelectionContext.Provider>;
}
