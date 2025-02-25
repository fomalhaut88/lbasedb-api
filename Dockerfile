FROM rustlang/rust:nightly-slim

ENV DATA_PATH=./db
ENV HOST=0.0.0.0
ENV PORT=8080

WORKDIR /app
EXPOSE 8080
COPY . .

RUN cargo build --release
CMD ./target/release/lbasedb-api
