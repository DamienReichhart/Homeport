/** Hindi text of the `updates` namespace. */
import type en from '../en/updates';
import type { Translation } from '../types';

export default {
  'updates.title': 'अपडेट',
  'updates.description': 'Homeport शुरू होते समय और हर कुछ घंटों में नया संस्करण खोज सकता है।',
  'updates.automatic': 'अपडेट अपने आप जाँचें',
  'updates.checking': 'जाँच हो रही है…',
  'updates.checkNow': 'अभी जाँचें',
  'updates.status.available': 'Homeport {version} उपलब्ध है (आपके पास {current} है)।',
  'updates.status.latest': 'Homeport {current} नवीनतम संस्करण है।',

  'updates.notice.label': 'अपडेट उपलब्ध है',
  'updates.notice.available': '<strong>Homeport {version}</strong> उपलब्ध है।',
  'updates.notice.availablePackageManager':
    '<strong>Homeport {version}</strong> उपलब्ध है। इसे अपने पैकेज मैनेजर से अपडेट करें:',
  'updates.notice.dismiss': 'खारिज करें',

  'updates.copyCommand': 'कमांड कॉपी करें',
  'updates.whatsNew': 'नया क्या है',
  'updates.install': 'इंस्टॉल करें और रीस्टार्ट करें',
  'updates.download': 'डाउनलोड करें',
  'updates.progress.downloading': 'डाउनलोड हो रहा है…',
  'updates.progress.downloadingPercent': 'डाउनलोड हो रहा है… {percent}%',
  'updates.progress.installing': 'इंस्टॉल हो रहा है…',

  'updates.method.inApp':
    'अपडेट डाउनलोड किए जाते हैं, Homeport की साइनिंग कुंजी से जाँचे जाते हैं और जब आप चुनें तब इंस्टॉल किए जाते हैं।',
  'updates.method.scoop': 'Scoop से इंस्टॉल किया गया: अपडेट Homeport bucket से आते हैं।',
  'updates.method.aur':
    'AUR (homeport-bin) से इंस्टॉल किया गया: आपका AUR helper अपडेट इंस्टॉल करता है।',
  'updates.method.packageManager':
    '{manager} से इंस्टॉल किया गया: अपडेट Homeport पैकेज रिपॉज़िटरी से आते हैं।',
  'updates.method.manual': 'यह एक डेवलपमेंट बिल्ड है: नए संस्करण वेबसाइट से डाउनलोड करें।',
  'updates.palette.check': 'अपडेट जाँचें',
} satisfies Translation<typeof en>;
