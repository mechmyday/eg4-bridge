FROM --platform=$BUILDPLATFORM rust:1.85 as builder
ARG TARGETPLATFORM
WORKDIR /usr/src/eg4-bridge
COPY Cargo.toml .
COPY Cargo.lock .
COPY .cargo .cargo
COPY src src
COPY db db
COPY ci ci
RUN ./ci/docker-buildx-platform-setup.sh $TARGETPLATFORM
RUN rustup target add $(cat /rust_target.txt)
RUN cargo install --path . --target $(cat /rust_target.txt)


FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/eg4-bridge /usr/local/bin/eg4-bridge
ENTRYPOINT ["eg4-bridge", "-c", "/etc/config.yaml"]
