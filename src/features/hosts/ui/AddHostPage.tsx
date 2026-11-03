import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { LocalEngines, useCreateHost, useHostSelection } from '@/core/host';
import { useCapabilities } from '@/shared/api';
import { Button, Card, CardContent, ErrorMessage, PageHeader } from '@/shared/ui';
import { emptyForm, toHostInput } from '../model/form';
import { HostForm } from './form/HostForm';

export function AddHostPage() {
  const navigate = useNavigate();
  const { select } = useHostSelection();
  const capabilities = useCapabilities();
  const [form, setForm] = useState(emptyForm('ssh'));
  // The page shows a failure in place.
  const create = useCreateHost((view) => {
    select(view.host.id);
    navigate(`/hosts/${view.host.id}`);
  }, true);

  return (
    <div className="max-w-2xl">
      <PageHeader title="Add host" description="Connect Homeport to a Docker Engine." />
      <div className="mb-4">
        <LocalEngines onAdd={(input) => create.mutate(input)} pending={create.isPending} />
      </div>
      <Card>
        <CardContent className="grid gap-4">
          <HostForm
            form={form}
            onChange={setForm}
            editing={false}
            keychain={capabilities.data?.keychain ?? true}
          />
          {create.error && <ErrorMessage error={create.error} />}
          <div className="flex gap-2">
            <Button
              disabled={create.isPending}
              onClick={() => create.mutate(toHostInput(form, false))}
            >
              Add host
            </Button>
            <Button variant="ghost" onClick={() => navigate(-1)}>
              Cancel
            </Button>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
