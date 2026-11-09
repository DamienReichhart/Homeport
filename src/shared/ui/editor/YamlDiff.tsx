import { yaml } from '@codemirror/lang-yaml';
import { unifiedMergeView } from '@codemirror/merge';
import { EditorState } from '@codemirror/state';
import { EditorView } from '@codemirror/view';
import CodeMirror from '@uiw/react-codemirror';
import { useMemo } from 'react';
import { useResolvedTheme } from '@/shared/lib';
import { editorLabel, editorTheme } from './editorTheme';

/**
 * A read-only YAML file with what changed from `before` shown inline:
 * removed lines struck through above the lines that replace them.
 */
export function YamlDiff({
  before,
  after,
  label,
  maxHeight = '24rem',
}: {
  before: string;
  after: string;
  /** Names the view for assistive technology. */
  label: string;
  maxHeight?: string;
}) {
  const theme = useResolvedTheme();
  const extensions = useMemo(
    () => [
      yaml(),
      EditorState.readOnly.of(true),
      EditorView.editable.of(false),
      unifiedMergeView({ original: before, mergeControls: false, gutter: true }),
      editorLabel(label),
    ],
    [before, label],
  );
  return (
    <div className="overflow-hidden rounded-md border">
      <CodeMirror
        value={after}
        extensions={extensions}
        theme={editorTheme(theme)}
        maxHeight={maxHeight}
        basicSetup={{ foldGutter: false, highlightActiveLine: false }}
      />
    </div>
  );
}
