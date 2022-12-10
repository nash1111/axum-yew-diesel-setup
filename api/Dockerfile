FROM rust:1.65-bullseye
WORKDIR /api
COPY . .

RUN cargo build --release
RUN cargo install --path .

CMD ["api"]