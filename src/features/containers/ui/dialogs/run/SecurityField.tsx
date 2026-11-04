import { Alert, AlertDescription, AlertTitle, Label, Switch } from '@/shared/ui';
import { CAPABILITIES, normaliseCapability } from '../../../model/capabilities';
import type { SecurityForm } from '../../../model/runAdvanced';
import { Field } from './Field';
import { TokenInput } from './TokenInput';

/** Capabilities and privileged mode. */
export function SecurityField({
  value,
  onChange,
}: {
  value: SecurityForm;
  onChange: (value: SecurityForm) => void;
}) {
  return (
    <div className="grid gap-3">
      <div className="grid gap-3 sm:grid-cols-2">
        <Field id="run-cap-add" label="Add capabilities" hint="Like NET_ADMIN for a VPN.">
          <TokenInput
            id="run-cap-add"
            values={value.capAdd}
            suggestions={CAPABILITIES}
            placeholder="NET_ADMIN"
            normalise={normaliseCapability}
            onChange={(capAdd) => onChange({ ...value, capAdd })}
          />
        </Field>
        <Field id="run-cap-drop" label="Drop capabilities" hint="ALL drops every one.">
          <TokenInput
            id="run-cap-drop"
            values={value.capDrop}
            suggestions={CAPABILITIES}
            placeholder="ALL"
            normalise={normaliseCapability}
            onChange={(capDrop) => onChange({ ...value, capDrop })}
          />
        </Field>
      </div>
      <div className="flex items-center gap-2">
        <Switch
          id="run-privileged"
          checked={value.privileged}
          onCheckedChange={(privileged) => onChange({ ...value, privileged })}
        />
        <Label htmlFor="run-privileged">Privileged</Label>
      </div>
      {value.privileged && (
        <Alert variant="destructive">
          <AlertTitle>The container gets full access to the host</AlertTitle>
          <AlertDescription>
            It can use every device and capability, and anything running in it can take over the
            host. Prefer adding the capabilities and devices it needs.
          </AlertDescription>
        </Alert>
      )}
    </div>
  );
}
