import { yaml } from '@codemirror/lang-yaml';
import CodeMirror from '@uiw/react-codemirror';
import { useResolvedTheme } from '@/shared/lib';
import { editorTheme } from './editorTheme';

/** A YAML file in an editor (a compose file, Kubernetes manifests); `readOnly` still lets it be selected and copied. */
export function YamlEditor({
  value,
  onChange,
  label,
  height = '28rem',
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
        extensions={[yaml()]}
        theme={editorTheme(theme)}
        height={height}
        basicSetup={{ foldGutter: true, highlightActiveLine: true }}
        aria-label={label}
      />
    </div>
  );
}
