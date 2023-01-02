# This is the build stage for Polkadot. Here we create the binary in a temporary image.
FROM rust:slim-buster as builder

ENV PATH=/usr/local/cargo/bin:$PATH \
		CC=clang-14 \
		CXX=clang-14
# those can be removed once jq is available through the distribution
ARG JQ_CHECKSUM=af986793a515d500ab2d35f8d2aecd656e764504b789b66d7e1a0b727a124c44
ARG JQ_PATH=/bin/jq

# install tools and dependencies
RUN set -eux; \
	apt-get -y update; \
	apt-get install -y --no-install-recommends \
		libssl-dev make cmake graphviz \
		git pkg-config curl time rhash ca-certificates \
		lsof git-restore-mtime xz-utils zstd unzip gnupg protobuf-compiler && \
# add clang 14 repo
	curl -s https://apt.llvm.org/llvm-snapshot.gpg.key | apt-key add -; \
	echo "deb http://apt.llvm.org/buster/ llvm-toolchain-buster-14 main" >> /etc/apt/sources.list.d/llvm-toochain-buster-14.list; \
	echo "deb-src http://apt.llvm.org/buster/ llvm-toolchain-buster-14 main" >> /etc/apt/sources.list.d/llvm-toochain-buster-14.list; \
	apt-get -y update; \
	apt-get install -y --no-install-recommends \
		clang-14 lldb-14 lld-14 libclang-14-dev && \
# add non-root user
  groupadd -g 1000 nonroot && \
  useradd -u 1000 -g 1000 -s /bin/bash -m nonroot && \
# install recent jq; this step can be removed once jq >= 1.6 is available
# through the distribution
  curl -sqL https://github.com/stedolan/jq/releases/download/jq-1.6/jq-linux64 -o "$JQ_PATH" && \
  chmod +x "$JQ_PATH" && \
  echo "$JQ_CHECKSUM $JQ_PATH" | sha256sum --check && \
  jq --version; \
# set a link to clang
	update-alternatives --install /usr/bin/cc cc /usr/bin/clang-14 100; \
# set a link to ldd
  update-alternatives --install /usr/bin/ld ld /usr/bin/ld.lld-14 100

RUN set -eux && \
	# install git from backports
	echo deb http://deb.debian.org/debian buster-backports main | tee /etc/apt/sources.list.d/buster-backports.list  && \
	apt-get -y update; \
	apt install -y -t buster-backports git; \
	git --version; \
	# install specific Rust nightly, default is stable, use minimum components
	rustup toolchain install nightly-2022-11-16 --profile minimal && \
	# "alias" pinned nightly-2022-11-16 toolchain as nightly
	ln -s /usr/local/rustup/toolchains/nightly-2022-11-16-x86_64-unknown-linux-gnu /usr/local/rustup/toolchains/nightly-x86_64-unknown-linux-gnu && \
	# install wasm toolchain
	rustup target add wasm32-unknown-unknown && \
	rustup target add wasm32-unknown-unknown --toolchain nightly-2022-11-16 && \
	rustup show && \
	cargo --version
	# apt clean up
#	apt-get autoremove -y && \
#	apt-get clean && \
#	rm -rf /var/lib/apt/lists/* && \
	# cargo clean up
	# removes compilation artifacts cargo install creates (>250M)
#	rm -rf "${CARGO_HOME}/registry" "${CARGO_HOME}/git" /root/.cache/sccache

WORKDIR /polkadot
COPY . /polkadot

RUN cargo build --locked --release

# This is the 2nd stage: a very small image where we copy the Polkadot binary."
FROM docker.io/library/ubuntu:20.04

LABEL description="Multistage Docker image for Polkadot: a platform for web3" \
	io.parity.image.type="builder" \
	io.parity.image.authors="chevdor@gmail.com, devops-team@parity.io" \
	io.parity.image.vendor="Parity Technologies" \
	io.parity.image.description="Polkadot: a platform for web3" \
	io.parity.image.source="https://github.com/paritytech/polkadot/blob/${VCS_REF}/scripts/ci/dockerfiles/polkadot/polkadot_builder.Dockerfile" \
	io.parity.image.documentation="https://github.com/paritytech/polkadot/"

COPY --from=builder /polkadot/target/release/polkadot /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /polkadot polkadot && \
	mkdir -p /data /polkadot/.local/share && \
	chown -R polkadot:polkadot /data && \
	ln -s /data /polkadot/.local/share/polkadot && \
# unclutter and minimize the attack surface
	rm -rf /usr/bin /usr/sbin && \
# check if executable works in this container
	/usr/local/bin/polkadot --version

USER polkadot

EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]

ENTRYPOINT ["/usr/local/bin/polkadot"]
