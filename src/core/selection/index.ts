// Public API of the selection core. Import from "@/core/selection" only.
export { useSelection, type SelectionValue } from './context';
export {
  rememberedSelection,
  resolveSelection,
  sameSelection,
  type Candidate,
  type Selection,
  type SelectionKind,
} from './model/selection';
export { SelectionProvider } from './ui/SelectionProvider';
