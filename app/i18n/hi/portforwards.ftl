### The `portforwards` namespace, in हिन्दी (hi).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/hi/portforwards.ts:
### change the text there, then run it again.

portforwards-nav = पोर्ट फ़ॉरवर्ड
portforwards-title = पोर्ट फ़ॉरवर्ड
portforwards-summary = { $count ->
        [one] { NUM($count) } फ़ॉरवर्ड
       *[other] { NUM($count) } फ़ॉरवर्ड
    }
portforwards-empty-title = कोई पोर्ट फ़ॉरवर्ड नहीं
portforwards-empty-description = इस कंप्यूटर से किसी Pod या Service तक पहुँचने के लिए उसके पेज से एक पोर्ट फ़ॉरवर्ड करें। Homeport बंद होने पर फ़ॉरवर्ड खत्म हो जाते हैं।
portforwards-column-local = लोकल पता
portforwards-column-target = किस पर फ़ॉरवर्ड
portforwards-column-namespace = Namespace
portforwards-column-cluster = क्लस्टर
portforwards-column-connections = कनेक्शन
portforwards-forward = फ़ॉरवर्ड करें…
portforwards-forwardPort = पोर्ट { $port } फ़ॉरवर्ड करें
portforwards-open = खोलें
portforwards-stop = रोकें
portforwards-stopFailed = Homeport फ़ॉरवर्ड को रोक नहीं सका
portforwards-started = { $address } को { $target }:{ $port } पर फ़ॉरवर्ड किया जा रहा है
portforwards-failed = एक पोर्ट फ़ॉरवर्ड विफल हो गया
portforwards-ended = एक पोर्ट फ़ॉरवर्ड खत्म हो गया
portforwards-dialog-title = { $target }:{ $port } फ़ॉरवर्ड करें
portforwards-dialog-description = Namespace { $namespace } में। फ़ॉरवर्ड चलने के दौरान लोकल पोर्ट पर इस तक पहुँचें।
portforwards-dialog-localPort = लोकल पोर्ट
portforwards-dialog-anyPort = कोई भी खाली पोर्ट
portforwards-dialog-loopback = सिर्फ़ यही कंप्यूटर कनेक्ट कर सकता है: फ़ॉरवर्ड 127.0.0.1 (और ::1) पर सुनता है।
portforwards-dialog-submit = फ़ॉरवर्ड करें
