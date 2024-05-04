FROM rust:1.76.0-slim as builder
ARG DATABASE_URL
RUN USER=root cargo new --bin app-build
WORKDIR /app-build
COPY ./Cargo.toml ./Cargo.toml
RUN apt-get update \
    && apt-get install -y pkg-config libssl-dev git m4 libssl3 openssl
RUN cargo --version
RUN rustc --version
RUN cargo build --release

FROM debian:bookworm as runtime
WORKDIR /bin
# Copy from builder and rename binary to "server".
COPY --from=builder /app-build/target/release/aaha-template /bin/server
RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*
ENV TZ=Etc/UTC \
    USER=appuser
RUN groupadd ${USER} \
    && useradd -g ${USER} ${USER} && \
    chown -R ${USER}:${USER} /bin
USER ${USER}
ENTRYPOINT ["/bin/server"]
