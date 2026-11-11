### The `networks` namespace, in हिन्दी (hi).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/hi/networks.ts:
### change the text there, then run it again.

networks-nav = नेटवर्क
networks-title = नेटवर्क
networks-summary = { $count ->
        [one] { NUM($count) } नेटवर्क
       *[other] { NUM($count) } नेटवर्क
    }
networks-cleanUp = साफ़ करें
networks-create = नेटवर्क बनाएँ
networks-search = नेटवर्क खोजें
networks-empty = कोई नेटवर्क नहीं
networks-column-name = नाम
networks-column-stack = स्टैक
networks-column-driver = ड्राइवर
networks-column-subnet = सबनेट
networks-column-containers = कंटेनर
networks-builtIn = बिल्ट-इन
networks-internal = आंतरिक
networks-inspectOne = { $name } का निरीक्षण करें
networks-removeOne = { $name } हटाएँ
networks-remove-title = नेटवर्क { $name } हटाएँ?
networks-remove-attached = इससे अभी भी { $count ->
        [one] { NUM($count) } कंटेनर जुड़ा है
       *[other] { NUM($count) } कंटेनर जुड़े हैं
    }; पहले उन्हें डिस्कनेक्ट करें।
networks-remove-unused = कोई कंटेनर इसका इस्तेमाल नहीं करता। जो कंटेनर इसके नाम से इसका संदर्भ देते हैं, वे शुरू नहीं हो पाएँगे।
networks-prune-title = इस्तेमाल न होने वाले नेटवर्क हटाएँ?
networks-prune-description = हर वह कस्टम नेटवर्क हटाया जाता है जिससे कोई कंटेनर जुड़ा नहीं है।
networks-connected = कनेक्ट किया गया
networks-disconnected = डिस्कनेक्ट किया गया
networks-removed = { $name } हटाया गया
networks-pruned = { $count ->
        [one] { NUM($count) } नेटवर्क हटाया गया
       *[other] { NUM($count) } नेटवर्क हटाए गए
    }
networks-createDialog-title = नेटवर्क बनाएँ
networks-createDialog-name = नाम
networks-createDialog-driver = ड्राइवर
networks-createDialog-subnet = सबनेट
networks-createDialog-gateway = गेटवे
networks-createDialog-automatic = स्वचालित
networks-createDialog-internal = आंतरिक — बाहरी नेटवर्क तक पहुँच नहीं
networks-createDialog-attachable = जोड़ने योग्य
networks-dialog-containers = कंटेनर
networks-dialog-raw = मूल JSON
networks-dialog-noContainer = कोई कंटेनर जुड़ा नहीं है।
networks-dialog-disconnect = डिस्कनेक्ट करें
networks-dialog-containerToConnect = कनेक्ट करने के लिए कंटेनर
networks-dialog-connectPlaceholder = कोई कंटेनर कनेक्ट करें…
networks-dialog-connect = कनेक्ट करें
