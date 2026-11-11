### The `search` namespace, in हिन्दी (hi).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/hi/search.ts:
### change the text there, then run it again.

search-kind-container = कंटेनर
search-kind-stack = स्टैक
search-kind-image = इमेज
search-kind-volume = वॉल्यूम
search-kind-network = नेटवर्क
search-onHostState = { $host } · { $state ->
        [running] चल रहा है
       *[other] रुका हुआ
    }
search-showAll = सभी { $count } दिखाएँ
search-skipped = { $count ->
        [one] { NUM($count) } होस्ट में
       *[other] { NUM($count) } होस्ट में
    } नहीं खोजा गया (कनेक्ट नहीं है)
