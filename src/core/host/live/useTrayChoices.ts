import { useEffect, useRef } from 'react';
import { events } from '@/shared/api';

export interface TrayChoices {
  /** A Host was picked in the tray's menu. */
  onHost: (hostId: string) => void;
  /** **Add host…** was picked in the tray's menu. */
  onAddHost: () => void;
}

/** Runs `choices` when the user picks a Host or **Add host…** in the system tray. */
export function useTrayChoices(choices: TrayChoices) {
  const latest = useRef(choices);
  useEffect(() => {
    latest.current = choices;
  });
  useEffect(() => {
    const host = events.trayHostChosen.listen(({ payload }) =>
      latest.current.onHost(payload.hostId),
    );
    const addHost = events.trayAddHostChosen.listen(() => latest.current.onAddHost());
    return () => {
      void host.then((stop) => stop());
      void addHost.then((stop) => stop());
    };
  }, []);
}
