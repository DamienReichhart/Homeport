import { serviceView } from '@/test/fixtures';
import {
  changesSomething,
  editForm,
  emptyServiceForm,
  toServiceChange,
  toServiceInput,
} from './serviceForm';

describe('the Create service form', () => {
  it('becomes the input the backend checks, dropping blank rows', () => {
    const input = toServiceInput({
      ...emptyServiceForm(),
      name: ' web ',
      image: 'nginx:1.27',
      replicas: '3',
      env: 'A=1\n\n B=2 ',
      ports: [
        { hostPort: '8080', containerPort: '80', protocol: 'tcp' },
        { hostPort: '', containerPort: '', protocol: 'tcp' },
      ],
      mounts: [{ source: 'data', target: '/data', readOnly: false }],
      secrets: [{ name: 'db_password', target: '' }],
      restart: 'on-failure',
    });
    expect(input).toEqual({
      name: 'web',
      image: 'nginx:1.27',
      mode: { kind: 'replicated', replicas: 3 },
      env: ['A=1', 'B=2'],
      command: null,
      ports: [{ target: 80, published: 8080, protocol: 'tcp', hostMode: false }],
      networks: [],
      mounts: [{ source: 'data', target: '/data', readOnly: false }],
      secrets: [{ name: 'db_password', target: null }],
      configs: [],
      restart: 'on-failure',
    });
  });

  it('needs a number of replicas, except for a global service', () => {
    expect(toServiceInput({ ...emptyServiceForm(), replicas: 'x' })).toBeNull();
    expect(toServiceInput({ ...emptyServiceForm(), mode: 'global', replicas: 'x' })?.mode).toEqual({
      kind: 'global',
    });
  });
});

describe('the Edit service form', () => {
  const service = serviceView({ image: 'nginx:1.27', replicas: 2 });
  const inspect = { Spec: { TaskTemplate: { ContainerSpec: { Env: ['A=1'] } } } };

  it('sends only what changed', () => {
    const initial = editForm(service, inspect);
    expect(initial).toEqual({ image: 'nginx:1.27', env: 'A=1', replicas: '2', redeploy: false });
    const unchanged = toServiceChange(service, initial, initial)!;
    expect(changesSomething(unchanged)).toBe(false);
    expect(toServiceChange(service, initial, { ...initial, image: 'nginx:1.28' })).toEqual({
      image: 'nginx:1.28',
      env: null,
      replicas: null,
      redeploy: false,
    });
    expect(
      toServiceChange(service, initial, { ...initial, env: 'A=1\nB=2', replicas: '4' }),
    ).toEqual({
      image: null,
      env: ['A=1', 'B=2'],
      replicas: 4,
      redeploy: false,
    });
  });

  it('never sends replicas for a global service', () => {
    const global = serviceView({ mode: 'global', replicas: null });
    const initial = editForm(global, inspect);
    expect(toServiceChange(global, initial, { ...initial, replicas: 'x' })?.replicas).toBeNull();
  });
});
