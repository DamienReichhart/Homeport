import { RotateCwIcon } from 'lucide-react';
import { useState } from 'react';
import { Button, Input, Label, terminalError, XTerm } from '@/shared/ui';
import { startContainerdExec } from '../api/streams';
import { t } from '@/shared/i18n';

interface Spec {
  key: number;
  command: string;
  user: string;
  workingDir: string;
}

/** A terminal in a containerd container's running task; closed on unmount. */
function Terminal({
  hostId,
  namespace,
  id,
  spec,
}: {
  hostId: string;
  namespace: string;
  id: string;
  spec: Spec;
}) {
  return (
    <XTerm
      className="h-full"
      onReady={({ terminal }) => {
        const session = startContainerdExec(
          hostId,
          namespace,
          id,
          { ...spec, cols: terminal.cols, rows: terminal.rows },
          {
            onEvent: (event) => {
              if (event.type === 'output') terminal.write(event.data);
              else if (event.type === 'exit') {
                const exited =
                  event.code === null
                    ? t('containerd.exec.exited')
                    : t('containerd.exec.exitedWithCode', { code: event.code });
                terminal.write(`\r\n\x1b[2m${exited}\x1b[22m\r\n`);
                if (event.error) terminal.write(terminalError(event.error));
              } else terminal.write(`\r\n${terminalError(event.error)}`);
            },
            onOpen: () => terminal.focus(),
            onError: (error) => terminal.write(terminalError(error)),
          },
        );
        const input = terminal.onData((data) => session.input(data));
        const resize = terminal.onResize(({ cols, rows }) => session.resize(cols, rows));
        return () => {
          input.dispose();
          resize.dispose();
          session.close();
        };
      }}
    />
  );
}

/**
 * A shell, or a command of the user's, in a running containerd container.
 * The Host's machine carries it through two FIFOs (ADR 0020).
 */
export function ExecTab({
  hostId,
  namespace,
  id,
  running,
}: {
  hostId: string;
  namespace: string;
  id: string;
  running: boolean;
}) {
  const [draft, setDraft] = useState({ command: '', user: '', workingDir: '' });
  const [spec, setSpec] = useState<Spec>({ key: 1, command: '', user: '', workingDir: '' });
  if (!running)
    return <p className="text-muted-foreground text-sm">{t('containerd.exec.startFirst')}</p>;
  return (
    <div className="flex h-full min-h-0 flex-col gap-2">
      <form
        className="flex flex-wrap items-end gap-2"
        onSubmit={(e) => {
          e.preventDefault();
          setSpec({ key: spec.key + 1, ...draft });
        }}
      >
        <div className="grid min-w-48 flex-1 gap-1">
          <Label htmlFor="exec-command">{t('containerd.exec.command')}</Label>
          <Input
            id="exec-command"
            className="font-mono"
            placeholder={t('containerd.exec.commandPlaceholder')}
            value={draft.command}
            onChange={(e) => setDraft({ ...draft, command: e.target.value })}
          />
        </div>
        <div className="grid w-28 gap-1">
          <Label htmlFor="exec-user">{t('containerd.exec.user')}</Label>
          <Input
            id="exec-user"
            className="font-mono"
            placeholder="0:0"
            value={draft.user}
            onChange={(e) => setDraft({ ...draft, user: e.target.value })}
          />
        </div>
        <div className="grid w-36 gap-1">
          <Label htmlFor="exec-dir">{t('containerd.exec.workingDir')}</Label>
          <Input
            id="exec-dir"
            className="font-mono"
            placeholder="/"
            value={draft.workingDir}
            onChange={(e) => setDraft({ ...draft, workingDir: e.target.value })}
          />
        </div>
        <Button type="submit" variant="outline">
          <RotateCwIcon /> {t('containerd.exec.open')}
        </Button>
      </form>
      <div className="min-h-[20rem] flex-1">
        <Terminal key={spec.key} hostId={hostId} namespace={namespace} id={id} spec={spec} />
      </div>
    </div>
  );
}
