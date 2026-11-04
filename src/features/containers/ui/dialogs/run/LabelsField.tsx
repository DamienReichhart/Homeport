import { Textarea } from '@/shared/ui';
import { Field } from './Field';
import { t } from '@/shared/i18n';

/** User labels, one `key=value` per line. */
export function LabelsField({ value, onChange }: { value: string; onChange: (v: string) => void }) {
  return (
    <Field
      id="run-labels"
      label={t('containers.advanced.labels')}
      hint={t('containers.advanced.labelsHint')}
    >
      <Textarea
        id="run-labels"
        value={value}
        onChange={(e) => onChange(e.target.value)}
        // eslint-disable-next-line homeport/no-literal-ui-string -- sample labels
        placeholder={'traefik.enable=true\ntraefik.http.routers.web.rule=Host(`web.home.arpa`)'}
        className="h-24 font-mono text-xs"
        spellCheck={false}
      />
    </Field>
  );
}
