import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { ArrowLeftIcon } from 'lucide-react';
import { useState } from 'react';
import { Link, useNavigate, useParams } from 'react-router-dom';
import { toast } from 'sonner';
import { containerTone, LogViewer } from '@/features/containers';
import { HostGate, useContainers } from '@/core/host';
import {
  commands,
  type ComposeCheck,
  keys,
  type ManagedStack,
  type Stack,
  useCapabilities,
} from '@/shared/api';
import {
  Badge,
  Button,
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
  ErrorMessage,
  PageHeader,
  Spinner,
  StatusDot,
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
  useConfirm,
} from '@/shared/ui';
import { STATE_LABEL, STATE_TONE, useStacks } from './api';
import { ComposeRunDialog, type ComposeJob } from './ComposeRunDialog';
import { StackActions } from './StackActions';
import { StackEditor } from './StackEditor';

function Services({ hostId, stack }: { hostId: string; stack: Stack }) {
  const containers = useContainers(hostId);
  const byId = new Map(containers.data?.map((c) => [c.id, c]));
  if (stack.services.length === 0)
    return <p className="text-muted-foreground text-sm">Not deployed yet.</p>;
  return (
    <div className="rounded-md border">
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead>Service</TableHead>
            <TableHead>Container</TableHead>
            <TableHead>Image</TableHead>
            <TableHead>Status</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {stack.services.flatMap((s) =>
            s.containerIds.map((id) => {
              const c = byId.get(id);
              return (
                <TableRow key={id}>
                  <TableCell className="font-medium">{s.name}</TableCell>
                  <TableCell>
                    <Link to={`/containers/${id}/logs`} className="text-primary hover:underline">
                      {c?.name ?? id.slice(0, 12)}
                    </Link>
                  </TableCell>
                  <TableCell>{c?.image}</TableCell>
                  <TableCell>
                    <span className="flex items-center gap-1.5">
                      {c && <StatusDot tone={containerTone(c.state, c.health)} />}
                      {c?.status}
                    </span>
                  </TableCell>
                </TableRow>
              );
            }),
          )}
        </TableBody>
      </Table>
    </div>
  );
}

function ManagedDefinition({
  hostId,
  managed,
  initialEnv,
  onDeploy,
}: {
  hostId: string;
  managed: ManagedStack;
  initialEnv: string;
  onDeploy: (job: ComposeJob) => void;
}) {
  const client = useQueryClient();
  const navigate = useNavigate();
  const capabilities = useCapabilities();
  const [dialog, confirm] = useConfirm();
  const [compose, setCompose] = useState(managed.compose);
  const [env, setEnv] = useState(initialEnv);
  const [check, setCheck] = useState<ComposeCheck | null>(null);
  const dirty = compose !== managed.compose || env !== initialEnv;
  const save = useMutation({
    meta: { silent: true },
    mutationFn: () => commands.stacksUpdate(managed.id, compose, env),
    onSuccess: async (saved, deploy: boolean) => {
      await client.invalidateQueries({ queryKey: keys.managedStack(managed.id) });
      toast.success('Definition saved');
      if (deploy) onDeploy({ stackId: saved.id, name: saved.name, action: 'up' });
    },
  });
  const canDeploy =
    capabilities.data?.compose.state === 'available' && check !== null && check.errors.length === 0;
  return (
    <div className="grid gap-4">
      <StackEditor
        compose={compose}
        env={env}
        onComposeChange={setCompose}
        onEnvChange={setEnv}
        keychain={capabilities.data?.keychain ?? true}
        onCheck={setCheck}
      />
      {save.error && <ErrorMessage error={save.error} />}
      <div className="flex flex-wrap gap-2">
        <Button disabled={save.isPending || !canDeploy} onClick={() => save.mutate(true)}>
          Save and redeploy
        </Button>
        <Button
          variant="outline"
          disabled={save.isPending || !dirty}
          onClick={() => save.mutate(false)}
        >
          Save
        </Button>
        <Button
          variant="ghost"
          className="ml-auto"
          onClick={() =>
            confirm({
              title: `Forget the definition of ${managed.name}?`,
              description:
                'Homeport deletes its stored compose file and environment. Running containers are left alone and the Stack becomes External.',
              confirmLabel: 'Forget definition',
              destructive: true,
              onConfirm: async () => {
                await commands.stacksForget(managed.id);
                await client.invalidateQueries({ queryKey: keys.stacks(hostId) });
                navigate('/stacks');
              },
            })
          }
        >
          Forget definition
        </Button>
      </div>
      {dialog}
    </div>
  );
}

function Definition({
  hostId,
  stack,
  onDeploy,
}: {
  hostId: string;
  stack: Stack;
  onDeploy: (job: ComposeJob) => void;
}) {
  const id = stack.managedId;
  const managed = useQuery({
    queryKey: keys.managedStack(id ?? ''),
    queryFn: () => commands.stacksGet(id ?? ''),
    enabled: !!id,
  });
  const env = useQuery({
    queryKey: [...keys.managedStack(id ?? ''), 'env'],
    queryFn: () => commands.stacksGetEnv(id ?? ''),
    enabled: !!id,
  });
  if (!id) {
    return (
      <Card>
        <CardHeader>
          <CardTitle>External Stack</CardTitle>
          <CardDescription>
            It was deployed outside Homeport
            {stack.configFiles ? (
              <>
                {' '}
                from <span className="font-mono">{stack.configFiles}</span> on that machine
              </>
            ) : null}
            . You can start, stop and remove it; adopt it with its compose file to edit and
            redeploy.
          </CardDescription>
        </CardHeader>
        <CardContent>
          <Button asChild>
            <Link to={`/stacks/new?adopt=${encodeURIComponent(stack.name)}`}>Adopt</Link>
          </Button>
        </CardContent>
      </Card>
    );
  }
  if (managed.error)
    return <ErrorMessage error={managed.error} onRetry={() => void managed.refetch()} />;
  if (env.error) return <ErrorMessage error={env.error} onRetry={() => void env.refetch()} />;
  if (!managed.data || env.data === undefined) return <Spinner />;
  return (
    <ManagedDefinition
      key={managed.data.updatedAt}
      hostId={hostId}
      managed={managed.data}
      initialEnv={env.data}
      onDeploy={onDeploy}
    />
  );
}

function StackDetail({ hostId, name }: { hostId: string; name: string }) {
  const stacks = useStacks(hostId);
  const navigate = useNavigate();
  const [dialog, confirm] = useConfirm();
  const [job, setJob] = useState<ComposeJob | null>(null);
  const stack = stacks.data?.find((s) => s.name === name);

  if (stacks.isPending) return <Spinner />;
  if (!stack)
    return (
      <ErrorMessage
        error={{ code: 'not_found', message: `There is no stack named ${name} on this host.` }}
      />
    );
  const sources = stack.services.flatMap((s) =>
    s.containerIds.map((containerId) => ({ containerId, label: s.name })),
  );

  return (
    <div className="flex h-full min-h-0 flex-col">
      <PageHeader
        title={
          <span className="flex items-center gap-2">
            <Button
              variant="ghost"
              size="icon-sm"
              aria-label="Back"
              onClick={() => navigate('/stacks')}
            >
              <ArrowLeftIcon />
            </Button>
            {stack.name}
            <Badge variant={stack.kind === 'managed' ? 'default' : 'outline'}>
              {stack.kind === 'managed' ? 'Managed' : 'External'}
            </Badge>
          </span>
        }
        description={
          <span className="flex items-center gap-1.5">
            <StatusDot tone={STATE_TONE[stack.state]} /> {STATE_LABEL[stack.state]}
            {stack.total > 0 && ` · ${stack.running} of ${stack.total} containers running`}
          </span>
        }
        actions={
          <StackActions hostId={hostId} stack={stack} confirm={confirm} onCompose={setJob} />
        }
      />
      <Tabs
        defaultValue={stack.state === 'notDeployed' ? 'definition' : 'services'}
        className="flex min-h-0 flex-1 flex-col"
      >
        <TabsList>
          <TabsTrigger value="services">Services</TabsTrigger>
          <TabsTrigger value="logs" disabled={sources.length === 0}>
            Logs
          </TabsTrigger>
          <TabsTrigger value="definition">Definition</TabsTrigger>
        </TabsList>
        <TabsContent value="services" className="overflow-auto">
          <Services hostId={hostId} stack={stack} />
        </TabsContent>
        <TabsContent value="logs" className="min-h-[24rem] flex-1">
          <LogViewer hostId={hostId} sources={sources} />
        </TabsContent>
        <TabsContent value="definition" className="overflow-auto">
          <Definition hostId={hostId} stack={stack} onDeploy={setJob} />
        </TabsContent>
      </Tabs>
      <ComposeRunDialog hostId={hostId} job={job} onClose={() => setJob(null)} />
      {dialog}
    </div>
  );
}

export function StackDetailPage() {
  const { name = '' } = useParams();
  return (
    <HostGate>
      {(hostId) => <StackDetail key={`${hostId}/${name}`} hostId={hostId} name={name} />}
    </HostGate>
  );
}
