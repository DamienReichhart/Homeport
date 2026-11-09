import changelog from './changelog';
import download from './download';
import home from './home';
import site from './site';

/** Every string of the marketing pages, in English: the keys every language must have. */
export default { ...site, ...home, ...download, ...changelog };
