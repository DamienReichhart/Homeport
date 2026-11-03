import { yaml } from '@codemirror/lang-yaml';
import CodeMirror from '@uiw/react-codemirror';
import { CircleXIcon, TriangleAlertIcon } from 'lucide-react';
import { useEffect, useState } from 'react';
import { commands, type ComposeCheck } from '@/shared/api';
import { useResolvedTheme } from '@/shared/lib';
import { editorTheme, Label, Textarea } from '@/shared/ui';

export const STARTER = `services:
  web:
    image: nginx:alpine
    ports:
      - "8080:80"
    restart: unless-stopped
`;

/** Re-checks the definition shortly after the user stops typing. */
function useCheck(compose: string): ComposeCheck | null {
  const [check, setCheck] = useState<ComposeCheck | null>(null);
  useEffect(() => {
    const timer = setTimeout(() => {
      commands
        .stacksCheck(compose)
        .then(setCheck)
        .catch(() => setCheck(null));
    }, 400);
    return () => clearTimeout(timer);
  }, [compose]);
  return check;
}

/** The compose definition and environment of a Managed Stack, with live checks. */
export function StackEditor({
  compose,
  env,
  onComposeChange,
  onEnvChange,
  keychain,
  onCheck,
}: {
  compose: string;
  env: string;
  onComposeChange: (value: string) => void;
  onEnvChange: (value: string) => void;
  keychain: boolean;
  onCheck?: (check: ComposeCheck | null) => void;
}) {
  const theme = useResolvedTheme();
  const check = useCheck(compose);
  useEffect(() => onCheck?.(check), [check, onCheck]);

  return (
    <div className="grid gap-4 lg:grid-cols-[1fr_20rem]">
      <div className="grid gap-1.5">
        <Label>compose.yaml</Label>
        <div className="overflow-hidden rounded-md border">
          <CodeMirror
            value={compose}
            onChange={onComposeChange}
            extensions={[yaml()]}
            theme={editorTheme(theme)}
            height="28rem"
            basicSetup={{ foldGutter: true, highlightActiveLine: true }}
            aria-label="Compose definition"
          />
        </div>
      </div>
      <div className="grid content-start gap-4">
        <div className="grid gap-1.5">
          <Label htmlFor="stack-env">Environment</Label>
          <Textarea
            id="stack-env"
            value={env}
            onChange={(e) => onEnvChange(e.target.value)}
            disabled={!keychain}
            placeholder={keychain ? 'DB_PASSWORD=…\nTAG=1.4' : 'Needs an OS keychain'}
            className="h-40 font-mono text-xs"
            spellCheck={false}
          />
          <p className="text-muted-foreground text-xs">
            {keychain
              ? 'Like a .env file: stored encrypted, passed to compose for ${VAR} substitution, never written to disk.'
              : 'No OS keychain is available, so a Stack environment can’t be stored.'}
          </p>
        </div>
        {check && (check.errors.length > 0 || check.warnings.length > 0) && (
          <ul className="grid gap-2 text-sm" aria-label="Problems">
            {check.errors.map((e) => (
              <li key={e} className="text-destructive flex gap-2">
                <CircleXIcon className="mt-0.5 size-4 shrink-0" /> {e}
              </li>
            ))}
            {check.warnings.map((w) => (
              <li key={w} className="text-warning flex gap-2">
                <TriangleAlertIcon className="mt-0.5 size-4 shrink-0" /> {w}
              </li>
            ))}
          </ul>
        )}
      </div>
    </div>
  );
}
