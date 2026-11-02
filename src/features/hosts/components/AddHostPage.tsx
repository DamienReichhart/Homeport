import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { MonitorIcon, PlusIcon } from 'lucide-react';
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { commands, keys, toAppError, type HostInput } from '@/shared/api';
import { Button } from '@/shared/ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/shared/ui/card';
import { ErrorMessage, PageHeader } from '@/shared/ui';
import { useCapabilities, useHosts } from '../api';
import { useHostSelection } from '../context';
import { emptyForm, toHostInput } from '../form';
import { HostForm } from './HostForm';

export function AddHostPage() {
  const navigate = useNavigate();
  const client = useQueryClient();
  const { select } = useHostSelection();
  const capabilities = useCapabilities();
  const hosts = useHosts();
  const detected = useQuery({
    queryKey: ['detect-local'],
    queryFn: () => commands.hostsDetectLocal(),
  });
  const [form, setForm] = useState(emptyForm('ssh'));

  const create = useMutation({
    mutationFn: (input: HostInput) => commands.hostsCreate(input),
    onSuccess: async (view) => {
      await client.invalidateQueries({ queryKey: keys.hosts });
      select(view.host.id);
      navigate(`/hosts/${view.host.id}`);
    },
  });

  const added = new Set(
    hosts.data?.flatMap(({ host }) =>
      host.connection.kind === 'local' ? [host.connection.socketPath] : [],
    ),
  );
  const local = detected.data?.filter((e) => !added.has(e.socketPath)) ?? [];

  return (
    <div className="max-w-2xl">
      <PageHeader title="Add host" description="Connect Homeport to a Docker Engine." />
      {local.length > 0 && (
        <Card className="mb-4">
          <CardHeader>
            <CardTitle>Found on this machine</CardTitle>
            <CardDescription>Engines answering on local sockets.</CardDescription>
          </CardHeader>
          <CardContent className="grid gap-2">
            {local.map((engine) => (
              <div key={engine.socketPath} className="flex items-center justify-between gap-3">
                <div className="flex min-w-0 items-center gap-2">
                  <MonitorIcon className="text-muted-foreground size-4" />
                  <div className="min-w-0">
                    <p className="text-sm font-medium">
                      {engine.label}{' '}
                      <span className="text-muted-foreground">· Engine {engine.engineVersion}</span>
                    </p>
                    <p className="text-muted-foreground truncate font-mono text-xs">
                      {engine.socketPath}
                    </p>
                  </div>
                </div>
                <Button
                  size="sm"
                  disabled={create.isPending}
                  onClick={() =>
                    create.mutate({
                      name: engine.label,
                      connection: { kind: 'local', socketPath: engine.socketPath },
                      credentials: null,
                    })
                  }
                >
                  <PlusIcon /> Add
                </Button>
              </div>
            ))}
          </CardContent>
        </Card>
      )}
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
