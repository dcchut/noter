FROM ekidd/rust-musl-builder:latest AS builder

# Add our source code & compile
COPY . .
RUN cargo build --release

# Now build our _real_ Docker container
FROM alpine:latest
COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/dcc-template-bin \
    /usr/local/bin

CMD ["/usr/local/bin/dcc-template-bin"]