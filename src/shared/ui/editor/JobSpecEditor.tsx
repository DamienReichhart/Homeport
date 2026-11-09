import CodeMirror from '@uiw/react-codemirror';
import { useResolvedTheme } from '@/shared/lib';
import { editorTheme } from './editorTheme';
import { jobSpec } from './jobSpec';

/** A Nomad job specification, HCL2 or JSON, in an editor. */
export function JobSpecEditor({
  value,
  onChange,
  label,
  height = '22rem',
  readOnly = false,
}: {
  value: string;
  onChange?: (value: string) => void;
  /** Names the editor for assistive technology. */
  label: string;
  height?: string;
  readOnly?: boolean;
}) {
  const theme = useResolvedTheme();
  return (
    <div className="overflow-hidden rounded-md border">
      <CodeMirror
        value={value}
        onChange={onChange}
        readOnly={readOnly}
        extensions={[jobSpec]}
        theme={editorTheme(theme)}
        height={height}
        basicSetup={{ foldGutter: true, highlightActiveLine: !readOnly }}
        aria-label={label}
      />
    </div>
  );
}
