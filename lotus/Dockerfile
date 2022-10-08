# syntax=docker/dockerfile:1.4

# See: https://github.com/moby/buildkit/blob/master/frontend/dockerfile/docs/syntax.md

FROM ubuntu:latest

RUN useradd ubuntu

RUN apt-get update
RUN DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends tzdata

RUN apt-get -y update
RUN apt-get -y install git

RUN apt install -y mesa-opencl-icd ocl-icd-opencl-dev gcc git bzr jq pkg-config curl clang build-essential hwloc libhwloc-dev wget
RUN apt install -y python3 vim less tmux psmisc htop rsync unzip

RUN apt upgrade -y

RUN wget https://go.dev/dl/go1.18.1.linux-amd64.tar.gz
RUN tar -C /usr/local -xf go*.tar.gz
RUN rm -f go*.tar.gz

ENV PATH="/usr/local/go/bin:${PATH}"

WORKDIR /home/ubuntu

RUN chown -R ubuntu. /home/ubuntu
USER ubuntu

# https://stackoverflow.com/questions/49676490/when-installing-rust-toolchain-in-docker-bash-source-command-doesnt-work
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y

RUN echo 'source $HOME/.cargo/env' >> $HOME/.bashrc

ENV PATH="/home/ubuntu/.cargo/bin:${PATH}"

WORKDIR /home/ubuntu/lotus

ENV LOTUS_PATH="~/.lotus-local-net"
ENV LOTUS_MINER_PATH="~/.lotus-miner-local-net"
ENV LOTUS_SKIP_GENESIS_CHECK=_yes_
ENV CGO_CFLAGS_ALLOW="-D__BLST_PORTABLE__"
ENV CGO_CFLAGS="-D__BLST_PORTABLE__"

ENV PATH="/home/ubuntu/lotus:${PATH}"

RUN git clone --recurse-submodules  https://github.com/filecoin-project/lotus.git .

RUN rm -rf ~/.genesis-sectors

RUN git checkout experimental/fvm-m2

RUN cd extern/filecoin-ffi && git checkout feat/m2-iron

# Build Lotus

RUN make clean 2k

# Fetch params

RUN ./lotus fetch-params 2048

RUN ./lotus-seed pre-seal --sector-size 2KiB --num-sectors 2
RUN ./lotus-seed genesis new localnet.json
RUN ./lotus-seed genesis add-miner localnet.json ~/.genesis-sectors/pre-seal-t01000.json

EXPOSE 1234

RUN <<'eot' bash
       set -e
       # Start node

       stdbuf -o0 ./lotus daemon --lotus-make-genesis=devgen.car --genesis-template=localnet.json --bootstrap=false 2>&1  | sed 's/^/[node] /' &
       NODE_PID=$!
       echo Node PID: $NODE_PID

       # Setup miner

       sleep 10
       ./lotus wait-api
       ./lotus wallet import --as-default ~/.genesis-sectors/pre-seal-t01000.key

       ./lotus-miner init \
	  --genesis-miner \
          --actor=t01000 \
          --sector-size=2KiB \
          --pre-sealed-sectors=~/.genesis-sectors \
          --pre-sealed-metadata=~/.genesis-sectors/pre-seal-t01000.json \
          --nosync
       echo Done miner init.
       kill $NODE_PID
eot