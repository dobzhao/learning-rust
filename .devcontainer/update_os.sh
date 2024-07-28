## update and install some things we should probably have
apt update
apt install -y \
  curl \
  git \
  gnupg2 \
  jq \
  sudo \
  libc6-dev \
  zsh \
  vim \
  valgrind \
  build-essential \
  openssl
apt autoclean
apt clean
