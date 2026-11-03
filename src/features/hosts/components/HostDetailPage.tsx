import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { RefreshCwIcon, ShieldAlertIcon, ShieldQuestionIcon, TrashIcon } from 'lucide-react';
import { useState } from 'react';
import { useNavigate, useParams } from 'react-router-dom';
import { toast } from 'sonner';
import { commands, keys, type HostView } from '@/shared/api';
import {
  Alert,
  AlertDescription,
  AlertTitle,
  Badge,
  Button,
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
  CopyButton,
  ErrorMessage,
  Label,
  PageHeader,
  Spinner,
  Switch,
  useConfirm,
} from '@/shared/ui';
import { useCapabilities, useHosts } from '../api';
import { formFromHost, toHostInput, type HostFormState } from '../form';
import { connectionLabel, describeStatus, statusError } from '../status';
import { HostForm } from './HostForm';
import { HostStatusBadge } from './HostStatusBadge';

function CommandBlock({ label, command }: { label: string; command: string }) {
  return (
    <div className="grid gap-1.5">
      <div className="flex items-center justify-between">
        <span className="text-sm font-medium">{label}</span>
        <CopyButton text={command} />
      </div>
      <pre className="bg-muted/50 overflow-x-auto rounded-md border p-2 font-mono text-xs whitespace-pre-wrap break-all">
        {command}
      </pre>
    </div>
  );
}

function SshSetupCard({ view }: { view: HostView }) {
  const setup = useQuery({
    queryKey: [...keys.sshSetup(view.host.id), view.host.connection],
    queryFn: () => commands.hostsSshSetup(view.host.id),
  });
  const client = useQueryClient();
  const [dialog, confirm] = useConfirm();
  if (!setup.data) return null;
  return (
    <Card>
      <CardHeader>
        <CardTitle>Authorize Homeport on the server</CardTitle>
        <CardDescription>
          Run these on{' '}
          <span className="font-mono">
            {view.host.connection.kind === 'ssh' && view.host.connection.host}
          </span>{' '}
          as the SSH user, then reconnect.
        </CardDescription>
      </CardHeader>
      <CardContent className="grid gap-4">
        <CommandBlock label="1. Add the public key" command={setup.data.installCommand} />
        <CommandBlock
          label="2. Let the user reach Docker (if needed)"
          command={setup.data.dockerGroupCommand}
        />
        <div className="text-muted-foreground flex flex-wrap items-center justify-between gap-2 text-xs">
          <span>
            Key fingerprint <span className="font-mono">{setup.data.fingerprint}</span>
          </span>
          {view.host.connection.kind === 'ssh' && view.host.connection.auth === 'generated' && (
            <Button
              variant="outline"
              size="sm"
              onClick={() =>
                confirm({
                  title: 'Regenerate the SSH key?',
                  description:
                    'The current key stops working. You will need to add the new public key to the server.',
                  confirmLabel: 'Regenerate',
                  onConfirm: async () => {
                    await commands.hostsRegenerateKey(view.host.id);
                    await client.invalidateQueries({ queryKey: keys.hosts });
                  },
                })
              }
            >
              Regenerate key
            </Button>
          )}
        </div>
      </CardContent>
      {dialog}
    </Card>
  );
}

function TrustPrompt({ view }: { view: HostView }) {
  const client = useQueryClient();
  const [dialog, confirm] = useConfirm();
  const trust = useMutation({
    mutationFn: (key: string) => commands.hostsTrustKey(view.host.id, key),
    onSuccess: () => client.invalidateQueries({ queryKey: keys.hosts }),
  });
  const status = view.status;
  if (status.state === 'hostKeyUntrusted') {
    return (
      <Alert>
        <ShieldQuestionIcon />
        <AlertTitle>Is this the right server?</AlertTitle>
        <AlertDescription className="grid gap-2">
          <p>
            First connection. The server identifies itself with key fingerprint{' '}
            <span className="font-mono">{status.fingerprint}</span>. Compare it with{' '}
            <span className="font-mono">ssh-keygen -lf /etc/ssh/ssh_host_ed25519_key.pub</span> on
            the server.
          </p>
          <Button size="sm" className="justify-self-start" onClick={() => trust.mutate(status.key)}>
            Trust this key
          </Button>
        </AlertDescription>
      </Alert>
    );
  }
  if (status.state === 'hostKeyMismatch') {
    return (
      <Alert variant="destructive">
        <ShieldAlertIcon />
        <AlertTitle>The server&apos;s key changed</AlertTitle>
        <AlertDescription className="grid gap-2">
          <p>
            Expected <span className="font-mono">{status.expectedFingerprint}</span> but the server
            presented <span className="font-mono">{status.fingerprint}</span>. This happens after
            reinstalling the server, but can also mean someone is intercepting the connection.
          </p>
          <Button
            size="sm"
            variant="destructive"
            className="justify-self-start"
            onClick={() =>
              confirm({
                title: 'Trust the new server key?',
                description: `Only do this if you know why the key changed. New fingerprint: ${status.fingerprint}.`,
                confirmLabel: 'Trust new key',
                destructive: true,
                onConfirm: () => trust.mutateAsync(status.key),
              })
            }
          >
            Trust the new key
          </Button>
        </AlertDescription>
        {dialog}
      </Alert>
    );
  }
  return null;
}

export function HostDetailPage() {
  const { hostId = '' } = useParams();
  const hosts = useHosts();
  const view = hosts.data?.find((h) => h.host.id === hostId);
  if (hosts.isPending) return <Spinner />;
  if (!view)
    return <ErrorMessage error={{ code: 'not_found', message: 'This host no longer exists.' }} />;
  // Keyed so the form starts from this host's saved settings.
  return <HostDetail key={hostId} view={view} />;
}

function HostDetail({ view }: { view: HostView }) {
  const hostId = view.host.id;
  const capabilities = useCapabilities();
  const client = useQueryClient();
  const navigate = useNavigate();
  const [dialog, confirm] = useConfirm();
  const [form, setForm] = useState<HostFormState>(() => formFromHost(view.host));

  const refresh = () => client.invalidateQueries({ queryKey: keys.hosts });
  const save = useMutation({
    meta: { silent: true },
    mutationFn: (f: HostFormState) => commands.hostsUpdate(hostId, toHostInput(f, true)),
    onSuccess: async () => {
      await refresh();
      toast.success('Host saved');
    },
  });
  const reconnect = useMutation({
    mutationFn: () => commands.hostsReconnect(hostId),
    onSuccess: refresh,
  });
  const enable = useMutation({
    mutationFn: (enabled: boolean) => commands.hostsSetEnabled(hostId, enabled),
    onSuccess: refresh,
  });

  const { detail } = describeStatus(view.status);

  return (
    <div className="grid max-w-3xl gap-4">
      <PageHeader
        title={
          <span className="flex items-center gap-2">
            {view.host.name}{' '}
            <Badge variant="outline">{connectionLabel(view.host.connection.kind)}</Badge>
          </span>
        }
        description={
          <span className="flex items-center gap-2">
            <HostStatusBadge status={view.status} />
            {detail && !statusError(view.status) && (
              <span className="text-muted-foreground">· {detail}</span>
            )}
          </span>
        }
        actions={
          <>
            <div className="flex items-center gap-2">
              <Switch
                id="enabled"
                checked={view.host.enabled}
                onCheckedChange={(v) => enable.mutate(v)}
              />
              <Label htmlFor="enabled">Enabled</Label>
            </div>
            <Button
              variant="outline"
              disabled={reconnect.isPending}
              onClick={() => reconnect.mutate()}
            >
              <RefreshCwIcon /> Reconnect
            </Button>
          </>
        }
      />
      <TrustPrompt view={view} />
      {statusError(view.status) && (
        <ErrorMessage error={statusError(view.status)} onRetry={() => reconnect.mutate()} />
      )}
      {view.host.connection.kind === 'ssh' && view.host.connection.publicKey && (
        <SshSetupCard view={view} />
      )}
      <Card>
        <CardHeader>
          <CardTitle>Connection</CardTitle>
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
            Save
          </Button>
        </CardContent>
      </Card>
      <Card className="border-destructive/40">
        <CardHeader>
          <CardTitle className="text-destructive">Remove host</CardTitle>
          <CardDescription>
            Homeport forgets this host, its credentials and its Managed Stack definitions. Nothing
            running on the host is touched.
          </CardDescription>
        </CardHeader>
        <CardContent>
          <Button
            variant="destructive"
            onClick={() =>
              confirm({
                title: `Remove ${view.host.name}?`,
                description:
                  'Deletes the stored connection, SSH key or certificates, and the compose definitions of its Managed Stacks. Containers, images and volumes on the host stay as they are.',
                confirmLabel: 'Remove host',
                destructive: true,
                onConfirm: async () => {
                  await commands.hostsDelete(hostId);
                  await refresh();
                  navigate('/hosts');
                },
              })
            }
          >
            <TrashIcon /> Remove host
          </Button>
        </CardContent>
      </Card>
      {dialog}
    </div>
  );
}
