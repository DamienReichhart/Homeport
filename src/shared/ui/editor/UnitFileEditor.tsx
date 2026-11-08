import CodeMirror from '@uiw/react-codemirror';
import { useResolvedTheme } from '@/shared/lib';
import { editorTheme } from './editorTheme';
import { unitFile } from './unitFile';

/** A systemd or Quadlet unit file in an editor; `readOnly` still lets it be selected and copied. */
export function UnitFileEditor({
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
        extensions={[unitFile]}
        theme={editorTheme(theme)}
        height={height}
        basicSetup={{ foldGutter: false, highlightActiveLine: !readOnly }}
        aria-label={label}
      />
    </div>
  );
}
