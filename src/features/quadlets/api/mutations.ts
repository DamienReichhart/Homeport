import { useMutation, useQueryClient } from '@tanstack/react-query';
import { toast } from 'sonner';
import { hostKeys } from '@/core/host';
import { commands, type QuadletInstall, type QuadletUnit, type UnitAction } from '@/shared/api';
import { toastError } from '@/shared/ui';
import { t, type MessageKey } from '@/shared/i18n';

/** Units and their containers change together. */
function useRefreshUnits(hostId: string) {
  const client = useQueryClient();
  return () =>
    Promise.all([
      client.invalidateQueries({ queryKey: hostKeys.quadlets(hostId) }),
      client.invalidateQueries({ queryKey: hostKeys.containers(hostId) }),
    ]);
}

const DONE: Record<UnitAction, MessageKey> = {
  start: 'quadlets.done.start',
  stop: 'quadlets.done.stop',
  restart: 'quadlets.done.restart',
};

/** Starts, stops or restarts a unit's service, reporting the outcome as a toast. */
export function useUnitAction(hostId: string) {
  const refresh = useRefreshUnits(hostId);
  return useMutation({
    mutationFn: ({ unit, action }: { unit: QuadletUnit; action: UnitAction }) =>
      commands.quadletsAct(hostId, unit.unitName ?? '', unit.scope, action),
    onSuccess: (_, { unit, action }) =>
      toast.success(t(DONE[action], { name: unit.unitName ?? '' })),
    onError: (e) => toastError(e),
    onSettled: () => refresh(),
  });
}

/**
 * Installs a unit, or replaces an installed one's file (Edit), refused when
 * it changed on the Host since `expected`; the dialog shows a failure.
 */
export function useInstallUnit(hostId: string, onDone: () => void) {
  const refresh = useRefreshUnits(hostId);
  return useMutation({
    meta: { silent: true },
    mutationFn: ({
      install,
      replace,
      expected,
    }: {
      install: QuadletInstall;
      replace: boolean;
      /** The unit file as the edit opened it; null overwrites what the Host has now. */
      expected: string | null;
    }) => commands.quadletsInstall(hostId, install, replace, expected),
    onSuccess: async (_, { install, replace }) => {
      await refresh();
      toast.success(
        t(replace ? 'quadlets.saved' : 'quadlets.installed', { name: install.unit.name }),
      );
      onDone();
    },
  });
}

/** Removes a unit; `force` stops its service first. */
export function useRemoveUnit(hostId: string) {
  const refresh = useRefreshUnits(hostId);
  return useMutation({
    mutationFn: ({ name, force }: { name: string; force: boolean }) =>
      commands.quadletsRemove(hostId, name, force),
    onSuccess: (_, { name }) => toast.success(t('quadlets.removed', { name })),
    onError: (e) => toastError(e),
    onSettled: () => refresh(),
  });
}
