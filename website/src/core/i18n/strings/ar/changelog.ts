import type { Translation } from '../../types';
import type en from '../en/changelog';

/** The changelog page, in Arabic. */
export default {
  'changelog.title': 'سجل التغييرات — Homeport',
  'changelog.description':
    'ما الجديد في كل إصدار من Homeport: الميزات والإصلاحات والتغييرات الأخرى.',
  'changelog.eyebrow': 'سجل التغييرات',
  'changelog.heading': 'ما الجديد في Homeport',
  'changelog.lead':
    'كل إصدار، الأحدث أولًا. تُولَّد الملاحظات من إيداعات المشروع؛ والملفات موجودة في {link}.',
  'changelog.downloadPage': 'صفحة التنزيل',
  'changelog.notesInEnglish': 'ملاحظات الإصدار أدناه بالإنجليزية، كما نُشرت على GitHub.',
  'changelog.empty': 'سجل الإصدارات موجود على {link}.',
  'changelog.github': 'GitHub',
  'changelog.latest': 'الأحدث',
  'changelog.prerelease': 'إصدار تجريبي',
  'changelog.onGitHub': 'الإصدار على GitHub',
  'changelog.download': 'التنزيل',
} satisfies Translation<typeof en>;
