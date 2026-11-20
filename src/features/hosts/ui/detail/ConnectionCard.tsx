import { useState } from 'react';
import { useCapabilities, type HostView } from '@/shared/api';
import { Button, Card, CardContent, CardHeader, CardTitle, ErrorMessage } from '@/shared/ui';
import { useUpdateHost } from '../../api/mutations';
import { formFromHost, type HostFormState } from '../../model/form';
import { HostForm } from '../form/HostForm';
import { t } from '@/shared/i18n';

const same = (a: HostFormState, b: HostFormState) => JSON.stringify(a) === JSON.stringify(b);

/**
 * Edits how Homeport reaches a Host, starting from its saved settings. Save
 * works once something changed, and the form starts over from what was saved.
 */
export function ConnectionCard({ view }: { view: HostView }) {
  const capabilities = useCapabilities();
  const [form, setForm] = useState<HostFormState>(() => formFromHost(view.host));
  const save = useUpdateHost(view.host.id);
  const changed = !same(form, formFromHost(view.host));
  return (
    <Card>
      <CardHeader>
        <CardTitle>{t('hosts.connection.title')}</CardTitle>
      </CardHeader>
      <CardContent className="grid gap-4">
        <HostForm
          form={form}
          onChange={setForm}
          editing
          keychain={capabilities.data?.keychain ?? true}
        />
        {save.error && <ErrorMessage error={save.error} />}
        <div className="flex gap-2">
          <Button
            disabled={!changed || save.isPending}
            onClick={() =>
              save.mutate(form, { onSuccess: (saved) => setForm(formFromHost(saved.host)) })
            }
          >
            {t('hosts.connection.save')}
          </Button>
          {changed && (
            <Button
              variant="ghost"
              disabled={save.isPending}
              onClick={() => {
                save.reset();
                setForm(formFromHost(view.host));
              }}
            >
              {t('hosts.connection.discard')}
            </Button>
          )}
        </div>
      </CardContent>
    </Card>
  );
}
