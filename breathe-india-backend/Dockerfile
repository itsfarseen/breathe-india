ARG RUST_IMG=rust:1.51.0-alpine3.13

# TODO, merge all apk adds. put --no-cache in rust_base
# Don't wanna do now cuz got no time to rebuild entire thing on my slow laptop.

FROM ${RUST_IMG} as rust_base
RUN apk add --no-cache musl-dev
RUN apk add lld
RUN apk add bash

FROM rust_base as cargo_chef_base
ENV RUSTFLAGS="-Clink-arg=-fuse-ld=lld"
RUN cargo install cargo-chef

FROM cargo_chef_base as planner
WORKDIR app
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM cargo_chef_base as cacher
WORKDIR app
COPY --from=planner /app/recipe.json recipe.json
ENV RUSTFLAGS="-Clink-arg=-fuse-ld=lld"
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust_base as builder
WORKDIR app
# Copy over the cached dependencies
COPY --from=cacher /app/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME
COPY . .
ENV RUSTFLAGS="-Clink-arg=-fuse-ld=lld"
ENV SQLX_OFFLINE=true
RUN cargo build --release 

FROM alpine:3.13 as runtime
WORKDIR app
COPY --from=builder /app/target/release/breathe-india-backend .
COPY ./docker-entrypoint.sh .
ENTRYPOINT ["./docker-entrypoint.sh"]
