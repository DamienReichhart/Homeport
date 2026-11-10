import { $, expect } from '@wdio/globals';
import { button, field, gone, goTo, paste, press, shown, text } from '../support/app';
import { SSH_PORT, startSshHost, type SshHost } from '../support/sshd';

const NAME = 'e2e-ssh';

describe('an SSH Host', () => {
  let server: SshHost;
  before(() => {
    server = startSshHost();
  });
  after(() => server?.stop());

  it('is added with an existing key, trusted on first use and connects', async () => {
    await shown(text('Overview'), 60_000);
    await goTo('Overview');
    await press(button('Add host'));
    await shown(field('Name'));
    await field('Name').setValue(NAME);
    // SSH is the form's first choice.
    await field('Server').setValue('127.0.0.1');
    await field('Port').clearValue();
    await field('Port').setValue(String(SSH_PORT));
    await field('User').setValue('root');
    await press(button('Existing key'));
    await paste(field('Private key'), server.privateKey);
    await press(button('Add host'));

    // Trust on first use: the server's key is shown and confirmed.
    await shown(text('Is this the right server?'), 60_000);
    await press(button('Trust this key'));
    await gone(text('Is this the right server?'));
    // Its own page, whose status no other Host's can stand in for.
    await shown($(`//h1[starts-with(normalize-space(.), '${NAME}')]`));
    await shown(text('Connected'), 60_000);
    // The Engine answered through the SSH server, which reaches its socket.
    await expect(text('Engine ')).toBeDisplayed();
  });
});
