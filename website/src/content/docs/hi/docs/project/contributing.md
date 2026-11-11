---
title: योगदान देना
description: समस्याओं की रिपोर्ट कैसे करें, सुविधाएँ कैसे सुझाएँ और Homeport में कोड का योगदान कैसे दें।
---

Homeport खुले में [github.com/DamienReichhart/Homeport](https://github.com/DamienReichhart/Homeport) पर विकसित किया जाता है। हर आकार के योगदान का स्वागत है।

## बग की रिपोर्ट करना

इनके साथ [एक issue खोलें](https://github.com/DamienReichhart/Homeport/issues/new/choose):

- आपने क्या किया, आपको क्या उम्मीद थी, और क्या हुआ — सटीक संदेश सबसे ज़्यादा मदद करता है;
- **सेटिंग्स → परिचय → डायग्नोस्टिक्स एक्सपोर्ट करें…** से [डायग्नोस्टिक्स बंडल](/docs/reference/troubleshooting/#बग-रिपोर्ट-के-लिए-डायग्नोस्टिक्स-एक्सपोर्ट-करना): एक zip, जिसमें Homeport का संस्करण और प्लैटफ़ॉर्म, आपके इस्तेमाल किए जा रहे होस्ट, इंजन और क्लस्टर के प्रकार, और सीक्रेट हटाए गए उसके लॉग होते हैं। सहेजने से पहले आप पढ़ते हैं कि इसमें क्या है, और कुछ भी कहीं नहीं भेजा जाता।

अगर Homeport शुरू नहीं होता, तो इसके बजाय उसका संस्करण और अपना ऑपरेटिंग सिस्टम बताएँ, और [लॉग फ़ाइल](/docs/reference/security/#लॉग) से कुछ भी प्रासंगिक।

कृपया सुरक्षा समस्याओं की रिपोर्ट निजी तौर पर, [GitHub की vulnerability reporting](https://github.com/DamienReichhart/Homeport/security/advisories/new) के ज़रिए करें।

## कोई सुविधा सुझाना

एक issue खोलें जिसमें उस समस्या का वर्णन हो जिसे आप हल करवाना चाहते हैं, सिर्फ़ समाधान का नहीं। यह जानना कि आप अपने इंजन के साथ कैसे काम करते हैं, उसके लिए सही रूप ढूँढने में मदद करता है।

## कोड का योगदान देना

1. [`docs/ARCHITECTURE.md`](https://github.com/DamienReichhart/Homeport/blob/main/docs/ARCHITECTURE.md) पढ़ें। सुविधाएँ आत्मनिर्भर vertical slice हैं, फ़्रंटएंड Rust को सिर्फ़ बनाई गई IPC बाइंडिंग के ज़रिए कॉल करता है, और Rust लॉजिक ऐसी सर्विस में रहता है जिनकी यूनिट टेस्टिंग Tauri के बिना होती है।
2. कोड, UI टेक्स्ट और कमिट में [`GLOSSARY.md`](https://github.com/DamienReichhart/Homeport/blob/main/GLOSSARY.md) से प्रोजेक्ट की शब्दावली का इस्तेमाल करें — Host, Stack, Managed, External, Adopt। मुख्य निर्णय [`docs/adr`](https://github.com/DamienReichhart/Homeport/tree/main/docs/adr) में दर्ज हैं।
3. रंगों और लहजे के लिए [ब्रांड गाइड](https://github.com/DamienReichhart/Homeport/blob/main/docs/BRAND.md) का पालन करें: सादा, सटीक, शांत, ब्रिटिश वर्तनी में।
4. पक्का करें कि ये पास होते हैं:

   ```sh
   bun run check
   bun run rust:lint
   bun run rust:test
   ```

5. डेटा का वादा निभाएँ: Homeport का हर संस्करण हर पिछले संस्करण का डेटा खोलता है, 0.1.0 तक, और कुछ भी खोए बिना उसे अपग्रेड करता है: होस्ट, स्टैक, क्लस्टर, सेटिंग्स और सीक्रेट। स्कीमा में कोई भी बदलाव एक नया माइग्रेशन है, किसी रिलीज़ हो चुके माइग्रेशन में संपादन कभी नहीं, और CI इसे हर रिलीज़ के एक नमूना डेटाबेस पर जाँचता है। देखें [Data compatibility](https://github.com/DamienReichhart/Homeport/blob/main/docs/RELEASING.md#data-compatibility)।
6. कमिट संदेश [Conventional Commits](https://www.conventionalcommits.org) के रूप में लिखें — `feat(stacks): …`, `fix(hosts): …`। रिलीज़ नोट उन्हीं से बनाए जाते हैं।
7. `dev` ब्रांच के ख़िलाफ़ एक pull request खोलें। CI Linux पर फ़्रंटएंड जाँच और लाइसेंस जाँच चलाता है, और Linux, macOS और Windows पर `cargo clippy` और `cargo test`, ताकि `#[cfg(windows)]` या `#[cfg(target_os = "macos")]` के पीछे का कोड मर्ज होने से पहले कंपाइल हो जाए। जिस टेस्ट को Docker Engine चाहिए, वह `#[ignore = "needs a local Docker Engine"]` से चिह्नित होता है; एक अलग Linux job उन्हें runner के इंजन पर चलाता है।

सेट अप करने के लिए [सोर्स से बनाना](/docs/project/building/) देखें।

### ऐप में टेक्स्ट जोड़ना

ऐप का टेक्स्ट एक कैटलॉग में रहता है, कभी inline नहीं, ताकि उसका अनुवाद हो सके ([ADR 0011](https://github.com/DamienReichhart/Homeport/blob/main/docs/adr/0011-formatjs-catalogues-and-keyed-errors.md))। एक string जोड़ने के लिए:

1. [`src/shared/i18n/en/`](https://github.com/DamienReichhart/Homeport/tree/main/src/shared/i18n/en) में सुविधा की फ़ाइल में एक key जोड़ें — इमेज सुविधा के लिए `images.ts`, UI kit के लिए `ui.ts`, `common.ts` सिर्फ़ उन शब्दों के लिए जिनका मतलब हर जगह एक ही है (Cancel, Close, Retry)। Key फ़ाइल के नाम से शुरू होती हैं और बताती हैं कि टेक्स्ट किसलिए है: `images.remove.title`, `images.column.size`।
2. मान को [ICU message](https://formatjs.github.io/docs/core-concepts/icu-syntax/) के रूप में लिखें: किसी मान के लिए `{name}`, हाथ से बहुवचन बनाने के बजाय `{count, plural, one {# image} other {# images}}`, `tRich` से रेंडर होने वाले ज़ोर के लिए `<strong>…</strong>`। हर वाक्य के लिए एक key; टुकड़ों को आपस में न जोड़ें, क्योंकि भाषाओं के बीच शब्द-क्रम बदलता है।
3. इसका इस्तेमाल `@/shared/i18n` से `t('images.remove.title', { name })` के साथ करें — कंपोनेंट, mutation और toast, सभी में एक जैसा। Key टाइप की हुई होती हैं, इसलिए कोई टाइपो टाइप जाँच को विफल कर देता है। शाब्दिक key का इस्तेमाल करें: जब टेक्स्ट किसी मान पर निर्भर हो, तो `` `containers.done.${action}` `` बनाने के बजाय मान को key से मैप करें (`{ start: 'containers.done.start', stop: 'containers.done.stop' }`)।

जब कोई कंपोनेंट inline टेक्स्ट लिखता है तो `bun run lint` विफल हो जाता है: JSX टेक्स्ट, वे props जिन्हें लोग पढ़ते हैं (`title`, `aria-label`, `placeholder`, `description`, `label`…), और toast और पुष्टि के टेक्स्ट। जो strings शब्दों के बजाय कोड जैसी पढ़ी जाती हैं — `tcp`, `nginx:latest`, `/dev/dri`, `max-size=10m` — और Docker या Podman जैसे व्यक्तिवाचक नाम पास हो जाते हैं। किसी शाब्दिक string को जान-बूझकर रखने के लिए, उससे पहले की लाइन में कारण बताएँ: `// eslint-disable-next-line homeport/no-literal-ui-string -- a shell command`। `bun run check` कोड में इस्तेमाल हुई लेकिन कैटलॉग में गायब key, किसी काम में न आने वाली key, और रनटाइम पर बनाई गई key पर भी विफल होता है।

त्रुटि संदेश Rust में लिखे जाते हैं: हर crate अपने `messages!` ब्लॉक में अपनी key और अंग्रेज़ी टेम्पलेट घोषित करता है, और `bun run bindings` उन्हें `src/shared/i18n/en/errors.json` में लिखता है। उस फ़ाइल को हाथ से संपादित न करें।

## Homeport का अनुवाद करना

ऐप और वेबसाइट अंग्रेज़ी, फ़्रेंच, स्पैनिश, जर्मन, इतालवी, ब्राज़ीलियाई पुर्तगाली, रूसी, सरलीकृत चीनी, जापानी, कोरियाई, हिन्दी और अरबी बोलते हैं। हर भाषा हर प्रोडक्ट शब्द के लिए एक शब्द इस्तेमाल करती है: अनुवाद करने से पहले उन्हें [`docs/i18n/`](https://github.com/DamienReichhart/Homeport/tree/main/docs/i18n) में देखें, और जब आप कोई नया शब्द चुनें तो वहाँ एक पंक्ति जोड़ें।

### कोई string जोड़ना या बदलना

String चाहे जो भी हो, `bun run check` (और इसलिए CI) तब तक विफल होता है जब तक हर भाषा में वह न हो, उन्हीं `{arguments}` और टैग के साथ, और हर उस बहुवचन रूप के साथ जो उस भाषा को चाहिए।

- **ऐप टेक्स्ट**: `src/shared/i18n/en/<namespace>.ts` में key जोड़ें ([ऐप में टेक्स्ट जोड़ना](#ऐप-में-टेक्स्ट-जोड़ना) देखें), फिर हर दूसरे भाषा फ़ोल्डर (`fr/`, `de/`, `zh-CN/`…) में उसी नाम की फ़ाइल में। हो सके तो उसका अनुवाद करें। अगर न हो सके, तो अंग्रेज़ी टेक्स्ट वहाँ कॉपी करें और pull request में यह बताएँ, ताकि कोई अनुवादक आगे काम कर सके; key को कभी छोड़ें नहीं।
- **कोई त्रुटि संदेश या संकेत**: इसे crate के `messages!` ब्लॉक में घोषित करें और `bun run bindings` चलाएँ, जो `en/errors.json` लिखता है; फिर हर भाषा की `errors.json` में key जोड़ें। ऑपरेटिंग सिस्टम जो टेक्स्ट दिखाता है — ट्रे मेनू और नोटिफ़िकेशन — वह `native.` key और `native.json` के साथ इसी तरह काम करता है।
- **वेबसाइट टेक्स्ट** (होम, डाउनलोड, changelog, हेडर और फ़ुटर): `website/src/core/i18n/strings/en/` की सही फ़ाइल (`site`, `home`, `download` या `changelog`) में और हर दूसरी भाषा के फ़ोल्डर की उसी फ़ाइल में key जोड़ें; किसी में न होने पर `astro check` विफल होता है। दस्तावेज़ के साइडबार का कोई लेबल `website/src/app/config/sidebar/<code>.ts` में जाता है।
- **कोई दस्तावेज़ पेज**: अंग्रेज़ी पेज `website/src/content/docs/docs/` में है। जब आप उसे बदलें, तो उसी pull request में हर भाषा में वही पेज अपडेट करें, `website/src/content/docs/<code>/docs/` के तहत उसी पाथ पर; जिस भाषा का पेज आप अपडेट नहीं कर सकते, वह तब तक अपना पुराना अनुवाद दिखाती है जब तक कोई अनुवादक उसे अपडेट न करे। अनुवादित पेजों के हेडिंग उसी क्रम में और उन्हीं स्तरों पर रखें जैसे अंग्रेज़ी में हैं, और लिंक ठीक वैसे ही जैसे अंग्रेज़ी उन्हें लिखती है, ऐंकर समेत: `bun run build` के बाद, `bun scripts/localize-anchors.ts <code>` हर ऐंकर को अनुवादित हेडिंग की ओर मोड़ता है, और दूसरे बिल्ड की लिंक जाँच इसकी पुष्टि करती है।

किसी एक भाषा को `bun run i18n:check --locale fr` से जाँचें।

### कोई भाषा जोड़ना

फ़ाइलें, और वेबसाइट के लिए एक लाइन:

1. **ऐप**: `src/shared/i18n/en/` को `src/shared/i18n/<tag>/` में कॉपी करें (एक BCP 47 टैग जैसे `pl` या `zh-TW`), उसकी `meta.ts` लिखें (`{ name: 'Polski', dir: 'ltr' }`, भाषा का अपने लिए अपना नाम; दाएँ-से-बाएँ लिपि के लिए `rtl`) और हर फ़ाइल का अनुवाद करें, हर key को रखते हुए और हर `export default { … }` को `export default { … } satisfies Translation<typeof en>` में बदलते हुए, जैसा दूसरी भाषाएँ करती हैं। `bun run i18n:check --locale <tag>` बताता है कि क्या गायब है। सेटिंग्स नई भाषा को सूचीबद्ध करती हैं, और OS की भाषा उसे चुन लेती है, बिना किसी और बदलाव के।
2. **वेबसाइट**: भाषा को `website/src/core/site/locales.ts` में जोड़ें, फिर उसकी strings `website/src/core/i18n/strings/en/` की `<code>/` नाम की कॉपी में (`core/i18n/index.ts` में `TABLES` में रजिस्टर की गई), उसके साइडबार लेबल `website/src/app/config/sidebar/<code>.ts` में (`sidebar.ts` में रजिस्टर किए गए; ये दो लाइनें ही वह एकमात्र कोड हैं जिसकी किसी भाषा को ज़रूरत है, क्योंकि वे फ़ाइलें Vite के बाहर लोड होती हैं, जो फ़ोल्डर ढूँढता है), और बिना अनुवाद वाले पेजों के लिए Starlight का नोट `website/src/content/i18n/<tag>.json` में। उसके पेज, URL, भाषा स्विचर की एंट्री, `hreflang` लिंक और sitemap विकल्प सूची से अपने-आप बनते हैं। उसके दस्तावेज़ `website/src/content/docs/<code>/docs/` के तहत जाते हैं: वहाँ हर पेज का अनुवाद करें (जो पेज न हो, वह एक नोट के साथ अंग्रेज़ी दिखाता है), फिर बिल्ड के बाद `bun scripts/localize-anchors.ts <code>` चलाएँ।
3. **स्क्रीनशॉट**: `bun run screenshots` हर शॉट को हर भाषा में, नई भाषा समेत, `website/src/assets/screenshots/<tag>/` में कैप्चर करता है, ताकि हर पेज ऐप को अपने पाठक की भाषा में दिखाए। जब तक किसी भाषा में कोई ऐसा शॉट नहीं है जो अंग्रेज़ी में है, तब तक वेबसाइट का `bun test` फ़ेल होता है।
4. **शब्द**: `docs/i18n/terms-<tag>.md` जोड़ें।

अगर कोई टेस्ट आपकी अभी जोड़ी गई भाषा को नहीं देखता, तो `node_modules/.vitest-cache` हटा दें: Vitest कैटलॉग फ़ाइलों की एक पुरानी सूची रख सकता है।

## इस दस्तावेज़ को बेहतर बनाना

हर दस्तावेज़ पेज के नीचे एक **पृष्ठ संपादित करें** लिंक है। पेज [`website/src/content/docs`](https://github.com/DamienReichhart/Homeport/tree/main/website/src/content/docs) में Markdown फ़ाइलें हैं।

## रिलीज़ कैसे होती हैं

नए संस्करण संख्या के साथ `main` पर हर push हर प्लैटफ़ॉर्म के लिए इंस्टॉलर बनाता है, पूरा टेस्ट सूट चलाता है, और चेकसम और बिल्ड प्रोवेनेंस के साथ एक GitHub रिलीज़ प्रकाशित करता है। यह प्रक्रिया [`docs/RELEASING.md`](https://github.com/DamienReichhart/Homeport/blob/main/docs/RELEASING.md) में बताई गई है।
