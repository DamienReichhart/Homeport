/** Hindi text of the `app` namespace. */
import type en from '../en/app';
import type { Translation } from '../types';

export default {
  'app.nav.main': 'मुख्य',
  'app.keychainMissing':
    'कोई OS कीचेन उपलब्ध नहीं है, इसलिए SSH कुंजियाँ, TLS प्रमाणपत्र और स्टैक के परिवेश सहेजे नहीं जा सकते। लोकल, TCP और ssh-agent होस्ट फिर भी काम करते हैं।',
  'app.crash.title': 'कुछ गलत हो गया',
  'app.crash.description':
    'इस पेज पर Homeport को एक अनपेक्षित समस्या आई। आम तौर पर फिर से लोड करने से मदद मिलती है; अगर यह बार-बार हो, तो विवरण से रिपोर्ट कॉपी करके एक issue में डालें।',
  'app.crash.reload': 'फिर से लोड करें',
  'app.crash.backToOverview': 'अवलोकन पर वापस जाएँ',
  'app.shortcuts.title': 'कीबोर्ड शॉर्टकट',
  'app.shortcuts.description':
    'यहाँ काम करने वाले शॉर्टकट। जब आप किसी फ़ील्ड में टाइप कर रहे हों, तब N और / जैसी कुंजियाँ काम नहीं करतीं।',
  'app.shortcuts.general': 'सामान्य',
  'app.shortcuts.navigation': 'यहाँ जाएँ',
  'app.shortcuts.hosts': 'होस्ट',
  'app.shortcuts.help': 'कीबोर्ड शॉर्टकट दिखाएँ',
  'app.shortcuts.settings': 'सेटिंग्स खोलें',
  'app.shortcuts.goTo': '{page} पर जाएँ',
  'app.shortcuts.previousHost': 'पिछला होस्ट',
  'app.shortcuts.nextHost': 'अगला होस्ट',
  'app.shortcuts.refresh': 'इस होस्ट की सूचियाँ रीफ़्रेश करें',
  'app.palette.title': 'कमांड पैलेट',
  'app.palette.description': 'किसी पेज पर जाएँ, होस्ट बदलें या कोई कमांड चलाएँ।',
  'app.palette.open': 'कमांड पैलेट खोलें',
  'app.palette.placeholder': 'कोई पेज, होस्ट या कमांड टाइप करें…',
  'app.palette.empty': 'कुछ भी मेल नहीं खाता।',
  'app.palette.recent': 'हाल के',
  'app.palette.goTo': 'यहाँ जाएँ',
  'app.palette.hosts': 'होस्ट',
  'app.palette.actions': 'कार्रवाइयाँ',
  'app.palette.switchTo': '{name} पर जाएँ',
} satisfies Translation<typeof en>;
