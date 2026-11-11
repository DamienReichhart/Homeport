### The `quadlets` namespace, in हिन्दी (hi).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/hi/quadlets.ts:
### change the text there, then run it again.

quadlets-nav = Quadlet
quadlets-title = Quadlet यूनिट
quadlets-summary = { $count ->
        [one] { NUM($count) } यूनिट
       *[other] { NUM($count) } यूनिट
    }
quadlets-search = यूनिट खोजें
quadlets-empty = कोई Quadlet यूनिट नहीं
quadlets-emptyDescription = Quadlet यूनिट वे फ़ाइलें हैं जिन्हें Podman systemd सर्विस में बदलता है, ताकि कंटेनर होस्ट के साथ शुरू हों और systemd उन्हें चलाता रहे। उसे यहाँ देखने के लिए एक यूनिट इंस्टॉल करें।
quadlets-new = नई यूनिट
quadlets-edit = संपादित करें
quadlets-notGenerated = अभी जनरेट नहीं हुई
quadlets-gone = { $name } अब इंस्टॉल नहीं है
quadlets-column-name = यूनिट
quadlets-column-kind = प्रकार
quadlets-column-service = सर्विस
quadlets-column-status = स्थिति
quadlets-column-containers = कंटेनर
quadlets-status-running = चल रही है
quadlets-status-starting = शुरू हो रही है
quadlets-status-stopping = रुक रही है
quadlets-status-stopped = रुकी हुई
quadlets-status-failed = विफल
quadlets-status-notLoaded = systemd ने लोड नहीं की
quadlets-status-template = टेम्पलेट
quadlets-action-start = शुरू करें
quadlets-action-stop = रोकें
quadlets-action-restart = फिर से शुरू करें
quadlets-actionOne-start = { $name } शुरू करें
quadlets-actionOne-stop = { $name } रोकें
quadlets-actionOne-restart = { $name } फिर से शुरू करें
quadlets-done-start = { $name } शुरू हुई
quadlets-done-stop = { $name } रुकी
quadlets-done-restart = { $name } फिर से शुरू हुई
quadlets-installed = { $name } इंस्टॉल की गई
quadlets-saved = { $name } सहेजी गई
quadlets-removed = { $name } हटाई गई
quadlets-blocked-noCommands = Homeport यूनिट को systemctl से शुरू और बंद करता है, जिसे वह सिर्फ़ SSH पर या इसी कंप्यूटर के Podman के लिए चला सकता है।
quadlets-blocked-notLoaded = systemd ने इस यूनिट की सर्विस जनरेट नहीं की है, इसलिए शुरू करने के लिए कुछ नहीं है।
quadlets-blocked-podmanTooOld = यूनिट पढ़ने, इंस्टॉल करने, संपादित करने और हटाने के लिए Podman 5.8 या बाद का संस्करण चाहिए; इस होस्ट पर Podman { $version } चलता है।
quadlets-blocked-noFile = यह यूनिट सिर्फ़ अपने कंटेनर से जानी जाती है, इसलिए इसकी फ़ाइल बदली नहीं जा सकती।
quadlets-note-labels = Podman { $version } Quadlet यूनिट की सूची नहीं दे सकता, इसलिए ये उन कंटेनर से ली गई हैं जो इन्होंने बनाए। इनकी फ़ाइलें, और वे यूनिट जिन्होंने कोई कंटेनर नहीं बनाया, Podman 5.7 और बाद के संस्करणों पर दिखती हैं।
quadlets-note-readOnly = Podman { $version } यूनिट की सूची देता है, लेकिन उन्हें पढ़, इंस्टॉल, संपादित या हटा नहीं सकता; इसके लिए Podman 5.8 या बाद का संस्करण चाहिए।
quadlets-note-noCommands = यूनिट शुरू और बंद करने के लिए इस होस्ट तक SSH से पहुँचना ज़रूरी है, या यह इसी कंप्यूटर का Podman होना चाहिए।
quadlets-detail-service = सर्विस
quadlets-detail-unit = systemd यूनिट
quadlets-detail-scope = इस रूप में चलती है
quadlets-detail-path = फ़ाइल
quadlets-detail-app = एप्लिकेशन
quadlets-detail-pod = पॉड
quadlets-detail-containers = { $count ->
        [0] कंटेनर
        [one] { NUM($count) } कंटेनर
       *[other] { NUM($count) } कंटेनर
    }
quadlets-detail-noContainers = इसने कोई कंटेनर नहीं बनाया है, या इसकी सर्विस रुकी हुई है।
quadlets-scope-user = यूज़र यूनिट (systemctl --user)
quadlets-scope-system = सिस्टम यूनिट
quadlets-file-title = यूनिट फ़ाइल
quadlets-file-unknown = यह यूनिट सिर्फ़ अपने कंटेनर के लेबल से जानी जाती है।
quadlets-file-tooOld = Podman { $version } यूनिट फ़ाइलें नहीं पढ़ सकता; उन्हें दिखाने के लिए Podman 5.8 या बाद का संस्करण चाहिए।
quadlets-form-newTitle = नई Quadlet यूनिट
quadlets-form-newDescription = Podman फ़ाइल को वहाँ लिखता है जहाँ उसके यूज़र की यूनिट रहती हैं, और systemd को फिर से लोड करता है। इंस्टॉल होने के बाद यूनिट शुरू करें।
quadlets-form-editTitle = { $name } संपादित करें
quadlets-form-editDescription = सहेजने से फ़ाइल बदल जाती है और systemd फिर से लोड होता है; चल रही सर्विस इसे इस्तेमाल करे, इसके लिए यूनिट फिर से शुरू करें। इसके साथ इंस्टॉल की गई फ़ाइलें बनी रहती हैं।
quadlets-form-name = फ़ाइल का नाम
quadlets-form-kind = प्रकार
quadlets-form-openFile = फ़ाइल खोलें…
quadlets-form-openTitle = Quadlet यूनिट फ़ाइल खोलें
quadlets-form-unitFiles = Quadlet यूनिट फ़ाइलें
quadlets-form-content = यूनिट फ़ाइल
quadlets-form-extra = इसके साथ की फ़ाइलें
quadlets-form-extraHint = यूनिट जिस Containerfile, kube YAML या env फ़ाइल का इस्तेमाल करती है, वह इसके एप्लिकेशन फ़ोल्डर में इंस्टॉल की जाती है।
quadlets-form-extraTitle = यूनिट के साथ इंस्टॉल करने के लिए फ़ाइलें चुनें
quadlets-form-addExtra = फ़ाइलें जोड़ें…
quadlets-form-removeExtra = { $name } हटाएँ
quadlets-form-install = इंस्टॉल करें
quadlets-form-save = सहेजें
quadlets-form-overwrite = ओवरराइट करें
quadlets-form-nameInvalid = इसका नाम web.container जैसा रखें, जो .container, .pod, .volume, .network, .kube, .image, .build या .artifact पर खत्म हो।
quadlets-form-sectionMissing = Quadlet इसे पढ़ सके, इसके लिए फ़ाइल में { $section } सेक्शन होना चाहिए।
quadlets-form-extraInvalid = यूनिट के साथ की हर फ़ाइल का अपना सादा नाम होना चाहिए, बिना फ़ोल्डर के।
quadlets-remove-title = { $name } हटाएँ?
quadlets-remove-description = Podman यूनिट फ़ाइल मिटाता है और systemd को फिर से लोड करता है। इसके बनाए कंटेनर, वॉल्यूम और इमेज बने रहते हैं।
quadlets-remove-stop = अगर इसकी सर्विस चल रही हो तो उसे रोकें
quadlets-remove-stopRunning = पहले इसकी चल रही सर्विस रोकें
