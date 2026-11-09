/** Hindi text of the `settings` namespace. */
import type en from '../en/settings';
import type { Translation } from '../types';

export default {
  'settings.nav': 'सेटिंग्स',
  'settings.title': 'सेटिंग्स',
  'settings.appearance.title': 'रूप-रंग',
  'settings.appearance.theme': 'थीम',
  'settings.theme.system': 'सिस्टम के अनुसार',
  'settings.theme.light': 'हल्की',
  'settings.theme.dark': 'गहरी',
  'settings.appearance.language': 'भाषा',
  'settings.language.system': 'सिस्टम के अनुसार ({language})',
  'settings.desktop.title': 'डेस्कटॉप',
  'settings.desktop.description': 'Homeport इस कंप्यूटर पर कैसे चलता है।',
  'settings.desktop.closeToTray': 'बंद करने पर सिस्टम ट्रे में रखें',
  'settings.desktop.closeToTrayHint':
    'विंडो बंद करने पर Homeport और उसके होस्ट कनेक्शन ट्रे में चलते रहते हैं। ट्रे आइकन से बंद करें। जो डेस्कटॉप ट्रे आइकन छिपाते हैं, जैसे बिना एक्सटेंशन वाला GNOME, उन पर आइकन शायद न दिखे।',
  'settings.desktop.noTray':
    'इस डेस्कटॉप में ऐसा कोई सिस्टम ट्रे नहीं है जिसे Homeport इस्तेमाल कर सके, इसलिए विंडो बंद करने से Homeport बंद हो जाता है।',
  'settings.desktop.startAtLogin': 'लॉगिन पर शुरू करें',
  'settings.desktop.startAtLoginHint':
    'जब आप इस कंप्यूटर में लॉग इन करते हैं तब Homeport शुरू हो जाता है, ताकि वह शुरुआत से ही आपके होस्ट पर नज़र रखे।',
  'settings.desktop.startHidden': 'ट्रे में छिपा हुआ शुरू करें',
  'settings.desktop.startHiddenHint':
    'लॉगिन पर शुरू होने पर Homeport अपनी विंडो खोले बिना सिस्टम ट्रे में रहता है।',
  'settings.desktop.startHiddenNoTray':
    'विंडो खोलने के लिए सिस्टम ट्रे चाहिए, जो इस डेस्कटॉप में नहीं है।',
  'settings.notifications.title': 'सूचनाएँ',
  'settings.notifications.description':
    'Homeport आपको बताता है, तब भी जब उसकी विंडो ट्रे में बंद हो, जब:',
  'settings.notifications.containerDied':
    'कोई कंटेनर त्रुटि के साथ रुक जाए या उसकी मेमोरी खत्म हो जाए',
  'settings.notifications.containerUnhealthy': 'कोई कंटेनर अस्वस्थ हो जाए',
  'settings.notifications.hostUnreachable': 'कोई कनेक्ट किया गया होस्ट पहुँच से बाहर हो जाए',
  'settings.notifications.hostBack': 'पहुँच से बाहर हुआ होस्ट वापस आ जाए',
  'settings.machine.title': 'यह मशीन',
  'settings.machine.description': 'Homeport यहाँ क्या इस्तेमाल कर सकता है।',
  'settings.machine.keychain': 'OS कीचेन',
  'settings.machine.keychainAvailable':
    'उपलब्ध — SSH कुंजियाँ, प्रमाणपत्र, स्टैक के परिवेश और रजिस्ट्री लॉगिन एन्क्रिप्ट करके सहेजे जाते हैं।',
  'settings.machine.keychainUnavailable':
    'उपलब्ध नहीं — SSH कुंजियाँ, TLS प्रमाणपत्र, स्टैक के परिवेश और रजिस्ट्री लॉगिन सहेजे नहीं जा सकते।',
  'settings.machine.compose': 'Compose CLI',
  'settings.machine.composeMissing':
    'स्टैक शुरू और बंद किए जा सकते हैं, लेकिन डिप्लॉय नहीं किए जा सकते।',
  'settings.machine.stackDeploy': 'Docker CLI, Swarm के लिए',
  'settings.machine.stackDeployMissing':
    'स्टैक को Swarm पर डिप्लॉय नहीं किया जा सकता; Swarm से जुड़ा बाकी सब काम करता है।',
  'settings.machine.git': 'git',
  'settings.machine.helm': 'Helm CLI',
  'settings.sync.title': 'सिंक्ड स्टैक',
  'settings.sync.interval': 'रिपॉज़िटरी इतने अंतराल पर जाँचें',
  'settings.sync.minutes': 'मिनट',
  'settings.data.title': 'डेटा',
  'settings.data.description': 'Homeport इस कंप्यूटर पर क्या रखता है।',
  'settings.data.backup': 'बैकअप लें…',
  'settings.data.restore': 'रीस्टोर करें…',
  'settings.data.backupHint':
    'हर होस्ट, स्टैक, रजिस्ट्री लॉगिन, रिपॉज़िटरी और सेटिंग को उनके सीक्रेट के साथ एक पासफ़्रेज़-सुरक्षित फ़ाइल में सहेजें, और उसे इस या किसी दूसरे कंप्यूटर पर रीस्टोर करें।',
  'settings.backup.title': 'Homeport के डेटा का बैकअप लें',
  'settings.backup.description':
    'फ़ाइल में हर होस्ट, क्लस्टर, स्टैक, रजिस्ट्री लॉगिन, रिपॉज़िटरी और सेटिंग, और उनकी कुंजियाँ, प्रमाणपत्र, टोकन, पासवर्ड और परिवेश होते हैं। जिसके पास यह फ़ाइल और इसका पासफ़्रेज़ हो, वह इन्हें इस्तेमाल कर सकता है, इसलिए दोनों को सुरक्षित रखें। इंजन के इवेंट और डिप्लॉय इतिहास शामिल नहीं होते।',
  'settings.backup.passphrase': 'पासफ़्रेज़',
  'settings.backup.passphraseHint': 'कम से कम {min} अक्षर। इसे वापस नहीं पाया जा सकता।',
  'settings.backup.confirmation': 'पासफ़्रेज़ फिर से',
  'settings.backup.problem.short': 'कम से कम {min} अक्षर इस्तेमाल करें।',
  'settings.backup.problem.mismatch': 'दोनों पासफ़्रेज़ अलग हैं।',
  'settings.backup.start': 'बैकअप लें…',
  'settings.backup.saveTitle': 'बैकअप इस रूप में सहेजें',
  'settings.backup.fileType': 'Homeport बैकअप',
  'settings.backup.done':
    '{hosts, plural, one {# होस्ट} other {# होस्ट}} और {stacks, plural, one {# स्टैक} other {# स्टैक}} का बैकअप लिया गया',
  'settings.backup.withoutSecrets':
    'इनका बैकअप इनके सीक्रेट के बिना लिया गया, जो खोले नहीं जा सके:',
  'settings.restore.pick': 'यहाँ से रीस्टोर करें',
  'settings.restore.title': 'Homeport का डेटा रीस्टोर करें',
  'settings.restore.from': '{path} से',
  'settings.restore.open': 'खोलें',
  'settings.restore.madeBy': '{date} को Homeport {version} से बैकअप लिया गया। इसमें है:',
  'settings.restore.holds': 'बैकअप में क्या है',
  'settings.restore.hosts': '{count, plural, one {# होस्ट} other {# होस्ट}}',
  'settings.restore.clusters': '{count, plural, one {# क्लस्टर} other {# क्लस्टर}}',
  'settings.restore.stacks': '{count, plural, one {# स्टैक} other {# स्टैक}}',
  'settings.restore.registries':
    '{count, plural, one {# रजिस्ट्री लॉगिन} other {# रजिस्ट्री लॉगिन}}',
  'settings.restore.repositories': '{count, plural, one {# रिपॉज़िटरी} other {# रिपॉज़िटरी}}',
  'settings.restore.settings': 'आपकी सेटिंग्स',
  'settings.restore.replaces':
    'रीस्टोर करने से आपके अभी के हर होस्ट, स्टैक, रजिस्ट्री लॉगिन, रिपॉज़िटरी और सेटिंग की जगह बैकअप वाले आ जाते हैं, और इंजन के इवेंट व डिप्लॉय इतिहास भुला दिए जाते हैं। पहले मौजूदा डेटा की एक कॉपी Homeport के डेटा फ़ोल्डर में रखी जाती है।',
  'settings.restore.confirm': 'सब कुछ बदलें',
  'settings.restore.done':
    '{hosts, plural, one {# होस्ट} other {# होस्ट}} और {stacks, plural, one {# स्टैक} other {# स्टैक}} रीस्टोर किए गए',
  'settings.data.keepEvents': 'इंजन के इवेंट इतने समय तक रखें',
  'settings.data.days': 'दिन',
  'settings.data.keepEventsHint':
    'इवेंट पेज के लिए हर होस्ट अधिकतम 10,000 इवेंट रखता है। पुराने इवेंट हर घंटे हटाए जाते हैं।',
  'settings.machine.gitMissing':
    'सिंक्ड स्टैक को इसकी ज़रूरत होती है; बाकी हर प्रकार का स्टैक इसके बिना काम करता है।',
  'settings.machine.helmMissing':
    'Helm रिलीज़ की सूची नहीं बन सकती और उन्हें इंस्टॉल नहीं किया जा सकता; क्लस्टर से जुड़ा बाकी सब काम करता है।',
  'settings.about.tagline': 'हर Docker Engine, एक बोर्ड पर।',
  'settings.about.openLogFolder': 'लॉग फ़ोल्डर खोलें',
  'settings.about.licenses': 'लाइसेंस',
  'settings.licenses.title': 'लाइसेंस',
  'settings.licenses.description':
    'Homeport, GNU Affero General Public License संस्करण 3 के तहत मुफ़्त सॉफ़्टवेयर है, जो नीचे दिए गए ओपन-सोर्स पैकेज पर बना है।',
  'settings.licenses.source': 'इस संस्करण का सोर्स कोड इसी लाइसेंस के तहत प्रकाशित है।',
  'settings.licenses.sourceLink': 'v{version} का सोर्स',
  'settings.licenses.thirdParty': 'तृतीय-पक्ष सॉफ़्टवेयर',
  'settings.licenses.missing.title': 'इस बिल्ड में शामिल नहीं',
  'settings.licenses.missing.description':
    'रिलीज़ बिल्ड में ये शामिल होते हैं; डेवलपमेंट बिल्ड के लिए इन्हें जनरेट करने के लिए <code>{command}</code> चलाएँ।',
  'settings.palette.group': 'सेटिंग्स',
  'settings.palette.themeLight': 'थीम: हल्की',
  'settings.palette.themeDark': 'थीम: गहरी',
  'settings.palette.themeSystem': 'थीम: सिस्टम के अनुसार',
} satisfies Translation<typeof en>;
