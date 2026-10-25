FROM rust:1-bookworm AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
RUN useradd --system --uid 10001 streamverse
COPY --from=builder /app/target/release/streamverse /usr/local/bin/streamverse
USER streamverse
ENV PORT=3000
EXPOSE 3000
CMD ["streamverse"]
