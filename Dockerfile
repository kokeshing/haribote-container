FROM rust:1.36

RUN mkdir -p /var/lib/haribote_container/haribote/rootfs
RUN chmod 700 /var/lib/haribote_container/haribote/rootfs
ADD rootfs.tar /var/lib/haribote_container/haribote/rootfs/

RUN mkdir /workspace
WORKDIR /workspace

CMD ["/bin/bash"]