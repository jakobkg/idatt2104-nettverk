FROM rust
COPY . .
RUN cargo build --release
ENTRYPOINT cargo run --release