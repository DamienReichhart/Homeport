import { emptyBuild, toBuildSpec } from './build';

describe('build form', () => {
  it('needs a folder', () => {
    expect(toBuildSpec(emptyBuild)).toEqual({ problem: 'Choose the folder to build.' });
  });

  it('reads tags, arguments and options', () => {
    expect(
      toBuildSpec({
        ...emptyBuild,
        context: ' /home/me/app ',
        dockerfile: '',
        tags: 'app:1, app:latest\nghcr.io/me/app:1',
        buildArgs: 'VERSION=1.2\n\nEMPTY=\nURL=https://x?a=b\nFROM_ENV',
        target: ' runtime ',
        noCache: true,
      }),
    ).toEqual({
      spec: {
        context: '/home/me/app',
        dockerfile: 'Dockerfile',
        tags: ['app:1', 'app:latest', 'ghcr.io/me/app:1'],
        buildArgs: [
          { name: 'VERSION', value: '1.2' },
          { name: 'EMPTY', value: '' },
          { name: 'URL', value: 'https://x?a=b' },
          { name: 'FROM_ENV', value: '' },
        ],
        target: 'runtime',
        noCache: true,
        pull: false,
      },
    });
  });

  it('explains an argument without a name', () => {
    expect(toBuildSpec({ ...emptyBuild, context: '/x', buildArgs: '=1' })).toEqual({
      problem: '"=1" needs a name before "=".',
    });
  });
});
