import { PlugIcon } from 'lucide-react';
import { useState } from 'react';
import { Button, Input, terminalError, XTerm } from '@/shared/ui';
import { startNomadTerminal } from '../../api/streams';
import { TaskSelect } from './TaskSelect';
import { t } from '@/shared/i18n';

/** A terminal in one of an allocation's running tasks: /bin/sh, or the command typed. */
export function AllocationShell({
  clusterId,
  allocId,
  tasks,
  running,
}: {
  clusterId: string;
  allocId: string;
  tasks: string[];
  /** The tasks that run now. */
  running: string[];
}) {
  const [task, setTask] = useState(() => running[0] ?? tasks[0] ?? '');
  const [draft, setDraft] = useState('/bin/sh');
  // Each connection: what it runs, and a number that makes Reconnect start afresh.
  const [session, setSession] = useState({ n: 1, task, command: '/bin/sh' });
  const [ended, setEnded] = useState(false);
  const connect = () => {
    setEnded(false);
    setSession((s) => ({ n: s.n + 1, task, command: draft.trim() }));
  };
  return (
    <div className="flex h-[60vh] flex-col gap-2">
      <form
        className="flex flex-wrap items-center gap-2"
        onSubmit={(e) => {
          e.preventDefault();
          connect();
        }}
      >
        {tasks.length > 1 && <TaskSelect tasks={tasks} value={task} onChange={setTask} />}
        <Input
          className="h-8 w-72 font-mono"
          aria-label={t('nomad.shell.command')}
          value={draft}
          onChange={(e) => setDraft(e.target.value)}
        />
        <Button type="submit" size="sm" variant={ended ? 'default' : 'outline'}>
          <PlugIcon /> {ended ? t('nomad.shell.reconnect') : t('nomad.shell.connect')}
        </Button>
        {!running.includes(session.task) && (
          <span className="text-muted-foreground text-sm">{t('nomad.shell.notRunning')}</span>
        )}
      </form>
      <div className="min-h-0 flex-1">
        <XTerm
          key={session.n}
          className="h-full"
          onReady={({ terminal }) => {
            const open = startNomadTerminal(
              clusterId,
              allocId,
              {
                task: session.task,
                command: session.command,
                cols: terminal.cols,
                rows: terminal.rows,
              },
              {
                onEvent: (event) => {
                  if (event.type === 'output') terminal.write(event.data);
                  else if (event.type === 'exit') {
                    const exited =
                      event.code === null
                        ? t('nomad.shell.exited')
                        : t('nomad.shell.exitedWithCode', { code: event.code });
                    terminal.write(`\r\n\x1b[2m${exited}\x1b[22m\r\n`);
                    if (event.error) terminal.write(terminalError(event.error));
                    setEnded(true);
                  } else {
                    terminal.write(`\r\n${terminalError(event.error)}`);
                    setEnded(true);
                  }
                },
                onOpen: () => terminal.focus(),
                onError: (error) => {
                  terminal.write(terminalError(error));
                  setEnded(true);
                },
              },
            );
            const input = terminal.onData((data) => open.input(data));
            const resize = terminal.onResize(({ cols, rows }) => open.resize(cols, rows));
            return () => {
              input.dispose();
              resize.dispose();
              open.close();
            };
          }}
        />
      </div>
    </div>
  );
}
