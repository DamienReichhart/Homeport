---
title: Локальные движки
description: Подключение к Docker Engine, Docker Desktop, Colima, OrbStack, Rancher Desktop или Podman на этом компьютере.
---

Хост типа **Локальный сокет** общается с движком на этом компьютере через его сокет Unix или, в Windows, через именованный канал. Ему не нужны ни учётные данные, ни связка ключей.

## Автоматическое обнаружение

Homeport ищет работающие движки в обычных местах и показывает те, что ответили, в разделе **Найдено на этом компьютере** на экране приветствия и на странице **Добавить хост**. Выберите **Добавить** рядом с одним из них, чтобы создать хост, названный по нему.

| Среда выполнения         | macOS и Linux                                                                                              | Windows                           |
| ------------------------ | ---------------------------------------------------------------------------------------------------------- | --------------------------------- |
| Docker Engine            | `/var/run/docker.sock`                                                                                     | —                                 |
| Docker Engine (rootless) | `$XDG_RUNTIME_DIR/docker.sock`                                                                             | —                                 |
| Docker Desktop           | `~/.docker/desktop/docker.sock`, `~/.docker/run/docker.sock`                                               | `//./pipe/docker_engine`          |
| Colima                   | `~/.colima/default/docker.sock`                                                                            | —                                 |
| OrbStack                 | `~/.orbstack/run/docker.sock`                                                                              | —                                 |
| Rancher Desktop          | `~/.rd/docker.sock`                                                                                        | —                                 |
| Podman                   | `$XDG_RUNTIME_DIR/podman/podman.sock`, `/run/podman/podman.sock`; [машины в macOS](#машины-podman-в-macos) | `//./pipe/podman-machine-default` |

Каждый кандидат проверяется запросом версии у движка, поэтому в списке только работающие движки. Если несколько путей ведут к одному сокету — в macOS `/var/run/docker.sock` обычно указывает на Docker Desktop, OrbStack или Rancher Desktop, — движок показывается один раз, под первым найденным именем.

### Машины Podman в macOS

В macOS Podman запускает свой движок в виртуальной машине (`podman machine`), сокет которой у каждой машины свой. Homeport запрашивает у CLI `podman` сокет каждой работающей машины (`podman machine list` и `podman machine inspect`) и показывает каждую как **Podman (_имя машины_)**, например **Podman (podman-machine-default)**. Остановленные машины не показываются; запустите машину командой `podman machine start` и снова откройте **Добавить хост**.

Если Homeport не может запустить `podman` — его нет ни в PATH, ни в `/opt/homebrew/bin` или `/usr/local/bin`, — он ищет сокеты там, куда их кладёт Podman: `$TMPDIR/podman/<machine>-api.sock` для Podman 5 и `~/.local/share/containers/podman/machine/…/podman.sock` для Podman 4. Если `podman-mac-helper` связал `/var/run/docker.sock` с машиной, эта машина показывается один раз, под своим именем Podman.

## Добавление сокета вручную

Если ваш движок не обнаружен — профиль Colima, отличный от `default`, машина Podman в Windows с именем, отличным от стандартного, или нестандартный путь, — добавьте его сами:

1. Выберите **Добавить хост…** в переключателе хостов.
2. Выберите **Локальный сокет**.
3. Введите **Имя** и **Путь к сокету**: сокет Unix, например `/run/user/1000/docker.sock`, или именованный канал, например `//./pipe/docker_engine` в Windows.
4. Выберите **Добавить хост**.

Чтобы узнать, какой сокет использует ваш Docker CLI, выполните:

```sh
docker context inspect --format '{{.Endpoints.docker.Host}}'
```

Для машины Podman:

```sh
podman machine inspect --format '{{.ConnectionInfo.PodmanSocket.Path}}'
```

## Права доступа в Linux

Стандартный сокет `/var/run/docker.sock` принадлежит группе `docker`. Если ваш пользователь в неё не входит, хост показывает **Недоступен** с ошибкой _Homeport не разрешено использовать Docker Engine по пути /var/run/docker.sock._ Добавьте себя в группу и войдите снова:

```sh
sudo usermod -aG docker $USER
```

Членство в группе `docker` равносильно правам root на этом компьютере; если это для вас важно, rootless Docker или Podman позволяют этого избежать.
