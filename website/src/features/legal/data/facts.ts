/**
 * Facts the legal documents share. Change them here, not in the documents, and
 * bump a document's `updated` date when what it says changes.
 */

export const PUBLISHER = {
  name: 'Damien REICHHART',
  email: 'contact@damien-reichhart.fr',
  /**
   * Leave these empty while Homeport is published by a private individual on a
   * non-professional basis. A professional publisher (a company, or a sole
   * trader such as a micro-entrepreneur) must show a postal address, a phone
   * number and its SIRET (LCEN).
   */
  address: '',
  phone: '',
  siret: '',
};

/** The hosting provider of the website, the update manifest and the releases. */
export const HOST = {
  name: 'GitHub, Inc.',
  service: 'GitHub Pages',
  address: '88 Colin P. Kelly Jr. Street, San Francisco, CA 94107, United States',
  phone: '+1 (877) 448-4820',
  url: 'https://github.com',
  privacy: 'https://docs.github.com/site-policy/privacy-policies/github-general-privacy-statement',
};

/** The provider of the contact mailbox (from the domain's MX records). */
export const MAIL_PROVIDER = {
  name: 'Hostinger International Ltd.',
  country: 'Cyprus (European Union)',
  privacy: 'https://www.hostinger.com/legal/privacy-policy',
};

export const CNIL = {
  name: 'Commission nationale de l’informatique et des libertés (CNIL)',
  address: '3 place de Fontenoy, TSA 80715, 75334 Paris Cedex 07, France',
  complaints: 'https://www.cnil.fr/fr/plaintes',
};
