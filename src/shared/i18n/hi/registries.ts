/** Hindi text of the `registries` namespace. */
import type en from '../en/registries';
import type { Translation } from '../types';

export default {
  'registries.title': 'रजिस्ट्री',
  'registries.description':
    'कंटेनर रजिस्ट्री के लॉगिन, जो हर होस्ट पर निजी इमेज पुल करने, पुश करने और बिल्ड करने के लिए इस्तेमाल होते हैं।',
  'registries.add': 'लॉगिन जोड़ें',
  'registries.noKeychain.title': 'कोई OS कीचेन नहीं',
  'registries.noKeychain.description':
    'रजिस्ट्री के पासवर्ड और टोकन OS कीचेन में रखी कुंजी से एन्क्रिप्ट किए जाते हैं, और कोई कीचेन उपलब्ध नहीं है। इमेज फिर भी अनाम रूप से पुल की जा सकती हैं।',
  'registries.empty': 'अभी कोई लॉगिन नहीं: इमेज अनाम रूप से पुल की जाती हैं।',
  'registries.editOne': '{registry} का लॉगिन संपादित करें',
  'registries.removeOne': '{registry} का लॉगिन हटाएँ',
  'registries.remove.title': '{registry} का लॉगिन हटाएँ?',
  'registries.remove.description':
    'जब तक फिर से लॉगिन नहीं जोड़ा जाता, {registry} की इमेज अनाम रूप से पुल की जाएँगी।',

  'registries.saved': '{registry} का लॉगिन सहेजा गया',
  'registries.removed': '{registry} का लॉगिन हटाया गया',

  'registries.dialog.editTitle': '{registry} का लॉगिन संपादित करें',
  'registries.dialog.addTitle': 'रजिस्ट्री लॉगिन जोड़ें',
  'registries.dialog.description':
    'हर होस्ट पर पुल, पुश और बिल्ड करने के लिए इस्तेमाल होता है। पासवर्ड या टोकन इस कंप्यूटर पर एन्क्रिप्ट किया जाता है और फिर कभी नहीं दिखाया जाता।',
  'registries.dialog.address': 'रजिस्ट्री',
  'registries.dialog.username': 'यूज़र नाम',
  'registries.dialog.secret': 'पासवर्ड या एक्सेस टोकन',
  'registries.dialog.keepSecret': 'सहेजा हुआ रखने के लिए खाली छोड़ें',
  'registries.dialog.accepted': '{registry} ने यह लॉगिन स्वीकार किया।',
  'registries.dialog.checking': 'जाँच हो रही है…',
  'registries.dialog.check': 'लॉगिन जाँचें',

  'registries.missing.address': 'रजिस्ट्री का पता डालें।',
  'registries.missing.username': 'यूज़र नाम डालें।',
  'registries.missing.secret': 'पासवर्ड या एक्सेस टोकन डालें।',

  'registries.login.as': '{registry} पर {username} के रूप में',
  'registries.login.anonymously': 'अनाम रूप से',
  'registries.login.pulledAs': '{registry} पर {username} के रूप में पुल किया गया',
  'registries.login.pulledAnonymously': 'अनाम रूप से पुल किया गया',
  'registries.login.pushedAs': '{registry} पर {username} के रूप में पुश किया गया',
  'registries.login.pushedAnonymously': 'अनाम रूप से पुश किया गया',

  'registries.noLogin.title': '{registry} के लिए कोई लॉगिन नहीं',
  'registries.noLogin.description':
    'ज़्यादातर रजिस्ट्री अनाम पुश अस्वीकार करती हैं। पहले <link>सेटिंग्स → रजिस्ट्री में लॉगिन जोड़ें</link>।',
} satisfies Translation<typeof en>;
