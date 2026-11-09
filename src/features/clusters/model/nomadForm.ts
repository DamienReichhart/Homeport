import type { NomadConnection, NomadForm } from '@/shared/api';

/** The credentials of a Nomad Cluster, which are sealed and never shown again. */
export type NomadSecretField = 'token' | 'certificateAuthority' | 'clientCertificate' | 'clientKey';

/** What the Nomad form holds while it's being filled in. */
export interface NomadValues {
  name: string;
  address: string;
  region: string;
  tlsServerName: string;
  token: string;
  certificateAuthority: string;
  clientCertificate: string;
  clientKey: string;
  /** Stored credentials the user chose to remove when editing. */
  removed: NomadSecretField[];
}

/** An empty form, or one showing an existing Cluster's clear settings. */
export function nomadValues(name = '', connection?: NomadConnection): NomadValues {
  return {
    name,
    address: connection?.address ?? '',
    region: connection?.region ?? '',
    tlsServerName: connection?.tlsServerName ?? '',
    token: '',
    certificateAuthority: '',
    clientCertificate: '',
    clientKey: '',
    removed: [],
  };
}

/** Which credentials an existing Cluster has stored. */
export function storedSecrets(connection: NomadConnection | undefined): NomadSecretField[] {
  if (!connection) return [];
  return [
    ...(connection.aclToken ? (['token'] as const) : []),
    ...(connection.customCa ? (['certificateAuthority'] as const) : []),
    ...(connection.clientCertificate ? (['clientCertificate', 'clientKey'] as const) : []),
  ];
}

const text = (value: string) => value.trim() || null;

/**
 * What the backend is sent. A credential left empty is `null`, which keeps
 * the stored one when editing; one the user removed is sent empty.
 */
export function nomadForm(values: NomadValues): NomadForm {
  const secret = (field: NomadSecretField) =>
    values.removed.includes(field) ? '' : text(values[field]);
  return {
    name: values.name.trim(),
    address: values.address.trim(),
    region: text(values.region),
    tlsServerName: text(values.tlsServerName),
    token: secret('token'),
    certificateAuthority: secret('certificateAuthority'),
    clientCertificate: secret('clientCertificate'),
    clientKey: secret('clientKey'),
  };
}
