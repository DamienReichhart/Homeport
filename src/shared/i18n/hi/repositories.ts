/** Hindi text of the `repositories` namespace. */
import type en from '../en/repositories';
import type { Translation } from '../types';

export default {
  'repositories.title': 'रिपॉज़िटरी',
  'repositories.description':
    'वे git रिपॉज़िटरी जिनसे हर होस्ट पर सिंक्ड स्टैक डिप्लॉय किए जाते हैं। Homeport उन्हें सिर्फ़ पढ़ता है।',
  'repositories.add': 'रिपॉज़िटरी जोड़ें',
  'repositories.empty': 'अभी कोई रिपॉज़िटरी नहीं। सिंक्ड स्टैक बनाने के लिए एक जोड़ें।',
  'repositories.manageOne': '{repository} की पहुँच प्रबंधित करें',
  'repositories.removeOne': '{repository} हटाएँ',
  'repositories.remove.title': '{repository} हटाएँ?',
  'repositories.remove.description':
    'Homeport इसकी डिप्लॉय कुंजी या टोकन भूल जाता है और इस कंप्यूटर पर इसका क्लोन मिटा देता है। रिपॉज़िटरी खुद वैसी ही रहती है। अपने फ़ोर्ज पर भी इससे डिप्लॉय कुंजी हटाएँ।',
  'repositories.noKeychain':
    'डिप्लॉय कुंजियाँ और टोकन OS कीचेन में रखी कुंजी से एन्क्रिप्ट किए जाते हैं, और कोई कीचेन उपलब्ध नहीं है, इसलिए सिर्फ़ सार्वजनिक रिपॉज़िटरी जोड़ी जा सकती हैं।',
  'repositories.public': 'एक सार्वजनिक रिपॉज़िटरी: Homeport इसे बिना क्रेडेंशियल के पढ़ता है।',

  'repositories.added': '{repository} जोड़ी गई',
  'repositories.removed': '{repository} हटाई गई',
  'repositories.keyRotated':
    'नई डिप्लॉय कुंजी बनाई गई। इसे पुरानी कुंजी की जगह रिपॉज़िटरी में जोड़ें।',
  'repositories.tokenReplaced': 'टोकन बदल दिया गया',
  'repositories.hostKeyReset': 'अगली बार सर्वर जो कुंजी दिखाएगा, Homeport उस पर भरोसा करेगा।',

  'repositories.auth.withKey': 'डिप्लॉय कुंजी के साथ, {host} पर',
  'repositories.auth.withToken': '{username} के रूप में टोकन के साथ, {host} पर',
  'repositories.auth.public': 'सार्वजनिक, {host} पर',

  'repositories.dialog.addTitle': 'रिपॉज़िटरी जोड़ें',
  'repositories.dialog.description':
    'सिंक्ड स्टैक डिप्लॉय करने के लिए Homeport इसे इस कंप्यूटर पर क्लोन करता है। यह कभी commit या पुश नहीं करता।',
  'repositories.dialog.url': 'रिपॉज़िटरी URL',
  'repositories.dialog.auth': 'पहुँच',
  'repositories.dialog.authKey': 'डिप्लॉय कुंजी',
  'repositories.dialog.authToken': 'टोकन',
  'repositories.dialog.authNone': 'सार्वजनिक',
  'repositories.dialog.authKeyHelp':
    'Homeport सिर्फ़ इस रिपॉज़िटरी के लिए एक कुंजी बनाता है। आप इसे पढ़ने की पहुँच के साथ रिपॉज़िटरी में जोड़ते हैं; आपकी अपनी SSH कुंजियाँ कभी इस्तेमाल नहीं होतीं।',
  'repositories.dialog.authTokenHelp':
    'एक एक्सेस टोकन जो रिपॉज़िटरी की सामग्री पढ़ सके, HTTPS पर भेजा जाता है। इस कंप्यूटर पर एन्क्रिप्ट किया जाता है और फिर कभी नहीं दिखाया जाता।',
  'repositories.dialog.authNoneHelp':
    'HTTPS पर किसी सार्वजनिक रिपॉज़िटरी के लिए। कुछ भी सहेजा नहीं जाता।',
  'repositories.dialog.username': 'यूज़र नाम',
  'repositories.dialog.token': 'एक्सेस टोकन',
  'repositories.dialog.add': 'जोड़ें',
  'repositories.dialog.addWithKey': 'जोड़ें और डिप्लॉय कुंजी बनाएँ',

  'repositories.missing.url': 'रिपॉज़िटरी का URL डालें।',
  'repositories.missing.validUrl':
    'कोई SSH या HTTPS URL डालें, जैसे git@github.com:owner/repo.git या https://gitlab.com/owner/repo.git।',
  'repositories.missing.auth': 'चुनें कि Homeport इस रिपॉज़िटरी तक कैसे पहुँचे।',
  'repositories.missing.username': 'वह यूज़र नाम डालें जिसका यह टोकन है।',
  'repositories.missing.token': 'एक्सेस टोकन डालें।',

  'repositories.deployKey.title': 'डिप्लॉय कुंजी',
  'repositories.deployKey.github':
    'GitHub: रिपॉज़िटरी की <strong>Settings → Deploy keys → Add deploy key</strong>।',
  'repositories.deployKey.gitlab':
    'GitLab: प्रोजेक्ट की <strong>Settings → Repository → Deploy keys</strong>।',
  'repositories.deployKey.gitea':
    'Gitea और Forgejo: रिपॉज़िटरी की <strong>Settings → Deploy keys → Add deploy key</strong>।',
  'repositories.deployKey.readOnly':
    'लिखने की पहुँच बंद रहने दें: Homeport सिर्फ़ पढ़ता है। फिर नीचे पहुँच की जाँच करें।',
  'repositories.deployKey.fingerprint': 'फ़िंगरप्रिंट: <mono>{fingerprint}</mono>',
  'repositories.rotate.button': 'नई कुंजी',
  'repositories.rotate.title': 'नई डिप्लॉय कुंजी बनाएँ?',
  'repositories.rotate.description':
    'मौजूदा कुंजी Homeport के लिए काम करना बंद कर देगी। नई कुंजी रिपॉज़िटरी में जोड़ें, और वहाँ से पुरानी हटाएँ।',
  'repositories.rotate.confirm': 'नई कुंजी बनाएँ',

  'repositories.token.replace': 'टोकन बदलें',
  'repositories.token.save': 'टोकन सहेजें',

  'repositories.hostKey.trusted': '{host} की भरोसेमंद कुंजी: <mono>{fingerprint}</mono>',
  'repositories.hostKey.reset': 'रीसेट करें',
  'repositories.hostKey.resetTitle': '{host} की भरोसेमंद कुंजी रीसेट करें?',
  'repositories.hostKey.resetDescription':
    'ऐसा सिर्फ़ तभी करें जब {host} ने अपनी SSH कुंजी बदली हो और आपने उसकी वेबसाइट पर नया फ़िंगरप्रिंट जाँच लिया हो। Homeport अगली बार दिखाई गई किसी भी कुंजी पर भरोसा करेगा।',

  'repositories.test.button': 'पहुँच जाँचें',
  'repositories.test.testing': 'जाँच हो रही है…',
  'repositories.test.success':
    'Homeport इसे पढ़ सकता है: {branches, plural, one {# ब्रांच} other {# ब्रांच}} और {tags, plural, one {# टैग} other {# टैग}}।',
} satisfies Translation<typeof en>;
