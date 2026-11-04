import { href } from '@/shared/lib/href';
import { LEGAL_DOCUMENTS } from '../data/documents';

/** The links to the legal documents that every page's footer shows. */
export function legalLinks(): { label: string; url: string }[] {
  return LEGAL_DOCUMENTS.map((document) => ({ label: document.label, url: href(document.path) }));
}
