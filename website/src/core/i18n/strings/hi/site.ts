import type { Translation } from '../../types';
import type en from '../en/site';

/** The layout, the header and the footer, in Hindi. */
export default {
  // The layout and the facts every page repeats.
  'site.tagline': 'हर Docker Engine, एक ही बोर्ड पर।',
  'site.description':
    'Homeport कहीं भी चल रहे Docker Engine प्रबंधित करने के लिए एक मुफ़्त, ओपन-सोर्स डेस्कटॉप ऐप है — इस मशीन का इंजन, या SSH, TLS या TCP से जुड़े रिमोट इंजन — Windows, macOS और Linux के लिए।',
  'site.ogImageAlt': 'Homeport — हर Docker Engine, एक ही बोर्ड पर।',
  'site.skipToContent': 'सामग्री पर जाएँ',

  // The header, the language menu and the theme switch.
  'header.home': 'Homeport होम',
  'header.logo': 'Homeport',
  'header.nav': 'मुख्य',
  'header.features': 'सुविधाएँ',
  'header.docs': 'दस्तावेज़',
  'header.changelog': 'चेंजलॉग',
  'header.download': 'डाउनलोड',
  'header.github': 'GitHub पर Homeport',
  'header.githubShort': 'GitHub',
  'header.menu': 'मेन्यू',
  'language.label': 'भाषा',
  'theme.toLight': 'लाइट थीम पर जाएँ',
  'theme.toDark': 'डार्क थीम पर जाएँ',

  // The footer.
  'footer.product': 'प्रोडक्ट',
  'footer.features': 'सुविधाएँ',
  'footer.download': 'डाउनलोड',
  'footer.changelog': 'चेंजलॉग',
  'footer.releases': 'सभी रिलीज़',
  'footer.documentation': 'दस्तावेज़',
  'footer.introduction': 'परिचय',
  'footer.installation': 'इंस्टॉलेशन',
  'footer.hosts': 'होस्ट कनेक्ट करना',
  'footer.troubleshooting': 'समस्या निवारण',
  'footer.project': 'प्रोजेक्ट',
  'footer.source': 'सोर्स कोड',
  'footer.issue': 'समस्या की रिपोर्ट करें',
  'footer.contributing': 'योगदान',
  'footer.security': 'सुरक्षा',
  'footer.legal': 'कानूनी',
  'footer.legalNotice': 'कानूनी सूचना',
  'footer.privacy': 'गोपनीयता',
  'footer.terms': 'शर्तें',
  'footer.cookies': 'कुकीज़',
  'footer.licence': '© {year} Damien Reichhart। Homeport, {licence} के तहत ओपन सोर्स है।',
  'footer.licenceName': 'GNU AGPL, संस्करण 3',
  'footer.trademark':
    'Docker, Docker, Inc. का ट्रेडमार्क है। Homeport एक स्वतंत्र प्रोजेक्ट है, जो न तो Docker, Inc. से जुड़ा है और न ही उसके द्वारा समर्थित है।',
} satisfies Translation<typeof en>;
