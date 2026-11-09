/** Hindi text of the `images` namespace. */
import type en from '../en/images';
import type { Translation } from '../types';

export default {
  'images.nav': 'इमेज',
  'images.title': 'इमेज',
  'images.summary': '{count, plural, one {# इमेज} other {# इमेज}} · {size}',
  'images.cleanUp': 'साफ़ करें',
  'images.cleanUp.dangling': 'Dangling इमेज',
  'images.cleanUp.all': 'सभी अप्रयुक्त इमेज',
  'images.saveSelected': '{count, plural, one {इमेज सहेजें…} other {# इमेज सहेजें…}}',
  'images.load': 'लोड करें…',
  'images.importTar': 'tar से इम्पोर्ट करें…',
  'images.build': 'बिल्ड करें',
  'images.pull': 'इमेज पुल करें',
  'images.search': 'इमेज खोजें',
  'images.empty.title': 'कोई इमेज नहीं',
  'images.empty.description': 'शुरू करने के लिए किसी रजिस्ट्री से एक इमेज पुल करें।',

  'images.column.name': 'नाम',
  'images.column.id': 'ID',
  'images.column.created': 'बनाई गई',
  'images.column.size': 'आकार',
  'images.column.status': 'स्थिति',
  'images.moreTags': '+{count} और टैग',
  'images.inUse': 'इस्तेमाल में · {count}',
  'images.unused': 'अप्रयुक्त',
  'images.runOne': '{name} चलाएँ',
  'images.action.inspect': 'निरीक्षण करें',
  'images.action.tag': 'टैग करें',
  'images.action.push': 'पुश करें',
  'images.action.save': 'सहेजें…',

  'images.remove.title': '{name} हटाएँ?',
  'images.remove.everyName':
    'इससे इमेज के सभी नाम ({names}) और उसकी वे लेयर हट जाती हैं जिनका कोई और इस्तेमाल नहीं करता।',
  'images.remove.description':
    'इमेज और वे लेयर जिनका कोई दूसरी इमेज इस्तेमाल नहीं करती, हटा दी जाती हैं। इसे बाद में फिर से पुल किया जा सकता है।',
  'images.prune.allTitle': 'सभी अप्रयुक्त इमेज हटाएँ?',
  'images.prune.danglingTitle': 'Dangling इमेज हटाएँ?',
  'images.prune.allDescription':
    'हर वह इमेज हटा दी जाती है जिसका कोई कंटेनर इस्तेमाल नहीं करता, टैग की गई इमेज भी।',
  'images.prune.danglingDescription': 'बिल्ड और पुल के बाद बची बिना टैग की इमेज हटा दी जाती हैं।',

  'images.removed': '{name} हटाई गई',
  'images.pruned':
    '{count, plural, one {# इमेज हटाई गई} other {# इमेज हटाई गईं}}, {space} खाली हुआ',
  'images.pulled': '{reference} पुल की गई',
  'images.pushed': '{reference} पुश की गई',
  'images.built': '{name} बिल्ड की गई',
  'images.builtUnnamed': 'इमेज बिल्ड की गई',
  'images.saved': '{path} सहेजा गया',
  'images.loadedOne': '{name} लोड की गई',
  'images.loaded': '{count, plural, one {# इमेज लोड की गई} other {# इमेज लोड की गईं}}',
  'images.imported': '{name} इम्पोर्ट की गई',

  'images.files.save': 'इमेज इस रूप में सहेजें',
  'images.files.load': 'इमेज यहाँ से लोड करें',
  'images.files.import': 'फ़ाइलसिस्टम यहाँ से इम्पोर्ट करें',
  'images.files.imageArchives': 'इमेज आर्काइव',
  'images.files.imageArchive': 'इमेज आर्काइव',
  'images.files.filesystemArchives': 'फ़ाइलसिस्टम आर्काइव',

  'images.buildDialog.title': 'इमेज बिल्ड करें',
  'images.buildDialog.description':
    'इस कंप्यूटर के किसी फ़ोल्डर से। Homeport इसे होस्ट को भेजता है, उसे छोड़कर जो इसकी .dockerignore बाहर रखती है, और होस्ट का इंजन इसे बिल्ड करता है।',
  'images.buildDialog.built': '✓ बिल्ड हो गई',
  'images.buildDialog.folder': 'फ़ोल्डर',
  'images.buildDialog.chooseFolder': 'बिल्ड करने के लिए फ़ोल्डर चुनें',
  'images.buildDialog.choose': 'चुनें…',
  'images.buildDialog.dockerfile': 'Dockerfile',
  'images.buildDialog.target': 'टारगेट स्टेज',
  'images.buildDialog.targetPlaceholder': 'आखिरी स्टेज',
  'images.buildDialog.names': 'नाम',
  'images.buildDialog.buildArgs': 'बिल्ड आर्ग्युमेंट',
  'images.buildDialog.buildArgsPlaceholder': 'VERSION=1.0, हर पंक्ति में एक',
  'images.buildDialog.noCache': 'बिल्ड कैश का इस्तेमाल न करें',
  'images.buildDialog.pull': 'नई बेस इमेज पुल करें',
  'images.buildForm.noFolder': 'बिल्ड करने के लिए फ़ोल्डर चुनें।',
  'images.buildForm.argNeedsName': '“{line}” में “=” से पहले एक नाम चाहिए।',

  'images.hub.unavailable':
    'अभी Docker Hub में खोज नहीं हो सकती: {error} nginx:1.27 जैसा पूरा नाम फिर भी पुल होता है।',
  'images.hub.noMatch': 'Docker Hub पर “{term}” से कुछ भी मेल नहीं खाता।',
  'images.hub.results': 'Docker Hub के नतीजे',
  'images.hub.result': '{name}, {stars} स्टार',
  'images.hub.resultOfficial': '{name}, आधिकारिक इमेज, {stars} स्टार',
  'images.hub.official': 'आधिकारिक',
  'images.hub.backToSearch': 'खोजें',
  'images.hub.tagCount': '{count, plural, one {# टैग} other {# टैग}}',
  'images.hub.filterTags': 'टैग फ़िल्टर करें',
  'images.hub.tags': 'टैग',
  'images.hub.noTagMatch': '“{filter}” से कोई टैग मेल नहीं खाता।',
  'images.hub.moreTags': 'और टैग',

  'images.importDialog.title': 'फ़ाइलसिस्टम को इमेज के रूप में इम्पोर्ट करें',
  'images.importDialog.description':
    '{path} एक-लेयर वाली इमेज बन जाता है जिसका अपना कोई इतिहास या सेटिंग नहीं होती; इसे चलाने योग्य बनाने के लिए नीचे एक कमांड दें।',
  'images.importDialog.name': 'इमेज का नाम',
  'images.importDialog.changes': 'बदलाव',
  'images.importDialog.changesHint':
    'Dockerfile निर्देश, हर पंक्ति में एक: CMD, ENTRYPOINT, ENV, EXPOSE, LABEL, ONBUILD, USER, VOLUME या WORKDIR।',
  'images.importDialog.import': 'इम्पोर्ट करें',

  'images.inspect.layers': 'लेयर',
  'images.inspect.raw': 'मूल JSON',
  'images.inspect.created': 'बनाई गई',
  'images.inspect.instruction': 'निर्देश',
  'images.inspect.size': 'आकार',

  'images.pullDialog.title': 'इमेज पुल करें',
  'images.pullDialog.description':
    'Docker Hub में नाम से खोजें, या कोई भी रेफ़रेंस टाइप करें जिस तक होस्ट पहुँच सके, जैसे nginx:1.27 या ghcr.io/org/app:v2।',
  'images.pullDialog.placeholder': 'postgres, या nginx:1.27',
  'images.pullDialog.reference': 'इमेज रेफ़रेंस',
  'images.pullDialog.pull': 'पुल करें',
  'images.done': 'हो गया।',

  'images.pushDialog.title': '{name} पुश करें',
  'images.pushDialog.description':
    'उस रजिस्ट्री में जिससे इसका नाम शुरू होता है, जैसे ghcr.io/org/app:v2, या org/app:v2 जैसे नामों के लिए Docker Hub में।',
  'images.pushDialog.target': 'पुश करने के लिए नाम',
  'images.pushDialog.push': 'पुश करें',
  'images.pushDialog.tagsFirst': 'पहले इमेज को यह नाम दिया जाता है, जैसा टैग करें करता है।',
  'images.pushDialog.uploading': '{current} / {total} अपलोड हो रहा है',
  'images.push.pushed': '{pushed, plural, one {# लेयर पुश हुई} other {# लेयर पुश हुईं}}',
  'images.push.pushedAndExisting':
    '{pushed, plural, one {# लेयर पुश हुई} other {# लेयर पुश हुईं}}, {existing} पहले से रजिस्ट्री में',

  'images.tagDialog.title': '{name} टैग करें',
  'images.tagDialog.newName': 'नया नाम',
  'images.tagDialog.add': 'टैग जोड़ें',

  'images.transfer.savingOne': '{name} सहेजी जा रही है',
  'images.transfer.saving':
    '{count, plural, one {# इमेज सहेजी जा रही है} other {# इमेज सहेजी जा रही हैं}}',
  'images.transfer.to': '{path} में',
  'images.transfer.from': '{path} से',
  'images.transfer.written': 'लिखा गया',
  'images.transfer.sent': 'भेजा गया',
  'images.transfer.loading': 'इमेज लोड हो रही हैं',
  'images.transfer.importing': '{reference} इम्पोर्ट हो रही है',
  'images.transfer.doneWith': 'हो गया: {images}',
  'images.palette.pull': 'इमेज पुल करें',
} satisfies Translation<typeof en>;
