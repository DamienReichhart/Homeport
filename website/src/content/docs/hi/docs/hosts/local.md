---
title: लोकल इंजन
description: इस मशीन पर Docker Engine, Docker Desktop, Colima, OrbStack, Rancher Desktop या Podman से कनेक्ट करें।
---

**लोकल सॉकेट** होस्ट इस मशीन पर किसी इंजन से उसके Unix सॉकेट, या Windows पर किसी named pipe के ज़रिए बात करता है। इसे न क्रेडेंशियल चाहिए और न कीचेन।

## अपने-आप पहचान

Homeport सामान्य जगहों पर चल रहे इंजन ढूँढता है और जो जवाब देते हैं उन्हें स्वागत स्क्रीन और **होस्ट जोड़ें** पेज पर **इस मशीन पर मिले** के नीचे सूचीबद्ध करता है। किसी एक के बगल में **जोड़ें** चुनें, ताकि उसके नाम पर एक होस्ट बन जाए।

| रनटाइम                   | macOS और Linux                                                                                               | Windows                           |
| ------------------------ | ------------------------------------------------------------------------------------------------------------ | --------------------------------- |
| Docker Engine            | `/var/run/docker.sock`                                                                                       | —                                 |
| Docker Engine (rootless) | `$XDG_RUNTIME_DIR/docker.sock`                                                                               | —                                 |
| Docker Desktop           | `~/.docker/desktop/docker.sock`, `~/.docker/run/docker.sock`                                                 | `//./pipe/docker_engine`          |
| Colima                   | `~/.colima/default/docker.sock`                                                                              | —                                 |
| OrbStack                 | `~/.orbstack/run/docker.sock`                                                                                | —                                 |
| Rancher Desktop          | `~/.rd/docker.sock`                                                                                          | —                                 |
| Podman                   | `$XDG_RUNTIME_DIR/podman/podman.sock`, `/run/podman/podman.sock`; [macOS पर मशीन](#macos-पर-podman-मशीन) | `//./pipe/podman-machine-default` |

हर उम्मीदवार की जाँच इंजन से उसका संस्करण पूछकर की जाती है, इसलिए सिर्फ़ चल रहे इंजन सूचीबद्ध होते हैं। जब कई पाथ एक ही सॉकेट तक ले जाते हैं — macOS पर, `/var/run/docker.sock` आम तौर पर Docker Desktop, OrbStack या Rancher Desktop की ओर इंगित करता है — तो इंजन एक बार, पहले मिले नाम के तहत सूचीबद्ध होता है।

### macOS पर Podman मशीन

macOS पर, Podman अपना इंजन एक VM (`podman machine`) में चलाता है, जिसका सॉकेट हर मशीन के साथ बदलता है। Homeport `podman` CLI से हर चल रही मशीन का सॉकेट पूछता है (`podman machine list` और `podman machine inspect`) और हर एक को **Podman (_मशीन का नाम_)** के रूप में सूचीबद्ध करता है, जैसे **Podman (podman-machine-default)**। रुकी हुई मशीनें सूचीबद्ध नहीं होतीं; `podman machine start` से कोई एक शुरू करें और **होस्ट जोड़ें** फिर से खोलें।

अगर Homeport `podman` नहीं चला पाता — वह उसके PATH में या `/opt/homebrew/bin` या `/usr/local/bin` में नहीं है — तो वह इसके बजाय वहाँ देखता है जहाँ Podman सॉकेट रखता है: Podman 5 के लिए `$TMPDIR/podman/<machine>-api.sock`, और Podman 4 के लिए `~/.local/share/containers/podman/machine/…/podman.sock`। जब `podman-mac-helper` ने `/var/run/docker.sock` को किसी मशीन से लिंक किया हो, तो वह मशीन एक बार, अपने Podman नाम के तहत सूचीबद्ध होती है।

## हाथ से सॉकेट जोड़ना

अगर आपका इंजन पहचाना नहीं जाता — `default` के अलावा कोई Colima प्रोफ़ाइल, Windows पर डिफ़ॉल्ट के अलावा किसी दूसरे नाम वाली Podman मशीन, या कोई कस्टम पाथ — तो उसे ख़ुद जोड़ें:

1. होस्ट स्विचर में **होस्ट जोड़ें…** चुनें।
2. **लोकल सॉकेट** चुनें।
3. एक **नाम** और **सॉकेट पाथ** दर्ज करें: कोई Unix सॉकेट जैसे `/run/user/1000/docker.sock`, या Windows पर कोई named pipe जैसे `//./pipe/docker_engine`।
4. **होस्ट जोड़ें** चुनें।

आपका Docker CLI जिस सॉकेट का इस्तेमाल करता है, उसे ढूँढने के लिए यह चलाएँ:

```sh
docker context inspect --format '{{.Endpoints.docker.Host}}'
```

Podman मशीन के लिए:

```sh
podman machine inspect --format '{{.ConnectionInfo.PodmanSocket.Path}}'
```

## Linux पर अनुमतियाँ

डिफ़ॉल्ट सॉकेट `/var/run/docker.sock` `docker` ग्रुप का होता है। अगर आपका यूज़र उसमें नहीं है, तो होस्ट _Permission denied_ के साथ **पहुँच से बाहर** दिखाता है। ख़ुद को जोड़ें और फिर से लॉग इन करें:

```sh
sudo usermod -aG docker $USER
```

`docker` ग्रुप की सदस्यता उस मशीन पर root के बराबर है; अगर यह आपके लिए मायने रखता है, तो rootless Docker या Podman इससे बचाते हैं।
