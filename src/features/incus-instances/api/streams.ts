import {
  commands,
  subscribe,
  type AppError,
  type InstanceSpec,
  type PullEvent,
} from '@/shared/api';

/** Makes an Instance; returns what cancels following it (Incus finishes anyway). */
export function startCreateInstance(
  hostId: string,
  project: string,
  spec: InstanceSpec,
  onEvent: (event: PullEvent) => void,
  onError: (error: AppError) => void,
): () => void {
  return subscribe<PullEvent>(
    (ch) => commands.incusInstanceCreateSubscribe(hostId, project, spec, ch),
    onEvent,
    onError,
  );
}
