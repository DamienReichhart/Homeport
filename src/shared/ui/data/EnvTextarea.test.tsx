import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { EnvTextarea, maskEnv } from './EnvTextarea';

describe('environment field', () => {
  it('hides every value and keeps keys, comments and blank lines', () => {
    expect(maskEnv('# db\nPASSWORD=s3cret\n\nTAG=2.4')).toBe(
      '# db\nPASSWORD=••••••••\n\nTAG=••••••••',
    );
  });

  it('shows the values only when asked, and only then can they be edited', async () => {
    const changes: string[] = [];
    render(<EnvTextarea id="env" value="TOKEN=hunter2" onChange={(v) => changes.push(v)} />);
    const field = screen.getByRole('textbox');
    expect(field).toHaveValue('TOKEN=••••••••');
    expect(field).toHaveAttribute('readonly');
    await userEvent.click(screen.getByRole('button', { name: 'Show values' }));
    expect(field).toHaveValue('TOKEN=hunter2');
    await userEvent.type(field, '2');
    expect(changes.at(-1)).toBe('TOKEN=hunter22');
  });

  it('opens an empty one ready to type in', () => {
    render(<EnvTextarea id="env" value="" onChange={() => undefined} />);
    expect(screen.getByRole('textbox')).not.toHaveAttribute('readonly');
    expect(screen.queryByRole('button')).toBeNull();
  });
});
