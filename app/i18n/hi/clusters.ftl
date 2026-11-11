### The `clusters` namespace, in हिन्दी (hi).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/hi/clusters.ts:
### change the text there, then run it again.

clusters-title = क्लस्टर
clusters-summary = { $count ->
        [one] { NUM($count) } क्लस्टर
       *[other] { NUM($count) } क्लस्टर
    }
clusters-add = क्लस्टर जोड़ें
clusters-notFound = यह क्लस्टर अब मौजूद नहीं है।
clusters-empty-title = अभी कोई क्लस्टर नहीं
clusters-empty-description = वे Kubernetes क्लस्टर जोड़ें जिन तक आपका kubeconfig पहुँचता है। Homeport उन पर कुछ भी इंस्टॉल नहीं करता।
clusters-column-name = नाम
clusters-column-status = स्थिति
clusters-column-kind = प्रकार
clusters-column-server = API
clusters-column-auth = साइन इन का तरीका
clusters-palette-add = क्लस्टर जोड़ें
clusters-palette-manage = क्लस्टर प्रबंधित करें
clusters-auth-none = कोई क्रेडेंशियल नहीं
clusters-auth-clientCertificate = क्लाइंट प्रमाणपत्र
clusters-auth-token = टोकन
clusters-auth-basic = यूज़र नाम और पासवर्ड
clusters-auth-exec = Exec प्लगइन ({ $command })
clusters-auth-oidc = OpenID Connect
clusters-auth-aclToken = ACL टोकन
clusters-addPage-description = आपका चुना हर Kubernetes context, या आपका बताया हर Nomad agent, एक क्लस्टर बन जाता है। इसके क्रेडेंशियल एन्क्रिप्ट करके सहेजे जाते हैं, डिस्क पर कभी सादे रूप में नहीं।
clusters-addPage-fromMachine = इस मशीन के kubeconfig से
clusters-addPage-paste = kubeconfig पेस्ट करें
clusters-addPage-nomad = Nomad
clusters-paste-description = kubeconfig मेमोरी में पढ़ा जाता है और कभी फ़ाइल के रूप में सहेजा नहीं जाता। जिन फ़ाइलों का यह संदर्भ देता है, उनके पूरे पाथ होने चाहिए।
clusters-paste-label = Kubeconfig
clusters-paste-read = Contexts पढ़ें
clusters-import-readFrom = { $paths } से पढ़ा गया
clusters-import-noContexts = इस kubeconfig में कोई context नहीं है
clusters-import-pick = { $context } इम्पोर्ट करें
clusters-import-name = { $context } से बने क्लस्टर का नाम
clusters-import-current = मौजूदा
clusters-import-added = पहले से { $name } के रूप में जोड़ा गया है।
clusters-import-refresh = इसे रीफ़्रेश करें
clusters-import-submit = { $count ->
        [0] इम्पोर्ट करें
        [one] { NUM($count) } क्लस्टर इम्पोर्ट करें
       *[other] { NUM($count) } क्लस्टर इम्पोर्ट करें
    }
clusters-import-done = { $count ->
        [one] { NUM($count) } क्लस्टर जोड़ा गया
       *[other] { NUM($count) } क्लस्टर जोड़े गए
    }
clusters-import-problem-empty = हर क्लस्टर को एक नाम दें।
clusters-import-problem-duplicate = दो क्लस्टर का नाम “{ $name }” हो जाएगा।
clusters-import-problem-taken = “{ $name }” नाम का क्लस्टर पहले से मौजूद है।
clusters-detail-enabled = सक्षम
clusters-detail-reconnect = फिर से कनेक्ट करें
clusters-detail-name = नाम
clusters-detail-rename = नाम बदलें
clusters-detail-renamed = क्लस्टर का नाम बदला गया
clusters-connection-title = कनेक्शन
clusters-connection-server = API सर्वर
clusters-connection-address = पता
clusters-connection-region = Region
clusters-connection-customCa = इसका अपना CA प्रमाणपत्र
clusters-connection-clientCertificate = क्लाइंट प्रमाणपत्र
clusters-connection-checkedAs = प्रमाणपत्र { $name } के रूप में जाँचा गया
clusters-connection-version = संस्करण
clusters-connection-platform = प्लेटफ़ॉर्म
clusters-connection-namespace = डिफ़ॉल्ट Namespace
clusters-connection-auth = साइन इन का तरीका
clusters-connection-tlsServerName = प्रमाणपत्र का नाम
clusters-connection-tls = TLS
clusters-connection-insecure = सर्वर का प्रमाणपत्र जाँचा नहीं जाता (insecure-skip-tls-verify)
clusters-connection-proxy = प्रॉक्सी
clusters-connection-source = यहाँ से इम्पोर्ट किया गया
clusters-source-machine = इस मशीन के kubeconfig का context { $context }
clusters-source-pasted = पेस्ट किए गए kubeconfig का context { $context }
clusters-nomad-description = Homeport इस कंप्यूटर से किसी Nomad agent, server या client के HTTP API से बात करता है। क्लस्टर पर कुछ भी इंस्टॉल नहीं होता, और यहाँ nomad CLI की ज़रूरत नहीं है।
clusters-nomad-name = नाम
clusters-nomad-address = पता
clusters-nomad-addressHelp = agent का HTTP API, जैसा NOMAD_ADDR इसे nomad CLI को देता है। कोई पोर्ट न दिया जाए तो 4646।
clusters-nomad-region = Region
clusters-nomad-regionHelp = agent के अपने region का इस्तेमाल करने के लिए इसे खाली छोड़ें।
clusters-nomad-token = ACL टोकन
clusters-nomad-tokenHelp = टोकन का secret ID, जब क्लस्टर पर ACL सक्षम हों। Homeport के दस्तावेज़ इसके लिए एक policy देते हैं।
clusters-nomad-tls = TLS सेटिंग्स
clusters-nomad-tlsHelp = सिर्फ़ ऐसे agent के लिए जो अपने CA के साथ HTTPS देता है, या क्लाइंट प्रमाणपत्र माँगता है। PEM फ़ाइलों की सामग्री पेस्ट करें।
clusters-nomad-ca = CA प्रमाणपत्र
clusters-nomad-caHelp = वह CA जिसने agent के प्रमाणपत्र पर हस्ताक्षर किए, जैसा NOMAD_CACERT इसे बताता है। इस कंप्यूटर के CA पर इसके बिना भी भरोसा किया जाता है।
clusters-nomad-clientCertificate = क्लाइंट प्रमाणपत्र
clusters-nomad-clientKey = क्लाइंट कुंजी
clusters-nomad-tlsServerName = TLS सर्वर नाम
clusters-nomad-tlsServerNameHelp = वह नाम जिससे agent का प्रमाणपत्र जाँचा जाता है, जब वह पते वाला नाम न हो, जैसा NOMAD_TLS_SERVER_NAME इसे सेट करता है।
clusters-nomad-keep = सहेजा गया; इसे रखने के लिए खाली छोड़ें
clusters-nomad-willBeRemoved = सहेजने पर हटा दिया जाएगा
clusters-nomad-remove = हटाएँ
clusters-nomad-keepIt = इसे रखें
clusters-nomad-test = कनेक्शन जाँचें
clusters-nomad-testing = जाँच हो रही है…
clusters-nomad-testedVersion = Nomad { $version }, region { $region } से कनेक्ट हुआ।
clusters-nomad-testedNoVersion = region { $region } से कनेक्ट हुआ। टोकन agent को नहीं पढ़ सकता, इसलिए उसका संस्करण ज्ञात नहीं है।
clusters-nomad-add = क्लस्टर जोड़ें
clusters-nomad-added = { $name } जोड़ा गया
clusters-nomad-settings = सेटिंग्स
clusters-nomad-settingsDescription = सहेजने पर फिर से कनेक्ट होता है। टोकन और TLS कुंजी फिर कभी नहीं दिखाए जाते: उन्हें रखने के लिए खाली छोड़ें।
clusters-nomad-save = बदलाव सहेजें
clusters-nomad-saved = क्लस्टर सहेजा गया
clusters-refresh-title = kubeconfig से रीफ़्रेश करें
clusters-refresh-machine = इस मशीन के kubeconfig से context { $context } फिर से पढ़ता है, जब क्लस्टर दोबारा बनाया गया हो या उसके क्रेडेंशियल नवीनीकृत हुए हों।
clusters-refresh-pasted = context { $context } नए सिरे से पढ़ने के लिए kubeconfig फिर से पेस्ट करें, जब क्लस्टर दोबारा बनाया गया हो या उसके क्रेडेंशियल नवीनीकृत हुए हों।
clusters-refresh-submit = रीफ़्रेश करें
clusters-refresh-done = क्लस्टर उसके kubeconfig से रीफ़्रेश किया गया
clusters-remove-title = क्लस्टर हटाएँ
clusters-remove-description = Homeport क्लस्टर और उसके सहेजे गए क्रेडेंशियल भूल जाता है। क्लस्टर पर या आपके kubeconfig में कुछ नहीं बदलता।
clusters-remove-confirmTitle = { $name } हटाएँ?
clusters-remove-confirmDescription = आप इसे इसके kubeconfig से फिर से जोड़ सकते हैं।
