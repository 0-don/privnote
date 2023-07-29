FROM messense/rust-musl-cross:x86_64-musl as builder
WORKDIR /app
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

# Create a new stage with a minimal image
FROM scratch
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/app /app
ENTRYPOINT ["/app"]
EXPOSE 3000
