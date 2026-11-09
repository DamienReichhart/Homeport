import { screen, waitFor, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { nomadCommands, nomadJobDetail } from '@/test/nomad';
import { renderApp } from '@/test/render';

// CodeMirror doesn't run in jsdom: the editor is a text area.
vi.mock('@/shared/ui', async (original) => ({
  ...(await original<object>()),
  JobSpecEditor: ({
    value,
    label,
    onChange,
  }: {
    value: string;
    label: string;
    onChange?: (value: string) => void;
  }) => <textarea aria-label={label} value={value} onChange={(e) => onChange?.(e.target.value)} />,
}));

const plan = (overrides: Record<string, unknown> = {}) => ({
  jobId: 'web',
  namespace: 'default',
  format: 'hcl2',
  job: { ID: 'web' },
  jobModifyIndex: 0,
  newJob: true,
  change: 'Added',
  groups: [
    {
      name: 'api',
      change: 'Added',
      place: 2,
      stop: 0,
      inPlace: 0,
      destructive: 0,
      migrate: 0,
      canary: 0,
      ignore: 0,
      fields: [{ path: 'Count', change: 'Added', old: null, new: '2' }],
      failure: null,
    },
  ],
  warnings: null,
  placement: null,
  ...overrides,
});

const error = (code: string, key: string, message: string, params = {}) => ({
  code,
  key,
  params,
  message,
  hint: null,
  detail: null,
});

function setup(extra: Record<string, (args: Record<string, unknown>) => unknown> = {}) {
  const calls: { command: string; args: Record<string, unknown> }[] = [];
  const record = (command: string, answer: () => unknown) => (args: Record<string, unknown>) => {
    calls.push({ command, args });
    return answer();
  };
  mockCommands({
    ...baseCommands,
    ...nomadCommands,
    clusters_nomad_jobs: () => [],
    clusters_nomad_job: () => nomadJobDetail(),
    clusters_nomad_job_plan: record('plan', () => plan()),
    clusters_nomad_job_run: record('run', () => ({
      jobId: 'web',
      namespace: 'default',
      evalId: 'e1',
      warnings: null,
    })),
    clusters_nomad_job_source: () => ({ source: 'job "web" {}', format: 'hcl2', submitted: true }),
    ...extra,
  });
  return calls;
}

async function openDialog(spec: string) {
  const user = userEvent.setup();
  renderApp('/nomad/jobs');
  await user.click(await screen.findByRole('button', { name: 'Run job' }));
  const dialog = await screen.findByRole('dialog');
  const editor = within(dialog).getByRole('textbox', { name: 'Job specification' });
  await user.click(editor);
  await user.keyboard(spec);
  return { user, dialog };
}

describe('running a Nomad job', () => {
  it('shows Nomad’s plan before running it, then runs what was planned', async () => {
    const calls = setup();
    const { user, dialog } = await openDialog('job "web" {{}');
    expect(within(dialog).getByText('HCL')).toBeInTheDocument();
    await user.click(within(dialog).getByRole('button', { name: 'Plan' }));
    expect(
      await within(dialog).findByText('This creates job web in namespace default.'),
    ).toBeInTheDocument();
    expect(within(dialog).getByText('2 to place')).toBeInTheDocument();
    expect(calls[0]?.args).toMatchObject({ id: 'n1', namespace: null });
    await user.click(within(dialog).getByRole('button', { name: 'Run' }));
    await waitFor(() => expect(calls.some((c) => c.command === 'run')).toBe(true));
    expect(calls.find((c) => c.command === 'run')?.args).toMatchObject({
      plan: { jobId: 'web', jobModifyIndex: 0, job: { ID: 'web' } },
    });
  });

  it('shows where a specification doesn’t parse', async () => {
    setup({
      clusters_nomad_job_plan: () => {
        throw error(
          'nomad_parse_failed',
          'errors.nomad.parseFailedAt',
          'The job specification doesn’t parse, at line 3, column 5.',
          { line: 3, column: 5 },
        );
      },
    });
    const { user, dialog } = await openDialog('job');
    await user.click(within(dialog).getByRole('button', { name: 'Plan' }));
    expect(await within(dialog).findByText(/at line 3, column 5/)).toBeInTheDocument();
  });

  it('refuses to overwrite a job changed since its plan', async () => {
    setup({
      clusters_nomad_job_run: () => {
        throw error(
          'nomad_job_modified',
          'errors.nomad.jobModified',
          "web changed since it was planned, so it wasn't run.",
          { job: 'web' },
        );
      },
    });
    const { user, dialog } = await openDialog('job');
    await user.click(within(dialog).getByRole('button', { name: 'Plan' }));
    await user.click(await within(dialog).findByRole('button', { name: 'Run' }));
    expect(
      await within(dialog).findByText("web changed since it was planned, so it wasn't run."),
    ).toBeInTheDocument();
  });

  it('offers to run anyway when allocations can’t be placed', async () => {
    setup({
      clusters_nomad_job_plan: () =>
        plan({
          placement: error(
            'nomad_placement_failed',
            'errors.nomad.placementFailed',
            'Nomad can’t place every allocation of web: api.',
          ),
        }),
    });
    const { user, dialog } = await openDialog('job');
    await user.click(within(dialog).getByRole('button', { name: 'Plan' }));
    expect(await within(dialog).findByRole('button', { name: 'Run anyway' })).toBeInTheDocument();
  });

  it('edits and runs a job from the specification Nomad kept', async () => {
    const calls = setup();
    const user = userEvent.setup();
    renderApp('/nomad/jobs/default/web');
    await user.click(await screen.findByRole('button', { name: 'Edit and run' }));
    const dialog = await screen.findByRole('dialog');
    expect(within(dialog).getByText('Edit and run web')).toBeInTheDocument();
    await user.click(within(dialog).getByRole('button', { name: 'Plan' }));
    await waitFor(() => expect(calls[0]?.command).toBe('plan'));
    expect(calls[0]?.args).toMatchObject({ namespace: 'default', spec: 'job "web" {}' });
  });
});
