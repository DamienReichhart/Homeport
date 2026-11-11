// Imported first by main.tsx, so it runs before any library reads the OS's
// languages: a C or POSIX locale on Linux would otherwise leave the window
// empty (see keepValidLanguages).
import { keepValidLanguages } from '@/shared/lib/languages';

keepValidLanguages();
