### The `containers` namespace, in हिन्दी (hi).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/hi/containers.ts:
### change the text there, then run it again.

containers-nav = कंटेनर
containers-title = कंटेनर
containers-summary = { $count } में से { $running } चल रहे हैं
containers-runContainer = कंटेनर चलाएँ
containers-search = कंटेनर खोजें
containers-onlyRunning = सिर्फ़ चल रहे
containers-empty-title = कोई कंटेनर नहीं
containers-empty-description = किसी इमेज से एक चलाएँ, या कोई स्टैक डिप्लॉय करें।
containers-noMatch = कोई कंटेनर मेल नहीं खाता।
containers-column-name = नाम
containers-column-image = इमेज
containers-column-status = स्थिति
containers-column-ports = पोर्ट
containers-column-cpu = CPU
containers-column-memory = मेमोरी
containers-column-created = बनाया गया
containers-selectOne = { $name } चुनें
containers-selectStack = स्टैक { $stack } चुनें
containers-stack-running = { $running }/{ $count } चल रहे हैं
containers-stack-swarm = Swarm
containers-stack-open = स्टैक खोलें
containers-inPod = Podman पॉड { $pod } में
containers-fromUnit = Quadlet यूनिट { $unit } द्वारा बनाया गया
containers-selected = { $count } चुने गए
containers-action-start = शुरू करें
containers-action-stop = रोकें
containers-action-restart = फिर से शुरू करें
containers-action-pause = विराम दें
containers-action-resume = जारी रखें
containers-action-kill = बलपूर्वक बंद करें
containers-action-rename = नाम बदलें
containers-action-edit = संपादित करें
containers-action-duplicate = डुप्लिकेट बनाएँ
containers-action-logs = लॉग
containers-action-terminal = टर्मिनल खोलें
containers-action-commit = इमेज में commit करें…
containers-action-export = फ़ाइलसिस्टम एक्सपोर्ट करें…
containers-kill-title = { $name } को बलपूर्वक बंद करें?
containers-kill-description = मुख्य प्रोसेस को तुरंत SIGKILL मिलता है, ठीक से बंद होने का मौका दिए बिना।
containers-done-start = { $count ->
        [one] कंटेनर शुरू हुआ
       *[other] { NUM($count) } कंटेनर शुरू हुए
    }
containers-done-stop = { $count ->
        [one] कंटेनर रुका
       *[other] { NUM($count) } कंटेनर रुके
    }
containers-done-restart = { $count ->
        [one] कंटेनर फिर से शुरू हुआ
       *[other] { NUM($count) } कंटेनर फिर से शुरू हुए
    }
containers-done-pause = { $count ->
        [one] कंटेनर को विराम दिया गया
       *[other] { NUM($count) } कंटेनर को विराम दिया गया
    }
containers-done-unpause = { $count ->
        [one] कंटेनर फिर से जारी हुआ
       *[other] { NUM($count) } कंटेनर फिर से जारी हुए
    }
containers-done-kill = { $count ->
        [one] कंटेनर बलपूर्वक बंद किया गया
       *[other] { NUM($count) } कंटेनर बलपूर्वक बंद किए गए
    }
containers-done-remove = { $count ->
        [one] कंटेनर हटाया गया
       *[other] { NUM($count) } कंटेनर हटाए गए
    }
containers-remove-title = { $name } हटाएँ?
containers-remove-titleMany = { $count ->
        [one] { NUM($count) } कंटेनर
       *[other] { NUM($count) } कंटेनर
    } हटाएँ?
containers-remove-deleted = { $count ->
        [one] कंटेनर और उसकी लिखने योग्य लेयर व अनाम वॉल्यूम मिटा दिए जाते हैं।
       *[other] कंटेनर और उनकी लिखने योग्य लेयर व अनाम वॉल्यूम मिटा दिए जाते हैं।
    }
containers-remove-running = चल रहे { $count } कंटेनर पहले रोके जाएँगे।
containers-remove-namedKept = नाम वाले वॉल्यूम रखे जाते हैं।
containers-detail-gone = यह कंटेनर अब मौजूद नहीं है।
containers-detail-backToList = कंटेनर पर वापस जाएँ
containers-tab-logs = लॉग
containers-tab-inspect = निरीक्षण
containers-tab-health = हेल्थ
containers-tab-exec = Exec
containers-tab-files = फ़ाइलें
containers-tab-stats = आँकड़े
containers-tab-processes = प्रोसेस
containers-notRunning = कंटेनर नहीं चल रहा।
containers-logs-tail-100 = आखिरी 100
containers-logs-tail-1000 = आखिरी 1 000
containers-logs-tail-10000 = आखिरी 10 000
containers-logs-tail-all = सब कुछ
containers-logs-since-all = कभी भी
containers-logs-since-300 = आखिरी 5 मिनट
containers-logs-since-3600 = आखिरी घंटा
containers-logs-since-86400 = आखिरी 24 घंटे
containers-logs-history = इतिहास
containers-logs-sinceLabel = कब से
containers-logs-timestamps = टाइमस्टैम्प
containers-logs-follow = फ़ॉलो करें
containers-logs-find = लॉग में खोजें
containers-logs-previous = पिछला मिलान
containers-logs-next = अगला मिलान
containers-logs-clear = साफ़ करें
containers-logs-download = लॉग डाउनलोड करें
containers-logs-saveTitle = लॉग सहेजें
containers-logs-saved = लॉग सहेजे गए
containers-logs-saving = लॉग सहेजे जा रहे हैं… { $percent }
containers-logs-stdout = stdout
containers-logs-stderr = stderr
containers-logs-end = — लॉग का अंत —
containers-inspect-raw = मूल JSON
containers-inspect-container = कंटेनर
containers-inspect-health = हेल्थ
containers-inspect-environment = परिवेश
containers-inspect-mounts = माउंट
containers-inspect-ports = पोर्ट
containers-inspect-networks = नेटवर्क
containers-inspect-labels = लेबल
containers-inspect-id = ID
containers-inspect-image = इमेज
containers-inspect-created = बनाया गया
containers-inspect-state = स्थिति
containers-inspect-started = शुरू हुआ
containers-inspect-entrypoint = Entrypoint
containers-inspect-command = कमांड
containers-inspect-workingDir = वर्किंग डायरेक्टरी
containers-inspect-user = यूज़र
containers-inspect-restartPolicy = रीस्टार्ट नीति
containers-inspect-networkMode = नेटवर्क मोड
containers-inspect-exited = { $status } (एग्ज़िट { $code })
containers-inspect-outOfMemory = { $state }, मेमोरी खत्म
containers-inspect-restarted = { $policy } · { $count }× रीस्टार्ट हुआ
containers-inspect-restartedMax = { $policy } (अधिकतम { $max }) · { $count }× रीस्टार्ट हुआ
containers-inspect-imageDefault = इमेज का डिफ़ॉल्ट
containers-inspect-notPublished = पब्लिश नहीं किया गया
containers-inspect-readWrite = पढ़ना-लिखना
containers-inspect-readOnly = सिर्फ़ पढ़ना
containers-inspect-failingStreak = लगातार { $count } बार विफल
containers-inspect-noHealthcheck = इमेज कोई हेल्थ चेक तय नहीं करती।
containers-inspect-column-started = शुरू हुआ
containers-inspect-column-exit = एग्ज़िट
containers-inspect-column-output = आउटपुट
containers-inspect-column-variable = वेरिएबल
containers-inspect-column-value = मान
containers-inspect-column-type = प्रकार
containers-inspect-column-source = स्रोत
containers-inspect-column-destination = गंतव्य
containers-inspect-column-mode = मोड
containers-inspect-column-container = कंटेनर
containers-inspect-column-publishedOn = यहाँ पब्लिश
containers-inspect-column-network = नेटवर्क
containers-inspect-column-ip = IP
containers-inspect-column-gateway = गेटवे
containers-inspect-column-aliases = Aliases
containers-inspect-column-label = लेबल
containers-health-none = इस कंटेनर में कोई हेल्थ चेक नहीं है।
containers-health-title = हेल्थ चेक
containers-health-command = कमांड
containers-health-timingLabel = समय
containers-health-timing = हर { $interval }, टाइमआउट { $timeout }, { $retries ->
        [one] { NUM($retries) } बार फिर से कोशिश
       *[other] { NUM($retries) } बार फिर से कोशिश
    }, शुरुआती अवधि { $start }
containers-health-failingStreak = लगातार विफल
containers-health-lastResults = आखिरी नतीजे
containers-health-noRuns = अभी तक कोई जाँच नहीं चली।
containers-health-noOutput = कोई आउटपुट नहीं।
containers-health-showLess = कम दिखाएँ
containers-health-showMore = { $count ->
        [one] { NUM($count) } और पंक्ति दिखाएँ
       *[other] { NUM($count) } और पंक्तियाँ दिखाएँ
    }
containers-health-exit = एग्ज़िट { $code }
containers-health-took = · { $duration } लगे
containers-exec-startFirst = टर्मिनल खोलने के लिए कंटेनर शुरू करें।
containers-exec-shell = शेल
containers-exec-shellNumber = शेल { $number }
containers-exec-closeOne = { $title } बंद करें
containers-exec-new = नया टर्मिनल
containers-exec-command = कमांड
containers-exec-commandPlaceholder = bash, या न हो तो sh
containers-exec-user = यूज़र
containers-exec-directory = डायरेक्टरी
containers-exec-default = डिफ़ॉल्ट
containers-exec-open = खोलें
containers-exec-none = कोई टर्मिनल खुला नहीं है।
containers-exec-exited = [प्रोसेस बंद हो गई]
containers-exec-exitedWithCode = [प्रोसेस कोड { $code } के साथ बंद हो गई]
containers-files-startFirst = इसकी फ़ाइलें देखने के लिए कंटेनर शुरू करें।
containers-files-saveOne = { $name } सहेजें
containers-files-uploadTo = { $path } में अपलोड करें
containers-files-uploadHere = यहाँ अपलोड करें
containers-files-saved = { $name } सहेजा गया
containers-files-savedArchive = { $name } को .tar आर्काइव के रूप में सहेजा गया
containers-files-downloading = { $name } सहेजा जा रहा है…
containers-files-uploading = { $path } में { $count } में से { $done } अपलोड हो रहा है…
containers-files-uploaded = { $path } में { $count ->
        [one] { NUM($count) } फ़ाइल
       *[other] { NUM($count) } फ़ाइलें
    } अपलोड की गईं
containers-stats-startFirst = इसका रिसोर्स इस्तेमाल देखने के लिए कंटेनर शुरू करें।
containers-stats-cpu = CPU
containers-stats-memory = मेमोरी
containers-stats-memoryNow = · { $limit } में से { $used }
containers-stats-used = इस्तेमाल
containers-stats-network = नेटवर्क
containers-stats-networkNow = · { $received } आया, { $sent } गया
containers-stats-received = आया
containers-stats-sent = गया
containers-stats-disk = डिस्क I/O
containers-stats-processes = · { $count ->
        [one] { NUM($count) } प्रोसेस
       *[other] { NUM($count) } प्रोसेस
    }
containers-stats-read = पढ़ा गया
containers-stats-written = लिखा गया
containers-run-title = कंटेनर चलाएँ
containers-run-description = अगर इस होस्ट पर इमेज नहीं है, तो पहले उसे पुल किया जाता है।
containers-run-submit = चलाएँ
containers-run-starting = शुरू हो रहा है…
containers-run-started = कंटेनर शुरू हुआ
containers-run-image = इमेज
containers-run-name = नाम
containers-run-namePlaceholder = खाली होने पर अपने आप बनाया जाएगा
containers-run-nameKept = रखा गया। इसे बदलने के लिए नाम बदलें इस्तेमाल करें।
containers-run-command = कमांड
containers-run-commandPlaceholder = इमेज का डिफ़ॉल्ट
containers-run-environment = परिवेश
containers-run-network = नेटवर्क
containers-run-pod = पॉड
containers-run-noPod = कोई नहीं
containers-run-podOwnsNetwork = { $pod } के कंटेनर उसका नेटवर्क साझा करते हैं: पोर्ट, नेटवर्क, होस्टनेम और DNS पॉड के होते हैं।
containers-run-defaultNetwork = डिफ़ॉल्ट (bridge)
containers-run-restart = रीस्टार्ट
containers-run-restartPolicy = रीस्टार्ट नीति
containers-run-restart-no = कभी नहीं
containers-run-restart-onFailure = विफल होने पर
containers-run-restart-unlessStopped = जब तक रोका न जाए
containers-run-restart-always = हमेशा
containers-run-autoRemove = कंटेनर बंद होने पर उसे हटा दें
containers-run-ports = पोर्ट
containers-run-hostPort = होस्ट पोर्ट
containers-run-hostPortPlaceholder = होस्ट पोर्ट (कोई भी)
containers-run-containerPort = कंटेनर पोर्ट
containers-run-protocol = प्रोटोकॉल
containers-run-removePort = पोर्ट हटाएँ
containers-run-addPort = पोर्ट जोड़ें
containers-run-volumes = वॉल्यूम
containers-run-source = स्रोत
containers-run-sourcePlaceholder = वॉल्यूम का नाम या /host/path
containers-run-containerPath = कंटेनर पाथ
containers-run-readOnly = सिर्फ़ पढ़ना
containers-run-removeVolume = वॉल्यूम हटाएँ
containers-run-addVolume = वॉल्यूम जोड़ें
containers-run-removeValue = { $value } हटाएँ
containers-run-error-size = 512m या 2g जैसा आकार लिखें।
containers-run-error-cpus = 0.5 या 2 जैसी CPU की संख्या लिखें।
containers-run-error-cpuShares = 512 जैसी पूर्ण संख्या लिखें।
containers-run-error-memorySwap = 1g जैसा आकार लिखें, या असीमित swap के लिए -1।
containers-run-error-gpuCount = कितने GPU, यह लिखें, जैसे 1।
containers-run-error-duration = 30s या 1m30s जैसी अवधि लिखें।
containers-run-error-retries = 3 जैसी पूर्ण संख्या लिखें।
containers-advanced-title = उन्नत विकल्प
containers-advanced-hasErrors = त्रुटियाँ हैं
containers-advanced-resources = रिसोर्स
containers-advanced-labels = लेबल
containers-advanced-security = सुरक्षा
containers-advanced-devices = डिवाइस
containers-advanced-healthcheck = हेल्थ चेक
containers-advanced-network = नेटवर्क पहचान
containers-advanced-logging = लॉगिंग
containers-advanced-cpus = CPU सीमा
containers-advanced-cpusHint = CPU, जैसे 0.5 या 2।
containers-advanced-cpusPlaceholder = हर CPU
containers-advanced-cpuShares = CPU shares
containers-advanced-cpuSharesHint = दूसरे कंटेनर के मुकाबले भार; 1024 डिफ़ॉल्ट है।
containers-advanced-memory = मेमोरी सीमा
containers-advanced-memoryHint = जैसे 512m या 2g।
containers-advanced-memoryPlaceholder = कोई सीमा नहीं
containers-advanced-memoryReservation = मेमोरी आरक्षण
containers-advanced-memoryReservationHint = एक नरम सीमा, जो होस्ट में कमी होने पर बनाए रखी जाती है।
containers-advanced-memorySwap = मेमोरी और swap सीमा
containers-advanced-memorySwapHint = मेमोरी और swap मिलाकर; असीमित swap के लिए -1।
containers-advanced-memorySwapPlaceholder = मेमोरी सीमा का दोगुना
containers-advanced-none = कोई नहीं
containers-advanced-labelsHint = हर पंक्ति में एक key=value, जैसे रिवर्स प्रॉक्सी और बैकअप टूल उन्हें पढ़ते हैं।
containers-advanced-capAdd = Capabilities जोड़ें
containers-advanced-capAddHint = जैसे VPN के लिए NET_ADMIN।
containers-advanced-capDrop = Capabilities हटाएँ
containers-advanced-capDropHint = ALL हर एक को हटा देता है।
containers-advanced-privileged = Privileged
containers-advanced-privilegedTitle = कंटेनर को होस्ट तक पूरी पहुँच मिलती है
containers-advanced-privilegedWarning = यह हर डिवाइस और capability इस्तेमाल कर सकता है, और इसमें चलने वाली कोई भी चीज़ होस्ट पर कब्ज़ा कर सकती है। बेहतर है कि इसे जिन capabilities और डिवाइस की ज़रूरत है, वही जोड़ें।
containers-advanced-hostDevice = होस्ट डिवाइस
containers-advanced-containerDevice = कंटेनर डिवाइस
containers-advanced-samePath = वही पाथ
containers-advanced-permissions = अनुमतियाँ
containers-advanced-removeDevice = डिवाइस हटाएँ
containers-advanced-addDevice = डिवाइस जोड़ें
containers-advanced-gpus = GPU
containers-advanced-gpusHint = होस्ट पर GPU निर्माता का container toolkit इंस्टॉल होना चाहिए।
containers-advanced-gpus-all = सभी GPU
containers-advanced-gpus-count = कुछ संख्या में GPU
containers-advanced-gpus-ids = चुने गए GPU
containers-advanced-gpuCount = कितने
containers-advanced-gpuIds = GPU इंडेक्स या UUID
containers-advanced-gpuIdsHint = कॉमा से अलग किए गए।
containers-advanced-health-image = इमेज का अपना, अगर हो
containers-advanced-health-custom = एक कमांड
containers-advanced-health-disabled = कोई नहीं (इमेज वाला बंद करें)
containers-advanced-healthCommand = जाँच कमांड
containers-advanced-healthCommandHint = /bin/sh -c से चलती है; एग्ज़िट कोड 0 का मतलब स्वस्थ है।
containers-advanced-interval = अंतराल
containers-advanced-timeout = टाइमआउट
containers-advanced-retries = फिर से कोशिशें
containers-advanced-startPeriod = शुरुआती अवधि
containers-advanced-hostname = होस्टनेम
containers-advanced-hostnamePlaceholder = कंटेनर ID
containers-advanced-domainname = डोमेन नाम
containers-advanced-dns = DNS सर्वर
containers-advanced-dnsHint = हर पंक्ति में एक पता।
containers-advanced-dnsSearch = DNS खोज डोमेन
containers-advanced-dnsSearchHint = हर पंक्ति में एक।
containers-advanced-extraHosts = अतिरिक्त होस्ट
containers-advanced-extraHostsHint = हर पंक्ति में एक name:address; host-gateway होस्ट के पते को दर्शाता है।
containers-advanced-logDriver = लॉग ड्राइवर
containers-advanced-logDriverHint = none के साथ लॉग टैब खाली रहता है।
containers-advanced-logDriverPlaceholder = इंजन का डिफ़ॉल्ट
containers-advanced-logOptions = ड्राइवर विकल्प
containers-advanced-logOptionsHint = हर पंक्ति में एक key=value; max-size और max-file json-file लॉग को रोटेट करते हैं।
containers-rename-title = { $name } का नाम बदलें
containers-rename-newName = नया नाम
containers-duplicate-title = { $name } का डुप्लिकेट बनाएँ
containers-duplicate-sharedPorts = { $name } पहले से { $count ->
        [one] होस्ट पोर्ट
       *[other] होस्ट पोर्ट
    } { $ports } पब्लिश करता है।
containers-duplicate-sharedPortsFix = { $count ->
        [one] जब तक दोनों इसका इस्तेमाल करते हैं, कॉपी शुरू नहीं हो सकती
       *[other] जब तक दोनों इनका इस्तेमाल करते हैं, कॉपी शुरू नहीं हो सकती
    }: होस्ट पोर्ट बदलें, या इंजन को कोई पोर्ट चुनने देने के लिए इसे खाली छोड़ें।
containers-edit-title = { $name } संपादित करें
containers-edit-description = लागू करने पर कंटेनर इन सेटिंग्स के साथ दोबारा बनाया जाता है, उसका नाम, नेटवर्क और नाम वाले वॉल्यूम रखते हुए।
containers-edit-apply = लागू करें
containers-edit-recreating = दोबारा बनाया जा रहा है…
containers-edit-recreated = कंटेनर दोबारा बनाया गया
containers-edit-confirmTitle = { $name } दोबारा बनाएँ?
containers-edit-nothingLost = और कुछ नहीं खोता: इसका नाम, नेटवर्क और नाम वाले वॉल्यूम रखे जाते हैं।
containers-edit-keptBut = इसका नाम, नेटवर्क और नाम वाले वॉल्यूम रखे जाते हैं, लेकिन ये नहीं:
containers-edit-commitFirst = पहले इमेज में commit करें…
containers-edit-recreate = दोबारा बनाएँ
containers-loss-anonymousVolume = { $path } पर अनाम वॉल्यूम: नया कंटेनर एक खाली वॉल्यूम से शुरू होता है। पुराना एक अप्रयुक्त वॉल्यूम के रूप में रखा जाता है।
containers-loss-filesystemChanges = कंटेनर बनने के बाद से इसके अंदर { $count ->
        [one] { NUM($count) } फ़ाइल बदली
       *[other] { NUM($count) } फ़ाइलें बदलीं
    }। उन्हें रखने के लिए पहले इसे इमेज में commit करें।
containers-loss-downtime = दोनों की अदला-बदली के दौरान यह रुकता है, आम तौर पर कुछ सेकंड के लिए।
containers-loss-stackRedeploy = यह { $stack } स्टैक का हिस्सा है: स्टैक का अगला डिप्लॉय इसे इसकी compose फ़ाइल से, इस संपादन के बिना, दोबारा बनाता है।
containers-commit-title = { $name } को इमेज में commit करें
containers-commit-description = नई इमेज में कंटेनर की फ़ाइलें अभी जैसी हैं वैसी होती हैं, अंदर किए गए बदलावों सहित, और इसकी सेटिंग्स भी। वॉल्यूम शामिल नहीं होते।
containers-commit-imageName = इमेज का नाम
containers-commit-imageNameHint = जैसे repo:tag।
containers-commit-nameTaken = { $name } नाम की इमेज पहले से है। यह नाम नई इमेज पर चला जाता है; पुरानी अपने दूसरे नाम रखती है, या बिना टैग की हो जाती है।
containers-commit-message = संदेश
containers-commit-messagePlaceholder = अपग्रेड से पहले
containers-commit-author = लेखक
containers-commit-changes = बदलाव
containers-commit-changesHint = Dockerfile निर्देश, हर पंक्ति में एक: CMD, ENTRYPOINT, ENV, EXPOSE, LABEL, ONBUILD, USER, VOLUME या WORKDIR।
containers-commit-pause = commit के दौरान कंटेनर को विराम दें
containers-commit-pauseOn = commit में लगने वाले कुछ सेकंड के लिए इसकी प्रोसेस फ़्रीज़ हो जाती हैं, ताकि लिखी जा रही फ़ाइलें सुसंगत रहें।
containers-commit-pauseOff = यह चलता रहता है, इसलिए लिखी जा रही फ़ाइलें अधूरी पकड़ी जा सकती हैं।
containers-commit-submit = Commit करें
containers-commit-committing = Commit हो रहा है…
containers-commit-done = { $reference } में commit किया गया
containers-commit-openImages = इमेज खोलें
containers-export-tarArchive = Tar आर्काइव
containers-export-pick = { $name } का फ़ाइलसिस्टम एक्सपोर्ट करें
containers-export-title = { $name } एक्सपोर्ट हो रहा है
containers-export-to = { $path } में
containers-export-written = लिखा गया
containers-export-finished = हो गया। इसे इमेज पेज पर, या docker import से इम्पोर्ट करें।
containers-export-done = { $destination } में एक्सपोर्ट किया गया
containers-palette-run = कंटेनर चलाएँ
