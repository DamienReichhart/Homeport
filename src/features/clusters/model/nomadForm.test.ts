import type { NomadConnection } from '@/shared/api';
import { nomadForm, nomadValues, storedSecrets } from './nomadForm';

const connection: NomadConnection = {
  address: 'https://nomad.example.com:4646',
  region: 'eu',
  tlsServerName: 'server.eu.nomad',
  aclToken: true,
  customCa: true,
  clientCertificate: false,
};

describe('the Nomad form', () => {
  it('starts from a Cluster’s clear settings, never its credentials', () => {
    const values = nomadValues('prod', connection);
    expect(values).toMatchObject({
      name: 'prod',
      address: 'https://nomad.example.com:4646',
      region: 'eu',
      token: '',
    });
    expect(storedSecrets(connection)).toEqual(['token', 'certificateAuthority']);
    expect(storedSecrets(undefined)).toEqual([]);
  });

  it('sends empty credentials as null, so the stored ones are kept', () => {
    const form = nomadForm({ ...nomadValues(' prod ', connection), token: '  ' });
    expect(form).toMatchObject({ name: 'prod', region: 'eu', token: null, clientKey: null });
  });

  it('sends a removed credential as empty text, and a new one trimmed', () => {
    const form = nomadForm({
      ...nomadValues('prod', connection),
      token: ' new-token ',
      removed: ['certificateAuthority'],
    });
    expect(form.token).toBe('new-token');
    expect(form.certificateAuthority).toBe('');
  });
});
