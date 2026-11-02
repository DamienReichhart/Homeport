import { useMutation, useQueryClient } from '@tanstack/react-query';
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { commands, keys, toAppError, type HostInput } from '@/shared/api';
import { Button } from '@/shared/ui/button';
import { Card, CardContent } from '@/shared/ui/card';
import { ErrorMessage, PageHeader } from '@/shared/ui';
import { useCapabilities } from '../api';
import { useHostSelection } from '../context';
import { emptyForm, toHostInput } from '../form';
import { HostForm } from './HostForm';
import { LocalEngines } from './LocalEngines';

export function AddHostPage() {
  const navigate = useNavigate();
  const client = useQueryClient();
  const { select } = useHostSelection();
  const capabilities = useCapabilities();
  const [form, setForm] = useState(emptyForm('ssh'));

  const create = useMutation({
    mutationFn: (input: HostInput) => commands.hostsCreate(input),
    onSuccess: async (view) => {
      await client.invalidateQueries({ queryKey: keys.hosts });
      select(view.host.id);
      navigate(`/hosts/${view.host.id}`);
    },
  });

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
          {create.error && <ErrorMessage error={toAppError(create.error)} />}
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
