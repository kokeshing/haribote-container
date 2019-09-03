# haribote-container

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
$ docker export [コンテナID] | ./rootfs
$ ls ./rootfs
bin  boot  dev  etc  home  lib  lib64  media  mnt  opt  proc  root  run  sbin  srv  sys  tmp  usr  var
$ docker-compose up -d
$ docker ps -a
CONTAINER ID        IMAGE                   COMMAND                  CREATED              STATUS                      PORTS               NAMES
[コンテナID]        haribote                  "/bin/bash"              About a minute ago   Up 14 seconds ago                       gifted_curran
$ docker exec -it [コンテナID] /bin/bash
root@haribote:/workspace# ls
proc rootfs src target
```

