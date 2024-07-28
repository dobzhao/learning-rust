## Install rustup and common components
export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
export RUSTUP_DIST_SERVER=https://mirrors.tuna.tsinghua.edu.cn/rustup

#curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rust.sh
sh rust.sh -y

source "$HOME/.cargo/env"
rustup component add rustfmt
rustup component add clippy

#cargo install cargo-expand
#cargo install cargo-edit
