import { useEffect } from 'react';
import type { ComposeCheck } from '@/shared/api';
import { Label, Textarea } from '@/shared/ui';
import { useComposeCheck } from '../api/useComposeCheck';
import { ComposeChecks } from './ComposeChecks';
import { YamlEditor } from './YamlEditor';
import { t } from '@/shared/i18n';

/** An example environment, written as a .env file. */
const ENV_EXAMPLE = 'DB_PASSWORD=…\nTAG=1.4';

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
  const check = useComposeCheck([compose], 'managed');
  useEffect(() => onCheck?.(check), [check, onCheck]);

  return (
    <div className="grid gap-4 lg:grid-cols-[1fr_20rem]">
      <div className="grid gap-1.5">
        <Label>compose.yaml</Label>
        <YamlEditor value={compose} onChange={onComposeChange} label={t('stacks.editor.compose')} />
      </div>
      <div className="grid content-start gap-4">
        <div className="grid gap-1.5">
          <Label htmlFor="stack-env">{t('stacks.editor.environment')}</Label>
          <Textarea
            id="stack-env"
            value={env}
            onChange={(e) => onEnvChange(e.target.value)}
            disabled={!keychain}
            placeholder={keychain ? ENV_EXAMPLE : t('stacks.editor.envNoKeychain')}
            className="h-40 font-mono text-xs"
            spellCheck={false}
          />
          <p className="text-muted-foreground text-xs">
            {keychain
              ? t('stacks.editor.envHint', { variable: '${VAR}' })
              : t('stacks.editor.envHintNoKeychain')}
          </p>
        </div>
        <ComposeChecks check={check} />
      </div>
    </div>
  );
}
