### The `board` namespace, in हिन्दी (hi).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/hi/board.ts:
### change the text there, then run it again.

board-nav = बोर्ड
board-title = बोर्ड
board-summary = { $hosts ->
        [one] { NUM($hosts) } होस्ट
       *[other] { NUM($hosts) } होस्ट
    } पर { $count ->
        [one] { NUM($count) } कंटेनर
       *[other] { NUM($count) } कंटेनर
    } में से { $running } चल रहे हैं
board-column-host = होस्ट
board-column-stack = स्टैक
board-sortBy = { $column } के अनुसार क्रमबद्ध करें
board-filter-hosts = होस्ट
board-filter-allHosts = सभी होस्ट
board-filter-someHosts = { $count ->
        [one] { NUM($count) } होस्ट
       *[other] { NUM($count) } होस्ट
    }
board-filter-state = स्थिति
board-filter-stack = स्टैक
board-filter-anyStack = कोई भी स्टैक
board-filter-clear = फ़िल्टर हटाएँ
board-state-all = कोई भी स्थिति
board-state-running = चल रहा है
board-state-stopped = रुका हुआ
board-state-unhealthy = अस्वस्थ
board-search = हर होस्ट में खोजें
board-skipped = { $name } ({ $status }) नहीं दिखाया गया है।
board-openHost = होस्ट खोलें
board-listFailed = { $name } के कंटेनर की सूची नहीं बन सकी।
board-noHosts-title = कोई होस्ट कनेक्ट नहीं है
board-noHosts-description = बोर्ड हर कनेक्ट किए गए होस्ट के कंटेनर दिखाता है।
board-empty-title = अभी कोई कंटेनर नहीं
board-empty-description = आपके किसी भी कनेक्ट किए गए होस्ट पर कोई कंटेनर नहीं चल रहा।
board-noMatch = इन फ़िल्टर से कोई कंटेनर मेल नहीं खाता।
board-selection = चुने गए कंटेनर
board-selectAll = दिखाए गए हर कंटेनर को चुनें
board-selectOne = { $host } पर { $name } चुनें
board-remove-title = { $count ->
        [one] { NUM($count) } कंटेनर
       *[other] { NUM($count) } कंटेनर
    } हटाएँ?
board-remove-description = वे अपने अनाम वॉल्यूम के साथ हटा दिए जाते हैं; नाम वाले वॉल्यूम रखे जाते हैं। चल रहे कंटेनर पहले रोके जाते हैं।
board-remove-onHost = { $host } पर: { $names }
