import { useMutation, useQueryClient } from '@tanstack/react-query';
import { useState } from 'react';
import { toast } from 'sonner';
import { cursorOf, EVENT_PAGE, hostKeys } from '@/core/host';
import { commands, type EngineEvent } from '@/shared/api';
import { toastError } from '@/shared/ui';
import { t } from '@/shared/i18n';

/**
 * The pages of a Host's kept events older than `latest`, loaded one at a
 * time on request; `hasMore` is false once the oldest kept event is listed.
 */
export function useOlderEvents(hostId: string, latest: EngineEvent[]) {
  const [older, setOlder] = useState<EngineEvent[]>([]);
  const [exhausted, setExhausted] = useState(false);
  const oldest = older[0] ?? latest[0];
  const load = useMutation({
    mutationFn: (from: EngineEvent) => commands.hostsEvents(hostId, EVENT_PAGE, cursorOf(from)),
    onSuccess: (page) => {
      setOlder((list) => [...page, ...list]);
      if (page.length < EVENT_PAGE) setExhausted(true);
    },
  });
  const reset = () => {
    setOlder([]);
    setExhausted(false);
  };
  return {
    older,
    hasMore: !exhausted && oldest !== undefined && older.length + latest.length >= EVENT_PAGE,
    loading: load.isPending,
    loadMore: () => {
      if (oldest && !load.isPending) load.mutate(oldest);
    },
    reset,
  };
}

/** Forgets a Host's kept events; new ones keep arriving. */
export function useClearEvents(hostId: string, onCleared: () => void) {
  const client = useQueryClient();
  return useMutation({
    mutationFn: () => commands.hostsEventsClear(hostId),
    onSuccess: () => {
      client.setQueryData<EngineEvent[]>(hostKeys.events(hostId), []);
      onCleared();
      toast.success(t('events.cleared'));
    },
    onError: (e) => toastError(e),
  });
}
