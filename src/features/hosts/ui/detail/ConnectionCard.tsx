import { useState } from 'react';
import { useCapabilities, type HostView } from '@/shared/api';
import { Button, Card, CardContent, CardHeader, CardTitle, ErrorMessage } from '@/shared/ui';
import { useUpdateHost } from '../../api/mutations';
import { formFromHost, type HostFormState } from '../../model/form';
import { HostForm } from '../form/HostForm';
import { t } from '@/shared/i18n';

/** Edits how Homeport reaches a Host, starting from its saved settings. */
export function ConnectionCard({ view }: { view: HostView }) {
  const capabilities = useCapabilities();
  const [form, setForm] = useState<HostFormState>(() => formFromHost(view.host));
  const save = useUpdateHost(view.host.id);
  return (
    <Card>
      <CardHeader>
        <CardTitle>{t('hosts.connection.title')}</CardTitle>
      </CardHeader>
      <CardContent className="grid gap-4">
        {form && (
          <HostForm
            form={form}
            onChange={setForm}
            editing
            keychain={capabilities.data?.keychain ?? true}
          />
        )}
        {save.error && <ErrorMessage error={save.error} />}
        <Button
          className="justify-self-start"
          disabled={!form || save.isPending}
          onClick={() => form && save.mutate(form)}
        >
          {t('common.save')}
        </Button>
      </CardContent>
    </Card>
  );
}
