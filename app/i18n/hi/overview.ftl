### The `overview` namespace, in हिन्दी (hi).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/hi/overview.ts:
### change the text there, then run it again.

overview-nav = अवलोकन
overview-title = अवलोकन
overview-addHost = होस्ट जोड़ें
overview-addCluster = क्लस्टर जोड़ें
overview-cluster-nodes = { $total } में से { $ready } Node तैयार
overview-cluster-nodesNotAllowed = Nodes: अनुमति नहीं
overview-cluster-pods = { $total } में से { $running } Pod चल रहे हैं
overview-cluster-podsNotAllowed = Pods: अनुमति नहीं
overview-cluster-notAllowed = अनुमति नहीं।
overview-cluster-versionTitle = क्लस्टर
overview-cluster-version = संस्करण
overview-cluster-platform = प्लेटफ़ॉर्म
overview-cluster-server = API सर्वर
overview-nomad-region = Region
overview-nomad-address = पता
overview-cluster-namespace = Namespace
overview-cluster-allNamespaces = सभी Namespace
overview-cluster-nodesTitle = Nodes
overview-cluster-nodesReady = तैयार
overview-cluster-allNodes = सभी Nodes
overview-cluster-podsTitle = Pods
overview-cluster-running = { $count } चल रहे हैं
overview-cluster-pending = { $count } लंबित
overview-cluster-crashing = { $count } विफल हो रहे हैं
overview-cluster-done = { $count } पूरे या अन्य
overview-cluster-allPods = सभी Pods
overview-cluster-unreadyTitle = जो वर्कलोड तैयार नहीं हैं
overview-cluster-allReady = हर Deployment, StatefulSet और DaemonSet के Pods तैयार हैं।
overview-cluster-warningsTitle = हाल की चेतावनियाँ
overview-cluster-noWarnings = कोई चेतावनी इवेंट नहीं।
overview-cluster-allEvents = सभी इवेंट
overview-host-running = <strong>{ $running }</strong> / { $total } चल रहे हैं
overview-host-images = { $count ->
        [one] { NUM($count) } इमेज
       *[other] { NUM($count) } इमेज
    }
overview-host-containerdContainers = { $namespaces ->
        [one] { NUM($namespaces) } नेमस्पेस
       *[other] { NUM($namespaces) } नेमस्पेस
    } में { $count ->
        [one] { NUM($count) } कंटेनर
       *[other] { NUM($count) } कंटेनर
    }
overview-host-incusInstances = { $projects ->
        [one] { NUM($projects) } प्रोजेक्ट
       *[other] { NUM($projects) } प्रोजेक्ट
    } में { $count ->
        [one] { NUM($count) } इंस्टेंस
       *[other] { NUM($count) } इंस्टेंस
    }
overview-incus-title = Incus
overview-incus-version = संस्करण
overview-incus-os = सिस्टम
overview-incus-arch = आर्किटेक्चर
overview-incus-projects = प्रोजेक्ट
overview-incus-shown = दिखाया गया
overview-incus-holds = { $instances ->
        [one] { NUM($instances) } इंस्टेंस
       *[other] { NUM($instances) } इंस्टेंस
    }, { $images ->
        [one] { NUM($images) } इमेज
       *[other] { NUM($images) } इमेज
    }
overview-incus-instances = इंस्टेंस
overview-incus-images = इमेज
overview-incus-storage = स्टोरेज
overview-containerd-title = containerd
overview-containerd-version = संस्करण
overview-containerd-revision = रिविज़न
overview-containerd-platform = प्लेटफ़ॉर्म
overview-containerd-namespaces = नेमस्पेस
overview-containerd-shown = दिखाया गया
overview-containerd-holds = { $containers ->
        [one] { NUM($containers) } कंटेनर
       *[other] { NUM($containers) } कंटेनर
    }, { $images ->
        [one] { NUM($images) } इमेज
       *[other] { NUM($images) } इमेज
    }
overview-containerd-containers = कंटेनर
overview-containerd-images = इमेज
overview-containerd-snapshots = स्नैपशॉट
overview-host-cpu = { $cpus ->
        [one] { NUM($cpus) } CPU
       *[other] { NUM($cpus) } CPU
    } में से { $percent }
overview-host-cpuTitle = कंटेनर का CPU इस्तेमाल, प्रति CPU 100%
overview-host-memoryTitle = कंटेनर का मेमोरी इस्तेमाल
overview-engine-title = इंजन
overview-engine-name = नाम
overview-engine-version = संस्करण
overview-engine-os = OS
overview-engine-kernel = कर्नेल
overview-engine-architecture = आर्किटेक्चर
overview-engine-resources = रिसोर्स
overview-engine-resourcesValue = { $cpus ->
        [one] { NUM($cpus) } CPU
       *[other] { NUM($cpus) } CPU
    } · { $memory }
overview-engine-storageDriver = स्टोरेज ड्राइवर
overview-engine-rootDir = रूट डायरेक्टरी
overview-disk-title = डिस्क इस्तेमाल
overview-disk-cleanUp = साफ़ करें…
overview-disk-column-type = प्रकार
overview-disk-column-total = कुल
overview-disk-column-active = इस्तेमाल में
overview-disk-column-size = आकार
overview-disk-column-reclaimable = खाली की जा सकने वाली जगह
overview-disk-images = इमेज
overview-disk-containers = कंटेनर
overview-disk-volumes = वॉल्यूम
overview-disk-buildCache = बिल्ड कैश
overview-disk-partial = यह Docker Engine संस्करण 29 से पुराना है और कम जानकारी देता है: इमेज के आकार अनुमानित हैं, और वॉल्यूम व बिल्ड कैश के आकार उपलब्ध नहीं हैं।
overview-events-title = हाल के इवेंट
overview-events-all = सभी इवेंट
overview-events-empty = Homeport के कनेक्ट होने के बाद से कुछ नहीं हुआ है।
overview-prune-title = साफ़ करें
overview-prune-description = चुनें कि इस होस्ट से क्या हटाना है। इसे पूर्ववत नहीं किया जा सकता।
overview-prune-confirm = साफ़ करें
overview-prune-confirmWithVolumes = हटाएँ, वॉल्यूम के डेटा सहित
overview-prune-containers = रुके हुए कंटेनर
overview-prune-containers-detail = हर वह कंटेनर हटाता है जो नहीं चल रहा।
overview-prune-images = Dangling इमेज
overview-prune-images-detail = बिल्ड और पुल के बाद बची बिना टैग की लेयर।
overview-prune-allImages = सभी अप्रयुक्त इमेज
overview-prune-allImages-detail = वे टैग की गई इमेज भी हटाता है जिनका कोई कंटेनर इस्तेमाल नहीं करता।
overview-prune-networks = अप्रयुक्त नेटवर्क
overview-prune-networks-detail = कस्टम नेटवर्क जिनसे कोई कंटेनर जुड़ा नहीं है।
overview-prune-buildCache = बिल्ड कैश
overview-prune-buildCache-detail = कैश की गई बिल्ड लेयर।
overview-prune-volumes = अप्रयुक्त वॉल्यूम
overview-prune-volumes-detail = हर उस वॉल्यूम का डेटा मिटाता है जिसका कोई कंटेनर इस्तेमाल नहीं करता, नाम वाले वॉल्यूम भी।
overview-pruned = { $containers ->
        [one] { NUM($containers) } कंटेनर
       *[other] { NUM($containers) } कंटेनर
    }, { $images ->
        [one] { NUM($images) } इमेज
       *[other] { NUM($images) } इमेज
    }, { $networks ->
        [one] { NUM($networks) } नेटवर्क
       *[other] { NUM($networks) } नेटवर्क
    } और { $volumes ->
        [one] { NUM($volumes) } वॉल्यूम
       *[other] { NUM($volumes) } वॉल्यूम
    } हटाए गए — { $space } खाली हुआ।
overview-palette-prune = चुना गया होस्ट साफ़ करें
overview-host-onBoard = बोर्ड पर इसके कंटेनर
