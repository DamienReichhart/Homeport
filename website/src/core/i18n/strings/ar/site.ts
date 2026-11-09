import type { Translation } from '../../types';
import type en from '../en/site';

/** The layout, the header and the footer, in Arabic. */
export default {
  // The layout and the facts every page repeats.
  'site.tagline': 'كل محركات Docker، على لوحة واحدة.',
  'site.description':
    'Homeport تطبيق سطح مكتب مجاني ومفتوح المصدر لإدارة محركات Docker أينما كانت — محرك هذا الجهاز، أو المحركات البعيدة عبر SSH أو TLS أو TCP — لأنظمة Windows وmacOS وLinux.',
  'site.ogImageAlt': 'Homeport — كل محركات Docker، على لوحة واحدة.',
  'site.skipToContent': 'الانتقال إلى المحتوى',

  // The header, the language menu and the theme switch.
  'header.home': 'الصفحة الرئيسية لـ Homeport',
  'header.logo': 'Homeport',
  'header.nav': 'التنقل الرئيسي',
  'header.features': 'الميزات',
  'header.docs': 'التوثيق',
  'header.changelog': 'سجل التغييرات',
  'header.download': 'التنزيل',
  'header.github': 'Homeport على GitHub',
  'header.githubShort': 'GitHub',
  'header.menu': 'القائمة',
  'language.label': 'اللغة',
  'theme.toLight': 'التبديل إلى المظهر الفاتح',
  'theme.toDark': 'التبديل إلى المظهر الداكن',

  // The footer.
  'footer.product': 'المنتج',
  'footer.features': 'الميزات',
  'footer.download': 'التنزيل',
  'footer.changelog': 'سجل التغييرات',
  'footer.releases': 'كل الإصدارات',
  'footer.documentation': 'التوثيق',
  'footer.introduction': 'مقدمة',
  'footer.installation': 'التثبيت',
  'footer.hosts': 'توصيل المضيفات',
  'footer.troubleshooting': 'استكشاف المشكلات وحلها',
  'footer.project': 'المشروع',
  'footer.source': 'الشيفرة المصدرية',
  'footer.issue': 'الإبلاغ عن مشكلة',
  'footer.contributing': 'المساهمة',
  'footer.security': 'الأمان',
  'footer.legal': 'قانوني',
  'footer.legalNotice': 'الإشعار القانوني',
  'footer.privacy': 'الخصوصية',
  'footer.terms': 'الشروط',
  'footer.cookies': 'ملفات تعريف الارتباط',
  'footer.licence': '© {year} Damien Reichhart. Homeport مفتوح المصدر بموجب {licence}.',
  'footer.licenceName': 'رخصة GNU AGPL، الإصدار 3',
  'footer.trademark':
    'Docker علامة تجارية لشركة Docker, Inc. وHomeport مشروع مستقل، غير تابع لشركة Docker, Inc. ولا معتمد منها.',
} satisfies Translation<typeof en>;
