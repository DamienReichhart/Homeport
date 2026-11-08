// Public API of the feature contract. Import from "@/core/feature" only.
export type { CommandContext, PaletteCommand } from './command';
export { defineFeature, type FeatureManifest, type NavEntry } from './manifest';
export { useActionRequest, type ActionRequest } from './useActionRequest';
export { usePageQuery } from './usePageQuery';
