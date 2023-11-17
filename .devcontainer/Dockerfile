FROM ubuntu:22.04

WORKDIR /home/

COPY . .

RUN bash ./update_os.sh
RUN bash ./install_rust.sh
RUN bash ./install_zsh.sh

ENV PATH="/root/.cargo/bin:$PATH"
