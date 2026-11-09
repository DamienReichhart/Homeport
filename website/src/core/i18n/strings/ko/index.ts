import type { Strings } from '../../types';
import changelog from './changelog';
import download from './download';
import home from './home';
import site from './site';

/** Every string of the marketing pages, in Korean. */
export default { ...site, ...home, ...download, ...changelog } satisfies Strings;
