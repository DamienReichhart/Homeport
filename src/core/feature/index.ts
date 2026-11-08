// Public API of the feature contract. Import from "@/core/feature" only.
export type { CommandContext, PaletteCommand } from './command';
export {
  defineFeature,
  featureScope,
  visibleNavEntries,
  type FeatureManifest,
  type NavEntry,
  type NavScope,
} from './manifest';
export { useActionRequest, type ActionRequest } from './useActionRequest';
export { usePageQuery } from './usePageQuery';
