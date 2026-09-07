FROM archlinux:base-devel@sha256:84cd9ef000b3cff245ec028e87965b84724f4bf1cc63fc2741ba927b88515ed6
RUN printf 'Server = https://archive.archlinux.org/repos/2026/09/07/$repo/os/$arch\n' > /etc/pacman.d/mirrorlist
RUN pacman -Syu --noconfirm clang rustup python qemu-user-static aarch64-linux-gnu-gcc
RUN rustup toolchain install 1.93.0 --profile minimal --component rustfmt && rustup default 1.93.0
RUN rustup target add aarch64-unknown-linux-gnu
RUN cargo install bindgen-cli --version 0.72.1 --locked
ENV PATH="/root/.cargo/bin:${PATH}"
WORKDIR /work
