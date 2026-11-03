import { Channel } from '@tauri-apps/api/core';
import { useState } from 'react';
import { commands, toAppError, type UpdateProgress } from '@/shared/api';
import type { InstallState } from '../model/updates';

/** Installs the update found by the last check. Homeport restarts when it's done. */
export function useInstallUpdate() {
  const [state, setState] = useState<InstallState>({ step: 'idle' });
  const install = () => {
    const channel = new Channel<UpdateProgress>();
    channel.onmessage = ({ downloaded, total }) => {
      if (total && downloaded >= total) setState({ step: 'installing' });
      else
        setState({
          step: 'downloading',
          percent: total ? Math.floor((downloaded / total) * 100) : null,
        });
    };
    setState({ step: 'downloading', percent: null });
    commands
      .updatesInstall(channel)
      .catch((error: unknown) => setState({ step: 'failed', error: toAppError(error) }));
  };
  return { state, install, busy: state.step === 'downloading' || state.step === 'installing' };
}
