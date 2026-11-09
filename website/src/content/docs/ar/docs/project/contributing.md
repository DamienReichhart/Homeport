---
title: المساهمة
description: كيفية الإبلاغ عن المشكلات واقتراح الميزات والمساهمة بالشيفرة في Homeport.
---

يُطوَّر Homeport بشكل علني على [github.com/DamienReichhart/Homeport](https://github.com/DamienReichhart/Homeport). المساهمات بكل أحجامها مرحب بها.

## الإبلاغ عن خطأ

[افتح مشكلة](https://github.com/DamienReichhart/Homeport/issues/new) تتضمن:

- إصدار Homeport ومنصتك، من **الإعدادات** (مثل `v0.2.0 · macos/aarch64`)؛
- نوع المضيف (محلي، أو SSH، أو TLS، أو TCP) وإصدار محركه؛
- ما فعلته، وما توقعته، وما حدث — الرسالة الحرفية هي الأكثر فائدة؛
- أي شيء ذي صلة من [ملف السجل](/docs/reference/security/#السجلات).

يُرجى الإبلاغ عن المشكلات الأمنية بشكل خاص، عبر [الإبلاغ عن الثغرات في GitHub](https://github.com/DamienReichhart/Homeport/security/advisories/new).

## اقتراح ميزة

افتح مشكلة تصف المشكلة التي تريد حلها، لا الحل وحده. معرفة طريقة عملك مع محركاتك تساعد على إيجاد الشكل المناسب لها.

## المساهمة بالشيفرة

1. اقرأ [`docs/ARCHITECTURE.md`](https://github.com/DamienReichhart/Homeport/blob/main/docs/ARCHITECTURE.md). الميزات شرائح عمودية مستقلة بذاتها، ولا تستدعي الواجهة الأمامية Rust إلا عبر روابط IPC المُنشأة، ويعيش منطق Rust في خدمات تُختبر وحداتها دون Tauri.
2. استخدم مفردات المشروع من [`GLOSSARY.md`](https://github.com/DamienReichhart/Homeport/blob/main/GLOSSARY.md) — Host وStack وManaged وExternal وAdopt — في الشيفرة ونصوص الواجهة والإيداعات. القرارات الرئيسية مدوّنة في [`docs/adr`](https://github.com/DamienReichhart/Homeport/tree/main/docs/adr).
3. اتبع [دليل العلامة التجارية](https://github.com/DamienReichhart/Homeport/blob/main/docs/BRAND.md) في الألوان والأسلوب: بسيط ومحدد وهادئ، بالتهجئة البريطانية.
4. تأكد من نجاح هذه الأوامر:

   ```sh
   bun run check
   bun run rust:lint
   bun run rust:test
   ```

5. اكتب رسائل الإيداع بصيغة [Conventional Commits](https://www.conventionalcommits.org) — `feat(stacks): …`، `fix(hosts): …`. تُنشأ ملاحظات الإصدار منها.
6. افتح طلب سحب على الفرع `dev`. يشغّل CI فحوصات الواجهة الأمامية وفحص التراخيص على Linux، و`cargo clippy` و`cargo test` على Linux وmacOS وWindows، لذا تُصرَّف الشيفرة الموجودة خلف `#[cfg(windows)]` أو `#[cfg(target_os = "macos")]` قبل دمجها. الاختبار الذي يحتاج إلى محرك Docker يُوسم بـ `#[ignore = "needs a local Docker Engine"]`؛ وتشغّل مهمة Linux منفصلة تلك الاختبارات على محرك المشغّل.

راجع [البناء من الشيفرة المصدرية](/docs/project/building/) لتجهيز بيئتك.

### إضافة نص إلى التطبيق

يعيش نص التطبيق في كتالوج، وليس مضمّنًا في الشيفرة أبدًا، لكي يمكن ترجمته ([ADR 0011](https://github.com/DamienReichhart/Homeport/blob/main/docs/adr/0011-formatjs-catalogues-and-keyed-errors.md)). لإضافة سلسلة نصية:

1. أضف مفتاحًا إلى ملف الميزة في [`src/shared/i18n/en/`](https://github.com/DamienReichhart/Homeport/tree/main/src/shared/i18n/en) — `images.ts` لميزة الصور، و`ui.ts` لمجموعة عناصر الواجهة، و`common.ts` فقط للكلمات التي تعني الشيء نفسه في كل مكان (Cancel وClose وRetry). تبدأ المفاتيح باسم الملف وتوضح الغرض من النص: `images.remove.title`، `images.column.size`.
2. اكتب القيمة كـ [رسالة ICU](https://formatjs.github.io/docs/core-concepts/icu-syntax/): `{name}` لقيمة، و`{count, plural, one {# image} other {# images}}` بدلًا من بناء صيغ الجمع يدويًا، و`<strong>…</strong>` للتأكيد الذي يُعرض باستخدام `tRich`. مفتاح واحد لكل جملة؛ لا تلصق الأجزاء معًا، لأن ترتيب الكلمات يتغير بين اللغات.
3. استخدمه عبر `t('images.remove.title', { name })` من `@/shared/i18n` — في المكونات وعمليات التعديل والإشعارات المنبثقة على حد سواء. المفاتيح ذات أنواع محددة، لذا يُفشل أي خطأ مطبعي فحص الأنواع. استخدم مفاتيح حرفية: عندما يعتمد النص على قيمة، اربط القيمة بالمفاتيح (`{ start: 'containers.done.start', stop: 'containers.done.stop' }`) بدلًا من بناء `` `containers.done.${action}` ``.

يفشل `bun run lint` عندما يكتب مكوّن نصًا مضمّنًا: نص JSX، والخصائص التي يقرؤها الناس (`title` و`aria-label` و`placeholder` و`description` و`label`…)، ونص الإشعارات المنبثقة ورسائل التأكيد. أما السلاسل التي تُقرأ كشيفرة لا ككلمات — `tcp` و`nginx:latest` و`/dev/dri` و`max-size=10m` — وأسماء العلم مثل Docker أو Podman فتمر. للإبقاء على نص حرفي عمدًا، اذكر السبب في السطر السابق: `// eslint-disable-next-line homeport/no-literal-ui-string -- a shell command`. ويفشل `bun run check` أيضًا عند وجود مفتاح مستخدم في الشيفرة ومفقود من الكتالوج، أو مفتاح لا يستخدمه شيء، أو مفتاح يُبنى وقت التشغيل.

تُكتب رسائل الأخطاء بلغة Rust: تعلن كل حزمة crate مفاتيحها وقوالبها الإنجليزية في كتلة `messages!` الخاصة بها، ويكتبها `bun run bindings` في `src/shared/i18n/en/errors.json`. لا تحرّر ذلك الملف يدويًا.

## ترجمة Homeport

يتحدث التطبيق والموقع الإنجليزية والفرنسية والإسبانية والألمانية والإيطالية والبرتغالية البرازيلية والروسية والصينية المبسطة واليابانية والكورية والهندية والعربية. تستخدم كل لغة كلمة واحدة لكل مصطلح من مصطلحات المنتج: ابحث عنها في [`docs/i18n/`](https://github.com/DamienReichhart/Homeport/tree/main/docs/i18n) قبل الترجمة، وأضف صفًا هناك عندما تختار مصطلحًا جديدًا.

### إضافة سلسلة نصية أو تغييرها

أيًّا كانت السلسلة، يفشل `bun run check` (ومن ثم CI) إلى أن تحتويها كل اللغات، بالـ `{arguments}` والوسوم نفسها، وبكل صيغة جمع تحتاجها تلك اللغة.

- **نص التطبيق**: أضف المفتاح إلى `src/shared/i18n/en/<namespace>.ts` (راجع [إضافة نص إلى التطبيق](#إضافة-نص-إلى-التطبيق))، ثم إلى الملف الذي يحمل الاسم نفسه في مجلد كل لغة أخرى (`fr/` و`de/` و`zh-CN/`…). ترجمه إن استطعت. وإن لم تستطع، فانسخ النص الإنجليزي هناك واذكر ذلك في طلب السحب، ليتابعه مترجم؛ ولا تحذف المفتاح أبدًا.
- **رسالة خطأ أو تلميح**: أعلنه في كتلة `messages!` الخاصة بالحزمة وشغّل `bun run bindings`، الذي يكتب `en/errors.json`؛ ثم أضف المفتاح إلى `errors.json` الخاص بكل لغة. والنص الذي يعرضه نظام التشغيل — قائمة العلبة والإشعارات — يعمل بالطريقة نفسها بمفتاح `native.` و`native.json`.
- **نص الموقع** (الصفحة الرئيسية، والتنزيل، وسجل التغييرات، والترويسة والتذييل): أضف المفتاح إلى الملف المناسب من `website/src/core/i18n/strings/en/` (`site` أو `home` أو `download` أو `changelog`) وإلى الملف نفسه في مجلد كل لغة أخرى؛ ويفشل `astro check` عندما يفتقر أحدها إليه. وتوضع تسمية الشريط الجانبي للوثائق في `website/src/app/config/sidebar/<code>.ts`.
- **صفحة وثائق**: توجد الصفحة الإنجليزية في `website/src/content/docs/docs/`. عندما تغيّرها، حدّث الصفحة نفسها في كل لغة، في المسار نفسه ضمن `website/src/content/docs/<code>/docs/`، في طلب السحب نفسه؛ واللغة التي لا تستطيع تحديث صفحتها تعرض ترجمتها الأقدم إلى أن يحدّثها مترجم. أبقِ عناوين الصفحات المترجمة بالترتيب نفسه وفي المستويات نفسها كما في الإنجليزية، والروابط تمامًا كما تكتبها الإنجليزية، بما فيها المراسي: بعد `bun run build`، يوجّه `bun scripts/localize-anchors.ts <code>` كل مرساة إلى العنوان المترجم، ويؤكد ذلك فحصُ الروابط في عملية بناء ثانية.

تحقق من لغة واحدة باستخدام `bun run i18n:check --locale fr`.

### إضافة لغة

ملفات، إضافة إلى سطر واحد للموقع:

1. **التطبيق**: انسخ `src/shared/i18n/en/` إلى `src/shared/i18n/<tag>/` (وسم BCP 47 مثل `pl` أو `zh-TW`)، واكتب ملف `meta.ts` الخاص بها (`{ name: 'Polski', dir: 'ltr' }`، أي اسم اللغة بلغتها نفسها؛ و`rtl` للكتابة من اليمين إلى اليسار) وترجم كل ملف، مع الإبقاء على كل مفتاح وتحويل كل `export default { … }` إلى `export default { … } satisfies Translation<typeof en>` كما تفعل اللغات الأخرى. يسرد `bun run i18n:check --locale <tag>` ما هو مفقود. تعرض الإعدادات اللغة الجديدة، وتختارها لغة نظام التشغيل، دون أي تغيير آخر.
2. **الموقع**: أضف اللغة إلى `website/src/core/site/locales.ts`، ثم سلاسلها النصية في نسخة من `website/src/core/i18n/strings/en/` باسم `<code>/` (مسجّلة في `TABLES` ضمن `core/i18n/index.ts`)، وتسميات الشريط الجانبي الخاصة بها في `website/src/app/config/sidebar/<code>.ts` (مسجّلة في `sidebar.ts`؛ هذان السطران هما الشيفرة الوحيدة التي تحتاجها اللغة، لأن تلك الملفات تُحمَّل خارج Vite، وهو ما يعثر على المجلدات)، وملاحظة Starlight للصفحات غير المترجمة في `website/src/content/i18n/<tag>.json`. أما صفحاتها وعناوين URL الخاصة بها ومدخلات مبدّل اللغة وروابط `hreflang` وبدائل خريطة الموقع فتنتج عن القائمة. وتوضع وثائقها ضمن `website/src/content/docs/<code>/docs/`: ترجم كل صفحة فيها (والصفحة الناقصة تعرض الإنجليزية مع ملاحظة)، ثم شغّل `bun scripts/localize-anchors.ts <code>` بعد عملية بناء.
3. **لقطات الشاشة**: يلتقط `bun run screenshots` كل لقطة بكل لغة، بما فيها الجديدة، في `website/src/assets/screenshots/<tag>/`، فتعرض كل صفحة التطبيق بلغة قارئها. ويفشل `bun test` الخاص بالموقع ما دامت لغة ما تفتقد لقطة موجودة في الإنجليزية.
4. **المصطلحات**: أضف `docs/i18n/terms-<tag>.md`.

إذا لم يرَ اختبارٌ لغةً أضفتها للتو، فاحذف `node_modules/.vitest-cache`: قد يحتفظ Vitest بقائمة قديمة لملفات الكتالوج.

## تحسين هذه الوثائق

تحتوي كل صفحة من صفحات الوثائق على رابط **تعديل الصفحة** في أسفلها. الصفحات ملفات Markdown في [`website/src/content/docs`](https://github.com/DamienReichhart/Homeport/tree/main/website/src/content/docs).

## كيف تتم الإصدارات

كل دفع إلى `main` برقم إصدار جديد يبني أدوات التثبيت لكل المنصات، ويشغّل مجموعة الاختبارات الكاملة، وينشر إصدارًا على GitHub مع مجاميع التحقق ومصدر البناء. العملية موصوفة في [`docs/RELEASING.md`](https://github.com/DamienReichhart/Homeport/blob/main/docs/RELEASING.md).
