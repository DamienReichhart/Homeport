/** Arabic text of the `repositories` namespace. */
import type en from '../en/repositories';
import type { Translation } from '../types';

export default {
  'repositories.title': 'المستودعات',
  'repositories.description':
    'مستودعات Git التي تُنشر منها المكدسات المتزامنة، على كل مضيف. يقرأها Homeport فقط.',
  'repositories.add': 'إضافة مستودع',
  'repositories.empty': 'لا توجد مستودعات بعد. أضف واحدًا لإنشاء مكدس متزامن.',
  'repositories.manageOne': 'إدارة الوصول إلى {repository}',
  'repositories.removeOne': 'إزالة {repository}',
  'repositories.remove.title': 'هل تريد إزالة {repository}؟',
  'repositories.remove.description':
    'ينسى Homeport مفتاح النشر أو الرمز الخاص به ويحذف نسخته على هذا الكمبيوتر. لا يُمسّ المستودع نفسه. أزل مفتاح النشر منه على منصة git الخاصة بك أيضًا.',
  'repositories.noKeychain':
    'تُشفَّر مفاتيح النشر والرموز بمفتاح يُحفظ في سلسلة مفاتيح نظام التشغيل، ولا تتوفر أي سلسلة، لذا لا يمكن إضافة سوى المستودعات العامة.',
  'repositories.public': 'مستودع عام: يقرأه Homeport دون بيانات اعتماد.',

  'repositories.added': 'تمت إضافة {repository}',
  'repositories.removed': 'تمت إزالة {repository}',
  'repositories.keyRotated': 'تم إنشاء مفتاح نشر جديد. أضفه إلى المستودع بدلًا من المفتاح القديم.',
  'repositories.tokenReplaced': 'تم استبدال الرمز',
  'repositories.hostKeyReset': 'سيثق Homeport بالمفتاح الذي يقدّمه الخادم في المرة القادمة.',

  'repositories.auth.withKey': 'بمفتاح نشر، على {host}',
  'repositories.auth.withToken': 'باسم {username} برمز، على {host}',
  'repositories.auth.public': 'عام، على {host}',

  'repositories.dialog.addTitle': 'إضافة مستودع',
  'repositories.dialog.description':
    'ينسخه Homeport على هذا الكمبيوتر لنشر المكدسات المتزامنة منه. لا يُجري أي إيداع أو دفع أبدًا.',
  'repositories.dialog.url': 'عنوان URL للمستودع',
  'repositories.dialog.auth': 'الوصول',
  'repositories.dialog.authKey': 'مفتاح النشر',
  'repositories.dialog.authToken': 'رمز',
  'repositories.dialog.authNone': 'عام',
  'repositories.dialog.authKeyHelp':
    'ينشئ Homeport مفتاحًا لهذا المستودع وحده. تضيفه أنت إلى المستودع بصلاحية القراءة؛ ولا تُستخدم مفاتيح SSH الخاصة بك أبدًا.',
  'repositories.dialog.authTokenHelp':
    'رمز وصول يمكنه قراءة محتويات المستودع، يُرسل عبر HTTPS. يُشفَّر على هذا الكمبيوتر ولا يُعرض مجددًا.',
  'repositories.dialog.authNoneHelp': 'لمستودع عام عبر HTTPS. لا يُخزَّن أي شيء.',
  'repositories.dialog.username': 'اسم المستخدم',
  'repositories.dialog.token': 'رمز الوصول',
  'repositories.dialog.add': 'إضافة',
  'repositories.dialog.addWithKey': 'إضافة وإنشاء مفتاح نشر',

  'repositories.missing.url': 'أدخل عنوان URL للمستودع.',
  'repositories.missing.validUrl':
    'أدخل عنوان URL لـ SSH أو HTTPS، مثل git@github.com:owner/repo.git أو https://gitlab.com/owner/repo.git.',
  'repositories.missing.auth': 'اختر طريقة وصول Homeport إلى هذا المستودع.',
  'repositories.missing.username': 'أدخل اسم المستخدم الذي يخصه الرمز.',
  'repositories.missing.token': 'أدخل رمز الوصول.',

  'repositories.deployKey.title': 'مفتاح النشر',
  'repositories.deployKey.github':
    'GitHub: في المستودع <strong>Settings → Deploy keys → Add deploy key</strong>.',
  'repositories.deployKey.gitlab':
    'GitLab: في المشروع <strong>Settings → Repository → Deploy keys</strong>.',
  'repositories.deployKey.gitea':
    'Gitea وForgejo: في المستودع <strong>Settings → Deploy keys → Add deploy key</strong>.',
  'repositories.deployKey.readOnly':
    'اترك صلاحية الكتابة معطّلة: يقرأ Homeport فقط. ثم اختبر الوصول أدناه.',
  'repositories.deployKey.fingerprint': 'البصمة: <mono>{fingerprint}</mono>',
  'repositories.rotate.button': 'مفتاح جديد',
  'repositories.rotate.title': 'هل تريد إنشاء مفتاح نشر جديد؟',
  'repositories.rotate.description':
    'يتوقف المفتاح الحالي عن العمل مع Homeport. أضف المفتاح الجديد إلى المستودع، وأزل القديم منه.',
  'repositories.rotate.confirm': 'إنشاء مفتاح جديد',

  'repositories.token.replace': 'استبدال الرمز',
  'repositories.token.save': 'حفظ الرمز',

  'repositories.hostKey.trusted': 'المفتاح الموثوق لـ {host}: <mono>{fingerprint}</mono>',
  'repositories.hostKey.reset': 'إعادة تعيين',
  'repositories.hostKey.resetTitle': 'هل تريد إعادة تعيين المفتاح الموثوق لـ {host}؟',
  'repositories.hostKey.resetDescription':
    'لا تفعل ذلك إلا إذا غيّر {host} مفتاح SSH الخاص به وتحققت من البصمة الجديدة على موقعه. سيثق Homeport بأي مفتاح يقدّمه في المرة القادمة.',

  'repositories.test.button': 'اختبار الوصول',
  'repositories.test.testing': 'جارٍ الاختبار…',
  'repositories.test.success':
    'يمكن لـ Homeport قراءته: {branches, plural, zero {لا فروع} one {فرع واحد} two {فرعان} few {# فروع} many {# فرعًا} other {# فرع}} و{tags, plural, zero {لا وسوم} one {وسم واحد} two {وسمان} few {# وسوم} many {# وسمًا} other {# وسم}}.',
} satisfies Translation<typeof en>;
