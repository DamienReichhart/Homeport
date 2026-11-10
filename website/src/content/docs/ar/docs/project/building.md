---
title: البناء من الشيفرة المصدرية
description: ابنِ Homeport وشغّله من شيفرته المصدرية.
---

Homeport تطبيق [Tauri 2](https://tauri.app): واجهة بلغتي React وTypeScript فوق نواة بلغة Rust، تتحدث مع محركات Docker باستخدام [bollard](https://github.com/fussybeaver/bollard) وتخزّن بياناتها في SQLite.

## المتطلبات المسبقة

- [Bun](https://bun.sh) 1.4 أو أحدث
- [Rust](https://www.rust-lang.org/tools/install)، الإصدار المستقر (يختار ملف `rust-toolchain.toml` في المستودع المكونات)
- [تبعيات النظام الخاصة بـ Tauri](https://tauri.app/start/prerequisites/) لمنصتك. على Debian وUbuntu:

  ```sh
  sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev libsoup-3.0-dev librsvg2-dev \
    libayatana-appindicator3-dev libssl-dev
  ```

- لبناء الإصدارات، [cargo-about](https://github.com/EmbarkStudios/cargo-about)، الذي ينشئ إشعارات تراخيص الجهات الخارجية:

  ```sh
  cargo install cargo-about --locked --features cli
  ```

## شغّله

```sh
git clone https://github.com/DamienReichhart/Homeport.git
cd Homeport
bun install
bun run tauri:dev
```

يشغّل `tauri:dev` التطبيق مع إعادة التحميل الفورية للواجهة؛ وتؤدي تغييرات Rust إلى إعادة بنائه وإعادة تشغيله.

## بناء أدوات التثبيت

```sh
bun run tauri:build
```

تُكتب أدوات التثبيت الخاصة بمنصتك في `target/release/bundle/`. ينشئ البناء إشعارات التراخيص أولًا، ويتوقف إذا لم يُراجَع ترخيص إحدى التبعيات في `about.toml`.

لا تُوقَّع عمليات البناء المحلية لأداة التحديث داخل التطبيق، وتترك حزمة `.rpm` غير موقّعة؛ إذ لا يملك مفاتيح التوقيع سوى [سير عمل الإصدار](https://github.com/DamienReichhart/Homeport/blob/main/docs/RELEASING.md). النسخة التي تبنيها بنفسك تظل تتحقق من التحديثات، لكن **تثبيت وإعادة التشغيل** فيها لا يثبّت إلا الإصدارات الموقّعة من سير عمل الإصدار.

## أوامر مفيدة

| الأمر                 | ما يفعله                                                                                                                                                                                               |
| --------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `bun run check`       | التحقق من الأنواع، والتدقيق، والتحقق من التنسيق، واختبارات الواجهة الأمامية                                                                                                                            |
| `bun run rust:lint`   | `cargo clippy` مع معاملة التحذيرات كأخطاء                                                                                                                                                              |
| `bun run rust:test`   | اختبارات وحدات Rust                                                                                                                                                                                    |
| `bun run bindings`    | إعادة إنشاء روابط TypeScript الخاصة بـ IPC                                                                                                                                                             |
| `bun run licenses`    | إنشاء إشعارات تراخيص الجهات الخارجية                                                                                                                                                                   |
| `bun run screenshots` | إعادة التقاط لقطات الشاشة المستخدمة في هذا الموقع، بكل لغة (`--locale fr` لإعادة التقاط لغة واحدة)، بحجم 2880×1800، أي ضعف كثافة البكسلات. ثبّت oxipng أولًا (`cargo install oxipng`) لتُضغط دون فقدان |

تتحدث اختبارات التكامل مع محرك Docker المحلي ومع Compose CLI. وهي موسومة بـ `#[ignore = "needs a local Docker Engine"]` (أو `"… and the compose CLI"`، وبعضها `"… access to Docker Hub"`)، ولا تنشئ إلا موارد باسم `homeport-test-*` وتزيلها بعد ذلك. شغّلها مع وجود محرك ومكوّن compose الإضافي على هذا الجهاز:

```sh
cargo test --workspace --exclude homeport -- --ignored
```

يشغّلها CI أيضًا، في مهمة `engine-tests` من `.github/workflows/ci.yml`، عند كل دفع إلى `dev`، وكل طلب سحب، وكل إصدار. ثم تشغّل المهمة `scripts/ci/engine-test-leftovers.sh`، الذي يفشل إذا ترك اختبار ما حاوية أو صورة أو وحدة تخزين أو شبكة باسم `homeport-test-*`؛ شغّله محليًا بعد الاختبارات للتحقق من الشيء نفسه.

### الاختبارات الشاملة

يقود `e2e/` التطبيق الحقيقي — الواجهة الخلفية بلغة Rust، وwebview، والاستدعاءات بينهما — باستخدام [WebdriverIO](https://webdriver.io) عبر `tauri-driver`، مقابل محرك Docker المحلي. السيناريوهات: يبدأ التطبيق ويتصل بالمحرك المحلي؛ وتتدفق سجلات حاوية وتتوقف من التطبيق؛ وتُسحب صورة؛ ويُنشر مكدس مُدار ببيئة مشفّرة؛ وتُبقي إعادة التشغيل المضيفات والمكدس وسرّه؛ ويُضاف مضيف SSH بمفتاح موجود، ويُوثق بمفتاح خادمه، ويتصل. وهي تعثر على العناصر حسب الدور والتسمية والنص، ولا تعتمد أبدًا على فئات CSS.

لا تعمل إلا على Linux: لا يوجد على macOS برنامج WebDriver لـ webview الخاص به. وإلى جانب [المتطلبات المسبقة](#prerequisites) ومحرك Docker، تحتاج إلى `tauri-driver` (`cargo install tauri-driver --locked`)، وبرنامج تشغيل WebKit، وشاشة افتراضية، وسلسلة مفاتيح: على Debian وUbuntu، `webkit2gtk-driver` و`xvfb` و`dbus-x11` و`gnome-keyring`.

```sh
bun tauri build --debug --no-bundle
cd e2e
bun install
./run.sh
```

يضمّن البناء الواجهة الأمامية، كما يفعل الإصدار، لذا فإن أمرًا ناقصًا من `commands::builder()`، أو صلاحية ناقصة من `src-tauri/capabilities/`، أو نمطًا تحظره سياسة أمان المحتوى (Content Security Policy) يفشل هنا. يعطي `run.sh` كل تشغيل مجلد بيانات جديدًا وسلسلة مفاتيح وشاشة خاصتين به، فلا يمس أبدًا Homeport الخاص بك. مرّر خيارات WebdriverIO بعده، مثل `./run.sh --spec specs/ssh.e2e.ts`. يبني سيناريو SSH صورة خادم SSH صغيرة من `e2e/support/sshd/`؛ اضبط `HOMEPORT_E2E_SSHD_IMAGE` لاستخدام صورة مبنية مسبقًا. تُسمّى الحاويات التي تشغّلها الاختبارات `homeport-e2e-*` وتُزال بعد ذلك.

يترك الفشل لقطة شاشة وسجل التطبيق في `e2e/artifacts/`. يشغّل CI الاختبارات في مهمة `e2e` من `.github/workflows/ci.yml`، عند كل دفع إلى `dev` وكل طلب سحب، ويحتفظ بذلك المجلد عندما يفشل أحدها.

## هذا الموقع

يوجد الموقع في `website/` ويُبنى باستخدام [Astro](https://astro.build) و[Starlight](https://starlight.astro.build):

```sh
cd website
bun install
bun run dev      # http://localhost:4321/Homeport/
bun run build    # checks types and links, then builds into website/dist/
```

يُنشر على GitHub Pages بواسطة سير العمل **Pages** عند كل دفع إلى `main` يمسّه، وبعد كل إصدار لتبقى روابط التنزيل محدّثة.
