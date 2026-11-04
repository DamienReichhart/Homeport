import { ChevronRightIcon } from 'lucide-react';
import { useState } from 'react';
import { cn } from '@/shared/lib';
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
} from '@/shared/ui';
import type { AdvancedErrors, AdvancedField, AdvancedForm } from '../../../model/runAdvanced';
import { DevicesField } from './DevicesField';
import { HealthcheckField } from './HealthcheckField';
import { LabelsField } from './LabelsField';
import { LoggingField } from './LoggingField';
import { NetworkIdentityField } from './NetworkIdentityField';
import { ResourcesField } from './ResourcesField';
import { SecurityField } from './SecurityField';
import { t, type MessageKey } from '@/shared/i18n';

const SECTIONS = [
  {
    value: 'resources',
    label: 'containers.advanced.resources',
    fields: ['cpus', 'cpuShares', 'memory', 'memoryReservation', 'memorySwap'],
  },
  { value: 'labels', label: 'containers.advanced.labels', fields: [] },
  { value: 'security', label: 'containers.advanced.security', fields: [] },
  { value: 'devices', label: 'containers.advanced.devices', fields: ['gpuCount'] },
  {
    value: 'health',
    label: 'containers.advanced.healthcheck',
    fields: ['interval', 'timeout', 'retries', 'startPeriod'],
  },
  { value: 'network', label: 'containers.advanced.network', fields: [] },
  { value: 'logging', label: 'containers.advanced.logging', fields: [] },
] as const satisfies { value: string; label: MessageKey; fields: AdvancedField[] }[];

/**
 * The Run dialog's less common options, collapsed by default so the usual
 * case stays short; one tab per section.
 */
export function AdvancedOptions({
  value,
  errors,
  defaultOpen = false,
  onChange,
}: {
  value: AdvancedForm;
  errors: AdvancedErrors;
  defaultOpen?: boolean;
  onChange: (value: AdvancedForm) => void;
}) {
  const [open, setOpen] = useState(defaultOpen);
  const set =
    <K extends keyof AdvancedForm>(key: K) =>
    (v: AdvancedForm[K]) =>
      onChange({ ...value, [key]: v });
  const failing = SECTIONS.some((s) => s.fields.some((f: AdvancedField) => errors[f]));

  return (
    <Collapsible open={open || failing} onOpenChange={setOpen} className="grid gap-3">
      <CollapsibleTrigger className="hover:text-primary flex items-center gap-1 justify-self-start text-sm font-medium">
        <ChevronRightIcon
          className={cn('size-4 transition-transform', (open || failing) && 'rotate-90')}
        />
        {t('containers.advanced.title')}
      </CollapsibleTrigger>
      <CollapsibleContent>
        <Tabs defaultValue="resources" className="gap-3">
          <TabsList className="h-auto flex-wrap justify-start">
            {SECTIONS.map((s) => (
              <TabsTrigger key={s.value} value={s.value} className="flex-none">
                {t(s.label)}
                {s.fields.some((f: AdvancedField) => errors[f]) && (
                  <span
                    className="bg-destructive size-1.5 rounded-full"
                    aria-label={t('containers.advanced.hasErrors')}
                  />
                )}
              </TabsTrigger>
            ))}
          </TabsList>
          <TabsContent value="resources">
            <ResourcesField value={value.resources} errors={errors} onChange={set('resources')} />
          </TabsContent>
          <TabsContent value="labels">
            <LabelsField value={value.labels} onChange={set('labels')} />
          </TabsContent>
          <TabsContent value="security">
            <SecurityField value={value.security} onChange={set('security')} />
          </TabsContent>
          <TabsContent value="devices">
            <DevicesField value={value.security} errors={errors} onChange={set('security')} />
          </TabsContent>
          <TabsContent value="health">
            <HealthcheckField
              value={value.healthcheck}
              errors={errors}
              onChange={set('healthcheck')}
            />
          </TabsContent>
          <TabsContent value="network">
            <NetworkIdentityField value={value.identity} onChange={set('identity')} />
          </TabsContent>
          <TabsContent value="logging">
            <LoggingField value={value.logging} onChange={set('logging')} />
          </TabsContent>
        </Tabs>
      </CollapsibleContent>
    </Collapsible>
  );
}
