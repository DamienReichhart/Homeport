import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { Button } from '../primitives/button';
import { useConfirmedAction } from './useConfirmedAction';

function Harness({ action }: { action: () => Promise<unknown> }) {
  const [dialog, run] = useConfirmedAction();
  return (
    <>
      <Button
        onClick={() =>
          run({ title: 'Remove it?', description: 'It is gone.', confirmLabel: 'Remove', action })
        }
      >
        Ask
      </Button>
      {dialog}
    </>
  );
}

describe('useConfirmedAction', () => {
  it('runs the action on confirmation and closes once it succeeds', async () => {
    const action = vi.fn(() => Promise.resolve());
    const user = userEvent.setup();
    render(<Harness action={action} />);
    await user.click(screen.getByRole('button', { name: 'Ask' }));
    await user.click(await screen.findByRole('button', { name: 'Remove' }));
    expect(action).toHaveBeenCalledOnce();
    await waitFor(() => expect(screen.queryByText('Remove it?')).toBeNull());
  });

  it('stays open after a failure so the user can retry or cancel', async () => {
    const action = vi
      .fn<() => Promise<unknown>>()
      .mockRejectedValueOnce(new Error('in use'))
      .mockResolvedValueOnce(undefined);
    const user = userEvent.setup();
    render(<Harness action={action} />);
    await user.click(screen.getByRole('button', { name: 'Ask' }));
    await user.click(await screen.findByRole('button', { name: 'Remove' }));

    await waitFor(() => expect(screen.getByRole('button', { name: 'Remove' })).toBeEnabled());
    expect(screen.getByText('Remove it?')).toBeInTheDocument();
    await user.click(screen.getByRole('button', { name: 'Remove' }));
    await waitFor(() => expect(screen.queryByText('Remove it?')).toBeNull());
    expect(action).toHaveBeenCalledTimes(2);
  });

  it('closes on cancel without acting', async () => {
    const action = vi.fn(() => Promise.resolve());
    const user = userEvent.setup();
    render(<Harness action={action} />);
    await user.click(screen.getByRole('button', { name: 'Ask' }));
    await user.click(await screen.findByRole('button', { name: 'Cancel' }));
    await waitFor(() => expect(screen.queryByText('Remove it?')).toBeNull());
    expect(action).not.toHaveBeenCalled();
  });
});
