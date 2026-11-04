import { Textarea } from '@/shared/ui';
import { Field } from './Field';

/** User labels, one `key=value` per line. */
export function LabelsField({ value, onChange }: { value: string; onChange: (v: string) => void }) {
  return (
    <Field
      id="run-labels"
      label="Labels"
      hint="One key=value per line, as reverse proxies and backup tools read them."
    >
      <Textarea
        id="run-labels"
        value={value}
        onChange={(e) => onChange(e.target.value)}
        placeholder={'traefik.enable=true\ntraefik.http.routers.web.rule=Host(`web.home.arpa`)'}
        className="h-24 font-mono text-xs"
        spellCheck={false}
      />
    </Field>
  );
}
