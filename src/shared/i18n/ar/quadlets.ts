/** Arabic text of the `quadlets` namespace. */
import type en from '../en/quadlets';
import type { Translation } from '../types';

export default {
  'quadlets.nav': 'Quadlet',
  'quadlets.title': 'وحدات Quadlet',
  'quadlets.summary':
    '{count, plural, zero {لا وحدات} one {وحدة واحدة} two {وحدتان} few {# وحدات} many {# وحدة} other {# وحدة}}',
  'quadlets.search': 'البحث في الوحدات',
  'quadlets.empty': 'لا توجد وحدات Quadlet',
  'quadlets.emptyDescription':
    'وحدات Quadlet ملفات يحوّلها Podman إلى خدمات systemd، فتبدأ الحاويات مع المضيف ويُبقيها systemd قيد التشغيل. ثبّت وحدة لتراها هنا.',
  'quadlets.new': 'وحدة جديدة',
  'quadlets.edit': 'تحرير',
  'quadlets.notGenerated': 'لم تُولَّد بعد',
  'quadlets.gone': 'لم تعد {name} مثبّتة',
  'quadlets.column.name': 'الوحدة',
  'quadlets.column.kind': 'النوع',
  'quadlets.column.service': 'الخدمة',
  'quadlets.column.status': 'الحالة',
  'quadlets.column.containers': 'الحاويات',
  'quadlets.status.running': 'قيد التشغيل',
  'quadlets.status.starting': 'جارٍ البدء',
  'quadlets.status.stopping': 'جارٍ الإيقاف',
  'quadlets.status.stopped': 'متوقفة',
  'quadlets.status.failed': 'فشلت',
  'quadlets.status.notLoaded': 'لم يحمّلها systemd',
  'quadlets.status.template': 'قالب',
  'quadlets.action.start': 'تشغيل',
  'quadlets.action.stop': 'إيقاف',
  'quadlets.action.restart': 'إعادة التشغيل',
  'quadlets.actionOne.start': 'تشغيل {name}',
  'quadlets.actionOne.stop': 'إيقاف {name}',
  'quadlets.actionOne.restart': 'إعادة تشغيل {name}',
  'quadlets.done.start': 'تم تشغيل {name}',
  'quadlets.done.stop': 'تم إيقاف {name}',
  'quadlets.done.restart': 'تمت إعادة تشغيل {name}',
  'quadlets.installed': 'تم تثبيت {name}',
  'quadlets.saved': 'تم حفظ {name}',
  'quadlets.removed': 'تمت إزالة {name}',
  'quadlets.blocked.noCommands':
    'يشغّل Homeport الوحدات ويوقفها باستخدام systemctl، ولا يمكنه تنفيذه إلا عبر SSH أو لـ Podman على هذا الكمبيوتر.',
  'quadlets.blocked.notLoaded': 'لم يولّد systemd خدمة هذه الوحدة، لذا لا يوجد ما يمكن تشغيله.',
  'quadlets.blocked.podmanTooOld':
    'تتطلب قراءة الوحدات وتثبيتها وتحريرها وإزالتها Podman 5.8 أو أحدث؛ ويشغّل هذا المضيف Podman {version}.',
  'quadlets.blocked.noFile': 'هذه الوحدة معروفة من حاوياتها فقط، لذا لا يمكن تغيير ملفها.',
  'quadlets.note.labels':
    'لا يستطيع Podman {version} سرد وحدات Quadlet، لذا تأتي هذه من الحاويات التي أنشأتها. تظهر ملفاتها، والوحدات التي لم تنشئ أي حاوية، على Podman 5.7 وما بعده.',
  'quadlets.note.readOnly':
    'يسرد Podman {version} الوحدات لكنه لا يستطيع قراءتها أو تثبيتها أو تحريرها أو إزالتها؛ فذلك يتطلب Podman 5.8 أو أحدث.',
  'quadlets.note.noCommands':
    'يتطلب تشغيل الوحدات وإيقافها الوصول إلى هذا المضيف عبر SSH، أو Podman على هذا الكمبيوتر.',
  'quadlets.detail.service': 'الخدمة',
  'quadlets.detail.unit': 'وحدة systemd',
  'quadlets.detail.scope': 'تعمل بصفة',
  'quadlets.detail.path': 'الملف',
  'quadlets.detail.app': 'التطبيق',
  'quadlets.detail.pod': 'Pod',
  'quadlets.detail.containers':
    '{count, plural, =0 {الحاويات} zero {الحاويات} one {حاوية واحدة} two {حاويتان} few {# حاويات} many {# حاوية} other {# حاوية}}',
  'quadlets.detail.noContainers': 'لم تنشئ أي حاوية، أو خدمتها متوقفة.',
  'quadlets.scope.user': 'وحدة مستخدم (systemctl --user)',
  'quadlets.scope.system': 'وحدة نظام',
  'quadlets.file.title': 'ملف الوحدة',
  'quadlets.file.unknown': 'هذه الوحدة معروفة من تسمية حاوياتها فقط.',
  'quadlets.file.tooOld':
    'لا يستطيع Podman {version} قراءة ملفات الوحدات؛ ويتطلب عرضها Podman 5.8 أو أحدث.',
  'quadlets.form.newTitle': 'وحدة Quadlet جديدة',
  'quadlets.form.newDescription':
    'يكتب Podman الملف حيث توضع وحدات مستخدمه ويعيد تحميل systemd. شغّل الوحدة بعد تثبيتها.',
  'quadlets.form.editTitle': 'تحرير {name}',
  'quadlets.form.editDescription':
    'يستبدل الحفظ الملف ويعيد تحميل systemd؛ أعد تشغيل الوحدة لتستخدمه خدمة قيد التشغيل. تبقى الملفات المثبتة بجانبه.',
  'quadlets.form.name': 'اسم الملف',
  'quadlets.form.kind': 'النوع',
  'quadlets.form.openFile': 'فتح ملف…',
  'quadlets.form.openTitle': 'فتح ملف وحدة Quadlet',
  'quadlets.form.unitFiles': 'ملفات وحدات Quadlet',
  'quadlets.form.content': 'ملف الوحدة',
  'quadlets.form.extra': 'الملفات المجاورة',
  'quadlets.form.extraHint':
    'ملف Containerfile أو kube YAML أو ملف env تستخدمه الوحدة، يُثبَّت في مجلد تطبيقها.',
  'quadlets.form.extraTitle': 'اختر ملفات لتثبيتها بجانب الوحدة',
  'quadlets.form.addExtra': 'إضافة ملفات…',
  'quadlets.form.removeExtra': 'إزالة {name}',
  'quadlets.form.install': 'تثبيت',
  'quadlets.form.save': 'حفظ',
  'quadlets.form.overwrite': 'الكتابة فوقها',
  'quadlets.form.nameInvalid':
    'سمّها مثل web.container، بحيث تنتهي بـ .container أو .pod أو .volume أو .network أو .kube أو .image أو .build أو .artifact.',
  'quadlets.form.sectionMissing': 'يحتاج الملف إلى قسم {section} ليتمكن Quadlet من قراءته.',
  'quadlets.form.extraInvalid': 'يحتاج كل ملف بجانب الوحدة إلى اسم بسيط خاص به، دون مجلدات.',
  'quadlets.remove.title': 'هل تريد إزالة {name}؟',
  'quadlets.remove.description':
    'يحذف Podman ملف الوحدة ويعيد تحميل systemd. تبقى الحاويات ووحدات التخزين والصور التي أنشأتها.',
  'quadlets.remove.stop': 'إيقاف خدمتها إن كانت قيد التشغيل',
  'quadlets.remove.stopRunning': 'إيقاف خدمتها قيد التشغيل أولًا',
} satisfies Translation<typeof en>;
