/** Arabic text of the `images` namespace. */
import type en from '../en/images';
import type { Translation } from '../types';

export default {
  'images.nav': 'الصور',
  'images.title': 'الصور',
  'images.summary':
    '{count, plural, zero {لا صور} one {صورة واحدة} two {صورتان} few {# صور} many {# صورة} other {# صورة}} · {size}',
  'images.cleanUp': 'تنظيف',
  'images.cleanUp.dangling': 'الصور المعلّقة',
  'images.cleanUp.all': 'كل الصور غير المستخدمة',
  'images.saveSelected':
    '{count, plural, zero {حفظ الصور…} one {حفظ الصورة…} two {حفظ الصورتين…} few {حفظ # صور…} many {حفظ # صورة…} other {حفظ # صورة…}}',
  'images.load': 'تحميل…',
  'images.importTar': 'استيراد من tar…',
  'images.build': 'بناء',
  'images.pull': 'سحب صورة',
  'images.search': 'البحث في الصور',
  'images.empty.title': 'لا توجد صور',
  'images.empty.description': 'اسحب صورة من سجل للبدء.',

  'images.column.name': 'الاسم',
  'images.column.id': 'المعرّف',
  'images.column.created': 'تاريخ الإنشاء',
  'images.column.size': 'الحجم',
  'images.column.status': 'الحالة',
  'images.moreTags': '+{count} وسوم أخرى',
  'images.inUse': 'قيد الاستخدام · {count}',
  'images.unused': 'غير مستخدمة',
  'images.runOne': 'تشغيل {name}',
  'images.action.inspect': 'فحص',
  'images.action.tag': 'وسم',
  'images.action.push': 'دفع',
  'images.action.save': 'حفظ…',

  'images.remove.title': 'هل تريد إزالة {name}؟',
  'images.remove.everyName':
    'يزيل هذا كل أسماء الصورة ({names}) وطبقاتها التي لا يستخدمها أي شيء آخر.',
  'images.remove.description':
    'تُحذف الصورة والطبقات التي لا تستخدمها أي صورة أخرى. يمكن سحبها مجددًا لاحقًا.',
  'images.prune.allTitle': 'هل تريد إزالة كل الصور غير المستخدمة؟',
  'images.prune.danglingTitle': 'هل تريد إزالة الصور المعلّقة؟',
  'images.prune.allDescription': 'تُحذف كل صورة لا تستخدمها أي حاوية، بما في ذلك الصور ذات الوسوم.',
  'images.prune.danglingDescription': 'تُحذف الصور بلا وسوم التي خلّفتها عمليات البناء والسحب.',

  'images.removed': 'تمت إزالة {name}',
  'images.pruned':
    '{count, plural, zero {لم تُزل أي صورة} one {تمت إزالة صورة واحدة} two {تمت إزالة صورتين} few {تمت إزالة # صور} many {تمت إزالة # صورة} other {تمت إزالة # صورة}}، وتم استرداد {space}',
  'images.pulled': 'تم سحب {reference}',
  'images.pushed': 'تم دفع {reference}',
  'images.built': 'تم بناء {name}',
  'images.builtUnnamed': 'تم بناء الصورة',
  'images.saved': 'تم حفظ {path}',
  'images.loadedOne': 'تم تحميل {name}',
  'images.loaded':
    '{count, plural, zero {لم تُحمَّل أي صورة} one {تم تحميل صورة واحدة} two {تم تحميل صورتين} few {تم تحميل # صور} many {تم تحميل # صورة} other {تم تحميل # صورة}}',
  'images.imported': 'تم استيراد {name}',

  'images.files.save': 'حفظ الصور باسم',
  'images.files.load': 'تحميل الصور من',
  'images.files.import': 'استيراد نظام ملفات من',
  'images.files.imageArchives': 'أرشيفات الصور',
  'images.files.imageArchive': 'أرشيف صورة',
  'images.files.filesystemArchives': 'أرشيفات أنظمة الملفات',

  'images.buildDialog.title': 'بناء صورة',
  'images.buildDialog.description':
    'من مجلد على هذا الكمبيوتر. يرسله Homeport إلى المضيف، مستثنيًا ما يستبعده ملف .dockerignore، ويبنيه محرك المضيف.',
  'images.buildDialog.built': '✓ تم البناء',
  'images.buildDialog.folder': 'المجلد',
  'images.buildDialog.chooseFolder': 'اختر المجلد المراد بناؤه',
  'images.buildDialog.choose': 'اختيار…',
  'images.buildDialog.dockerfile': 'Dockerfile',
  'images.buildDialog.target': 'المرحلة الهدف',
  'images.buildDialog.targetPlaceholder': 'المرحلة الأخيرة',
  'images.buildDialog.names': 'الأسماء',
  'images.buildDialog.buildArgs': 'وسائط البناء',
  'images.buildDialog.buildArgsPlaceholder': 'VERSION=1.0، واحد في كل سطر',
  'images.buildDialog.noCache': 'عدم استخدام ذاكرة التخزين المؤقت للبناء',
  'images.buildDialog.pull': 'سحب صور أساسية أحدث',
  'images.buildForm.noFolder': 'اختر المجلد المراد بناؤه.',
  'images.buildForm.argNeedsName': 'يحتاج «{line}» إلى اسم قبل «=».',

  'images.hub.unavailable':
    'لا يمكن البحث في Docker Hub حاليًا: {error} لا يزال الاسم الكامل مثل nginx:1.27 يُسحب.',
  'images.hub.noMatch': 'لا يوجد في Docker Hub ما يطابق «{term}».',
  'images.hub.results': 'نتائج Docker Hub',
  'images.hub.result': '{name}، {stars} نجمة',
  'images.hub.resultOfficial': '{name}، صورة رسمية، {stars} نجمة',
  'images.hub.official': 'رسمية',
  'images.hub.backToSearch': 'بحث',
  'images.hub.tagCount':
    '{count, plural, zero {لا وسوم} one {وسم واحد} two {وسمان} few {# وسوم} many {# وسمًا} other {# وسم}}',
  'images.hub.filterTags': 'تصفية الوسوم',
  'images.hub.tags': 'الوسوم',
  'images.hub.noTagMatch': 'لا يوجد وسم يطابق «{filter}».',
  'images.hub.moreTags': 'مزيد من الوسوم',

  'images.importDialog.title': 'استيراد نظام ملفات كصورة',
  'images.importDialog.description':
    'يصبح {path} صورة بطبقة واحدة دون سجل أو إعدادات خاصة بها؛ أعطها أمرًا أدناه لجعلها قابلة للتشغيل.',
  'images.importDialog.name': 'اسم الصورة',
  'images.importDialog.changes': 'التغييرات',
  'images.importDialog.changesHint':
    'تعليمات Dockerfile، واحدة في كل سطر: CMD أو ENTRYPOINT أو ENV أو EXPOSE أو LABEL أو ONBUILD أو USER أو VOLUME أو WORKDIR.',
  'images.importDialog.import': 'استيراد',

  'images.inspect.layers': 'الطبقات',
  'images.inspect.raw': 'JSON الخام',
  'images.inspect.created': 'تاريخ الإنشاء',
  'images.inspect.instruction': 'التعليمة',
  'images.inspect.size': 'الحجم',

  'images.pullDialog.title': 'سحب صورة',
  'images.pullDialog.description':
    'ابحث في Docker Hub بالاسم، أو اكتب أي مرجع يمكن للمضيف الوصول إليه، مثل nginx:1.27 أو ghcr.io/org/app:v2.',
  'images.pullDialog.placeholder': 'postgres، أو nginx:1.27',
  'images.pullDialog.reference': 'مرجع الصورة',
  'images.pullDialog.pull': 'سحب',
  'images.done': 'تم.',

  'images.pushDialog.title': 'دفع {name}',
  'images.pushDialog.description':
    'إلى السجل الذي يبدأ به اسمها، مثل ghcr.io/org/app:v2، أو إلى Docker Hub لأسماء مثل org/app:v2.',
  'images.pushDialog.target': 'الاسم المراد دفعه',
  'images.pushDialog.push': 'دفع',
  'images.pushDialog.tagsFirst': 'تُعطى الصورة هذا الاسم أولًا، كما يفعل «وسم».',
  'images.pushDialog.uploading': 'جارٍ الرفع {current} / {total}',
  'images.push.pushed':
    '{pushed, plural, zero {لم تُدفع أي طبقة} one {تم دفع طبقة واحدة} two {تم دفع طبقتين} few {تم دفع # طبقات} many {تم دفع # طبقة} other {تم دفع # طبقة}}',
  'images.push.pushedAndExisting':
    '{pushed, plural, zero {لم تُدفع أي طبقة} one {تم دفع طبقة واحدة} two {تم دفع طبقتين} few {تم دفع # طبقات} many {تم دفع # طبقة} other {تم دفع # طبقة}}، و{existing} موجودة بالفعل في السجل',

  'images.tagDialog.title': 'وسم {name}',
  'images.tagDialog.newName': 'الاسم الجديد',
  'images.tagDialog.add': 'إضافة وسم',

  'images.transfer.savingOne': 'جارٍ حفظ {name}',
  'images.transfer.saving':
    'جارٍ حفظ {count, plural, zero {0 صورة} one {صورة واحدة} two {صورتين} few {# صور} many {# صورة} other {# صورة}}',
  'images.transfer.to': 'إلى {path}',
  'images.transfer.from': 'من {path}',
  'images.transfer.written': 'المكتوب',
  'images.transfer.sent': 'المُرسَل',
  'images.transfer.loading': 'جارٍ تحميل الصور',
  'images.transfer.importing': 'جارٍ استيراد {reference}',
  'images.transfer.doneWith': 'تم: {images}',
  'images.palette.pull': 'سحب صورة',
} satisfies Translation<typeof en>;
