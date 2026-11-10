---
title: المحركات المحلية
description: اتصل بـ Docker Engine أو Docker Desktop أو Colima أو OrbStack أو Rancher Desktop أو Podman على هذا الجهاز.
---

يتحدث مضيف **مقبس محلي** مع محرك على هذا الجهاز عبر مقبس Unix الخاص به، أو عبر أنبوب مسمّى على Windows. لا يحتاج إلى بيانات اعتماد ولا إلى سلسلة مفاتيح.

## الاكتشاف التلقائي

يبحث Homeport عن المحركات العاملة في الأماكن المعتادة ويسرد المحركات التي تجيب ضمن **تم العثور عليها على هذا الجهاز**، في شاشة الترحيب وفي صفحة **إضافة مضيف**. اختر **إضافة** بجانب أحدها لإنشاء مضيف يحمل اسمه.

| بيئة التشغيل             | macOS وLinux                                                                                                 | Windows                           |
| ------------------------ | ------------------------------------------------------------------------------------------------------------ | --------------------------------- |
| Docker Engine            | `/var/run/docker.sock`                                                                                       | —                                 |
| Docker Engine (rootless) | `$XDG_RUNTIME_DIR/docker.sock`                                                                               | —                                 |
| Docker Desktop           | `~/.docker/desktop/docker.sock`، `~/.docker/run/docker.sock`                                                 | `//./pipe/docker_engine`          |
| Colima                   | `~/.colima/default/docker.sock`                                                                              | —                                 |
| OrbStack                 | `~/.orbstack/run/docker.sock`                                                                                | —                                 |
| Rancher Desktop          | `~/.rd/docker.sock`                                                                                          | —                                 |
| Podman                   | `$XDG_RUNTIME_DIR/podman/podman.sock`، `/run/podman/podman.sock`؛ [الآلات على macOS](#آلات-podman-على-macos) | `//./pipe/podman-machine-default` |

يُفحص كل مرشح بسؤال المحرك عن إصداره، لذا لا تُسرد إلا المحركات العاملة. عندما تؤدي عدة مسارات إلى المقبس نفسه — على macOS، يشير `/var/run/docker.sock` عادةً إلى Docker Desktop أو OrbStack أو Rancher Desktop — يُسرد المحرك مرة واحدة، تحت أول اسم يُعثر عليه.

### آلات Podman على macOS

على macOS، يشغّل Podman محركه في آلة افتراضية (`podman machine`) يتغير مقبسها مع كل آلة. يسأل Homeport أداة `podman` CLI عن مقبس كل آلة عاملة (`podman machine list` و`podman machine inspect`) ويسرد كلًا منها باسم **Podman (_اسم الآلة_)**، مثل **Podman (podman-machine-default)**. لا تُسرد الآلات المتوقفة؛ شغّل إحداها باستخدام `podman machine start` وافتح **إضافة مضيف** مجددًا.

إذا لم يستطع Homeport تشغيل `podman` — لأنه ليس في PATH الخاص به ولا في `/opt/homebrew/bin` أو `/usr/local/bin` — فإنه يبحث بدلًا من ذلك حيث يضع Podman المقابس: `$TMPDIR/podman/<machine>-api.sock` لـ Podman 5، و`~/.local/share/containers/podman/machine/…/podman.sock` لـ Podman 4. عندما يكون `podman-mac-helper` قد ربط `/var/run/docker.sock` بآلة، تُسرد تلك الآلة مرة واحدة، تحت اسمها في Podman.

## إضافة مقبس يدويًا

إذا لم يُكتشف محركك — ملف تعريف Colima غير `default`، أو آلة Podman على Windows باسم غير الاسم الافتراضي، أو مسار مخصص — فأضفه بنفسك:

1. اختر **إضافة مضيف…** في مبدّل المضيفات.
2. حدّد **مقبس محلي**.
3. أدخل **الاسم** و**مسار المقبس**: مقبس Unix مثل `/run/user/1000/docker.sock`، أو أنبوبًا مسمّى مثل `//./pipe/docker_engine` على Windows.
4. اختر **إضافة مضيف**.

لمعرفة المقبس الذي يستخدمه Docker CLI لديك، شغّل:

```sh
docker context inspect --format '{{.Endpoints.docker.Host}}'
```

لآلة Podman:

```sh
podman machine inspect --format '{{.ConnectionInfo.PodmanSocket.Path}}'
```

## الأذونات على Linux

ينتمي المقبس الافتراضي `/var/run/docker.sock` إلى مجموعة `docker`. إذا لم يكن مستخدمك فيها، يعرض المضيف **يتعذّر الوصول إليه** مع _لا يُسمح لـ Homeport باستخدام محرك Docker على /var/run/docker.sock._ أضف نفسك وسجّل الدخول مجددًا:

```sh
sudo usermod -aG docker $USER
```

العضوية في مجموعة `docker` تعادل صلاحيات root على ذلك الجهاز؛ ويتجنب Docker أو Podman بدون صلاحيات root ذلك إذا كان الأمر يهمك.
