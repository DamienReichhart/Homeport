// Public API of the stacks feature. Import from "@/features/stacks" only.
export { useSyncedUpdates } from './api/useSyncedUpdates';
export { stacksFeature } from './manifest';
export { KIND_BADGE, KIND_LABEL } from './model/stackKind';
export { STATE_LABEL, STATE_TONE } from './model/stackState';
export { hostOwner, stackPath } from './api/owner';
export type { ComposeJob } from './model/composeJob';
export { deployTool } from './model/deployTool';
export { ComposeRunDialog } from './ui/dialogs/ComposeRunDialog';
export { RegistryAuthSwitch } from './ui/RegistryAuthSwitch';
export { StackDefinition } from './ui/detail/StackDefinition';
export { StackHistory } from './ui/detail/StackHistory';
