FROM rust:1.67-slim as build

RUN USER=root cargo new --bin boxes
WORKDIR /boxes

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/boxes*
RUN cargo build --release

FROM rust:1.67-slim
COPY --from=build /boxes/target/release/boxes .
CMD ["./boxes"]
