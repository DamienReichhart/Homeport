import type { Translation } from '../../types';
import type en from '../en/download';

/** The download page, and the download button other pages show, in Hindi. */
export default {
  'download.title': 'Homeport डाउनलोड करें',
  'download.description':
    'Windows, macOS और Linux के लिए Homeport डाउनलोड करें — x64 और ARM इंस्टॉलर और Linux पैकेज रिपॉज़िटरी, चेकसम और बिल्ड प्रोवेनेंस के साथ।',

  // The download button and the line under it; the notes replace the default once the visitor's system is known.
  'download.button': 'Homeport डाउनलोड करें',
  'download.button.for': '{os} के लिए डाउनलोड करें',
  'download.meta.version': 'संस्करण {version}',
  'download.meta.everywhere': 'Windows, macOS और Linux',
  'download.meta.all': 'सभी डाउनलोड',
  'download.note.windowsArm': 'ARM पर Windows के लिए इंस्टॉलर',
  'download.note.windowsX64': 'Windows x64 इंस्टॉलर',
  'download.note.macosArm': 'Apple silicon वाले Mac के लिए',
  'download.note.macosX64': 'Intel वाले Mac के लिए',
  'download.note.macos': 'Apple silicon और Intel',
  'download.note.linux': '.deb, .rpm और AppImage',

  // The hero.
  'download.hero.eyebrow': 'डाउनलोड',
  'download.hero.title': 'Homeport पाएँ',
  'download.hero.release': 'संस्करण {version}, {date} को जारी हुआ।',
  'download.hero.whatsNew': 'नया क्या है',
  'download.hero.noRelease': 'हर प्लेटफ़ॉर्म के इंस्टॉलर {link} पर हैं।',
  'download.hero.releasesPage': 'GitHub रिलीज़ पेज',
  'download.recommend': 'आप {platform} पर हैं। हमने उसके डाउनलोड सबसे ऊपर रखे हैं।',
  'download.recommend.arch': '{arch} पर {os}',

  // The platforms and their installers.
  'download.preview': '{label} {name} परीक्षण के लिए एक प्रीरिलीज़ है। {link}।',
  'download.preview.label': 'प्रीव्यू उपलब्ध है:',
  'download.preview.link': 'इसे GitHub से डाउनलोड करें',
  'download.windows.requirement': 'Windows 10 या 11',
  'download.macos.requirement': 'macOS 10.15 या उसके बाद का',
  'download.linux.requirement': 'Ubuntu 22.04, Debian 12, Fedora 36 या नया',
  'download.platform.yours': 'आपका सिस्टम',
  'download.platform.guide': '{platform} के लिए इंस्टॉलेशन गाइड',
  'download.arch.appleSilicon': 'Apple silicon',
  'download.arch.intel': 'Intel',
  'download.arch.arm64': 'ARM64',
  'download.arch.x64': 'x64',
  'download.format.nsis.label': 'इंस्टॉलर',
  'download.format.nsis.hint': 'सुझाया गया। मौजूदा यूज़र के लिए इंस्टॉल करता है।',
  'download.format.msi.label': 'MSI पैकेज',
  'download.format.msi.hint': 'प्रबंधित और साइलेंट इंस्टॉल के लिए।',
  'download.format.dmg.label': 'डिस्क इमेज',
  'download.format.dmg.hint': 'Homeport को Applications में खींचें।',
  'download.format.deb.label': 'Debian पैकेज',
  'download.format.deb.hint': 'Debian, Ubuntu, Linux Mint, Pop!_OS।',
  'download.format.rpm.label': 'RPM पैकेज',
  'download.format.rpm.hint': 'Fedora, RHEL, Rocky, openSUSE।',
  'download.format.appimage.label': 'AppImage',
  'download.format.appimage.hint': 'कोई भी डिस्ट्रीब्यूशन, इंस्टॉल करने की ज़रूरत नहीं।',
  'download.files.file': '{format} ({file})',
  'download.files.checksums': 'SHA-256 चेकसम',
  'download.files.none': '{link} देखें।',
  'download.files.latest': 'GitHub पर नवीनतम रिलीज़',

  // The Linux package repository.
  'download.repo.title': 'झटपट इंस्टॉल',
  'download.repo.body':
    'Homeport की पैकेज रिपॉज़िटरी जोड़ता है और Homeport इंस्टॉल करता है, ताकि apt, dnf या zypper उसे अप-टू-डेट रखे।',
  'download.repo.copy': 'कॉपी करें',
  'download.repo.copied': 'कॉपी हो गया',
  'download.repo.script': 'चाहें तो पहले {link}, या रिपॉज़िटरी खुद सेट करें:',
  'download.repo.scriptLink': 'स्क्रिप्ट पढ़ें',
  'download.repo.manual': 'मैन्युअल सेटअप',
  'download.repo.apt': 'Debian 12, Ubuntu 22.04, Linux Mint 21, Pop!_OS 22.04 या नया',
  'download.repo.dnf': 'Fedora 36, RHEL 10, Rocky Linux और AlmaLinux 10 या नया',
  'download.repo.zypper': 'openSUSE Tumbleweed और Leap 16',
  'download.repo.packages':
    'या नीचे से कोई पैकेज डाउनलोड करें। {deb} और {rpm} भी रिपॉज़िटरी जोड़ते हैं, इसलिए उन्हें भी अपडेट मिलते हैं।',

  // What to know around the install.
  'download.before.title': 'इंस्टॉल करने से पहले',
  'download.before.body':
    'Homeport, Docker Engine प्रबंधित करता है — इसमें कोई Docker Engine शामिल नहीं है। इसे Docker Engine, Docker Desktop, Colima, OrbStack, Rancher Desktop या Podman के साथ चलाएँ, या इसे SSH, TLS या TCP से किसी रिमोट सर्वर से जोड़ें।',
  'download.before.keychain':
    '{keychain} (macOS Keychain, Windows Credential Manager, या GNOME Keyring जैसी कोई Secret Service), SSH कुंजियाँ, TLS प्रमाणपत्र, स्टैक के परिवेश और रिपॉज़िटरी की डिप्लॉय कुंजियाँ और टोकन सहेजने के लिए।',
  'download.before.keychainName': 'OS कीचेन',
  'download.before.compose':
    'प्रबंधित, लिंक्ड और सिंक्ड स्टैक डिप्लॉय करने के लिए इस मशीन पर {tool}।',
  'download.before.git':
    'git रिपॉज़िटरी से डिप्लॉय होने वाले सिंक्ड स्टैक के लिए इस मशीन पर {tool}।',
  'download.before.optional':
    'ये सभी वैकल्पिक हैं: इनके बिना क्या उपलब्ध नहीं है, यह Homeport आपको बताता है। {link} देखें।',
  'download.before.requirements': 'सिस्टम आवश्यकताएँ',
  'download.first.title': 'पहली बार खोलना',
  'download.first.body':
    'इंस्टॉलर अभी किसी सशुल्क प्रमाणपत्र से साइन नहीं किए गए हैं, इसलिए आपका सिस्टम एक बार पुष्टि माँग सकता है:',
  'download.first.windows':
    '{os}: SmartScreen “Windows protected your PC” दिखाता है। {moreInfo} चुनें, फिर {runAnyway}।',
  'download.first.moreInfo': 'More info',
  'download.first.runAnyway': 'Run anyway',
  'download.first.macos':
    '{os}: अगर संदेश आए कि Homeport “can’t be opened”, तो उसे {settings} में अनुमति दें और {openAnyway} चुनें।',
  'download.first.settings': 'System Settings → Privacy & Security',
  'download.first.openAnyway': 'Open Anyway',
  'download.first.link': 'चरण-दर-चरण निर्देश',
  'download.updates.title': 'अप-टू-डेट रहना',
  'download.updates.body':
    'Windows और macOS पर, और AppImage के साथ, नया संस्करण आने पर Homeport आपको बताता है और जब आप चुनें तब उसे इंस्टॉल करता है। हर अपडेट साइन किया हुआ होता है और इंस्टॉल होने से पहले जाँचा जाता है।',
  'download.updates.packages':
    '{deb} और {rpm} पैकेज आपके बाकी सिस्टम के साथ, Homeport की पैकेज रिपॉज़िटरी से अपडेट होते हैं।',
  'download.updates.link': 'अपडेट कैसे काम करते हैं',
  'download.verify.title': 'अपना डाउनलोड सत्यापित करें',
  'download.verify.body':
    'हर रिलीज़ GitHub Actions से सार्वजनिक सोर्स कोड से बनाई जाती है, हर फ़ाइल के लिए SHA-256 चेकसम और साइन किए गए बिल्ड प्रोवेनेंस अटेस्टेशन के साथ।',
  'download.verify.link': 'सत्यापित कैसे करें',

  // Other ways to get it.
  'download.other.title': 'Homeport पाने के दूसरे तरीके',
  'download.other.previous.title': 'पिछले संस्करण',
  'download.other.previous.body': 'हर रिलीज़, नोट्स और फ़ाइलों के साथ, GitHub पर।',
  'download.other.build.title': 'सोर्स से बिल्ड करें',
  'download.other.build.body': 'Bun, Rust और Tauri की पूर्व-आवश्यकताएँ।',
  'download.other.source.title': 'सोर्स कोड',
  'download.other.source.body': 'AGPL के तहत ओपन सोर्स, GitHub पर।',
  'download.later.title': 'बाद में आएँगे',
  'download.later.homebrew': 'brew install --cask homeport, macOS पर',
  'download.later.winget': 'winget install, Windows पर',
  'download.later.scoop': 'एक Homeport bucket, Windows पर',
  'download.later.aur': 'homeport-bin, Arch Linux पर',
  'download.later.flathub': 'एक Flatpak, किसी भी Linux डिस्ट्रीब्यूशन पर',
  'download.later.snap': 'Snap Store, Ubuntu और दूसरे सिस्टम पर',
} satisfies Translation<typeof en>;
