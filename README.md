# haribote-container

## コンテナのファイルシステムの用意

```
$ docker pull ubuntu:18.04
$ docker run -it ubuntu
root@[コンテナID]:/# exit
exit
$ docker ps -a
CONTAINER ID        IMAGE                   COMMAND                  CREATED              STATUS                      PORTS               NAMES
[コンテナID]        ubuntu                  "/bin/bash"              About a minute ago   Exited (0) 14 seconds ago                       gifted_curran
$ mkdir -p /var/lib/haribote_container/haribote/rootfs
$ sudo chmod 700 /var/lib/haribote_container/haribote/rootfs
$ docker export [コンテナID] | sudo tar -xv -f - -C /var/lib/haribote_container/haribote/rootfs
$ ls /var/lib/haribote_container/haribote/rootfs
bin  boot  dev  etc  home  lib  lib64  media  mnt  opt  proc  root  run  sbin  srv  sys  tmp  usr  var
```