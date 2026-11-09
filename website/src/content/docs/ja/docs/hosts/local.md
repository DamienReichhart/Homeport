---
title: ローカルのエンジン
description: このマシン上の Docker Engine、Docker Desktop、Colima、OrbStack、Rancher Desktop、Podman に接続します。
---

**ローカルソケット** のホストは、このマシン上のエンジンと、その Unix ソケット（Windows では名前付きパイプ）を通じて通信します。認証情報もキーチェーンも必要ありません。

## 自動検出

Homeport は、動作しているエンジンを通常の場所で探し、応答したものをウェルカム画面と **ホストを追加** ページの **このマシンで見つかったもの** に一覧表示します。その横の **追加** を選ぶと、そのエンジンにちなんだ名前のホストが作成されます。

| ランタイム                | macOS と Linux                                                                                                | Windows                           |
| ------------------------- | ------------------------------------------------------------------------------------------------------------- | --------------------------------- |
| Docker Engine             | `/var/run/docker.sock`                                                                                        | —                                 |
| Docker Engine（rootless） | `$XDG_RUNTIME_DIR/docker.sock`                                                                                | —                                 |
| Docker Desktop            | `~/.docker/desktop/docker.sock`、`~/.docker/run/docker.sock`                                                  | `//./pipe/docker_engine`          |
| Colima                    | `~/.colima/default/docker.sock`                                                                               | —                                 |
| OrbStack                  | `~/.orbstack/run/docker.sock`                                                                                 | —                                 |
| Rancher Desktop           | `~/.rd/docker.sock`                                                                                           | —                                 |
| Podman                    | `$XDG_RUNTIME_DIR/podman/podman.sock`、`/run/podman/podman.sock`、[macOS のマシン](#macos-の-podman-マシン) | `//./pipe/podman-machine-default` |

候補ごとにエンジンにバージョンを問い合わせて確認するため、一覧に表示されるのは動作中のエンジンだけです。複数のパスが同じソケットにつながっている場合（macOS では通常、`/var/run/docker.sock` が Docker Desktop、OrbStack、Rancher Desktop のいずれかを指しています）、そのエンジンは最初に見つかった名前で 1 回だけ表示されます。

### macOS の Podman マシン

macOS では、Podman はエンジンを VM（`podman machine`）の中で実行し、そのソケットはマシンごとに場所が変わります。Homeport は `podman` CLI に、動作中のすべてのマシンのソケットを問い合わせ（`podman machine list` と `podman machine inspect`）、それぞれを **Podman (_machine name_)**（例: **Podman (podman-machine-default)**）として表示します。停止しているマシンは表示されません。`podman machine start` でマシンを起動してから、もう一度 **ホストを追加** を開いてください。

Homeport が `podman` を実行できない場合（PATH 上にも、`/opt/homebrew/bin` や `/usr/local/bin` にもない場合）は、Podman がソケットを置く場所を代わりに探します。Podman 5 では `$TMPDIR/podman/<machine>-api.sock`、Podman 4 では `~/.local/share/containers/podman/machine/…/podman.sock` です。`podman-mac-helper` が `/var/run/docker.sock` をマシンにリンクしている場合、そのマシンは Podman での名前で 1 回だけ表示されます。

## ソケットを手動で追加する

エンジンが検出されない場合（`default` 以外の Colima プロファイル、既定以外の名前を持つ Windows の Podman マシン、カスタムのパスなど）は、自分で追加します:

1. ホストの切り替えメニューで **ホストを追加…** を選びます。
2. **ローカルソケット** を選択します。
3. **名前** と **ソケットのパス** を入力します。`/run/user/1000/docker.sock` のような Unix ソケットか、Windows では `//./pipe/docker_engine` のような名前付きパイプです。
4. **ホストを追加** を選びます。

Docker CLI が使っているソケットを調べるには、次を実行します:

```sh
docker context inspect --format '{{.Endpoints.docker.Host}}'
```

Podman マシンの場合:

```sh
podman machine inspect --format '{{.ConnectionInfo.PodmanSocket.Path}}'
```

## Linux での権限

既定のソケット `/var/run/docker.sock` は `docker` グループに属しています。ユーザーがこのグループに属していない場合、ホストは _Permission denied_ とともに **到達不能** と表示されます。自分をグループに追加して、ログインし直してください:

```sh
sudo usermod -aG docker $USER
```

`docker` グループのメンバーであることは、そのマシンの root と同等です。それが問題になる場合は、rootless の Docker か Podman を使えば避けられます。
