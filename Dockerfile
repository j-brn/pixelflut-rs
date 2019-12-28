FROM rust:latest as builder
RUN rustup target add x86_64-unknown-linux-musl
COPY . .
RUN cargo build --release --target=x86_64-unknown-linux-musl
RUN strip target/x86_64-unknown-linux-musl/release/pixelflut

FROM scratch
COPY --from=builder target/x86_64-unknown-linux-musl/release/pixelflut /
CMD ["/pixelflut"]