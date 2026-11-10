---
title: सोर्स से बनाना
description: Homeport को उसके सोर्स कोड से बनाएँ और चलाएँ।
---

Homeport एक [Tauri 2](https://tauri.app) ऐप है: Rust कोर के ऊपर एक React और TypeScript इंटरफ़ेस, जो [bollard](https://github.com/fussybeaver/bollard) के साथ Docker Engine से बात करता है और अपना डेटा SQLite में सहेजता है।

## पूर्व-आवश्यकताएँ

- [Bun](https://bun.sh) 1.4 या बाद का संस्करण
- [Rust](https://www.rust-lang.org/tools/install), stable (रिपॉज़िटरी की `rust-toolchain.toml` कंपोनेंट चुनती है)
- आपके प्लैटफ़ॉर्म के लिए [Tauri सिस्टम डिपेंडेंसी](https://tauri.app/start/prerequisites/)। Debian और Ubuntu पर:

  ```sh
  sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev libsoup-3.0-dev librsvg2-dev \
    libayatana-appindicator3-dev libssl-dev
  ```

- रिलीज़ बिल्ड के लिए, [cargo-about](https://github.com/EmbarkStudios/cargo-about), जो थर्ड-पार्टी लाइसेंस सूचनाएँ बनाता है:

  ```sh
  cargo install cargo-about --locked --features cli
  ```

## इसे चलाएँ

```sh
git clone https://github.com/DamienReichhart/Homeport.git
cd Homeport
bun install
bun run tauri:dev
```

`tauri:dev` इंटरफ़ेस के लिए hot reload के साथ ऐप शुरू करता है; Rust में बदलाव होने पर वह फिर से बनता और रीस्टार्ट होता है।

## इंस्टॉलर बनाएँ

```sh
bun run tauri:build
```

आपके प्लैटफ़ॉर्म के इंस्टॉलर `target/release/bundle/` में लिखे जाते हैं। बिल्ड पहले लाइसेंस सूचनाएँ बनाता है, और अगर किसी डिपेंडेंसी के लाइसेंस की `about.toml` में समीक्षा नहीं हुई है तो रुक जाता है।

लोकल बिल्ड ऐप के अंदर के अपडेटर के लिए साइन नहीं किए जाते और `.rpm` को बिना साइन किए छोड़ते हैं; सिर्फ़ [रिलीज़ वर्कफ़्लो](https://github.com/DamienReichhart/Homeport/blob/main/docs/RELEASING.md) के पास साइनिंग कुंजियाँ हैं। आपकी ख़ुद बनाई कॉपी फिर भी अपडेट जाँचती है, लेकिन उसका **इंस्टॉल करें और रीस्टार्ट करें** सिर्फ़ रिलीज़ वर्कफ़्लो से साइन किए गए संस्करण इंस्टॉल करता है।

## काम के कमांड

| कमांड                 | यह क्या करता है                                                                                                                                                                                                                                          |
| --------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `bun run check`       | टाइपचेक, लिंट, फ़ॉर्मेट जाँच और फ़्रंटएंड टेस्ट                                                                                                                                                                                                          |
| `bun run rust:lint`   | चेतावनियों को त्रुटि मानते हुए `cargo clippy`                                                                                                                                                                                                            |
| `bun run rust:test`   | Rust यूनिट टेस्ट                                                                                                                                                                                                                                         |
| `bun run bindings`    | IPC के लिए TypeScript बाइंडिंग फिर से बनाता है                                                                                                                                                                                                           |
| `bun run licenses`    | थर्ड-पार्टी लाइसेंस सूचनाएँ बनाता है                                                                                                                                                                                                                     |
| `bun run screenshots` | इस वेबसाइट पर इस्तेमाल होने वाले स्क्रीनशॉट हर भाषा में फिर से लेता है (एक भाषा दोबारा लेने के लिए `--locale fr`), 2880×1800 में, यानी दोगुने पिक्सेल घनत्व पर। पहले oxipng इंस्टॉल करें (`cargo install oxipng`) ताकि वे बिना किसी हानि के संपीड़ित हों |

इंटीग्रेशन टेस्ट लोकल Docker Engine और Compose CLI से बात करते हैं। वे `#[ignore = "needs a local Docker Engine"]` (या `"… and the compose CLI"`, और कुछ `"… access to Docker Hub"`) से चिह्नित होते हैं, सिर्फ़ `homeport-test-*` नाम वाले रिसोर्स बनाते हैं और बाद में उन्हें हटा देते हैं। उन्हें इस मशीन पर एक इंजन और compose प्लगइन के साथ चलाएँ:

```sh
cargo test --workspace --exclude homeport -- --ignored
```

CI भी उन्हें चलाता है, `.github/workflows/ci.yml` के `engine-tests` job में, `dev` पर हर push, हर pull request और हर रिलीज़ पर। फिर job `scripts/ci/engine-test-leftovers.sh` चलाता है, जो विफल हो जाता है अगर किसी टेस्ट ने कोई `homeport-test-*` कंटेनर, इमेज, वॉल्यूम या नेटवर्क पीछे छोड़ा हो; यही जाँचने के लिए टेस्ट के बाद इसे लोकल रूप से चलाएँ।

### एंड-टू-एंड टेस्ट

`e2e/` असली ऐप को चलाता है — Rust बैकएंड, webview और उनके बीच की कॉल — [WebdriverIO](https://webdriver.io) के साथ `tauri-driver` के ज़रिए, लोकल Docker Engine पर। परिदृश्य: ऐप शुरू होता है और लोकल इंजन से कनेक्ट होता है; किसी कंटेनर के लॉग स्ट्रीम होते हैं और वह ऐप से रुकता है; एक इमेज पुल की जाती है; एक प्रबंधित स्टैक एन्क्रिप्टेड परिवेश के साथ डिप्लॉय किया जाता है; रीस्टार्ट के बाद होस्ट, स्टैक और उसका सीक्रेट बने रहते हैं; और एक SSH होस्ट मौजूदा कुंजी के साथ जोड़ा जाता है, उसकी सर्वर कुंजी पर भरोसा किया जाता है, और वह कनेक्ट होता है। वे एलिमेंट को role, label और टेक्स्ट से ढूँढ़ते हैं, कभी CSS class से नहीं।

वे सिर्फ़ Linux पर चलते हैं: macOS पर उसके webview के लिए कोई WebDriver नहीं है। [पूर्व-आवश्यकताओं](#prerequisites) और एक Docker Engine के अलावा, उन्हें `tauri-driver` (`cargo install tauri-driver --locked`), WebKit का ड्राइवर, एक वर्चुअल डिस्प्ले और एक keyring चाहिए: Debian और Ubuntu पर, `webkit2gtk-driver`, `xvfb`, `dbus-x11` और `gnome-keyring`।

```sh
bun tauri build --debug --no-bundle
cd e2e
bun install
./run.sh
```

बिल्ड फ़्रंटएंड को एम्बेड करता है, जैसे कोई रिलीज़ करती है, इसलिए `commands::builder()` से छूटा कोई कमांड, `src-tauri/capabilities/` से छूटी कोई अनुमति या Content Security Policy से ब्लॉक होने वाली कोई स्टाइल यहीं विफल होती है। `run.sh` हर रन को एक नया डेटा फ़ोल्डर, उसका अपना keyring और डिस्प्ले देता है, इसलिए यह आपके अपने Homeport को कभी नहीं छूता। WebdriverIO के विकल्प इसके बाद दें, उदाहरण के लिए `./run.sh --spec specs/ssh.e2e.ts`। SSH परिदृश्य `e2e/support/sshd/` से एक छोटी SSH सर्वर इमेज बिल्ड करता है; पहले से बिल्ड की गई इमेज इस्तेमाल करने के लिए `HOMEPORT_E2E_SSHD_IMAGE` सेट करें। टेस्ट जो कंटेनर शुरू करते हैं, उनके नाम `homeport-e2e-*` होते हैं और बाद में हटा दिए जाते हैं।

विफलता `e2e/artifacts/` में एक स्क्रीनशॉट और ऐप का लॉग छोड़ती है। CI टेस्ट को `.github/workflows/ci.yml` के `e2e` job में, `dev` पर हर push और हर pull request पर चलाता है, और कोई टेस्ट विफल होने पर वह फ़ोल्डर रखता है।

## यह वेबसाइट

साइट `website/` में है और [Astro](https://astro.build) और [Starlight](https://starlight.astro.build) से बनी है:

```sh
cd website
bun install
bun run dev      # http://localhost:4321/Homeport/
bun run build    # checks types and links, then builds into website/dist/
```

इसे छूने वाले `main` पर हर push पर, और हर रिलीज़ के बाद ताकि डाउनलोड लिंक अप-टू-डेट रहें, **Pages** वर्कफ़्लो इसे GitHub Pages पर डिप्लॉय करता है।
