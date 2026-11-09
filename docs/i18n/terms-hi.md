# Hindi (`hi`) terms for Homeport

Register: standard Hindi in Devanagari, Windows/Android Hindi UI style. User addressed as "आप".
Commands use the polite imperative (हटाएँ, सहेजें, रद्द करें, फिर से डिप्लॉय करें).
Sentence end: Devanagari danda "।" everywhere (never a full stop at sentence end); "?" for questions; "…" kept.
Plurals: `one` and `other` (Hindi CLDR integer categories).
Technical nouns transliterated as Indian developers say them; Kubernetes object kinds and product names stay in Latin script.

| English term               | Hindi term                           | Grammatical notes            | Avoid                      |
| -------------------------- | ------------------------------------ | ---------------------------- | -------------------------- |
| Host                       | होस्ट                                | m.; plural unchanged (होस्ट) | मेज़बान, सर्वर, इंस्टेंस   |
| Engine                     | इंजन                                 | m.                           | डेमन, सर्वर, बैकएंड        |
| Engine kind                | इंजन का प्रकार                       | m.                           | किस्म                      |
| Instance (Incus)           | इंस्टेंस                             | m.; plural unchanged         | VM, उदाहरण                 |
| Project (Incus)            | प्रोजेक्ट                            | m.                           | परियोजना                   |
| Profile (Incus)            | प्रोफ़ाइल                            | f.                           | रूपरेखा                    |
| Pod (Kubernetes)           | Pod                                  | Latin, capital; treated m.   | पॉड (for Kubernetes)       |
| Podman pod                 | Podman पॉड / पॉड                     | m.                           | समूह                       |
| Quadlet unit               | Quadlet यूनिट                        | f.                           | इकाई                       |
| Bastion                    | बैस्टियन                             | m.                           | जंप होस्ट, प्रॉक्सी, गेटवे |
| Swarm                      | Swarm                                | Latin, m.                    | झुंड, क्लस्टर (alone)      |
| Cluster                    | क्लस्टर                              | m.                           | कॉन्टेक्स्ट, k8s, परिवेश   |
| Port forward               | पोर्ट फ़ॉरवर्ड                       | m.                           | टनल, प्रॉक्सी              |
| Release (Helm)             | रिलीज़                               | f.                           | डिप्लॉयमेंट, ऐप            |
| Chart repository           | चार्ट रिपॉज़िटरी                     | f.                           | Helm repo, रजिस्ट्री       |
| Node (Swarm)               | नोड                                  | m.                           | सदस्य, मशीन                |
| Node (Kubernetes)          | Node                                 | Latin, capital               | —                          |
| Service (Swarm)            | सर्विस                               | f.                           | ऐप, सेवा                   |
| Service (Kubernetes)       | Service                              | Latin, capital               | —                          |
| Task (Swarm / containerd)  | टास्क                                | m.                           | कार्य, रेप्लिका            |
| Board                      | बोर्ड                                | m.                           | डैशबोर्ड                   |
| Command palette            | कमांड पैलेट                          | m.                           | लॉन्चर                     |
| Stack                      | स्टैक                                | m.                           | प्रोजेक्ट, एप्लिकेशन       |
| Managed Stack              | प्रबंधित स्टैक                       | m.                           | लोकल स्टैक                 |
| Linked Stack               | लिंक्ड स्टैक                         | m.                           | फ़ोल्डर स्टैक              |
| Synced Stack               | सिंक्ड स्टैक                         | m.                           | Git स्टैक                  |
| External Stack             | बाहरी स्टैक                          | m.                           | अप्रबंधित स्टैक            |
| Repository                 | रिपॉज़िटरी                           | f.                           | रेपो                       |
| Deploy key                 | डिप्लॉय कुंजी                        | f.                           | SSH कुंजी (alone)          |
| Adopt                      | अपनाना (button: अपनाएँ)              | verb                         | आयात करना                  |
| deploy                     | डिप्लॉय करना                         | verb (करना)                  | परिनियोजित करना            |
| redeploy                   | फिर से डिप्लॉय करना                  | verb                         | पुनः परिनियोजन             |
| pull                       | पुल करना                             | verb                         | खींचना                     |
| push                       | पुश करना                             | verb                         | धकेलना                     |
| prune                      | प्रून करना                           | verb                         | छाँटना                     |
| drain                      | ड्रेन करना                           | verb                         | खाली करना                  |
| container                  | कंटेनर                               | m.                           | पात्र                      |
| image                      | इमेज                                 | f.                           | छवि                        |
| volume                     | वॉल्यूम                              | m.                           | आयतन                       |
| network                    | नेटवर्क                              | m.                           | —                          |
| Namespace (containerd)     | नेमस्पेस                             | m.                           | —                          |
| Namespace (Kubernetes)     | Namespace                            | Latin, capital               | —                          |
| Snapshot                   | स्नैपशॉट                             | m.                           | —                          |
| Registry                   | रजिस्ट्री                            | f.                           | —                          |
| Registry login             | रजिस्ट्री लॉगिन                      | m.                           | क्रेडेंशियल, खाता          |
| Helper container           | सहायक कंटेनर                         | m.                           | साइडकार, एजेंट             |
| Trust token                | ट्रस्ट टोकन                          | m.                           | —                          |
| Settings                   | सेटिंग्स                             | f. pl.                       | —                          |
| Remove / Delete            | हटाएँ                                | —                            | मिटाएँ (keep for "erase")  |
| Cancel                     | रद्द करें                            | —                            | —                          |
| Save                       | सहेजें                               | —                            | —                          |
| Start / Stop / Restart     | शुरू करें / रोकें / फिर से शुरू करें | —                            | —                          |
| Pause / Unpause (Resume)   | विराम दें / जारी रखें                | —                            | रोकें (that is Stop)       |
| Suspend (CronJob) / Resume | निलंबित करें / जारी रखें             | —                            | —                          |
| Scale                      | स्केल करें                           | —                            | —                          |
| Kill                       | बलपूर्वक बंद करें                    | —                            | —                          |
| Logs                       | लॉग                                  | m.                           | —                          |
