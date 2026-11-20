import { useId, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { LocalEngines, useCreateHost, useHostSelection } from '@/core/host';
import { useCapabilities, type HostView } from '@/shared/api';
import { Button, Card, CardContent, ErrorMessage, PageHeader } from '@/shared/ui';
import { useTrustIncusHost } from '../api/mutations';
import { defaultName, emptyForm, toHostInput, usesTrustToken } from '../model/form';
import { HostForm } from './form/HostForm';
import { ImportCard } from './import/ImportCard';
import { t } from '@/shared/i18n';

/**
 * Adding a Host, fastest way first: one click for an Engine found on this
 * computer, then importing the SSH config or Docker contexts, then the form.
 */
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
  const manualId = useId();

  return (
    <div className="max-w-2xl">
      <PageHeader title={t('hosts.add')} description={t('hosts.addPage.description')} />
      <div className="grid gap-8">
        {/* Fastest first: an Engine already answering here, then what this computer describes. */}
        <LocalEngines onAdd={(input) => create.mutate(input)} pending={create.isPending} />
        <ImportCard />
        <section aria-labelledby={manualId} className="grid gap-3">
          <h2 id={manualId} className="text-base font-semibold">
            {t('hosts.addPage.manual')}
          </h2>
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
                      ? trust.mutate({
                          name: form.name.trim() || defaultName(form),
                          token: form.trustToken,
                        })
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
        </section>
      </div>
    </div>
  );
}
