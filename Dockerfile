# https://github.com/emk/rust-musl-builder
FROM ekidd/rust-musl-builder AS builder

COPY . .

# Fix permissions on source code.
RUN sudo chown -R rust:rust /home/rust

RUN cargo build --release
RUN strip target/x86_64-unknown-linux-musl/release/revelio

# --

FROM alpine:latest
RUN apk --no-cache add ca-certificates
COPY --from=builder \
  /home/rust/src/target/x86_64-unknown-linux-musl/release/revelio \
  /usr/local/bin/
ENTRYPOINT [ "/usr/local/bin/revelio" ]
