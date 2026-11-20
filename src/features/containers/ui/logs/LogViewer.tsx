import {
  ArrowDownToLineIcon,
  ChevronDownIcon,
  ChevronUpIcon,
  DownloadIcon,
  EraserIcon,
  SlidersHorizontalIcon,
} from 'lucide-react';
import { useEffect, useRef, useState, type ReactNode } from 'react';
import type { AppError, LogEvent, LogOptions, LogSource } from '@/shared/api';
import { pickSavePath } from '@/shared/lib';
import {
  Button,
  DropdownMenu,
  DropdownMenuCheckboxItem,
  DropdownMenuContent,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
  Input,
  SEARCH_DECORATIONS,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
  Separator,
  terminalError,
  Toggle,
  Tooltip,
  TooltipContent,
  TooltipTrigger,
  XTerm,
  type XTermHandle,
} from '@/shared/ui';
import { useSaveLogs } from '../../api/mutations';
import { streamLogs } from '../../api/streams';
import { MAX_KEPT_LINES, renderLine, SINCE, TAILS } from '../../model/logs';
import { t } from '@/shared/i18n';

/** Opens a log stream; the returned function stops it. */
export type LogStreamer = (
  options: LogOptions,
  onEvent: (event: LogEvent) => void,
  onError: (error: AppError) => void,
) => () => void;

/**
 * Live logs of one or more containers of a Host, or of another `stream`
 * (a Swarm service's tasks) told apart by `streamKey`. Several sources are
 * interleaved with a coloured label per source, like `docker compose logs`.
 */
export function LogViewer(
  props: ({ hostId: string; sources: LogSource[] } | { stream: LogStreamer; streamKey: string }) & {
    /** Offers stdout and stderr apart; Kubernetes merges them, so its logs don't. */
    streamToggles?: boolean;
    /** Controls shown before the viewer's own, such as a container picker. */
    toolbar?: ReactNode;
  },
) {
  const { streamToggles = true, toolbar } = props;
  const hostId = 'hostId' in props ? props.hostId : '';
  const sources = 'sources' in props ? props.sources : [];
  const custom = 'stream' in props ? props.stream : null;
  const customRef = useRef(custom);
  // Before the stream's effect, which reads it.
  useEffect(() => {
    customRef.current = custom;
  });
  const customKey = 'streamKey' in props ? props.streamKey : null;
  const handle = useRef<XTermHandle | null>(null);
  const kept = useRef<string[]>([]);
  const [ready, setReady] = useState(false);
  const [tail, setTail] = useState('1000');
  const [since, setSince] = useState('all');
  const [timestamps, setTimestamps] = useState(false);
  const [stdout, setStdout] = useState(true);
  const [stderr, setStderr] = useState(true);
  const [follow, setFollow] = useState(true);
  const [query, setQuery] = useState('');
  const saveLogs = useSaveLogs();
  const followRef = useRef(follow);
  const sourcesKey = JSON.stringify(sources);
  // How many streams are hidden, shown on the Show button so it isn't missed.
  const hidden = streamToggles ? Number(!stdout) + Number(!stderr) : 0;

  useEffect(() => {
    followRef.current = follow;
    if (follow) handle.current?.terminal.scrollToBottom();
  }, [follow]);

  useEffect(() => {
    const term = handle.current?.terminal;
    if (!ready || !term) return;
    term.reset();
    kept.current = [];
    const parsed = JSON.parse(sourcesKey) as LogSource[];
    const labels = parsed.flatMap((s) => (s.label ? [s.label] : []));
    const open: LogStreamer =
      customRef.current ?? ((...args) => streamLogs(hostId, parsed, ...args));
    return open(
      {
        tail: tail === 'all' ? null : Number(tail),
        since: since === 'all' ? null : Math.floor(Date.now() / 1000) - Number(since),
        timestamps,
        stdout,
        stderr,
        follow: true,
      },
      (event) => {
        if (event.type === 'lines') {
          // A stream's sources may only be known from its lines.
          for (const line of event.lines)
            if (line.source && !labels.includes(line.source)) labels.push(line.source);
          const rendered = event.lines.map((l) => renderLine(l, labels, timestamps));
          term.write(`${rendered.join('\r\n')}\r\n`);
          kept.current.push(
            ...event.lines.map((l) => (l.source ? `${l.source} | ${l.text}` : l.text)),
          );
          if (kept.current.length > MAX_KEPT_LINES)
            kept.current.splice(0, kept.current.length - MAX_KEPT_LINES);
          if (followRef.current) term.scrollToBottom();
        } else if (event.type === 'error') {
          term.write(`\r\n${terminalError(event.error)}`);
        } else {
          term.write(`\r\n\x1b[2m${t('containers.logs.end')}\x1b[22m\r\n`);
        }
      },
      (error) => term.write(terminalError(error)),
    );
  }, [ready, hostId, sourcesKey, customKey, tail, since, timestamps, stdout, stderr]);

  const find = (forward: boolean) => {
    const search = handle.current?.search;
    if (!search || !query) return;
    const options = { decorations: SEARCH_DECORATIONS };
    if (forward) search.findNext(query, options);
    else search.findPrevious(query, options);
  };

  return (
    <div className="flex h-full min-h-0 flex-col gap-2">
      <div className="flex flex-wrap items-center gap-x-3 gap-y-2">
        {/* What is fetched, and how it shows. */}
        <div className="flex flex-wrap items-center gap-2">
          {toolbar}
          <Select value={tail} onValueChange={setTail}>
            <SelectTrigger className="w-32" size="sm" aria-label={t('containers.logs.history')}>
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              {TAILS.map((option) => (
                <SelectItem key={option.value} value={option.value}>
                  {t(option.label)}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
          <Select value={since} onValueChange={setSince}>
            <SelectTrigger className="w-32" size="sm" aria-label={t('containers.logs.sinceLabel')}>
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              {SINCE.map((s) => (
                <SelectItem key={s.value} value={s.value}>
                  {t(s.label)}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <Button variant="outline" size="sm">
                <SlidersHorizontalIcon /> {t('containers.logs.show')}
                {hidden > 0 && (
                  <span className="text-muted-foreground tabular-nums">· {hidden}</span>
                )}
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="start" className="w-48">
              <DropdownMenuCheckboxItem
                checked={timestamps}
                onCheckedChange={setTimestamps}
                onSelect={(e) => e.preventDefault()}
              >
                {t('containers.logs.timestamps')}
              </DropdownMenuCheckboxItem>
              {streamToggles && (
                <>
                  <DropdownMenuSeparator />
                  <DropdownMenuCheckboxItem
                    checked={stdout}
                    onCheckedChange={setStdout}
                    onSelect={(e) => e.preventDefault()}
                    className="font-mono"
                  >
                    stdout
                  </DropdownMenuCheckboxItem>
                  <DropdownMenuCheckboxItem
                    checked={stderr}
                    onCheckedChange={setStderr}
                    onSelect={(e) => e.preventDefault()}
                    className="font-mono"
                  >
                    stderr
                  </DropdownMenuCheckboxItem>
                </>
              )}
            </DropdownMenuContent>
          </DropdownMenu>
        </div>
        {/* Finding in what's shown, then following, clearing and saving it. */}
        <div className="ms-auto flex items-center gap-1">
          <Input
            className="h-7 w-40"
            placeholder={t('containers.logs.find')}
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            onKeyDown={(e) => e.key === 'Enter' && find(!e.shiftKey)}
            aria-label={t('containers.logs.find')}
          />
          <IconButton label={t('containers.logs.previous')} onClick={() => find(false)}>
            <ChevronUpIcon />
          </IconButton>
          <IconButton label={t('containers.logs.next')} onClick={() => find(true)}>
            <ChevronDownIcon />
          </IconButton>
          <Separator orientation="vertical" className="mx-1 h-5" />
          <Toggle size="sm" variant="outline" pressed={follow} onPressedChange={setFollow}>
            <ArrowDownToLineIcon /> {t('containers.logs.follow')}
          </Toggle>
          <Separator orientation="vertical" className="mx-1 h-5" />
          <IconButton
            label={t('containers.logs.clear')}
            onClick={() => {
              handle.current?.terminal.clear();
              kept.current = [];
            }}
          >
            <EraserIcon />
          </IconButton>
          <IconButton
            label={t('containers.logs.download')}
            onClick={async () => {
              const path = await pickSavePath(t('containers.logs.saveTitle'), 'logs.txt');
              if (!path) return;
              saveLogs.mutate({ path, lines: [...kept.current] });
            }}
          >
            <DownloadIcon />
          </IconButton>
        </div>
      </div>
      <XTerm
        readOnly
        className="flex-1"
        onReady={(h) => {
          handle.current = h;
          setReady(true);
          return () => {
            handle.current = null;
            setReady(false);
          };
        }}
      />
    </div>
  );
}

/** A ghost icon button of the toolbar, named by its tooltip. */
function IconButton({
  label,
  onClick,
  children,
}: {
  label: string;
  onClick: () => void;
  children: ReactNode;
}) {
  return (
    <Tooltip>
      <TooltipTrigger asChild>
        <Button variant="ghost" size="icon-sm" aria-label={label} onClick={onClick}>
          {children}
        </Button>
      </TooltipTrigger>
      <TooltipContent>{label}</TooltipContent>
    </Tooltip>
  );
}
