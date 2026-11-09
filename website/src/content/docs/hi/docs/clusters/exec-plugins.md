---
title: Exec प्लगइन
description: Homeport EKS, GKE और AKS क्लस्टर में उस exec credential प्लगइन से कैसे साइन इन करता है जिसका नाम उनका kubeconfig देता है, और क्या कहाँ चलता है।
---

क्लाउड kubeconfig में कोई पासवर्ड या प्रमाणपत्र नहीं होता: उनका यूज़र एक **exec प्लगइन** का नाम देता है, आपके कंप्यूटर पर एक प्रोग्राम जो माँगे जाने पर कम समय तक चलने वाला टोकन प्रिंट करता है। Homeport इस कंप्यूटर पर वही प्रोग्राम चलाता है जो `kubectl` चलाता, Kubernetes के `ExecCredential` प्रोटोकॉल (`client.authentication.k8s.io/v1` और `v1beta1`) के अनुसार।

## आम प्रोवाइडर

| प्रोवाइडर | kubeconfig का यूज़र चलाता है         | इंस्टॉल करें                                                             |
| --------- | ------------------------------------ | ------------------------------------------------------------------------ |
| EKS       | `aws eks get-token --cluster-name …` | AWS CLI, संस्करण 2                                                       |
| GKE       | `gke-gcloud-auth-plugin`             | Google Cloud CLI, फिर `gcloud components install gke-gcloud-auth-plugin` |
| AKS       | `kubelogin get-token …`              | `az aks install-cli`, या उसकी releases से kubelogin                      |

kubeconfig हमेशा की तरह लिखें — `aws eks update-kubeconfig`, `gcloud container clusters get-credentials`, `az aks get-credentials` और उसके बाद `kubelogin convert-kubeconfig` — फिर [इसके contexts जोड़ें](/docs/clusters/)।

## क्या कहाँ चलता है

- प्लगइन **इस कंप्यूटर पर** चलता है, आपके यूज़र के रूप में, उन आर्ग्युमेंट और environment variables के साथ जो kubeconfig उसे देता है (उदाहरण के लिए `AWS_PROFILE`)। Homeport उससे कभी इंटरैक्टिव होने को नहीं कहता और उसे कोई इनपुट नहीं देता।
- डेस्कटॉप लॉन्चर Homeport को छोटे `PATH` के साथ शुरू करता है, इसलिए Homeport AWS, Google Cloud और Azure CLI और kubelogin की आम इंस्टॉल जगहों में भी देखता है (Homebrew, SDK का अपना फ़ोल्डर, `~/.local/bin`, `~/.krew/bin`, Program Files…)। kubeconfig में पूरे पाथ से दिया गया प्लगइन वहीं से चलता है।
- यह जो टोकन या प्रमाणपत्र प्रिंट करता है, वह **सिर्फ़ मेमोरी में** रखा जाता है, उसकी समय-सीमा खत्म होने से थोड़ा पहले तक; तब प्लगइन फिर से चलता है। यह हर टोकन की अवधि में एक बार चलता है, हर अनुरोध पर नहीं। जब API कोई टोकन अस्वीकार करता है, तो Homeport **पहुँच अस्वीकृत** दिखाने से पहले प्लगइन से एक बार नया टोकन माँगता है।
- प्लगइन के अपने cache (`~/.kube/cache`, `~/.aws/sso/cache`, gcloud config) उसका अपना मामला हैं; Homeport वहाँ कुछ नहीं लिखता।

## जब यह साइन इन न कर सके

- **"… इस कंप्यूटर पर इंस्टॉल नहीं है"** — प्रोग्राम नहीं मिला। इसे इंस्टॉल करें (संदेश बताता है कैसे, और अगर kubeconfig में `installHint` हो तो उसका इस्तेमाल करता है), जाँचें कि यह टर्मिनल में काम करता है, फिर **फिर से कनेक्ट करें**।
- **"… चाहता है कि आप साइन इन करें, जो यह Homeport के अंदर नहीं कर सकता"** — प्लगइन को ब्राउज़र या device code चाहिए (AWS SSO, Azure device login, समाप्त हो चुका `gcloud` सेशन)। उस context के साथ टर्मिनल में एक बार `kubectl` चलाएँ, वहाँ साइन इन करें, फिर **फिर से कनेक्ट करें** चुनें: तब प्लगइन को अपना cache किया हुआ साइन इन मिल जाता है।
- **"… को … के लिए क्रेडेंशियल नहीं मिल सके"** — यह चला लेकिन विफल रहा; **विवरण** इसके प्रिंट किए गए आउटपुट का आखिरी हिस्सा दिखाता है। Homeport अपने आप फिर से कोशिश करता है।

दूसरे संदेशों के लिए [समस्या निवारण](/docs/reference/troubleshooting/#क्लस्टर) देखें।
