FROM rust:1.86 AS rust_builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin api

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=rust_builder /app/target/release/api /app/api
EXPOSE 3000
CMD ["/app/api"]