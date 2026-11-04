import { screen, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands } from '@/test/fixtures';
import { mockCommands, sendToChannel } from '@/test/ipc';
import { renderApp } from '@/test/render';

describe('build dialog', () => {
  it('sends the form as a build spec and explains a failed step', async () => {
    let received: unknown = null;
    mockCommands({
      ...baseCommands,
      images_list: () => [],
      images_build_subscribe: ({ spec, onEvent }) => {
        received = spec;
        setTimeout(() =>
          sendToChannel(
            onEvent,
            { type: 'line', text: 'Step 2/2 : RUN make' },
            {
              type: 'error',
              error: {
                code: 'build_failed',
                message: 'The build failed at step 2/2 (RUN make).',
                hint: { text: 'The end of the build’s output is in Details.', commands: [] },
                detail: 'make: not found',
              },
            },
          ),
        );
        return 3;
      },
    });
    const user = userEvent.setup();
    renderApp('/images');
    await user.click(await screen.findByRole('button', { name: 'Build' }));
    const dialog = await screen.findByRole('dialog');
    const submit = within(dialog).getByRole('button', { name: 'Build' });
    expect(submit).toBeDisabled();

    await user.type(within(dialog).getByLabelText('Folder'), '/home/me/app');
    await user.type(within(dialog).getByLabelText('Names'), 'app:1 app:latest');
    await user.type(within(dialog).getByLabelText('Build arguments'), 'VERSION=1');
    await user.click(submit);

    expect(
      await within(dialog).findByText('The build failed at step 2/2 (RUN make).'),
    ).toBeInTheDocument();
    expect(received).toEqual({
      context: '/home/me/app',
      dockerfile: 'Dockerfile',
      tags: ['app:1', 'app:latest'],
      buildArgs: [{ name: 'VERSION', value: '1' }],
      target: null,
      noCache: false,
      pull: false,
    });
  });
});
