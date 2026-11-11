### The `pods` namespace, in हिन्दी (hi).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/hi/pods.ts:
### change the text there, then run it again.

pods-nav = Pods
pods-title = Pods
pods-summary = { $count ->
        [one] { NUM($count) } Pod
       *[other] { NUM($count) } Pod
    }
pods-search = Pods खोजें
pods-empty = यहाँ कोई Pod नहीं।
pods-column-name = नाम
pods-column-namespace = Namespace
pods-column-status = स्थिति
pods-column-ready = तैयार
pods-column-restarts = रीस्टार्ट
pods-column-node = Node
pods-column-age = उम्र
pods-tab-overview = अवलोकन
pods-tab-conditions = कंडीशन
pods-tab-events = इवेंट
pods-tab-source = YAML
pods-detail-readyOf = { $total } में से { $ready } कंटेनर तैयार
pods-detail-namespace = Namespace
pods-detail-node = Node
pods-detail-ip = Pod IP
pods-detail-hostIp = Node IP
pods-detail-qos = QoS क्लास
pods-detail-serviceAccount = सर्विस अकाउंट
pods-detail-owner = नियंत्रक
pods-detail-age = उम्र
pods-detail-labels = लेबल
pods-detail-annotations = एनोटेशन
pods-container-name = कंटेनर
pods-container-init = init
pods-container-image = इमेज
pods-container-state = स्थिति
pods-container-ready = तैयार
pods-container-isReady = हाँ
pods-container-notReady = नहीं
pods-container-restarts = रीस्टार्ट
pods-container-restartsWhy = { $count } (आखिरी: { $reason })
pods-container-started = शुरू हुआ
pods-container-ports = पोर्ट
pods-state-running = चल रहा है
pods-state-waiting = प्रतीक्षा में
pods-state-terminated = समाप्त
pods-tab-logs = लॉग
pods-logs-container = कंटेनर
pods-logs-initContainer = { $name } (init)
pods-logs-previous = पिछला इंस्टेंस
pods-logs-noRestart = यह रीस्टार्ट नहीं हुआ है, इसलिए कोई पिछला इंस्टेंस नहीं है।
pods-logs-pod = Pod
pods-logs-allPods = { $count ->
        [one] Pod
       *[other] सभी { NUM($count) } Pods
    }
pods-logs-noPods = इस समय यह कोई Pod नहीं चला रहा।
pods-tab-terminal = टर्मिनल
pods-terminal-shell = शेल
pods-terminal-command = कमांड
pods-terminal-connect = कनेक्ट करें
pods-terminal-reconnect = फिर से कनेक्ट करें
pods-terminal-notRunning = यह कंटेनर नहीं चल रहा।
pods-terminal-exited = प्रोसेस बंद हो गई।
pods-terminal-exitedWithCode = प्रोसेस कोड { $code } के साथ बंद हो गई।
pods-column-cpu = CPU
pods-column-memory = मेमोरी
pods-tab-metrics = मेट्रिक्स
pods-metrics-noSample = metrics-server के पास अभी इस Pod का कोई सैंपल नहीं है: हो सकता है यह अभी शुरू हुआ हो, या चल न रहा हो।
