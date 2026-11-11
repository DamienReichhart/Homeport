### The `podmanpods` namespace, in हिन्दी (hi).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/hi/podmanpods.ts:
### change the text there, then run it again.

podmanpods-nav = पॉड
podmanpods-title = पॉड
podmanpods-summary = { $count ->
        [one] { NUM($count) } पॉड
       *[other] { NUM($count) } पॉड
    }
podmanpods-search = पॉड खोजें
podmanpods-empty = कोई पॉड नहीं
podmanpods-emptyDescription = पॉड उन कंटेनर को समूह में रखता है जो एक नेटवर्क साझा करते हैं और साथ में शुरू और बंद होते हैं। एक पॉड बनाएँ, फिर उसमें कंटेनर चलाएँ।
podmanpods-new = नया पॉड
podmanpods-column-name = नाम
podmanpods-column-status = स्थिति
podmanpods-column-containers = कंटेनर
podmanpods-column-networks = नेटवर्क
podmanpods-column-created = बनाया गया
podmanpods-running = { $count } में से { $running } चल रहे हैं
podmanpods-status-running = चल रहा है
podmanpods-status-degraded = आंशिक रूप से चल रहा है
podmanpods-status-paused = विराम में
podmanpods-status-created = बनाया गया
podmanpods-status-stopped = रुका हुआ
podmanpods-status-failed = विफल
podmanpods-action-start = शुरू करें
podmanpods-action-stop = रोकें
podmanpods-action-restart = फिर से शुरू करें
podmanpods-action-pause = विराम दें
podmanpods-action-unpause = जारी रखें
podmanpods-actionOne-start = { $name } शुरू करें
podmanpods-actionOne-stop = { $name } रोकें
podmanpods-actionOne-restart = { $name } फिर से शुरू करें
podmanpods-actionOne-pause = { $name } को विराम दें
podmanpods-actionOne-unpause = { $name } जारी रखें
podmanpods-removeOne = { $name } हटाएँ
podmanpods-done-start = { $name } शुरू हुआ
podmanpods-done-stop = { $name } रुका
podmanpods-done-restart = { $name } फिर से शुरू हुआ
podmanpods-done-pause = { $name } को विराम दिया गया
podmanpods-done-unpause = { $name } फिर से जारी हुआ
podmanpods-removed = { $name } हटाया गया, { $count ->
        [0] बिना किसी कंटेनर के
        [one] { NUM($count) } कंटेनर के साथ
       *[other] { NUM($count) } कंटेनर के साथ
    }
podmanpods-created = { $name } बनाया गया
podmanpods-remove-title = { $name } हटाएँ?
podmanpods-remove-stopped = { $count ->
        [0] पॉड और उसका infra कंटेनर हटा दिए जाते हैं।
        [one] पॉड, उसका infra कंटेनर और उसका { NUM($count) } कंटेनर हटा दिए जाते हैं।
       *[other] पॉड, उसका infra कंटेनर और उसके { NUM($count) } कंटेनर हटा दिए जाते हैं।
    } उनके वॉल्यूम रखे जाते हैं।
podmanpods-remove-running = { $count ->
        [0] पॉड चल रहा है: उसे रोका जाता है, फिर उसके infra कंटेनर के साथ हटा दिया जाता है।
        [one] पॉड चल रहा है: उसका { NUM($count) } कंटेनर रोका जाता है, फिर पॉड और उसके infra कंटेनर के साथ हटा दिया जाता है।
       *[other] पॉड चल रहा है: उसके { NUM($count) } कंटेनर रोके जाते हैं, फिर पॉड और उसके infra कंटेनर के साथ हटा दिए जाते हैं।
    } उनके वॉल्यूम रखे जाते हैं।
podmanpods-remove-stopAndRemove = रोकें और हटाएँ
podmanpods-detail-containers = { $count ->
        [0] कंटेनर
        [one] { NUM($count) } कंटेनर
       *[other] { NUM($count) } कंटेनर
    }
podmanpods-detail-noContainers = इस पॉड में इसके infra कंटेनर के अलावा अभी कुछ नहीं चल रहा।
podmanpods-detail-infra = Infra
podmanpods-detail-settings = सेटिंग्स
podmanpods-detail-ports = पब्लिश किए गए पोर्ट
podmanpods-detail-noPorts = कोई नहीं
podmanpods-detail-shared = साझा नेमस्पेस
podmanpods-detail-networks = नेटवर्क
podmanpods-detail-hostname = होस्टनेम
podmanpods-detail-created = बनाया गया
podmanpods-detail-labels = लेबल
podmanpods-detail-command = इससे बनाया गया
podmanpods-detail-runContainer = कंटेनर चलाएँ
podmanpods-detail-runContainerIn = { $name } में कंटेनर चलाएँ
podmanpods-newDialog-title = नया पॉड
podmanpods-newDialog-description = इसके कंटेनर एक ही नेटवर्क साझा करते हैं, इसलिए पोर्ट पॉड पर, उन सभी के लिए, पब्लिश किए जाते हैं।
podmanpods-newDialog-name = नाम
podmanpods-newDialog-portsHint = Podman को कोई खाली पोर्ट चुनने देने के लिए होस्ट पोर्ट खाली छोड़ें। पॉड में चलने वाले कंटेनर अपने खुद के पोर्ट पब्लिश नहीं कर सकते।
podmanpods-newDialog-labels = लेबल
podmanpods-newDialog-create = पॉड बनाएँ
podmanpods-form-nameInvalid = किसी अक्षर या अंक से शुरू करें, फिर अक्षर, अंक, _, . या - इस्तेमाल करें।
podmanpods-form-portInvalid = पोर्ट 1 से 65535 तक की संख्याएँ होते हैं।
podmanpods-form-labelInvalid = हर पंक्ति में एक key=value लेबल लिखें।
