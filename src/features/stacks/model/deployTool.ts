import type { Capabilities, ComposeSupport, StackOwner } from '@/shared/api';

/**
 * What deploys a Stack of `owner` from this computer: the compose CLI for a
 * Host, the docker CLI (`docker stack deploy`) for a Swarm.
 */
export function deployTool(
  capabilities: Capabilities | undefined,
  owner: StackOwner,
): ComposeSupport | undefined {
  return owner.kind === 'swarm' ? capabilities?.stackDeploy : capabilities?.compose;
}
