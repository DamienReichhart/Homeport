### The `config` namespace, in हिन्दी (hi).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/hi/config.ts:
### change the text there, then run it again.

config-nav = कॉन्फ़िगरेशन
config-title = कॉन्फ़िगरेशन
config-configMaps = ConfigMaps
config-secrets = Secrets
config-configMap = ConfigMap
config-secret = Secret
config-configMapsSummary = { $count ->
        [one] { NUM($count) } ConfigMap
       *[other] { NUM($count) } ConfigMap
    }
config-secretsSummary = { $count ->
        [one] { NUM($count) } Secret
       *[other] { NUM($count) } Secret
    }
config-searchConfigMaps = ConfigMaps खोजें
config-searchSecrets = Secrets खोजें
config-column-name = नाम
config-column-namespace = Namespace
config-column-type = प्रकार
config-column-keys = कुंजियाँ
config-column-age = उम्र
config-tab-data = डेटा
config-tab-overview = अवलोकन
config-tab-events = इवेंट
config-tab-source = YAML
config-detail-namespace = Namespace
config-detail-type = प्रकार
config-detail-labels = लेबल
config-detail-annotations = एनोटेशन
config-noKeys = कोई कुंजी नहीं।
config-binary = (बाइनरी डेटा)
config-base64 = base64
config-reveal = दिखाएँ
config-revealOne = { $key } दिखाएँ
config-hide = छिपाएँ
config-copy = { $key } कॉपी करें
config-reveal-failed = Homeport { $key } नहीं दिखा सका
