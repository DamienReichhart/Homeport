import { yaml } from '@codemirror/lang-yaml';
import CodeMirror from '@uiw/react-codemirror';
import { useResolvedTheme } from '@/shared/lib';
import { editorTheme } from '@/shared/ui';

/** A compose file in the YAML editor. */
export function YamlEditor({
  value,
  onChange,
  label,
  height = '28rem',
}: {
  value: string;
  onChange: (value: string) => void;
  /** Names the editor for assistive technology. */
  label: string;
  height?: string;
}) {
  const theme = useResolvedTheme();
  return (
    <div className="overflow-hidden rounded-md border">
      <CodeMirror
        value={value}
        onChange={onChange}
        extensions={[yaml()]}
        theme={editorTheme(theme)}
        height={height}
        basicSetup={{ foldGutter: true, highlightActiveLine: true }}
        aria-label={label}
      />
    </div>
  );
}
