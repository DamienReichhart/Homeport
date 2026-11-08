import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { LocalEngines, useCreateHost, useHostSelection } from '@/core/host';
import { useCapabilities, type HostView } from '@/shared/api';
import { Button, Card, CardContent, ErrorMessage, PageHeader } from '@/shared/ui';
import { useTrustIncusHost } from '../api/mutations';
import { emptyForm, toHostInput, usesTrustToken } from '../model/form';
import { HostForm } from './form/HostForm';
import { ImportCard } from './import/ImportCard';
import { t } from '@/shared/i18n';

export function AddHostPage() {
  const navigate = useNavigate();
  const { select } = useHostSelection();
  const capabilities = useCapabilities();
  const [form, setForm] = useState(emptyForm('ssh'));
  const added = (view: HostView) => {
    select(view.host.id);
    navigate(`/hosts/${view.host.id}`);
  };
  // The page shows a failure in place.
  const create = useCreateHost(added, true);
  const trust = useTrustIncusHost(added);
  const pending = create.isPending || trust.isPending;
  const error = usesTrustToken(form) ? trust.error : create.error;

  return (
    <div className="max-w-2xl">
      <PageHeader title={t('hosts.add')} description={t('hosts.addPage.description')} />
      <div className="mb-4 grid gap-4">
        <LocalEngines onAdd={(input) => create.mutate(input)} pending={create.isPending} />
        <ImportCard />
      </div>
      <Card>
        <CardContent className="grid gap-4">
          <HostForm
            form={form}
            onChange={setForm}
            editing={false}
            keychain={capabilities.data?.keychain ?? true}
          />
          {error && <ErrorMessage error={error} />}
          <div className="flex gap-2">
            <Button
              disabled={pending}
              onClick={() =>
                usesTrustToken(form)
                  ? trust.mutate({ name: form.name, token: form.trustToken })
                  : create.mutate(toHostInput(form, false))
              }
            >
              {t('hosts.add')}
            </Button>
            <Button variant="ghost" onClick={() => navigate(-1)}>
              {t('common.cancel')}
            </Button>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
