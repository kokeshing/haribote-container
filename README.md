# haribote-container

## お願い

本文中やソースコードに誤っているところやおかしいところがあった場合issueへ建てていてだけると大変ありがたいです.

## コンテナのファイルシステムの用意

### docker-composeのvolumeの設定

Windowsなどはvolumeの設定に注意 絶対パスのほうが安心かもしれない

- 例
    /C/Users/user/src/haribote-container:/workspace

レポジトリがそのままマウントされるように設定

### 起動

```
$ git clone https://github.com/kokeshing/haribote-container.git
$ cd haribote-container
$ mkdir rootfs
$ docker pull ubuntu:18.04
$ docker run -it ubuntu
root@[コンテナID]:/# exit
exit
$ docker ps -a
CONTAINER ID        IMAGE                   COMMAND                  CREATED              STATUS                      PORTS               NAMES
[コンテナID]        ubuntu                  "/bin/bash"              About a minute ago   Exited (0) 14 seconds ago                       gifted_curran
$ docker export [コンテナID] > rootfs.tar
$ docker-compose up -d
$ docker ps -a
CONTAINER ID        IMAGE                   COMMAND                  CREATED              STATUS                      PORTS               NAMES
[コンテナID]        haribote                  "/bin/bash"              About a minute ago   Up 14 seconds ago                       gifted_curran
$ docker exec -it [コンテナID] /bin/bash
root@haribote:/workspace# ls
proc rootfs src target
```

## 本文中に出てきたソースコードへのリンク(登場順)

- [TCP request](https://github.com/moby/moby/blob/94b1cf4be3606ff731b3ca012e5eddf021f4608f/docker/docker.go#L170)
- [createContainer()](https://github.com/moby/moby/blob/94b1cf4be3606ff731b3ca012e5eddf021f4608f/container.go#L50)
- [createContainer()](https://github.com/moby/moby/blob/94b1cf4be3606ff731b3ca012e5eddf021f4608f/container.go#L84)
- [start](https://github.com/moby/moby/blob/94b1cf4be3606ff731b3ca012e5eddf021f4608f/container.go#L193)
- [LXC start()](https://github.com/lxc/lxc/blob/bdb3f44147bc1a55a97131b4b39d42844ae4fb9e/src/lxc/start.c#L823)

## コンテナ関係で非常に参考になるサイトのリンク集

本文で解説したのはかなりゆるくしていたりはしょっていたりするので是非興味があればリンク先も見てみるといいと思います.

- [コンテナ未経験新人が学ぶコンテナ技術入門](https://www.slideshare.net/KoheiTokunaga/ss-122754942)

コンテナについて非常に詳しくまとめられたスライド.読んでて自分の書いてるやついらないんじゃないか？になるほど詳しく分かりやすくまとめられている.

- [コマンドを叩いて遊ぶ 〜コンテナ仮想、その裏側〜](https://tech.retrieva.jp/entry/2019/04/16/155828)

シェルでコマンドを叩いてコンテナ環境を作成している.これも読んでて自分の書いてるやついらないんじゃないか？になるほど丁寧に解説されている.

- [How does Docker work?](https://cameronlonsdale.com/2019/03/25/how-does-docker-work/)

英語.Dockerの2013からなんと2019の動きも解説されているサイト.これも読んでて自分の書いてるやついらないんじゃないか？になるほど詳しく分かりやすくまとめられている.

- [コンテナのしくみ](https://medium.com/@11Takanori/コンテナのしくみ-ac6f4a83eda4)

まさかRustで自作コンテナ書くなんて人はあんまいないだろうと思っていたら締切1週間前頃に見つけてアになったサイト.自分の書いてるやつの新規性が素人の詳しい解説だけになった瞬間である.

- [MINCSによるLinuxコンテナ実装の裏側](https://qiita.com/mhiramat/items/5edd7eb479f9dca45b9c#%E3%81%93%E3%82%8C%E3%81%AF%E4%BD%95%E3%81%8B)

シェルで実装されたコンテナランタイムのMINCSの作者による解説.これも読んでて(ry

### ファイルが存在するのにexecvがENOENTになる場合

備忘録

```
# ./target/debug/haribote-container
Unshared CLONE_NEWPID CLONE_NEWNS
[SUCCESS] set hostname to haribote_container
[SUCCESS] mount procfs and sysfs
[SUCCESS] pivot_root
Failed to execv: ENOENT: No such file or directory
Stopped container
pid:Pid(145) status:0
```

#### readelf -eでただしく.soが存在するかチェック

```
# readelf -e /var/lib/haribote_container/haribote/rootfs/bin/bash
ELF Header:
  Magic:   7f 45 4c 46 02 01 01 00 00 00 00 00 00 00 00 00
  Class:                             ELF64
  Data:                              2's complement, little endian
  Version:                           1 (current)

===============================省略===============================

Program Headers:
  Type           Offset             VirtAddr           PhysAddr
                 FileSiz            MemSiz              Flags  Align
  PHDR           0x0000000000000040 0x0000000000400040 0x0000000000400040
                 0x00000000000001f8 0x00000000000001f8  R E    0x8
  INTERP         0x0000000000000238 0x0000000000400238 0x0000000000400238
                 0x000000000000001c 0x000000000000001c  R      0x1
      [Requesting program interpreter: /lib64/ld-linux-x86-64.so.2]
  LOAD           0x0000000000000000 0x0000000000400000 0x0000000000400000
                 0x00000000000ffd0c 0x00000000000ffd0c  R E    0x200000
  LOAD           0x0000000000100548 0x0000000000700548 0x0000000000700548
                 0x000000000000b6fc 0x0000000000015440  RW     0x200000
  DYNAMIC        0x0000000000102df0 0x0000000000702df0 0x0000000000702df0
                 0x00000000000001f0 0x00000000000001f0  RW     0x8
  NOTE           0x0000000000000254 0x0000000000400254 0x0000000000400254
                 0x0000000000000044 0x0000000000000044  R      0x4
  GNU_EH_FRAME   0x00000000000e3750 0x00000000004e3750 0x00000000004e3750
                 0x0000000000004324 0x0000000000004324  R      0x4
  GNU_STACK      0x0000000000000000 0x0000000000000000 0x0000000000000000
                 0x0000000000000000 0x0000000000000000  RW     0x10
  GNU_RELRO      0x0000000000100548 0x0000000000700548 0x0000000000700548
                 0x0000000000002ab8 0x0000000000002ab8  R      0x1
```

/lib64/ld-linux-x86-64.so.2を要求している.

```
# ls -al /lib64/
total 8
drwxr-xr-x 2 root root 4096 Aug 12 00:00 .
drwxr-xr-x 1 root root 4096 Sep  5 04:35 ..
lrwxrwxrwx 1 root root   32 Feb  6  2019 ld-linux-x86-64.so.2 -> /lib/x86_64-linux-gnu/ld-2.24.so
```

の通り,シンボリックリンクとなっており,これが用意したルートファイルシステムにコピーできていなかった.
Windowsのフォルダを経由したりするときには注意. tarで固めてADD(自動的に展開される)などを使う.
