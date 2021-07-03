FROM anton1234/td:latest
RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y pkg-config libssl-dev libpq-dev
RUN apt-get install curl && curl https://sh.rustup.rs -sSf > ri.sh && chmod +x ri.sh && ./ri.sh -y && $HOME/.cargo/bin/rustup update
COPY . /tmp/build
RUN cd /tmp/build && $HOME/.cargo/bin/cargo build --release -p interface && mkdir /opt/app && mv target/release/interface /opt/app && cd / && rm -rf /tmp/build