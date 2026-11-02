import { Channel } from '@tauri-apps/api/core';
import { toAppError, type AppError } from './errors';
import { commands } from './invoke';

/**
 * Starts a backend stream (any `*Subscribe` command) and returns a function
 * that cancels it. Safe to call the canceller before the stream has started.
 */
export function subscribe<T>(
  start: (channel: Channel<T>) => Promise<number>,
  onMessage: (message: T) => void,
  onError?: (error: AppError) => void,
): () => void {
  const channel = new Channel<T>();
  channel.onmessage = onMessage;
  let id: number | null = null;
  let cancelled = false;
  start(channel)
    .then((streamId) => {
      if (cancelled) void commands.streamCancel(streamId);
      else id = streamId;
    })
    .catch((error: unknown) => onError?.(toAppError(error)));
  return () => {
    cancelled = true;
    if (id !== null) void commands.streamCancel(id);
    channel.onmessage = () => {};
  };
}
